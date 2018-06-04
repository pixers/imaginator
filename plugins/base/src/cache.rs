use std::sync::{Mutex, Once, ONCE_INIT, MutexGuard};
use std::collections::HashMap;
use lru_disk_cache::LruDiskCache;
use imaginator::cfg::config;
use cfg::Config;
use failure::Error;
use futures::{future, Future as FutureTrait};
use imaginator::prelude::*;
use imaginator::filter::{FilterArg, Context, Args, Future, FilterResult, exec_filter};
use imaginator::img::Image;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs::{File};
use std::rc::Rc;
use serde_json;

static INIT_CACHE: Once = ONCE_INIT;
static mut FILE_CACHE: Option<HashMap<String, Mutex<LruDiskCache>>> = None;

fn cache(name: &str) -> Result<MutexGuard<'static, LruDiskCache>, Error> {
    INIT_CACHE.call_once(|| {
        let mut map = HashMap::new();
        for (name, cache) in &config::<Config>().unwrap().caches {
            map.insert(name.clone(), 
                Mutex::new(LruDiskCache::new(&cache.dir, cache.size).unwrap())
            );
        }
        unsafe { FILE_CACHE = Some(map); }
    });
    unsafe {
        Ok(FILE_CACHE.as_ref().unwrap().get(name).unwrap().lock().unwrap())
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

    fn content(&self) -> Rc<Vec<u8>>
    {
        self.buffer.clone()
    }

    fn image(self: Box<Self>) -> Result<Image, Error> {
        Image::new(&*self.buffer, self.metadata.dpi.map(|dpi| dpi.0)).into()
    }
}

fn cache_path(url: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(url);
    let hash_result = hasher.result_str();
    let (a, b) = hash_result.split_at(2);
    a.to_owned() + "/" + b
}

fn save(cache_name: &str, path: String, result: &Box<FilterResult>) -> Result<(), Error> {
    let mut meta_name = cache(&cache_name)?.path().to_str().unwrap().to_owned();
    meta_name.push_str("/");
    meta_name.push_str(&path);
    meta_name.push_str(".meta");
    let metadata = CacheMetadata {
        content_type: format!("{}", result.content_type()?.0),
        dpi: result.dpi().ok()
    };
    cache(cache_name)?.insert_bytes(path, result.content().as_ref())?;
    serde_json::to_writer(File::create(meta_name)?, &metadata)?;
    Ok(())
}

fn filter_result(context: &Context, cache_name: String, args: &Args) -> Result<Box<Future>, Error> {
    if let Some(&FilterArg::Img(ref filter)) = args.get(0) {
        let params = cache_path(&format!("{:?}", args[0]));

        let mut meta_name = cache(&cache_name)?.path().to_str().unwrap().to_owned();
        meta_name.push_str("/");
        meta_name.push_str(&params);
        meta_name.push_str(".meta");
        if let Ok(mut reader) = cache(&cache_name)?.get(params.clone()) {
            let content_type: CacheMetadata = serde_json::from_reader(File::open(meta_name)?)?;
            let mut buf = vec![];
            reader.read_to_end(&mut buf)?;
            Ok(Box::new(future::ok(Box::new(CacheEntry {
                metadata: content_type,
                buffer: Rc::new(buf)
            }).into())))
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

pub fn filter(context: &Context, args: &Args) -> Box<Future> {
    let cache_name = arg_type!(cache, args, 1, String);
    match filter_result(context, cache_name, args) {
        Ok(future) => future,
        Err(err) => Box::new(future::err(err))
    }
}
