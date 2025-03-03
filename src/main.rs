mod cli;
mod fusion;
mod utils;

use clap::Parser;

use self::cli::CLI;
use self::fusion::infrastructure::disk_file_repository::DiskFileRepository;
use self::fusion::application::cli_process::CLIProcess;

fn main() {
  let cli = match CLI::try_parse() {
    Ok(cli) => cli, // Если всё хорошо, получаем CLI
    Err(e) => {
        e.exit(); // Если есть ошибка, позволяем clap обработать её (например, --help или --version)
    }
  };


  if cli.fusion.is_some() {
    let file_paths = cli.fusion.unwrap();

    let file_repository = DiskFileRepository::new(file_paths);
    let _ = CLIProcess::process(file_repository);
    return;
  }

  println!("No fusion requested.");
}
