use hframe::Aware;

const INITIAL_MARKDOWN: &str = r#"
# Hello

![comrak author gh profile picture](https://avatars.githubusercontent.com/u/1915?v=4)

- World
    - !!!
"#;

pub struct App {
    markdown_input: String,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let style = egui::Style {
            visuals: egui::Visuals::light(),
            ..Default::default()
        };

        cc.egui_ctx.set_style(style);

        Self {
            markdown_input: INITIAL_MARKDOWN.into(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let full_height = ui.available_height();
            let half_width = ui.available_width() / 2.0;

            ui.horizontal(|ui| {
                ui.add_sized(
                    [half_width, full_height],
                    egui::TextEdit::multiline(&mut self.markdown_input),
                );

                let html =
                    comrak::markdown_to_html(&self.markdown_input, &comrak::Options::default());
                let styled_html = format!("<div style=\"font-family: sans-serif;\">{}</div>", html);
                let html_widget = hframe::BareHtml::new("render_html").content(&styled_html);

                ui.add_sized([half_width, full_height], html_widget);
            });
        });

        for i in 0..2 {
            egui::Window::new("Move me")
                .id(egui::Id::new(i))
                .show(ctx, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Use these floating resizable windows test the composition.");
                    });
                })
                .aware();
        }

        hframe::sync(ctx);
    }
}
