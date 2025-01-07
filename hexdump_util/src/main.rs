use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let mut file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open the file: {}", e);
            return Ok(());
        }
    };

    let mut buff = [0; 16];
    let mut count = 0;

    loop {
        let bytes_read = file.read(&mut buff)?;
        if bytes_read == 0 {
            break;
        }

        print!("{:08x}: ", count);

        for n in 0..bytes_read {
            print!("{:02x}", buff[n]);
            if (n + 1) % 2 == 0 {
                print!(" ");
            }
        }

        print!("{:>width$}", "", width = (16 - bytes_read) * 3);
        let ascii: String = buff[..bytes_read]
            .iter()
            .map(|&b| {
                if b.is_ascii() && !b.is_ascii_control() {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();
        println!("{}", ascii);

        count += bytes_read;
    }

    Ok(())
}

