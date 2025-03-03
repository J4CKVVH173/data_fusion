use std::fmt::Debug;

use crate::fusion::domain::files_fusion::ExtendedFile;

pub trait PrepareFiles {
  fn prepare_files(&mut self) -> Vec<ExtendedFile>;
}

pub trait SaveFuse {
  /**
   * Метод для сохранения соединенного файла.
   */
  fn save_fuse(&self, fuse: Vec<u8>) -> std::io::Result<()>;
}

pub trait FileAccess: Debug {
  /**
   * Набор путей до файлов.
   */
  fn get_file_paths(&self) -> Vec<String>;
}
