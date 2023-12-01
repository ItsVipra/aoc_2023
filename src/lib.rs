use std::fs::File;
use std::io::Read;
// use clap::error::ContextValue::String;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub day: Option<u32>,

    #[arg(short, long)]
    pub input: Option<String>,
}

pub fn load_input(path: &String) -> String {
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let mut contents = String::new();
    if let Err(why) = file.read_to_string(&mut contents) { panic!("couldn't read file: {}", why) }
    contents
}