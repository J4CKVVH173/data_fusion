use std::{path::Path, sync::{Arc, RwLock}};
use std::thread::{available_parallelism, self};
use std::fs;

use crate::{fusion::{application::traits::PrepareFiles, domain::files_fusion::ExtendedFile}, utils::inner_errors::InnerErrors};

// Структура отвечает за чтение файлов и получение их.
#[derive(Debug)]
pub struct FilePreparation {
	// Вектор путей до файлов.
	file_paths: Arc<RwLock<Vec<String>>>,
}

impl FilePreparation {
	/// Конструктор
	pub fn new(paths: Vec<String>) -> Result<Self, InnerErrors> {
		for file in &paths {
			let mut lost_files= vec![];
			if !Path::new(&file).is_file() {
				lost_files.push(file.clone());
			}
			if lost_files.len() > 0 {
				eprintln!("Lost files: {:?}", lost_files);
        return Err(InnerErrors::FilesNotFound(lost_files.join(" ")));
			}
		}
		let file_paths = Arc::new(RwLock::new(paths));
		Ok(Self { file_paths })
	}

}

impl PrepareFiles for FilePreparation {
	/// Метод для подготовки файлов в формате пригодный для работы с ними.
  fn prepare_files(&mut self) -> Vec<ExtendedFile> {
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
		match Arc::try_unwrap(files) {
			Ok(unwrapped_files) => {
				match unwrapped_files.into_inner() {
					Ok(files) => {
            return files;
          },
          Err(_) => {
            // todo заменить здесь на создание массива нормальных ошибок, которые потом можно вывести
            todo!()
          }
				}
			},
			Err(_) => {
        // todo заменить здесь на создание массива нормальных ошибок, которые потом можно вывести
				todo!()
      }
		}
	}
}
