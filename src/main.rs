mod cli;

use clap::Parser;

use self::cli::CLI;

fn main() {
    let cli = match CLI::try_parse() {
        Ok(cli) => cli, // Если всё хорошо, получаем CLI
        Err(e) => {
            e.exit(); // Если есть ошибка, позволяем clap обработать её (например, --help или --version)
        }
    };

    println!("{:#?}", cli); // Выводим распарсенные аргументы
}
