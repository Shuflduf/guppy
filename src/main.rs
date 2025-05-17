mod gemini;
mod gui;
mod windows;

#[tokio::main]
async fn main() {
    gui::spawn().await.unwrap();
}
