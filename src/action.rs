use std::ops::Deref;

pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    pub fn prepend(mut self, prefix: &str) -> Self {
        self.msg = prefix.to_string() + self.get_msg();
        self
    }
}

impl<T> From<T> for Error
where
    T: Deref<Target = str>,
{
    fn from(value: T) -> Self {
        Self::new(&value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Action {
    fn execute(&mut self) -> Result<()>;
    fn undo(&mut self) -> Result<()>;
}
