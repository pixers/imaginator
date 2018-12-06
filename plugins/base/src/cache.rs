use std::sync::{Mutex, Once, ONCE_INIT, MutexGuard};
use std::collections::HashMap;
use imaginator::cfg::config;
use cfg::Config;
use failure::Error;
use futures::{future, Future as FutureTrait};
use imaginator::prelude::*;
use imaginator::filter::{FilterArg, Context, Args, Future, FilterResult, exec_filter};
use imaginator::img::Image;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::rc::Rc;
use byteorder::{ReadBytesExt, WriteBytesExt, NativeEndian};
use std::io::Write;
use serde_json;

use lru_cache::LruCache;

#[derive(Debug, Clone, Eq, PartialEq, Fail)]
#[fail(display="No such cache: {}", _0)]
struct NoSuchCache(String);

static INIT_CACHE: Once = ONCE_INIT;
static mut FILE_CACHE: Option<HashMap<String, Mutex<LruCache>>> = None;

pub fn cache(name: &str) -> Result<MutexGuard<'static, LruCache>, Error> {
    INIT_CACHE.call_once(|| {
        let mut map = HashMap::new();
        for (name, cache) in &config::<Config>().unwrap().caches {
            map.insert(name.clone(), 
                Mutex::new(LruCache::new(&cache.dir, cache.size).unwrap())
            );
        }
        unsafe { FILE_CACHE = Some(map); }
    });
    unsafe {
        Ok(FILE_CACHE.as_ref().unwrap() // This should never fail, because we initialize the FILE_CACHE above
           .get(name).ok_or_else(|| NoSuchCache(name.to_owned()))?
           .lock().unwrap() // If the mutex is poisoned, there's no sensible thing we can do anyway.
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CacheMetadata {
    content_type: String,
    dpi: Option<(f64, f64)>
}

#[derive(Debug)]
struct CacheEntry {
    metadata: CacheMetadata,
    buffer: Rc<Vec<u8>>
}

impl FilterResult for CacheEntry {
    fn content_type(&self) -> Result<ContentType, Error> {
        Ok(ContentType(self.metadata.content_type.parse().unwrap()))
    }

    fn dpi(&self) -> Result<(f64, f64), Error> {
        self.metadata.dpi.ok_or(format_err!("This filter does not return an image!"))
    }

    fn content(&self) -> Result<Rc<Vec<u8>>, Error> {
        Ok(self.buffer.clone())
    }

    fn image(self: Box<Self>) -> Result<Image, Error> {
        Image::new(&*self.buffer, self.metadata.dpi.map(|dpi| dpi.0)).into()
    }
}

fn cache_path(url: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(url);
    let hash_result = hasher.result_str();
    let mut path = String::new();
    path.push_str(&hash_result[0..2]);
    path.push_str("/");
    path.push_str(&hash_result[2..4]);
    path.push_str("/");
    path.push_str(&hash_result[4..6]);
    path.push_str("/");
    path.push_str(&hash_result[6..]);
    path
}

fn save(cache_name: &str, path: String, result: &Box<FilterResult>) -> Result<(), Error> {
    let metadata = CacheMetadata {
        content_type: format!("{}", result.content_type()?.0),
        dpi: result.dpi().ok()
    };
    let mut output: Vec<u8> = vec![];
    let meta = serde_json::to_string(&metadata)?;
    output.write_u32::<NativeEndian>(meta.len() as u32)?;
    output.write(meta.as_bytes())?;
    output.write(result.content()?.as_ref())?;
    cache(cache_name)?.insert_bytes(path, output.as_slice())?;
    Ok(())
}

fn get_cache_entry(cache_name: &str, params: &str) -> Result<CacheEntry, Error> {
    if let Ok(mut reader) = cache(&cache_name)?.get(params.clone()) {
        let size = reader.read_u32::<NativeEndian>()?;
        let mut json_buf = vec![0; size as usize];
        reader.read_exact(json_buf.as_mut_slice())?;
        let content_type: CacheMetadata = serde_json::from_reader(json_buf.as_slice())?;
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;
        Ok(CacheEntry {
            metadata: content_type,
            buffer: Rc::new(buf)
        })
    } else {
        Err(format_err!("Entry not in cache!"))
    }
}

fn filter_result(context: &mut Context, cache_name: String, args: &Args) -> Result<Box<Future>, Error> {
    if let Some(&FilterArg::Img(ref filter)) = args.get(0) {
        let params = cache_path(&format!("{:?}", args[0]));

        if let Ok(entry) = get_cache_entry(&cache_name, &params) {
            Ok(Box::new(future::ok(Box::new(entry).into())))
        } else {
            Ok(Box::new(exec_filter(context, filter).map(move |img| {
                save(&cache_name, params, &img).unwrap_or_else(|e| eprintln!("{}", e));
                img.into()
            })))
        }
    } else {
        bail!("Argument 1 to `cache` must be an image")
    }
}

pub fn filter(context: &mut Context, args: &Args) -> Box<Future> {
    let cache_name = arg_type!(cache, args, 1, String);
    context.log_filters_header.as_ref().map(|header_name|
        context.response_headers.entry(header_name.clone()).and_modify(|value| {
            value.push_str("(");
            value.push_str(cache_name.as_str());
            value.push_str(")");
        })
    );
    match filter_result(context, cache_name, args) {
        Ok(future) => future,
        Err(err) => Box::new(future::err(err))
    }
}
