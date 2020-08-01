use octane::config::Config;
use octane::server::Octane;
use octane::{
    route,
    router::{Flow, Route},
};
use std::path::PathBuf;
use std::process;

#[tokio::main]
async fn main() {
    let mut app = Octane::new();
    app.add_static_dir("/", "docs");
    // Server index.html at `/`
    app.get(
        "/",
        route!(|req, res| {
            res.send_file(PathBuf::from("docs/index.html"))
                .await
                .expect("index file not found!");
            Flow::Next
        }),
    )
    .unwrap();
    app.get(
        "css",
        route!(|req, res| {
            res.send_file(PathBuf::from("docs/css/main.css"))
                .await
                .expect("index file not found!");
            Flow::Next
        }),
    )
    .unwrap();
    // Build scss, located in docs/css
    let _process = process::Command::new("./bin/build-css")
        .output()
        .expect("Failed to build css");
    app.listen(8000).await.expect("The server was not started!");
}
