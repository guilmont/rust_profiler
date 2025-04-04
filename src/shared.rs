#![allow(dead_code)]
use std::sync::{Arc, Mutex, MutexGuard};

pub struct SharedObject<T> {
    inner: Arc<Mutex<T>>
}

impl<T> SharedObject<T> {
    pub fn new(value: T) -> Self {
        Self { inner: Arc::new(Mutex::new(value)) }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock().expect("Failed lock object!")
    }

    pub fn with_locked_mut_ref<F> (&self,func: F)
        where F: FnOnce(&mut T)  {
        func(&mut self.lock());
    }

    pub fn set_value(&self,val: T)  {
        *self.lock() = val;
    }
}

impl<T>  Clone for SharedObject<T> {
    fn clone(&self) -> Self {
        SharedObject { inner: self.inner.clone() }
    }
}
