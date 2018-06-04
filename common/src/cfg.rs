use std::collections::HashMap;
use std::any::{TypeId, Any};

pub static mut CONFIG: Option<HashMap<TypeId, Box<Any>>> = None;

pub fn config<T: 'static>() -> Option<&'static T> {
    unsafe {
        CONFIG.as_ref().and_then(|cfg|
            cfg.get(&TypeId::of::<T>()).and_then(|val| val.downcast_ref())
        )
    }
}
