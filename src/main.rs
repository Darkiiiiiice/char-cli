mod ascii;

use std::io;
use std::io::Read;
use std::mem::size_of_val;
use std::slice::SliceIndex;
use clap::Parser;
use crate::ascii::ASCII;

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

    if args.ascii {
        output_ascii_table();
    } else {
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
}

fn output_ascii_table() {
    for i in 0..u8::MAX {
        if i < ASCII.len() as u8 {
            print!("{:0>4} 0x{:0>2x} {:<}\t", ASCII[i as usize].0, ASCII[i as usize].0, ASCII[i as usize].1);
        } else {
            print!("{:0>4} 0x{:0>2x} {:<}\t", i, i, char::from(i));
        }

        if (i+1) % 4==0 {
            println!();
        }
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
            let index = ch as usize;
            print!(r#"{:<width$}"#, ASCII[index].1, width = ch_width);
        } else {
            print!(r#"{:<width$}"#, ch, width = ch_width);
        }

        let size = size_of_val(&ch);
        print!("{:0>size$b} ", ch as u64, size = size << 3);
        print!("0x{:0>size$X} ", ch as u64, size = size << 1);
        print!("0o{:0>11o}", ch as u64);

        println!();
    }
}
