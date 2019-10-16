#[macro_use]
extern crate may;

use may::net::TcpListener;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};

mod cli_util;
mod optimizer;
mod tcp_server;
mod types;
mod uskv;

use types::{HasherType, Reader, Writer};
use uskv::Uskv;

fn main() {
    let (host, store_path, fragment_path) = cli_util::set_opts_get_opts();
    cli_util::print_banner();

    let (buckets_r, buckets_w): (Reader, Writer) = evmap::Options::default()
        .with_hasher(HasherType::default())
        .construct::<u64, u64>();

    let arc_writer = Arc::new(Mutex::new(buckets_w));

    Uskv::recover_from_uskv(&store_path, &fragment_path, arc_writer.clone());

    let listener = TcpListener::bind(host.clone()).unwrap();
    println!("Listening started at host : {}", host);

    let store_file = OpenOptions::new().append(true).open(&store_path).unwrap();
    let fss = Arc::new(Mutex::new(store_file));
    let fragment_file = OpenOptions::new()
        .append(true)
        .open("fragment.uskv")
        .unwrap();
    let frag_fss = Arc::new(Mutex::new(fragment_file));
    let thread_handle =
        tcp_server::start_server(buckets_r, arc_writer.clone(), fss, frag_fss, listener);
    optimizer::start_optimizer(arc_writer);
    println!("Press Ctrl+C to stop");
    thread_handle.join().unwrap();
}
