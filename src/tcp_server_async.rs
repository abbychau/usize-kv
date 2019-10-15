use crate::uskv;
use evmap::{self, ReadHandle, WriteHandle};
use may::net::TcpListener;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn start_server(
    buckets_r: ReadHandle<u64, u64>,
    writers: Arc<Mutex<WriteHandle<u64, u64>>>,
    fss: Arc<Mutex<::std::fs::File>>,
    frag_fss: Arc<Mutex<::std::fs::File>>,
    listener: TcpListener,
) -> JoinHandle<()> {
    let writers_clone = writers.clone();
    let fss_clone = fss.clone();
    let frag_fss_clone = frag_fss.clone();
    let thread_handle = thread::spawn(move || {
        for stream in listener.incoming() {
            let mut c = [0 as u8; 24];
            let br = buckets_r.clone();
            let bw = writers_clone.clone();
            let fs = fss_clone.clone();
            let frag_fs = frag_fss_clone.clone();
            go!(move || {
                let mut ms = stream.unwrap();
                ms.read(&mut c).unwrap();
                //let ops : [u8;8] = [c[0],c[1],c[2],c[3],c[4],c[5],c[6],c[7]];
                let key: [u8; 8] = [c[8], c[9], c[10], c[11], c[12], c[13], c[14], c[15]];
                let val: [u8; 8] = [c[16], c[17], c[18], c[19], c[20], c[21], c[22], c[23]];
                match c[7] {
                    0 => {
                        let res = uskv::read(u64::from_be_bytes(key), br);

                        if res.is_some() {
                            for item in res.unwrap() {
                                ms.write(&item.to_be_bytes()).unwrap();
                            }
                        }
                    }
                    1 => {
                        uskv::append(
                            u64::from_be_bytes(key),
                            u64::from_be_bytes(val),
                            &mut fs.lock().unwrap(),
                            bw,
                        );
                        ms.write(b"0").unwrap();
                    }
                    2 => {
                        uskv::update(
                            u64::from_be_bytes(key),
                            u64::from_be_bytes(val),
                            (&mut fs.lock().unwrap(), &mut frag_fs.lock().unwrap()),
                            bw,
                        );
                        ms.write(b"0").unwrap();
                    }
                    _ => {}
                }
            });
        }
    });
    thread_handle
}
