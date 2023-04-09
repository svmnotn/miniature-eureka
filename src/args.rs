use std::path::PathBuf;

pub struct Args {
    /// Source File Path, this file will be stolen completetly
    pub src: PathBuf,
    /// Destination File Path, this file's contents will be removed from start_marker to end_marker
    pub dest: PathBuf,
    /// Every line below this marker up to the first end_marker will be removed
    pub start_marker: String,
    /// Marks the end of the replace
    pub end_marker: String,
}

impl Args {
    pub fn parse_args() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut src = None;
        let mut dest = None;
        let mut start_marker = None;
        let mut end_marker = None;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Long("src") | Long("source") => {
                    src = Some(parser.value()?.parse()?);
                }
                Value(source) if src.is_none() => {
                    src = Some(source.parse()?);
                }
                Long("dst") | Long("dest") | Long("destination") => {
                    dest = Some(parser.value()?.parse()?);
                }
                Value(destination) if dest.is_none() => {
                    dest = Some(destination.parse()?);
                }
                Long("start") | Long("start-marker") => {
                    start_marker = Some(parser.value()?.parse()?);
                }
                Long("end") | Long("end-marker") => {
                    end_marker = Some(parser.value()?.parse()?);
                }
                Short('h') | Long("help") => {
                    println!("Usage: section-replacer [PATH|--src|--source=PATH] [PATH|--dst|--dest|--destination=PATH] [--start|--start-marker=MARKER] [--end|--end-marker=MARKER]");
                    std::process::exit(0);
                }
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(Args {
            src: src.ok_or("source file missing!")?,
            dest: dest.ok_or("destination file missing!")?,
            start_marker: start_marker.unwrap_or("SECTION-REPLACE-START".into()),
            end_marker: end_marker.unwrap_or("SECTION-REPLACE-END".into()),
        })
    }
}
