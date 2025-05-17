use eframe::egui;

use crate::gemini::prompt;

pub async fn spawn() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_decorations(false),
        ..Default::default()
    };

    let mut query_current = String::new();
    let mut query_final = String::new();

    eframe::run_simple_native("Guppy", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guppy");
            ui.horizontal(|ui| {
                let prompt_label = ui.label("Prompt");
                let query_edit = ui
                    .text_edit_singleline(&mut query_current)
                    .labelled_by(prompt_label.id);

                if query_edit.lost_focus() {
                    query_final = query_current.clone();
                    query_current.clear();
                    println!("Name changed to: {}", query_final);
                    let query = query_final.clone();
                    tokio::spawn(async move {
                        if let Err(e) = prompt(&query).await {
                            println!("Error: {}", e);
                        }
                    });
                }
            });
            ui.label(&query_final)
        });
    })
}

