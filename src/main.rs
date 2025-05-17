mod gemini;
mod windows;
mod gui;

#[tokio::main]
async fn main() {
    gui::spawn().await.unwrap();
}
