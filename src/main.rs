use std::io;
use std::io::Read;
use std::mem::size_of_val;
use clap::Parser;

#[derive(Parser)]
#[command(author = "darkiiiiiice dariiiiiice@gmail.com")]
#[command(author, version, about, long_about = None)]
struct CommonArgs {
    /// Convert unicode string to unicode escape sequences
    #[arg(short, long, default_value_t = false)]
    escape: bool,

    /// Convert unicode escape sequences to unicode string
    #[arg(short, long, default_value_t = false)]
    unescape: bool,

    /// Output ascii table
    #[arg(short, long, default_value_t = false)]
    ascii: bool,
}


fn main() {
    let args = CommonArgs::parse();

    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {
            let buffer = buffer.trim();

            if args.escape {
                output_unicode_escape_sequences(buffer);
            } else if args.unescape {
                output_unicode_string(buffer);
            } else {
                output_default(buffer);
            }
        }
        Err(error) => println!("error: {error}"),
    }
}

fn output_unicode_string(buffer: &str) {
    println!("{:} ", buffer);
}

fn output_unicode_escape_sequences(buffer: &str) {
    for ch in buffer.chars() {
        let val = ch as u32;
        if val >= u32::MIN && val <= 0xFFFF {
            print!("\\u{:0>4X}", val)
        } else if val >= 0x10000 && val <= 0x10FFFF {
            print!("\\U{:0>8X}", val)
        }
    }
    println!()
}

fn output_default(buffer: &str) {
    for ch in buffer.chars() {
        let len_utf8 = ch.len_utf8();
        let mut ch_width: usize = 6;
        if len_utf8 <= 2 {
            ch_width += 1;
        }
        if ch.is_ascii_control() && ch.is_control() {
            continue;
        }
        print!(r#"{:<width$}"#, ch, width = ch_width);

        let size = size_of_val(&ch);
        print!("{:0>size$b} ", ch as u64, size = size << 3);
        print!("0x{:0>size$X} ", ch as u64, size = size << 1);
        print!("0o{:0>o}", ch as u64);

        println!();
    }
}
