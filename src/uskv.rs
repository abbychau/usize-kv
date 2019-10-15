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

pub fn update(
    key: u64,
    value: u64,
    file_handle: (&mut File, &mut File),
    w: Arc<Mutex<WriteHandle<u64, u64>>>,
) {
    let mut buckets_w = w.lock().unwrap();

    file_handle.0.write(&key.to_be_bytes()).unwrap();
    file_handle.0.write(&value.to_be_bytes()).unwrap();
    file_handle.1.write(&key.to_be_bytes()).unwrap();
    buckets_w.insert(key, value);
    buckets_w.refresh();
}

pub fn recover_from_uskv(store_path: &str,fragment_path: &str, w: Arc<Mutex<WriteHandle<u64, u64>>>) {
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
