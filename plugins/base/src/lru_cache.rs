use linked_hash_map::LinkedHashMap;
use failure::Error;
use std::path::PathBuf;
use std::fs::{File, create_dir_all};
use std::io::{Read, Seek, Write};
use bincode::{deserialize_from, serialize_into};

pub trait ReadSeek: Read + Seek + Send {}
impl<T: Read + Seek + Send> ReadSeek for T {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Fail)]
#[fail(display="Data is too big to insert into the cache. {} bytes > {} bytes of capacity", size, capacity)]
struct DataTooBigError {
    pub size: usize,
    pub capacity: usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Fail)]
#[fail(display="The file is not in cache")]
struct NotInCache {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LruCache {
    root: PathBuf,
    size: usize,
    capacity: usize,
    items: LinkedHashMap<String, usize>
}

impl LruCache {
    pub fn size(&self) -> usize { self.size }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn len(&self) -> usize { self.items.len() }

    pub fn new<P: Into<PathBuf>>(root: P, capacity: usize) -> Result<LruCache, Error> {
        let root = root.into();
        if let Some(imported) = LruCache::import(&root)? {
            return Ok(imported);
        }

        let mut cache = LruCache {
            root: root,
            size: 0,
            capacity: capacity,
            items: LinkedHashMap::new()
        };

        if cache.root.exists() {
            let path = cache.root.clone();
            cache.load(path)?;
        } else {
            create_dir_all(&cache.root)?;
        }
        Ok(cache)
    }

    fn load(&mut self, path: PathBuf) -> Result<(), Error> {
        let root_path_length = self.root.to_str().unwrap().len();
        for entry in path.read_dir()? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                self.load(entry.path())?;
            } else {
                // We do not support non-utf8 paths, hence the unwrap below().
                let entry_name = entry.path().to_str().unwrap()[root_path_length..].to_owned();
                self.size += metadata.len() as usize;
                self.items.insert(entry_name, metadata.len() as usize);
            }
        }
        Ok(())
    }

    pub fn import<P: Into<PathBuf>>(root: P) -> Result<Option<LruCache>, Error> {
        let mut path = root.into().to_str().unwrap().to_owned();
        path.push_str(".cache");
        if !PathBuf::from(&path).exists() {
            return Ok(None)
        }
        Ok(Some(deserialize_from(File::open(path)?)?))
    }

    pub fn export(&self) -> Result<(), Error> {
        let mut path = self.root.to_str().unwrap().to_owned();
        path.push_str(".cache");
        let file = File::create(path)?;
        serialize_into(file, self)?;
        Ok(())
    }

    pub fn get(&mut self, name: &str) -> Result<Box<ReadSeek>, Error> {
        if self.items.get_refresh(name).is_some() {
            let mut path = self.root.clone();
            path.push(name);
            let file = File::open(path)?;
            Ok(Box::new(file) as Box<ReadSeek>)
        } else {
            Err(NotInCache {}.into())
        }
    }

    pub fn insert_bytes(&mut self, name: String, value: &[u8]) -> Result<(), Error> {
        if value.len() > self.capacity {
            return Err(DataTooBigError { size: value.len(), capacity: self.capacity }.into())
        }

        if self.size + value.len() > self.capacity {
            self.items.pop_front().map(|(_, size)| {
                self.size -= size;
            });
        }

        let mut path = self.root.clone();
        path.push(&name);
        create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        file.write_all(value)?;
        self.items.insert(name, value.len());

        Ok(())
    }

    pub fn iter(&self) -> linked_hash_map::Iter<String, usize> {
        self.items.iter()
    }

    pub fn keys(&self) -> linked_hash_map::Keys<String, usize> {
        self.items.keys()
    }

    pub fn values(&self) -> linked_hash_map::Values<String, usize> {
        self.items.values()
    }
}
