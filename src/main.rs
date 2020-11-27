use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::string::String;
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect(); //读取控制台输出内容
    let config = Config::new(&args).unwrap_or_else(|err| {
        print!("参数不对 {}", err);
        process::exit(1);
    });
    print!(" In file {}", config.filename);
    // let contents = fs::read_to_string(config.filename).expect("读不到文件");
    //  println!("文件是\n{}", contents);
    run(config);
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("文件是\n{}", contents);
    Ok(())
}
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不对@!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
