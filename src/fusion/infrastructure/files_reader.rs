use std::{fmt::Result, path::Path, sync::{Arc, RwLock}};
use std::thread::available_parallelism;

use crate::fusion::domain::files_fusion::{ExtendedFile, FileAccess};

// Структура отвечает за чтение файлов и получение их.
struct FilesReader {
	// Вектор путей до файлов.
	file_paths: Arc<RwLock<Vec<String>>>,
	// Набор преобразованных файлов
	files: Option<Vec<ExtendedFile>>,
}

impl FilesReader {
		// Путь до файлов.
	pub fn new(paths: Vec<String>) -> Self {
		for file in &paths {
			if !Path::new(&file).is_file() {
				// todo заменить здесь на создание массива нормальных ошибок, которые потом можно вывести
				panic!("File {} does not exist.", file);
			}
		}
		let file_paths = Arc::new(RwLock::new(paths));
		FilesReader { file_paths, files: None }
	}

	pub fn prepare_files(&self) -> Result {
		let CPUS = available_parallelism().unwrap().get();
		return Ok(())
	}
}

impl FileAccess for FilesReader {
	fn get_files(&self) -> Vec<ExtendedFile> {
		return vec![]
	}
}
