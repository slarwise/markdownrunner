use std::{env, fs};

use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).unwrap();

    let code_blocks = markdownrunner::run(&contents).unwrap();
    dbg!("{:?}", &code_blocks);

    // Move this to the http response code
    let json = serde_json::to_value(code_blocks).unwrap();
    dbg!(json);
}
