#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, MutexGuard};

type SharedObject<T> = Arc<Mutex<T>>;

#[cfg(feature = "profiler")] #[macro_export]
macro_rules! profile_scope {
    ($id: expr) => {
        let identifier = $id.to_string();
        let _obj = crate::profiler::Item::new(identifier.clone());
        crate::profiler::register(identifier);
    }
}

#[cfg(feature = "profiler")] #[macro_export]
macro_rules! profiler_summary { () => { crate::profiler::summary(); } }


#[cfg(not(feature = "profiler"))] #[macro_export]
macro_rules! profiler_summary { () => {} }

#[cfg(not(feature = "profiler"))] #[macro_export]
macro_rules! profile_scope { ($id: expr) => {} }


//////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////

pub struct Item {
    identifier: String,
    start: std::time::SystemTime
}

impl Item {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            start: std::time::SystemTime::now()
        }
    }
}
impl Drop for Item {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().unwrap();
        let mut obj = get_registry();
        let meta = obj.table.get_mut(&self.identifier).unwrap();
        meta.count += 1;
        meta.elapsed += elapsed;
    }
}

////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Metadata {
    identifier: String,
    elapsed: std::time::Duration,
    count: usize,
}

impl Metadata {
    fn new(identifier: String) -> Self {
        Self {
            identifier,
            elapsed: std::time::Duration::ZERO,
            count:0,
        }
    }
}

////////////////////////////////////////////////////////////

struct Registry  {
    table: HashMap<String, Metadata>
}

// Kind of a sungleton so we can profile the whole application
static GLOBAL_REGISTRY: OnceLock<SharedObject<Registry>> = OnceLock::new();
fn get_registry() -> MutexGuard<'static, Registry> {
    let obj = GLOBAL_REGISTRY.get_or_init(|| Arc::new(Mutex::new(Registry{ table: HashMap::new() })));
    obj.lock().expect("Failed to lock profiler registry!")
}

pub fn register(identifier: String) {
    get_registry().table.entry(identifier.clone()).or_insert(Metadata::new(identifier));
}

pub fn summary() {
    let table = &get_registry().table;
    for (key, meta) in table {
        println!("{}: count = {} -- time = {:?}", key, meta.count, meta.elapsed);
    }
}
