use crate::iframe::IframeRegistry;

const IFRAME_URL: &str = r#"
<iframe src="https://www.example.com/"></iframe>
"#;

const VIDEO_URL: &str = r#"
<video controls>
    <source src="https://www.w3schools.com/html/mov_bbb.mp4" type="video/mp4">
    <source src="https://www.w3schools.com/html/mov_bbb.ogg" type="video/ogg">
    Your browser does not support HTML5 video.
</video>
"#;

const YT_URL: &str = r#"
<iframe width="1280" height="720" src="https://www.youtube.com/embed/PCp2iXA1uLE" title="FREDERIC 「oddloop」Music Video" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
"#;

#[allow(unused_macros)]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

pub struct TemplateApp {
    iframe_id_counter: usize,
    new_iframe_content: String,
    iframes: IframeRegistry,
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            new_iframe_content: IFRAME_URL.to_string(),
            iframes: IframeRegistry::new(),
            iframe_id_counter: 0,
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.iframes.aware({
            egui::Window::new("None").show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("");
                });
            })
        });

        let devtools = egui::Window::new("Devtools").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Spawn Iframe").clicked() {
                        self.iframe_id_counter += 1;

                        self.iframes.insert(
                            &format!("iframe-{}", self.iframe_id_counter),
                            &format!("Iframe {}", self.iframe_id_counter),
                            IFRAME_URL,
                        );
                    }

                    if ui.button("Spawn Video").clicked() {
                        self.iframe_id_counter += 1;

                        self.iframes.insert(
                            &format!("video-{}", self.iframe_id_counter),
                            &format!("Video {}", self.iframe_id_counter),
                            VIDEO_URL,
                        );
                    }

                    if ui.button("Spawn Youtube").clicked() {
                        self.iframe_id_counter += 1;

                        self.iframes.insert(
                            &format!("yt-{}", self.iframe_id_counter),
                            &format!("Youtube {}", self.iframe_id_counter),
                            YT_URL,
                        );
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Custom content:");
                    ui.text_edit_multiline(&mut self.new_iframe_content);
                    if ui.button("Add").clicked() {
                        self.iframe_id_counter += 1;

                        self.iframes.insert(
                            &format!("custom-{}", self.iframe_id_counter),
                            &format!("custom {}", self.iframe_id_counter),
                            &self.new_iframe_content,
                        );
                    }
                });
            });
        });

        self.iframes.aware(devtools);
        self.iframes.show(ctx);
    }
}
