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
    next: Vec<SharedObject<Metadata>>,
}

impl Metadata {
    fn new(identifier: String) -> Self {
        Self {
            identifier,
            elapsed: std::time::Duration::ZERO,
            count:0,
            next: Vec::new(),
        }
    }

    fn fmt_with_identation(&self, f: &mut std::fmt::Formatter<>, indent: usize) -> std::fmt::Result {
        let mut prefix = "";
        let mut step = 1;
        let mut dw = 0;
        if indent > 0 {
            prefix = "\x1b[93m└─\x1b[0m ";
            step = 2;
            dw = 9;
        }

        let id = " ".repeat(indent) + &prefix + "\x1b[1;36m" + &self.identifier + "\x1b[0m";
        writeln!(f, "{:<width$} {:>5}     {:?}", id, self.count, self.elapsed, width = 41 + dw)?;

        for elem in self.next.iter() {
            elem.lock().unwrap().fmt_with_identation(f, indent + step)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for Metadata {
    fn fmt(&self,f: &mut std::fmt::Formatter<>) -> std::fmt::Result {
        self.fmt_with_identation(f, 0)
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
                last.lock().unwrap().next.push(meta.clone());
            }
            obj.stack.push(meta);
        }
    }
}

pub fn summary() {
    println!("{:<43} {}    {}", "\x1b[1;37mFunction:\x1b[0m", "\x1b[1;37mCount:\x1b[0m", "\x1b[1;37mTime:\x1b[0m");
    for meta in get_registry().ordered.iter() {
        println!("{}", meta.lock().unwrap());

    }
}
