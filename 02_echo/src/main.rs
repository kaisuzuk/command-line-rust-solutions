use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(required = true)]
    text: Vec<String>,
    #[arg(short = 'n', long, help = "Do not print new line")]
    omit_newline: bool,
}

fn main() {
    let args = Cli::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
