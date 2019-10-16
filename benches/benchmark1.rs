#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn test1(n: u64) {
    for _ in 0..1 {
        match TcpStream::connect("127.0.0.1:9123") {
            Ok(mut stream) => {
                //println!("Successfully connected to server in port 9123");

                stream
                    .write(&[
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 15, 0, 0, 0, 0, 0, 0, 15,
                    ])
                    .unwrap();

                let mut data = [0 as u8; 24]; // using 6 byte buffer
                match stream.read(&mut data[..]) {
                    Ok(_) => {
                        //println!("Reply is {:?}",&data);
                    }
                    Err(e) => {
                        //println!("Failed to receive data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
    //println!("Terminated.");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test1 20", |b| b.iter(|| test1(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
