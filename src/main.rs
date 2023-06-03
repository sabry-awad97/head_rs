use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "head", about = "Print the first N lines of a file.")]
struct Cli {
    #[structopt(parse(from_os_str))]
    file_path: PathBuf,

    #[structopt(short = "n", long = "lines", default_value = "10")]
    num_lines: usize,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let file = File::open(args.file_path)?;
    let reader = BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        if line_number >= args.num_lines {
            break;
        }

        if let Ok(line) = line {
            println!("{}", line);
        }
    }

    Ok(())
}
