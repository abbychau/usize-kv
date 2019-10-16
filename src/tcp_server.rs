use crate::uskv;
use crate::Reader;
use crate::Writer;
use may::net::TcpListener;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn start_server(
    buckets_r: Reader,
    writers: Arc<Mutex<Writer>>,
    fss: Arc<Mutex<::std::fs::File>>,
    frag_fss: Arc<Mutex<::std::fs::File>>,
    listener: TcpListener,
) -> JoinHandle<()> {
    let engine_master = uskv::Uskv {
        store_file: fss,
        fragment_file: frag_fss,
        read_handle: buckets_r,
        write_arc: writers,
    };

    let thread_handle = thread::spawn(move || {
        for stream in listener.incoming() {
            let mut c = [0 as u8; 24];
            let engine = engine_master.clone();
            go!(move || {
                let mut ms = stream.unwrap();
                ms.read(&mut c).unwrap();
                let key: [u8; 8] = [c[8], c[9], c[10], c[11], c[12], c[13], c[14], c[15]];
                let val: [u8; 8] = [c[16], c[17], c[18], c[19], c[20], c[21], c[22], c[23]];
                match c[7] {
                    0 => {
                        // non-blocking // victim-immune
                        let res = engine.read(u64::from_be_bytes(key));

                        if res.is_some() {
                            for item in res.unwrap() {
                                ms.write(&item.to_be_bytes()).unwrap();
                            }
                        }
                    }
                    1 => {
                        // need store lock
                        engine.append(u64::from_be_bytes(key), u64::from_be_bytes(val));
                        ms.write(&[0]).unwrap();
                    }
                    2 => {
                        // need store lock and fragment lock
                        engine.update(u64::from_be_bytes(key), u64::from_be_bytes(val));
                        ms.write(&[0]).unwrap();
                    }
                    3 => {
                        // stop the world
                        engine.purge();
                        ms.write(&[0]).unwrap();
                    }
                    _ => {
                        ms.write(&[1]).unwrap();
                    }
                }
            });
        }
    });
    thread_handle
}
