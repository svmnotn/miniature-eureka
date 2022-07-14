use std::fs::{read_to_string, write};
mod args;
use args::Args;
mod error;
use error::Error;

fn replace(dest: &str, src: &str, args: &Args) -> String {
    // Allocate a worst case start size of the whole file
    let mut start = String::with_capacity(dest.len());
    let mut start_found = false;
    
    // Allocate a worst case end size of the whole file
    let mut end = String::with_capacity(dest.len());
    let mut end_found = false;

    for file_line in dest.lines() {
        match ((file_line.contains(&args.start_marker), start_found), (file_line.contains(&args.end_marker), end_found)) {
            ((false, false), _) => {
                start.push_str(file_line);
                start.push('\n');
            },
            ((true, false), _) => {
                start_found = true;
                start.push_str(file_line);
                start.push('\n');
            },
            ((_, true), (false, false)) => {}
            ((_, true), (true, false)) => {
                end_found = true;
                end.push_str(file_line);
                end.push('\n');
            },
            ((_, true), (_, true)) => {
                end.push_str(file_line);
                end.push('\n');
            },
        }
    }
    
    format!("{start}{src}\n{end}", start = start, src = src, end = end)
}

fn main() -> Result<(), Error> {
    let args = Args::parse_args()?;
    let src_body = read_to_string(&args.src)?;
    let dst_body = read_to_string(&args.dest)?;
    let replacement = replace(&dst_body, &src_body, &args);
    write(&args.dest, replacement).map_err(Into::into)
}
