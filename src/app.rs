use hyper::server::{Service, Request, Response};
use futures::{future, Future};
use tokio_core::reactor::{Handle, Remote};
use std::collections::HashMap;
use failure::Error;
use regex::Regex;
use chrono::prelude::*;
use crate::cfg::CONFIG;
use hyper;
use crate::url;
use crate::imaginator::filter::{self, FilterResult};
use std::rc::Rc;

type FilterMap = HashMap<&'static str, &'static (Fn(&mut filter::Context, &filter::Args) -> Box<filter::Future> + Sync)>;
lazy_static! {
    static ref FILTERS: FilterMap = {
        use imaginator_plugins;

        let mut map: FilterMap = HashMap::new();
        for (_, plugin) in imaginator_plugins::plugins().iter() {
            for (name, filter) in &plugin.filters {
                map.insert(name, filter.clone());
            }
        }
        map
    };
}

pub struct App {
    pub tokio_core: Handle,
}

fn apply_alias_args(mut filter: filter::Filter, args: &filter::Args) -> filter::Filter {
    lazy_static! {
        static ref RE_ARG: Regex = Regex::new(r"\{(\d+)\}").unwrap();
    }
    let mut new_args = Vec::with_capacity(filter.args.len());
    for arg in filter.args.into_iter() {
        match arg {
            filter::FilterArg::Img(filter) => new_args.push(filter::FilterArg::Img(apply_alias_args(filter, args))),
            filter::FilterArg::String(ref s) => if let Some(m) = RE_ARG.captures(s) {
                let arg_num: usize = m[1].parse().unwrap();
                new_args.push(args.get(arg_num).unwrap().clone())
            } else {
                // This clone() will potentially be unnecessary when non-lexical lifetimes are
                // introduced.
                new_args.push(arg.clone())
            },
            _ => new_args.push(arg)
        }
    }
    filter.args = new_args;
    filter
}

fn apply_filter_aliases(mut filter: filter::Filter) -> Result<filter::Filter, Error> {
    let mut new_args = Vec::with_capacity(filter.args.len());
    for arg in filter.args.into_iter() {
        match arg {
            filter::FilterArg::Img(filter) => new_args.push(filter::FilterArg::Img(apply_filter_aliases(filter)?)),
            _ => new_args.push(arg)
        }
    }
    filter.args = new_args;
    if let Some(value) = CONFIG.aliases.get(&filter.name) {
        let new_filter = apply_alias_args(url::parse(value)?, &filter.args);
        Ok(new_filter)
    } else {
        if CONFIG.allow_builtin_filters {
            Ok(filter)
        } else {
            bail!("no such filter: {}", filter.name);
        }
    }
}

pub fn exec_from_url(remote: &Remote, url: &str) -> Box<Future<Item = (Rc<HashMap<String, String>>, Box<FilterResult>), Error = Error>> {
    let filter = match url::parse(url).map_err(Error::from).and_then(apply_filter_aliases) {
        Ok(filter) => filter,
        Err(e) => return Box::new(future::err(e))
    };
    let mut context = filter::Context {
        filters: &FILTERS,
        remote: remote.clone(),
        log_filters_header: &CONFIG.log_filters_header,
        response_headers: Rc::new(HashMap::new())
    };
    Box::new(filter::exec_filter(&mut context, &filter).map(move |result| (context.response_headers, result)))
}

impl App {
    pub fn new(tokio_core: Handle) -> Self {
        App {
            tokio_core: tokio_core,
        }
    }
}

impl Service for App {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let remote = self.tokio_core.remote().clone();
        let mut url = req.uri().path()[1..].to_owned();
        if let Some(query) = req.uri().query() {
            url.push_str("?");
            url.push_str(query);
        }
        let log_req = format!("- - - [{}] \"{} {} {}\"",
            Utc::now().format("%d/%b/%Y:%H:%M:%S %z"), req.method(),
            req.uri(), req.version()
        );
        if req.method() != &hyper::Method::Get {
            return Box::new(future::ok(
                Response::new()
                    .with_status(hyper::StatusCode::MethodNotAllowed)
            ));
        }
        Box::new(exec_from_url(&remote, &url)
            .and_then(|(headers, img)| {
                Ok((headers, img.content_type()?, img.content()?))
            }).map(|(headers, content_type, body)| {
                let mut response = Response::new()
                   .with_header(hyper::header::ContentLength(body.len() as u64))
                   .with_header(content_type);
                {
                    let response_headers = response.headers_mut();
                    for (name, value) in headers.iter() {
                        response_headers.set_raw(name.clone(), value.as_str());
                    }
                }
                response.with_body(Rc::try_unwrap(body).unwrap())
            }).or_else(|err| {
                if let Some(error_response) = err.downcast_ref::<filter::ErrorResponse>() {
                    let error_response: &filter::FilterResult = error_response;
                    Ok(Response::new()
                        .with_status(error_response.status_code())
                        .with_body(Rc::try_unwrap(error_response.content().unwrap()).unwrap())
                    )
                } else {
                    let mut response = Response::new()
                        .with_body(format!("{}", err))
                        .with_header(hyper::header::ContentType::plaintext());
                    if err.downcast_ref::<url::UrlParseError>().is_some() {
                        response.set_status(hyper::StatusCode::BadRequest);
                    } else {
                        response.set_status(hyper::StatusCode::InternalServerError);
                    }
                    Ok(response)
                }
            }).map(move |response| { // Logging
                println!("{} {}", log_req, response.status().as_u16());
                response
            })
        )
    }
}
