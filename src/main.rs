#[macro_use]
extern crate may;

use evmap::{self, ReadHandle, WriteHandle};
use may::net::TcpListener;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};

mod cli_util;
mod tcp_server;
mod uskv;

fn main() {
    let (host, filename) = cli_util::set_opts_get_opts();
    cli_util::print_banner();

    let (buckets_r, buckets_w): (ReadHandle<u64, u64>, WriteHandle<u64, u64>) = evmap::new();
    let writers = Arc::new(Mutex::new(buckets_w));

    uskv::recover_from_uskv(&filename, writers.clone());

    let listener = TcpListener::bind(host.clone()).unwrap();
    println!("Listening started at host : {}", host);

    let store_file = OpenOptions::new().append(true).open(&filename).unwrap();
    let fss = Arc::new(Mutex::new(store_file));
    let thread_handle = tcp_server::start_server(buckets_r, writers, fss, listener);
    println!("Press Ctrl+C to stop");
    thread_handle.join().unwrap();
}
