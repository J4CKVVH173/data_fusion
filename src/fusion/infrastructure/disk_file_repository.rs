use std::path::Path;
use std::fs;

use crate::{fusion::application::traits::{FileAccess, SaveFuse}, utils::inner_errors::InnerErrors};

// Структура отвечает за чтение файлов и получение их.
#[derive(Debug)]
pub struct DiskFileRepository {
	// Вектор путей до файлов.
	file_paths: Vec<String>
}

impl DiskFileRepository {
	/// Конструктор
	pub fn new(paths: Vec<String>) -> Result<Self, InnerErrors> {
		for file in &paths {
			if !Path::new(&file).is_file() {
				return Err(InnerErrors::FilesNotFound(file.clone()));
			}
		}
		Ok(DiskFileRepository { file_paths: paths })
	}
}

impl FileAccess for DiskFileRepository {
	/// Метод для выдачи набора путей файлов, которые нужно преобразовать.
	fn get_file_paths(&self) -> Vec<String> {
		self.file_paths.clone()
	}
}

impl SaveFuse for DiskFileRepository {
	/// Метод для сохранения соединенного файла в raw формате.
	fn save_fuse(&self, fuse: Vec<u8>) -> Result<(), InnerErrors> {
		match fs::write("fuse.raw", fuse) {
			Ok(_) => Ok(()),
      Err(_) => Err(InnerErrors::CantWriteFuse),
		}
	}
}
