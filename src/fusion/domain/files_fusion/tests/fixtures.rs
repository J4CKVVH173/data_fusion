use crate::{fusion::domain::files_fusion::ExtendedFile, lib::constants::{CHECK_BYTES, get_version}};

pub fn get_income_file(name: &str, body: Vec<u8>) -> ExtendedFile {
  ExtendedFile::new(String::from(name), body)
}

pub fn get_result() -> Vec<u8> {
  // two files with name f1.txt and body ab
  // f22.svg and body ttt
  // [0, 6] [0, 0, 0, 2]
  // [0, 7] [0, 0, 0, 3]
  let mut result = CHECK_BYTES.to_vec();
  result.extend_from_slice(&get_version());
  // name_length u16 + body length u32
  let header_1 = [0, 6, 0, 0, 0, 2];
  let body_1: [u8; 8] = [102, 49, 46, 116, 120, 116, 97, 98];
  let header_2 = [0, 7, 0, 0, 0, 3];
  let body_2: [u8; 10] = [102, 50, 50, 46, 115, 118, 103, 116, 116, 116];
  result.extend_from_slice(&header_1);
  result.extend_from_slice(&body_1);
  result.extend_from_slice(&header_2);
  result.extend_from_slice(&body_2);
  result
}
