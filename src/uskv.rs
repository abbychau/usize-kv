use evmap::{self, WriteHandle};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::Reader;
use crate::Writer;

pub struct Uskv {
    pub store_file: Arc<Mutex<::std::fs::File>>,
    pub fragment_file: Arc<Mutex<::std::fs::File>>,
    pub read_handle: Reader,
    pub write_arc: Arc<Mutex<Writer>>,
}
impl Uskv {
    pub fn append(&self, key: u64, value: u64) {
        {
            let mut slock = self.store_file.lock().unwrap();
            slock.write(&key.to_be_bytes()).unwrap();
            slock.write(&value.to_be_bytes()).unwrap();
        }
        {
            let mut buckets_w = self.write_arc.lock().unwrap();
            buckets_w.insert(key, value);
            buckets_w.refresh();
        }
    }

    pub fn read(&self, key: u64) -> Option<Vec<u64>> {
        self.read_handle.get_and(&key, |x| x.to_vec())
    }

    pub fn update(&self, key: u64, value: u64) {
        {
            let mut slock = self.store_file.lock().unwrap();
            slock.write(&key.to_be_bytes()).unwrap();
            slock.write(&value.to_be_bytes()).unwrap();
        }

        let mut flock = self.fragment_file.lock().unwrap();
        flock.write(&key.to_be_bytes()).unwrap();
        drop(flock);
        let mut buckets_w = self.write_arc.lock().unwrap();
        buckets_w.insert(key, value);
        buckets_w.refresh();
    }

    pub fn removekey() {
        //TODO:
    }

    pub fn removeItemsByValue() {
        //TODO:
    }

    pub fn purge(&self) {
        let mut buckets_w = self.write_arc.lock().unwrap();
        let slock = self.store_file.lock().unwrap();
        let flock = self.fragment_file.lock().unwrap();
        slock.set_len(0).unwrap();
        flock.set_len(0).unwrap();
        buckets_w.purge();
        buckets_w.refresh();
    }

    pub fn recover_from_uskv(
        store_path: &str,
        fragment_path: &str,
        w: Arc<Mutex<Writer>>,
    ) {
        let mut buckets_w = w.lock().unwrap();

        if !Path::new(store_path).exists() {
            File::create(store_path).unwrap_or_else(|_| panic!("Cannot create store file."));
        }
        if !Path::new(fragment_path).exists() {
            File::create(fragment_path).unwrap_or_else(|_| panic!("Cannot create fragment file."));
        }
        let mut key_store: [u8; 8] = [0; 8];
        let mut value_store: [u8; 8] = [0; 8];
        let mut accum: usize = 0;
        for data in fs::read(store_path).expect("Error reading file for recovery") {
            if accum <= 7 {
                key_store[accum] = data;
            } else if accum <= 15 {
                value_store[accum - 8] = data;
            }
            if accum == 15 {
                buckets_w.insert(
                    u64::from_be_bytes(key_store),
                    u64::from_be_bytes(value_store),
                );
                accum = 0;
            } else {
                accum += 1;
            }
        }
        buckets_w.refresh();
    }
}
