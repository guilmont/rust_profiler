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
        let mut reg = get_registry();
        {
            // Update meta data information
            let mut meta = reg.table.get(&self.identifier).unwrap().lock().unwrap();
            meta.count += 1;
            meta.elapsed += elapsed;
        }{
            // Remove metadata from stack
            reg.stack.pop();
        }
    }
}

////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Metadata {
    identifier: String,
    elapsed: std::time::Duration,
    count: usize,
    next: Option<SharedObject<Metadata>>,
}

impl Metadata {
    fn new(identifier: String) -> Self {
        Self {
            identifier,
            elapsed: std::time::Duration::ZERO,
            count:0,
            next: None,
        }
    }
}

////////////////////////////////////////////////////////////

struct Registry  {
    table: HashMap<String, SharedObject<Metadata>>,
    ordered: Vec<SharedObject<Metadata>>,
    stack: Vec<SharedObject<Metadata>>
}

// Kind of a sungleton so we can profile the whole application
static GLOBAL_REGISTRY: OnceLock<SharedObject<Registry>> = OnceLock::new();
fn get_registry() -> MutexGuard<'static, Registry> {
    let reg = Registry {
        table: HashMap::new(),
        ordered: Vec::new(),
        stack: Vec::new()
    };
    let obj = GLOBAL_REGISTRY.get_or_init(|| Arc::new(Mutex::new(reg)));
    obj.lock().expect("Failed to lock profiler registry!")
}

pub fn register(identifier: String) {
    let meta = Arc::new(Mutex::new(Metadata::new(identifier.clone())));
    let mut obj = get_registry();

    use std::collections::hash_map::Entry;
    match obj.table.entry(identifier) {
        Entry::Occupied(_) => {}
        Entry::Vacant(elem) => {
            elem.insert(meta.clone());
            if obj.stack.is_empty() {
                obj.ordered.push(meta.clone());
            } else {
                let last = obj.stack.last().unwrap();
                last.lock().unwrap().next = Some(meta.clone());
            }
            obj.stack.push(meta);
        }
    }
}

pub fn summary() {
    for meta in get_registry().ordered.iter() {
        let mut spc : usize = 0;
        let mut curr = meta.clone();
        loop {
            // Lock current element
            let elem = curr.lock().unwrap();
            println!("{:width$}{}: count = {} -- time = {:?}",
                     "", elem.identifier, elem.count, elem.elapsed, width = spc);
            spc += 4;

            match &elem.next {
                Some(next_arc) => {
                    let next_elem = next_arc.clone();
                    drop(elem);
                    curr = next_elem;
                },
                None => break,
            }

        }
    }
}
