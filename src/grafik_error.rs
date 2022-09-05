#[derive(Debug)]
pub enum GrafikError { 
    IOError(std::io::Error),  
}

impl From<std::io::Error> for GrafikError {
    fn from(error: std::io::Error) -> Self {
        GrafikError::IOError(error)
    }
}