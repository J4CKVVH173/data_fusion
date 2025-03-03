use crate::fusion::domain::files_fusion::fusion::Fusion;

use super::fixtures::{get_income_file, get_result};

#[test]
// Проверяем процесс объединения файлов.
fn test_merge_files() {
  let f1 = get_income_file("f1.txt", "ab".as_bytes().to_vec());
  let f2 = get_income_file("f22.svg", "ttt".as_bytes().to_vec());

  let fusion = Fusion::new(vec![f1, f2]);

  let fused_files = fusion.fuse();

  assert_eq!(fused_files, get_result());
}
