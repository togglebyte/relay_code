use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Exception {
    pub kind: &'static str,
    pub message: String,
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Exception: {}", self.kind, self.message)
    }
}

pub struct TryCatch<Tee> {
    r#try: Box<dyn Fn() -> Result<Tee, Exception>>,
    catch: Box<dyn Fn(Exception) -> Result<(), Exception>>,
}

impl<Tee> TryCatch<Tee> {
    pub fn new(
        r#try: Box<dyn Fn() -> Result<Tee, Exception>>,
        catch: Box<dyn Fn(Exception) -> Result<(), Exception>>,
    ) -> Self {
        Self { r#try, catch }
    }

    pub fn do_try(self) -> Result<(), Exception> {
        if let Err(err) = (self.r#try)() {
            return (self.catch)(err);
        }

        Ok(())
    }
}
