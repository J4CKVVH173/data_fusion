use std::fmt::Result;

use super::traits::{FileAccess, PrepareFiles, SaveFuse};

use crate::fusion::domain::files_fusion::fusion::Fusion;
use crate::fusion::application::file_preparation::FilePreparation;



/**
 * Use case для работы с файлами через обработчик cli.
 */
pub struct CLIProcess;


impl CLIProcess {
  pub fn process<T: SaveFuse + FileAccess>(repository: T) -> Result {
    let files = repository.get_file_paths();
    let mut file_processor = FilePreparation::new(files);
    let files = file_processor.prepare_files();
    let fusion = Fusion::new(files);
    let fused_files = fusion.fuse();
    let _ = repository.save_fuse(fused_files);
    Ok(())
  }
}
