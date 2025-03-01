pub struct ExtendedFile {
  name: String,
  length: usize,
  body: Vec<u8>
}

pub trait FileAccess {
  fn get_files(&self) -> Vec<ExtendedFile>;
}
