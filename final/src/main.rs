#![feature(try_blocks)]

extern crate malvi_final as malvi;

use self::malvi::{Malvi,Mal};

fn interactive_mode<T:Mal>(m:&mut T) -> malvi::Result<()> {
    let mut line = "".to_string();
    loop {
        line.clear();
        if 0 == ::std::io::stdin().read_line(&mut line)? {
            break;
        }
        let ret : malvi::Result<()> = try {
            let i = m.read(&line)?;
            for xx in i {
                let o = m.eval(&xx)?;
                line = m.print(&o)?;
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

fn main() -> malvi::Result<()> {
    let mut m = Malvi::new();

    let tracemode = ::std::env::var("MAL_TRACE").is_ok();
    if tracemode {
        let cmd = m.read("(trace-mode true)")?.pop_front().unwrap();
        let _ = m.eval(&*cmd)?;
    }

    let mut args = ::std::env::args();

    if args.len() <= 1 {
        interactive_mode(&mut m)?;
    } else {
        let _ = args.next().unwrap();
        let fa = args.next().unwrap();
        if fa == "--help" || fa == "-?" || fa == "-h" {
            println!("Usage: malvi [{{-c command|[-i] file}}]");
            println!("   malvi - interactive mode");
            println!("   malvi file.mal - interpreter mode");
            println!("   malvi -i file.mal - load file, then go interactive");
            println!("   malvi -c '()' - execute one thing");
            return Ok(())
        }
        if fa == "-c" {
            let na = args.next().unwrap();
            for x in m.read(&na)? {
                let _ = m.eval(&x)?;
            }
            return Ok(())
        }
        if fa == "-i" {
            if args.len() == 0 {
                interactive_mode(&mut m)?;
                return Ok(())
            }
            let na = args.next().unwrap();
            let s = ::std::fs::read_to_string(na)?;
            for x in m.read(&s)? {
                let _ = m.eval(&x)?;
            }
            interactive_mode(&mut m)?;
            return Ok(())
        }

        let s = ::std::fs::read_to_string(fa)?;
        for x in m.read(&s)? {
            if tracemode {
                eprintln!(
                    "top-level line: {}", 
                    malvi::BoundAstRef(&x,&m,malvi::DisplayMode::WithMeta),
                );
            }
            let res = m.eval(&x)?;
            if tracemode {
                eprintln!(
                    "top-level result: {}",
                    malvi::BoundAstRef(&res,&m,malvi::DisplayMode::WithMeta),
                )
            }
        }
    }

    Ok(())
}
