use clap::{Args, Parser};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "rust cat", long_about = None)]
pub struct Cli {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[command(flatten)]
    number_lines: NumberLines,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct NumberLines {
    /// Number lines
    #[arg(short, long = "number")]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_result(buffer: Box<dyn BufRead>, config: &NumberLines) -> MyResult<()> {
    let mut row_cnt = 0;
    for line in buffer.lines() {
        let line_str = &line?;

        if config.number_lines {
            row_cnt += 1;
            println!("{:>6}\t{}", row_cnt, line_str);
        } else if config.number_nonblank_lines && line_str != "" {
            row_cnt += 1;
            println!("{:>6}\t{}", row_cnt, line_str);
        } else {
            println!("{}", line_str);
        }
    }
    Ok(())
}

pub fn run(config: Cli) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buffer) => print_result(buffer, &config.number_lines)?,
        }
    }
    Ok(())
}
