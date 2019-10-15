use evmap::{self, WriteHandle};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn append(key: u64, value: u64, file_handle: &mut File, w: Arc<Mutex<WriteHandle<u64, u64>>>) {
    let mut buckets_w = w.lock().unwrap();

    file_handle.write(&key.to_be_bytes()).unwrap();
    file_handle.write(&value.to_be_bytes()).unwrap();
    buckets_w.insert(key, value);
    buckets_w.refresh();
}

pub fn read(key: u64, map: evmap::ReadHandle<u64, u64>) -> Option<Vec<u64>> {
    map.get_and(&key, |x| x.to_vec())
}

pub fn recover_from_uskv(filename: &str, w: Arc<Mutex<WriteHandle<u64, u64>>>) {
    let mut buckets_w = w.lock().unwrap();

    if !Path::new(filename).exists() {
        File::create(filename).unwrap_or_else(|_| panic!("Cannot create file."));
    }
    let mut key_store: [u8; 8] = [0; 8];
    let mut value_store: [u8; 8] = [0; 8];
    let mut accum: usize = 0;
    for data in fs::read(filename).expect("Error reading file for recovery") {
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
