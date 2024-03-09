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
        egui::SidePanel::right("render_panel").show(ctx, |ui| {
            let md = comrak::markdown_to_html(&self.markdown_input, &comrak::Options::default());
            let html = format!("<div style=\"font-family: sans-serif;\">{}</div>", md);
            ui.add(hframe::BareHtml::new("render_html").content(&html));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.markdown_input),
            )
        });

        hframe::sync(ctx);
    }
}
