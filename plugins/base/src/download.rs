use std::rc::Rc;
use hyper_tls::HttpsConnector;
use hyper::{Client, StatusCode};
use futures::{Future as FutureTrait, Stream};
use imaginator::img;
use imaginator::filter::{Args, Future, ErrorResponse, Context};
use imaginator::prelude::*;
use imaginator::cfg::config;
use ::Config;

#[derive(PartialEq,Eq,Debug,Clone,Fail)]
#[fail(display="Url {} returned {}.", url, status_code)]
pub struct DownloadError {
    pub url: String,
    pub status_code: StatusCode
}

#[derive(PartialEq,Debug,Clone)]
pub struct DownloadResult {
    dpi: Option<f64>,
    buffer: Rc<Vec<u8>>
}

impl FilterResult for DownloadError {
    fn content_type(&self) -> Result<ContentType, Error> {
        Ok(ContentType::plaintext())
    }

    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn content(&self) -> Result<Rc<Vec<u8>>, Error> {
        Ok(Rc::new(format!("{}", self).into_bytes()))
    }
}

impl FilterResult for DownloadResult {
    fn content_type(&self) -> Result<ContentType, Error> {
        let mut image = img::Image::new(None, self.dpi)?;
        if image.ping(&*self.buffer).is_err() {
            return Ok(ContentType("application/octet-stream".parse().unwrap()))
        }
        let image_format = image.format()?;
        if let Some(ref formats) = config::<Config>().unwrap().image.supported_formats {
            if !formats.contains(&image_format) {
                bail!("Unsupported image format: {:?}", image_format);
            }
        }
        Ok(image.format()?.into())
    }

    fn content(&self) -> Result<Rc<Vec<u8>>, Error> {
        Ok(self.buffer.clone())
    }

    fn image(self: Box<Self>) -> Result<img::Image, Error> {
        let image = img::Image::new(&*self.buffer, self.dpi)?;
        let image_format = image.format()?;
        if let Some(ref formats) = config::<Config>().unwrap().image.supported_formats {
            if !formats.contains(&image_format) {
                bail!("Unsupported image format: {:?}", image_format);
            }
        }
        Ok(image)
    }
}

pub fn decode_url(url: &str) -> String {
    let mut split = url.splitn(2, ':');
    if let Some(domain) = config::<Config>().unwrap().domains.get(split.next().unwrap()) {
        let mut url = domain.clone();
        url.push_str(split.next().unwrap());
        url
    } else {
        url.to_owned()
    }
}

pub fn download_url(context: &Context, url: &str) -> Box<FutureTrait<Item = Vec<u8>, Error = Error>> {
    let url = url.to_owned();
    let handle = context.remote.handle().unwrap();
    let client = Client::configure().connector(HttpsConnector::new(1, &handle).unwrap()).build(&handle);
    let parsed_url = url.parse().into_future().from_err();
    let response = parsed_url.and_then(move |url| client.get(url).from_err());
    let body = response.and_then(move |res| {
        if res.status() != StatusCode::Ok {
            return Err(Error::from(
                ErrorResponse(Box::new(
                    DownloadError { url, status_code: res.status()}
                ))
            )).into_future();
        }
        Ok(res).into_future()
    }).and_then(|res| {
        res.body().concat2().from_err()
    });
    Box::new(body.map(move |body| body.to_vec()))
}

pub fn filter(context: &mut Context, args: &Args) -> Box<Future> {
    let url_arg = arg_type!(download, args, 0, String);
    context.log_filters_header.as_ref().map(|header_name|
        Rc::get_mut(&mut context.response_headers).map(|h|
            h.entry(header_name.clone()).and_modify(|value| {
                value.push_str("(");
                value.push_str(url_arg.splitn(2, ':').next().unwrap());
                value.push_str(")");
            })
        )
    );
    let url = decode_url(&url_arg);
    let dpi = if args.len() > 1 {
        Some(arg_type!(download, args, 1, isize) as f64)
    } else { None };
    let body = download_url(context, &url);
    let img = body.and_then(move |body| {
        let body = (&*body).to_vec();
        Ok(DownloadResult {
            dpi: dpi,
            buffer: body.into()
        })
    });
    Box::new(img.map(|img| Box::new(img).into()))
}
