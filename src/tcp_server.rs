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
            //println!("income");
            go!(move || {
                let mut ms = stream.unwrap();
                //ms.set_read_timeout(Some(::std::time::Duration::from_millis(1000))).unwrap();
                //let mut out = 0;
                loop {
                    if let Err(_) = ms.read_exact(&mut c) {
                        break;
                    }
                    let key: [u8; 8] = [c[8], c[9], c[10], c[11], c[12], c[13], c[14], c[15]];
                    let val: [u8; 8] = [c[16], c[17], c[18], c[19], c[20], c[21], c[22], c[23]];
                    match c[7] {
                        0 => {//READ
                            // non-blocking // victim-immune
                            let res = engine.read(u64::from_be_bytes(key));

                            if res.is_some() {
                                let mut out_store: Vec<u8> = Vec::new();
                                for item in res.unwrap() {
                                    out_store.extend_from_slice(&item.to_be_bytes());
                                }
                                //out+=1;
                                // print!("[{:?}]",&out_store.len());
                                if let Err(_) = ms.write(&out_store){
                                    break;
                                }
                            }
                        }
                        1 => {//APPEND
                            // need store lock
                            engine.append(u64::from_be_bytes(key), u64::from_be_bytes(val));
                            if let Err(_) = ms.write(&[0]){
                                break;
                            }
                        }
                        2 => {//UPDATE
                            // need store lock and fragment lock
                            engine.update(u64::from_be_bytes(key), u64::from_be_bytes(val));
                            match ms.write(&[0]){
                                Ok(_o)=>{},
                                Err(_e)=>{}
                            }
                        }
                        3 => {//REMOVE
                            engine.remove(u64::from_be_bytes(key), u64::from_be_bytes(val));
                            ms.write(&[0]).unwrap();
                        }
                        4 => {
                            // stop the world
                            engine.purge();
                            ms.write(&[0]).unwrap();
                        }
                        5 => {
                            engine.read_handle.for_each(|k, vs| {
                                println!("{:?} : {:?}", k, vs);
                            });
                        }
                        9 => {
                            engine.optimize_store_file();
                            ms.write(&[0]).unwrap();
                        }
                        _ => {
                            ms.write(&[255, 255]).unwrap();
                        }
                    }
                }
                //println!("A Client Connection is Ended");
            });
        }
    });
    thread_handle
}
