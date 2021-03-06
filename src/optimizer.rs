//use crate::Reader;
use crate::Writer;
use std::sync::{Arc, Mutex};

// pub fn optimize(_w: Arc<Mutex<Writer>>) {}

pub fn start_optimizer(w: Arc<Mutex<Writer>>) {
    ::std::thread::spawn(move || {
        w.lock().unwrap().fit_all();
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    });
}
