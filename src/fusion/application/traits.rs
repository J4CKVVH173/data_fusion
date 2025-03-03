use std::fmt::Debug;

use crate::fusion::domain::files_fusion::ExtendedFile;

/// Интерфейс описывающий поведение необходимое для подготовки файлов в формат
/// пригодный для работы с ними.
pub trait PrepareFiles {
  fn prepare_files(&mut self) -> Vec<ExtendedFile>;
}

/// Интерфейс для работы с сохранением файла.
pub trait SaveFuse {
  /**
   * Метод для сохранения соединенного файла.
   */
  fn save_fuse(&self, fuse: Vec<u8>) -> std::io::Result<()>;
}

/// Метод для доступа к файлам.
pub trait FileAccess: Debug {
  /// Метод для получения набора путей до файлов.
  fn get_file_paths(&self) -> Vec<String>;
}
