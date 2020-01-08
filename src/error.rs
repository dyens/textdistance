use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ValueError {
    details: String,
}

impl ValueError {
    pub fn new(msg: &str) -> ValueError {
        ValueError {
            details: msg.to_string(),
        }
    }
}

impl error::Error for ValueError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

#[derive(Debug)]
pub struct TextDistanceError {
    details: String,
}

impl TextDistanceError {
    pub fn new(msg: &str) -> TextDistanceError {
        TextDistanceError {
            details: msg.to_string(),
        }
    }
}

impl error::Error for TextDistanceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for TextDistanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<ValueError> for TextDistanceError {
    fn from(err: ValueError) -> Self {
        TextDistanceError::new(&err.to_string())
    }
}

impl From<TextDistanceError> for ValueError {
    fn from(err: TextDistanceError) -> Self {
        ValueError::new(&err.to_string())
    }
}
