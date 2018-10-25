#![feature(try_blocks)]


extern crate malvi_step8 as malvi;

use std::io::{BufRead};
use self::malvi::{Malvi,Mal};

fn main() -> malvi::Result<()> {
    let si = ::std::io::stdin();
    let mut si = si.lock();
    let mut line = "".to_string();
    let mut p = Malvi::new();
    loop {
        line.clear();
        if 0 == si.read_line(&mut line)? {
            break;
        }
        let ret : malvi::Result<()> = try {
            let i = p.read(&line)?;
            for xx in i {
                let o = p.eval(&xx)?;
                line = p.print(&o)?;
                println!(";=>{}", line);
            }
            println!();
            ()
        };
        if let Err(e) = ret {
            eprintln!("error: {}", e);
        }
    };
    Ok(())
}
