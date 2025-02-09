use std::io::Read;

use crate::prelude::*;

pub fn load_font_raw(file: &str) -> Result<Vec<u8>> {
    let file = std::fs::File::open(file)?;
    let mut buf = Vec::new();
    std::io::BufReader::new(file).read_to_end(&mut buf)?;
    Ok(buf)
}
