pub mod graph;

use std::io::Write;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Linux,
    Macos,
}

impl Target {
    pub fn host() -> Self {
        if cfg!(target_os = "macos") {
            Target::Macos
        } else {
            Target::Linux
        }
    }

    pub fn mangle_symbol(&self, name: &str) -> String {
        match self {
            Target::Macos => format!("_{name}"),
            Target::Linux => name.to_string(),
        }
    }
}

pub trait Generator {
    type Error;
    fn generate(&mut self) -> Result<(), Self::Error>;
    fn output<W: Write>(&self, w: &mut W) -> Result<(), Self::Error>;
}
