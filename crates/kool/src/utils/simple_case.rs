use std::fmt::{Display, Write};

#[derive(Clone, Copy, Debug)]
pub struct SimpleCase<'a>(&'a str);

impl<'a> SimpleCase<'a> {
    pub fn new(string: &'a str) -> Self {
        Self(string)
    }
}

impl<'a> Display for SimpleCase<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.chars() {
            if c.is_ascii_alphanumeric() {
                f.write_char(c.to_ascii_lowercase())?;
            }
        }

        Ok(())
    }
}
