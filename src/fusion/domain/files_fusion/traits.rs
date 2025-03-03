use super::extended_file::ExtendedFile;

pub trait FileAccess {
  fn get_files(&self) -> Vec<ExtendedFile>;
}
