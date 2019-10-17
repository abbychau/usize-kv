use std::io::prelude::*;
use std::net::TcpStream;

fn main()
{
    let mut stream = TcpStream::connect("127.0.0.1:9123").unwrap();

    match stream.write(&[0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,1,
    0,0,0,0,0,0,0,1,
    ]){
        Ok(_)=>{
            println!("sent");
        }
        Err(e)=>{
            println!("{}",e);
        }
    }
    println!("k");
    let mut buffer = [0u8;8];
    loop{
        println!("b4r");
        let data = stream.read(&mut buffer);
        print!("a4r");
        print!("{:?} ", buffer);
    }
}