use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SakuseiError {
    IOError(std::io::Error),
    InvalidArguments(String),
    InvalidPath(String),
    FileAlreadyExists(String),
}

impl Display for SakuseiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SakuseiError::IOError(err) => write!(f, "IO error: {}", err),
            SakuseiError::InvalidArguments(err) => write!(f, "Invalid arguments: {}", err),
            SakuseiError::InvalidPath(err) => write!(f, "Invalid path: {}", err),
            SakuseiError::FileAlreadyExists(file) => write!(
                f,
                "File already exists: {}. Use '-y' or '--yes' to replace it or '-d' or '--duplicate' to create a duplicate with '(1)' suffix.",
                file
            ),
        }
    }
}

impl std::error::Error for SakuseiError {}

impl From<std::io::Error> for SakuseiError {
    fn from(err: std::io::Error) -> Self {
        SakuseiError::IOError(err)
    }
}
