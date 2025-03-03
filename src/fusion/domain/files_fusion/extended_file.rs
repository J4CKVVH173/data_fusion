#[derive(Debug)]
pub struct ExtendedFile {
  pub name: String,
  pub body: Vec<u8>
}

impl ExtendedFile {
  pub fn new(name: String, body: Vec<u8>) -> ExtendedFile {
    ExtendedFile { name,  body }
  }
}
