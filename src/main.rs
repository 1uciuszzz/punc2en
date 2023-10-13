use colored::Colorize;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = match punc2en::Config::new(&args) {
        Err(e) => {
            println!("Problem parsing arguments: {}", e);
            std::process::exit(1);
        }
        Ok(config) => config,
    };
    if !config.is_dir {
        return punc2en::run(config.path);
    }
    let read_dir = match std::fs::read_dir(config.path) {
        Ok(files) => files,
        Err(e) => {
            println!(
                "{} {}",
                "Problem reading directory: ".red().bold(),
                e.to_string().red().bold()
            );
            std::process::exit(1);
        }
    };
    read_dir.for_each(|f| {
        let file = match f {
            Err(e) => {
                println!(
                    "{} {}",
                    "Problem reading file: ".red().bold(),
                    e.to_string().red().bold()
                );
                std::process::exit(1);
            }
            Ok(f) => f,
        };
        let is_file = match file.file_type() {
            Ok(t) => t.is_file(),
            Err(e) => {
                println!(
                    "{} {}",
                    "Problem reading file type: ".red().bold(),
                    e.to_string().red().bold()
                );
                std::process::exit(1);
            }
        };
        if is_file {
            let file_path = file.path().to_str().unwrap().to_string();
            punc2en::run(file_path);
        }
    })
}
