use std::{fmt::Result, path::Path, sync::{Arc, RwLock}};
use std::thread::{available_parallelism, self};
use std::fs;

use crate::fusion::domain::files_fusion::{ExtendedFile, FileAccess};

// Структура отвечает за чтение файлов и получение их.
#[derive(Debug)]
pub struct FilesReader {
	// Вектор путей до файлов.
	file_paths: Arc<RwLock<Vec<String>>>,
	// Набор преобразованных файлов
	files: Option<Arc<RwLock<Vec<ExtendedFile>>>>,
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

	pub fn prepare_files(&mut self) -> Result {
		let cores = available_parallelism().unwrap().get();
		let paths_count = self.file_paths.read().unwrap().len();
		let files: Arc<RwLock<Vec<ExtendedFile>>> = Arc::new(RwLock::new(Vec::with_capacity(paths_count)));
		if cores > paths_count {
			let mut threads: Vec<thread::JoinHandle<()>> = vec![];
			for i in 0..paths_count {
				let moving_file_paths  = Arc::clone(&self.file_paths);
				let moving_files = Arc::clone(&files);

				let sub_thread = thread::spawn(move || {
					let rw_path = moving_file_paths.read().unwrap();
					let path = rw_path.get(i);
					match path {
						Some(path) => {
              let file_body = fs::read(path).unwrap();
							let name = path.split("/").last().unwrap();
							let file = ExtendedFile::new(String::from(name),  file_body);
              moving_files.write().unwrap().push(file);
            },
            None => {
              // todo заменить здесь на создание массива нормальных ошибок, которые потом можно вывести
              panic!("File path at index {} is None.", i);
            }
					}
				});
				threads.push(sub_thread);
			}

			for thread in threads {
        thread.join().unwrap();
      }
		} else {
			todo!()
		}

		self.files = Some(files);
		return Ok(())
	}
}

impl FileAccess for FilesReader {
	fn get_files(&self) -> Vec<ExtendedFile> {
		return vec![]
	}
}
