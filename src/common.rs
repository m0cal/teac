pub mod graph;

use std::io::Write;

pub trait Generator {
    type Error;
    fn generate(&mut self) -> Result<(), Self::Error>;
    fn output<W: Write>(&self, w: &mut W) -> Result<(), Self::Error>;
}
