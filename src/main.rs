use evmap::{self, ReadHandle, WriteHandle};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use std::net::{TcpListener};
use std::path::Path;
use std::thread;
fn main() {
    let (buckets_r, buckets_w): (ReadHandle<u64, u64>, WriteHandle<u64, u64>) = evmap::new();
    let writers = Arc::new(Mutex::new(buckets_w));

    let filename = "store.uskv";
    recover_from_uskv(filename, writers.clone());

    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");

    let store_file = OpenOptions::new().append(true).open(filename).unwrap();
    let fss = Arc::new(Mutex::new(store_file));

    let writers_clone = writers.clone();
    let fss_clone = fss.clone();
    let thread_handle = thread::spawn(move || {
        for stream in listener.incoming() {
            let mut c = [0 as u8; 24];
            let br = buckets_r.clone();
            let bw = writers_clone.clone();
            let fs = fss_clone.clone();
            thread::spawn(move || {
                let mut ms = stream.unwrap();
                ms.read(&mut c).unwrap();
                //let ops : [u8;8] = [c[0],c[1],c[2],c[3],c[4],c[5],c[6],c[7]];
                let key: [u8; 8] = [c[8], c[9], c[10], c[11], c[12], c[13], c[14], c[15]];
                let val: [u8; 8] = [c[16], c[17], c[18], c[19], c[20], c[21], c[22], c[23]];

                if c[7] == 0 {
                    let res = read(u64::from_be_bytes(key), br);

                    if res.is_some() {
                        for item in res.unwrap() {
                            ms.write(&item.to_be_bytes()).unwrap();
                        }
                    }
                } else {
                    
                    append(u64::from_be_bytes(key), u64::from_be_bytes(val), &mut fs.lock().unwrap(), bw);
                    ms.write(b"0").unwrap();
                }
            });
        }
    });
    thread_handle.join();
    //append(u64::max_value(), 200000, &mut store_file, &mut buckets_w);
}

fn append(key: u64, value: u64, file_handle: &mut File, w: Arc<Mutex<WriteHandle<u64, u64>>>) {
    let mut buckets_w = w.lock().unwrap();

    file_handle.write(&key.to_be_bytes()).unwrap();
    file_handle.write(&value.to_be_bytes()).unwrap();
    buckets_w.insert(key, value);
    buckets_w.refresh();
}

fn read(key: u64, map: evmap::ReadHandle<u64, u64>) -> Option<Vec<u64>> {
    map.get_and(&key, |x| x.to_vec())
}

fn recover_from_uskv(filename: &str, w: Arc<Mutex<WriteHandle<u64, u64>>>) {
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
