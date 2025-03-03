use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum InnerErrors {
  FilesNotFound(String),
  CantWriteFuse,
}

impl fmt::Display for InnerErrors {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        InnerErrors::FilesNotFound(file_paths) => write!(f, "Lost files: {}", file_paths),
        InnerErrors::CantWriteFuse => write!(f, "Error during writing file on disk"),
      }
  }
}

// Реализуем Error для использования как ошибка
impl Error for InnerErrors {}
