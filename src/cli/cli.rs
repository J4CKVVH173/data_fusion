use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Tool to fusion a bunch of files into one stream.", long_about = None)]
pub struct CLI {
  #[arg(short, long, value_name = "fusion")]
  // Набор путей до файлов которые будут сливаться
  fusion: Option<Vec<String>>,
  #[arg(short, long, value_name = "defusion")]
  // Путь до файла, который попытается система разобрать на составные файлы
  defusion: Option<String>,
}

