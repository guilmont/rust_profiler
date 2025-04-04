#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::shared::SharedObject;

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
        get_registry().with_locked_mut_ref(|obj| {
            let meta = obj.table.get_mut(&self.identifier).unwrap();
            meta.count += 1;
            meta.elapsed += elapsed;
        });
    }
}


//////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////

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


struct Registry  {
    table: HashMap<String, Metadata>
}


// Kind of a sungleton so we can profile the whole application
static GLOBAL_REGISTRY: OnceLock<SharedObject<Registry>> = OnceLock::new();
fn get_registry() -> &'static SharedObject<Registry> {
    GLOBAL_REGISTRY.get_or_init(|| SharedObject::new(Registry{ table: HashMap::new() }))
}

pub fn register(identifier: String) {
    get_registry().with_locked_mut_ref(|obj| {
        obj.table.entry(identifier.clone()).or_insert(Metadata::new(identifier));
    });
}


pub fn summary() {
    println!("{:?}", get_registry().lock().table);
}


//////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "profiler")]
#[macro_export]
macro_rules! profile_scope {
    ($id: expr) => {
        let identifier = $id.to_string();
        let _obj = crate::profiler::Item::new(identifier.clone());
        crate::profiler::register(identifier);
    };
}

#[cfg(not(feature = "profiler"))]
#[macro_export]
macro_rules! profile_scope {
    ($id: expr) => {};
}
