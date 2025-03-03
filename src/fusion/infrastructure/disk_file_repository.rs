use std::path::Path;
use std::fs;

use crate::fusion::application::traits::{FileAccess, SaveFuse};

// Структура отвечает за чтение файлов и получение их.
#[derive(Debug)]
pub struct DiskFileRepository {
	// Вектор путей до файлов.
	file_paths: Vec<String>
}

impl DiskFileRepository {
		// Путь до файлов.
	pub fn new(paths: Vec<String>) -> Self {
		for file in &paths {
			if !Path::new(&file).is_file() {
				// todo заменить здесь на создание массива нормальных ошибок, которые потом можно вывести
				panic!("File {} does not exist.", file);
			}
		}
		DiskFileRepository { file_paths: paths }
	}
}

impl FileAccess for DiskFileRepository {
	fn get_file_paths(&self) -> Vec<String> {
		self.file_paths.clone()
	}
}

impl SaveFuse for DiskFileRepository {
	fn save_fuse(&self, fuse: Vec<u8>) -> std::io::Result<()> {
		fs::write("fuse.raw", fuse)?;
		Ok(())
	}
}
