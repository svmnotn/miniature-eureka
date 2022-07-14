#[derive(Debug)]
pub enum Error {
    ArgParsing(lexopt::Error),
    FileOperations(std::io::Error),
}

impl From<lexopt::Error> for Error {
    fn from(e: lexopt::Error) -> Self {
        Error::ArgParsing(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::FileOperations(e)
    }
}