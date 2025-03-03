
use super::ExtendedFile;

use crate::utils::constants::{CHECK_BYTES, get_version};

/// Основная структура для выполнения логики объединения файлов.
#[derive(Debug)]
pub struct Fusion {
  files: Vec<ExtendedFile>
}

impl Fusion {
  pub fn new(files: Vec<ExtendedFile>) -> Self {
    Fusion { files }
  }

  /// Объединяем все файлы в один с объединенным заголовком и телом.
  pub fn fuse(self) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend_from_slice(&CHECK_BYTES);
    result.extend_from_slice(&get_version());
    for file in self.files {
      let header = Self::construct_header(&file);
      let body = Self::construct_body(file);
      result.extend(header);
      result.extend(body);
    }
    result
  }

  /// Собираем заголовок для указанного файла.
  fn construct_header(file: &ExtendedFile) -> Vec<u8> {
    let mut header = Vec::new();
    let name_length = file.name.len() as u16;
    let body_length = file.body.len() as u32;

    header.extend(name_length.to_be_bytes());
    header.extend(body_length.to_be_bytes());
    header
  }

  /// Собираем тело для расширенного файла.
  fn construct_body(file: ExtendedFile) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend(file.name.into_bytes());
    body.extend(file.body);
    body
  }
}

