use getopts::Options;
use std::env;
use std::process;

pub fn print_banner() {
    println!(
        r#"
           _               _            
 _   _ ___(_)_______      | | ____   __ 0001 FFFF 0001 <- 1
| | | / __| |_  / _ \_____| |/ /\ \ / / 0000 FFFF 0000 -> 1
| |_| \__ \ |/ /  __/_____|   <  \ V /  0001 0001 000F <- F
 \__,_|___/_/___\___|     |_|\_\  \_/   0000 0001 0000 -> F
"#
    );
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn set_opts_get_opts() -> (String, String, String) {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "port", "set port of server (default: 9123)", "PORT");
    opts.optopt(
        "s",
        "store",
        "set uskv storage path (default: ./store.uskv)",
        "FILE",
    );
    opts.optopt(
        "f",
        "fragment",
        "set uskv fragment path (default: ./fragment.uskv)",
        "FRAGMENT",
    );
    opts.optflag("h", "help", "print this help menu");
    let program = args[0].clone();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        ::std::process::exit(0);
    }

    let host = format!(
        "127.0.0.1:{}",
        match matches.opt_str("p") {
            Some(p) => p,
            None => {
                println!("Using default port : 9123 .");
                "9123".to_string()
            }
        }
    );
    let filename = match matches.opt_str("s") {
        Some(p) => p,
        None => {
            println!("Using default store file path, 'store.uskv' .");
            "store.uskv".to_string()
        }
    };
    let fragment = match matches.opt_str("f") {
        Some(p) => p,
        None => {
            println!("Using default fragment file path, 'fragment.uskv' .");
            "fragment.uskv".to_string()
        }
    };
    println!("Process ID: {}", process::id());


    (host, filename, fragment)
}
