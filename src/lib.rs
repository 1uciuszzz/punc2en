use colored::Colorize;

pub struct Config {
    pub path: String,
    pub is_dir: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let path = args[1].clone();
        let is_dir = args.len() == 3 && args[2] == "-d";
        Ok(Config { path, is_dir })
    }
}

pub fn run(path: String) {
    let content: String = match std::fs::read_to_string(&path) {
        Err(e) => {
            println!(
                "{} {}",
                "Something went wrong reading the file: ".red().bold(),
                e.to_string().red().bold()
            );
            std::process::exit(1);
        }
        Ok(content) => content,
    };
    let mut res: Vec<String> = vec![];
    content.lines().for_each(|line| {
        let mut muted = line.replace("，", ",");
        muted = muted.replace("。", ".");
        muted = muted.replace("、", "\\");
        muted = muted.replace("；", ";");
        muted = muted.replace("‘", "\'");
        muted = muted.replace("’", "\'");
        muted = muted.replace("【", "[");
        muted = muted.replace("】", "]");
        muted = muted.replace("·", "`");
        muted = muted.replace("《", "<");
        muted = muted.replace("》", ">");
        muted = muted.replace("？", "?");
        muted = muted.replace("：", ":");
        muted = muted.replace("“", "\"");
        muted = muted.replace("”", "\"");
        muted = muted.replace("！", "!");
        muted = muted.replace("￥", "$");
        muted = muted.replace("……", "...");
        muted = muted.replace("（", "(");
        muted = muted.replace("）", ")");
        muted = muted.replace("——", "_");
        res.push(muted);
    });
    match std::fs::write(&path, res.join("\n")) {
        Err(e) => {
            println!(
                "{} {}",
                "Something went wrong writing the file: ".red().bold(),
                e.to_string().red().bold()
            );
            std::process::exit(1);
        }
        Ok(_) => {
            println!("{}", "Done!".green().bold());
        }
    }
}
