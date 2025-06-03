use windows::get_volume;

mod gemini;
mod gui;
mod windows;

#[tokio::main]
async fn main() {
    println!("wkjsdmsufjsdfsdfhj");
    println!("Set the system volume (Currently {})", get_volume());
    gui::spawn().await.unwrap();
}
