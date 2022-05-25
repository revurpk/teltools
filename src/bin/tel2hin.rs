use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use teltools::tel2hin;

fn main() {
    for arg in std::env::args().skip(1) {
        if let Ok(lines) = read_lines(arg) {
            for resultline in lines {
                if let Ok(line) = resultline {
                    for c in line.chars() {
                        print!("{}", tel2hin(c))
                    }
                }
                println!();
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
