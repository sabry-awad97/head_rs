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

    #[structopt(short = "o", long = "offset")]
    byte_offset: Option<usize>,

    #[structopt(short = "p", long = "paginate")]
    paginate: bool,

    #[structopt(short = "t", long = "truncate")]
    truncate: bool,

    #[structopt(short = "r", long = "reverse")]
    reverse: bool,
}

fn read_lines<R: Read>(
    reader: R,
    num_lines: usize,
    line_numbers: bool,
    num_bytes: Option<usize>,
    byte_offset: Option<usize>,
    paginate: bool,
    truncate: bool,
    reverse: bool,
) -> io::Result<()> {
    let reader = BufReader::new(reader);
    let mut byte_count = 0;
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    if reverse {
        lines.reverse();
    }

    for (line_count, line) in lines.iter().enumerate() {
        if line_count <= num_lines {
            if let Some(num_bytes) = num_bytes {
                if byte_count >= num_bytes {
                    break;
                }
            }

            if let Some(byte_offset) = byte_offset {
                if byte_count < byte_offset {
                    byte_count += line.len() + 1;
                    continue;
                }
            }

            if truncate && line.len() > 80 {
                if line_numbers {
                    println!("{:>6} {}", line_count + 1, line[..77].to_string() + "...");
                } else {
                    println!("{}", line[..77].to_string() + "...");
                }
            } else {
                if line_numbers {
                    println!("{:>6} {}", line_count + 1, line);
                } else {
                    println!("{}", line);
                }
            }

            byte_count += line.len() + 1;

            if paginate {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim().to_lowercase() == "q" {
                    break;
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    if args.use_stdin {
        let stdin = io::stdin();
        let handle = stdin.lock();
        read_lines(
            handle,
            args.num_lines,
            args.line_numbers,
            args.num_bytes,
            args.byte_offset,
            args.paginate,
            args.truncate,
            args.reverse,
        )?;
    } else if let Some(file_path) = args.file_path {
        let file = File::open(file_path)?;
        read_lines(
            file,
            args.num_lines,
            args.line_numbers,
            args.num_bytes,
            args.byte_offset,
            args.paginate,
            args.truncate,
            args.reverse,
        )?;
    } else {
        eprintln!("No file or stdin specified.");
    }

    Ok(())
}
