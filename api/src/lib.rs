use axum::{routing::post, Json, Router};

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

pub async fn post_extract(body: String) -> Json<Vec<String>> {
    let code_blocks = extract(&body[..]);
    return Json(code_blocks);
}

pub async fn serve() -> () {
    let app = Router::new().route("/extract", post(post_extract));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
