mod cli;
mod fusion;
mod lib;

use clap::Parser;

use self::cli::CLI;
use self::fusion::infrastructure::FilesReader;

fn main() {
  let cli = match CLI::try_parse() {
    Ok(cli) => cli, // Если всё хорошо, получаем CLI
    Err(e) => {
        e.exit(); // Если есть ошибка, позволяем clap обработать её (например, --help или --version)
    }
  };

  let a = String::from("1;");
  println!("{:?}", a.as_bytes());
  if cli.fusion.is_some() {
    let file_paths = cli.fusion.unwrap();
    let mut files_reader = FilesReader::new(file_paths);
    let _ = files_reader.prepare_files();
  }
}
