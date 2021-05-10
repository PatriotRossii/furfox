use crossterm::{terminal, ErrorKind};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, ErrorKind> {
        let size = terminal::size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }
}
