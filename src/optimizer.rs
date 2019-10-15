use evmap::{self, WriteHandle};
use std::sync::{Arc, Mutex};

pub fn optimize(w: Arc<Mutex<WriteHandle<u64, u64>>>) {}

pub fn start_optimizer(w: Arc<Mutex<WriteHandle<u64, u64>>>) {
    ::std::thread::spawn(move || {
        w.lock().unwrap().fit_all();
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    });
}
