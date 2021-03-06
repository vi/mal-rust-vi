#![feature(try_blocks)]
#![allow(unused)]

extern crate malvi_step0 as malvi;

use std::io::{BufRead,Result};
use self::malvi::{Malvi,Mal,Ast};

fn main() -> Result<()> {
    let si = ::std::io::stdin();
    let mut si = si.lock();
    let mut line = "".to_string();
    let mut p = Malvi::new();
    loop {
        line.clear();
        if 0 == si.read_line(&mut line)? {
            break;
        }
        let ret : Result<()> = try {
            let i = p.read(&line)?;
            let o = p.eval(i)?;
            line = p.print(o)?;
            println!(";=>{}", line);
            ()
        };
        if let Err(e) = ret {
            eprintln!("error: {}", e);
        }
    };
    Ok(())
}
