use hframe::Aware;

const INITIAL_MARKDOWN: &str = r#"
# Kiryu & Majima Dancing

<iframe width="640" height="360" src="https://www.youtube.com/embed/qKWRUpYvFzY" title="Friday Night -- Yakuza 0 -- 1 hour" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

- Amazing
    - Characters
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

                let mut options = comrak::Options::default();
                options.render.unsafe_ = true;

                let html = comrak::markdown_to_html(&self.markdown_input, &options);
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
