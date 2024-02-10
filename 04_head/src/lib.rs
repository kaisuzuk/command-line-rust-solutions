use clap::{Args, Parser};
// use core::num;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::num::NonZeroUsize;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "rust head", long_about = None)]
pub struct Cli {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[command(flatten)]
    limit_output: LimitOutput,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct LimitOutput {
    /// Number of lines
    #[arg(short = 'n', long, default_value = "10")]
    lines: NonZeroUsize,

    /// Number of bytes
    #[arg(short = 'c', long)]
    bytes: Option<NonZeroUsize>,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_lines(mut reader: Box<dyn BufRead>, lines: NonZeroUsize) -> MyResult<()> {
    let mut line_string = String::new();

    for _ in 0..lines.get() {
        let bytes = reader.read_line(&mut line_string)?;
        if bytes == 0 {
            break;
        }
        print!("{}", line_string);
        line_string.clear();
    }

    Ok(())
}

fn print_bytes(reader: Box<dyn BufRead>, bytes: usize) -> MyResult<()> {
    let mut handler = reader.take(bytes as u64);
    let mut buffer = vec![0 as u8; bytes];
    let bytes_read = handler.read(&mut buffer)?;

    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    Ok(())
}

pub fn run(config: Cli) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.into_iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.limit_output.bytes {
                    print_bytes(reader, num_bytes.into())?;
                } else {
                    print_lines(reader, config.limit_output.lines)?;
                }
            }
        }
    }
    Ok(())
}
