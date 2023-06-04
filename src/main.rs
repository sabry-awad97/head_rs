use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "head", about = "Print the first N lines of a file.")]
struct Cli {
    #[structopt(parse(from_os_str))]
    file_path: Option<PathBuf>,

    #[structopt(short = "n", long = "lines", default_value = "10")]
    num_lines: usize,

    #[structopt(short = "s", long = "stdin")]
    use_stdin: bool,

    #[structopt(short = "l", long = "line-numbers")]
    line_numbers: bool,

    #[structopt(short = "b", long = "bytes")]
    num_bytes: Option<usize>,
}

fn read_lines<R: Read>(
    reader: R,
    num_lines: usize,
    line_numbers: bool,
    num_bytes: Option<usize>,
) -> io::Result<()> {
    let reader = BufReader::new(reader);
    let mut line_count = 0;
    let mut byte_count = 0;

    for line in reader.lines() {
        if line_count >= num_lines {
            break;
        }

        if let Some(num_bytes) = num_bytes {
            if byte_count >= num_bytes {
                break;
            }
        }

        if let Ok(line) = line {
            if line_numbers {
                println!("{:>6} {}", line_count + 1, line);
            } else {
                println!("{}", line);
            }
            line_count += 1;
            byte_count += line.len() + 1;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    if args.use_stdin {
        let stdin = io::stdin();
        let handle = stdin.lock();
        read_lines(handle, args.num_lines, args.line_numbers, args.num_bytes)?;
    } else if let Some(file_path) = args.file_path {
        let file = File::open(file_path)?;
        read_lines(file, args.num_lines, args.line_numbers, args.num_bytes)?;
    } else {
        eprintln!("No file or stdin specified.");
    }

    Ok(())
}
