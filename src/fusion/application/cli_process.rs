use super::traits::{FileAccess, PrepareFiles, SaveFuse};

use crate::fusion::domain::files_fusion::fusion::Fusion;
use crate::fusion::application::file_preparation::FilePreparation;
use crate::utils::inner_errors::InnerErrors;


/**
 * Use case для работы с файлами через обработчик cli.
 */
pub struct CLICase;


impl CLICase {
  /// Полный кейс когда нужно для CLI подготовить спаянный файл.
  pub fn fuse<T: SaveFuse + FileAccess>(repository: T) -> Result<(), InnerErrors> {
    let files = repository.get_file_paths();
    let mut file_processor = FilePreparation::new(files)?;
    let files = file_processor.prepare_files();
    let fusion = Fusion::new(files);
    let fused_files = fusion.fuse();
    repository.save_fuse(fused_files)?;
    Ok(())
  }
}
