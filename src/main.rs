use std::io;
use std::mem::size_of_val;
use clap::Parser;

#[derive(Parser)]
#[command(author = "darkiiiiiice dariiiiiice@gmail.com")]
#[command(author, version, about, long_about = None)]
struct CommonArgs {

}


fn main() -> io::Result<()> {
    let args = CommonArgs::parse();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim();

    output_all(buffer);

    Ok(())
}

fn output_all(buffer: &str) {
    for ch in buffer.chars() {
        let len_utf8 = ch.len_utf8();
        let mut ch_width: usize = 6;
        if len_utf8 <= 2 {
            ch_width += 1;
        }
        print!("{:<width$}", ch, width = ch_width);

        let size = size_of_val(&ch);
        print!("{:0>size$b} ", ch as u64, size = size << 3);
        print!("0x{:0>size$X} ", ch as u64, size = size << 1);
        print!("0o{:0>o}", ch as u64);

        println!();
    }
}
