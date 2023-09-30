pub fn extract(contents: &str) -> Vec<String> {
    let pattern = &"```";
    let mut inside_code_block = false;
    let mut code_blocks: Vec<String> = Vec::new();
    let mut code_block: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.starts_with(pattern) && !inside_code_block {
            inside_code_block = true;
        } else if line.starts_with(pattern) && inside_code_block {
            code_blocks.push(code_block.join("\n"));
            inside_code_block = false;
            code_block = Vec::new();
        } else if inside_code_block {
            code_block.push(line);
        }
    }
    return code_blocks;
}
