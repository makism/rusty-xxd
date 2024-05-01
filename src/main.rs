use clap::Parser;
use colored::*;

const SPACE_BEFORE_TEXT: usize = 2;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    path: Option<std::path::PathBuf>,

    #[arg(long, default_value = "false")]
    colored: bool,

    #[arg(short, long, default_value = "4")]
    columns: i32,
}

struct Iteration {
    total_length: usize,
    stride: usize,
    start: usize,
    end: usize,
}

impl Iteration {
    pub fn next(&mut self) {
        self.start = self.end;
        self.end += self.stride;

        if self.end > self.total_length {
            self.end = self.total_length;
        }
    }
}

fn process_file(file_path: &std::path::PathBuf, colored_output: bool, mut iters: Iteration) {
    match std::fs::read_to_string(file_path) {
        Ok(contents) => {
            iters.total_length = contents.len();
            let total_iters = (iters.total_length as f32 / (iters.stride as f32)).ceil();

            for i in 0..total_iters as usize {
                print!("{:08x}: ", i * iters.stride);

                let slice = &contents[iters.start..iters.end];
                let bytes = slice.as_bytes();
                for b in bytes {
                    if colored_output && (b.is_ascii_control() || b.is_ascii_whitespace()) {
                        let byte_to_str = String::from(format!("{:02x} ", b));
                        print!("{}", byte_to_str.red());
                    } else {
                        print!("{:02x} ", b);
                    }
                }

                if slice.len() < iters.stride {
                    let diff = iters.stride - slice.len();
                    print!("{}", str::repeat(" ", SPACE_BEFORE_TEXT));
                    for _ in 0..diff{
                        print!("  ");
                    }
                }

                print!("{}", str::repeat(" ", SPACE_BEFORE_TEXT));

                println!(
                    "{}",
                    slice
                        .replace(" ", "•")
                        .replace('\n', "•")
                        .replace('\r', "•")
                );

                iters.next();
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

impl Cli {
    pub fn go(self) {
        match self.path {
            Some(file_path) => {
                let iters = Iteration {
                    total_length: 0,
                    stride: self.columns as usize,
                    start: 0,
                    end: self.columns as usize,
                };

                process_file(&file_path, self.colored, iters);
            }
            None => {
                println!("No input file");
            }
        }
    }
}

fn main() {
    let args = Cli::parse();
    args.go();
}
