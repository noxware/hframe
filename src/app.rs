use crate::hframe::HframeRegistry;

const IFRAME: &str = r#"
<iframe src="https://www.example.com/"></iframe>
"#;

const VIDEO: &str = r#"
<video controls>
    <source src="https://www.w3schools.com/html/mov_bbb.mp4" type="video/mp4">
    <source src="https://www.w3schools.com/html/mov_bbb.ogg" type="video/ogg">
    Your browser does not support HTML5 video.
</video>
"#;

const YT: &str = r#"
<iframe width="1280" height="720" src="https://www.youtube.com/embed/PCp2iXA1uLE" title="FREDERIC 「oddloop」Music Video" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
"#;

const COUNTER_TEMPLATE: &str = r#"
<div style="display: flex; justify-content: center; align-items: center; padding: 8px; color: red; font: 36px sans-serif;">
    <span>{count}</span>
</div>
"#;

#[allow(unused_macros)]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

pub struct TemplateApp {
    count: i32,
    video_open: bool,
    hframes: HframeRegistry,
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let style = egui::Style {
            visuals: egui::Visuals::dark(),
            ..Default::default()
        };

        cc.egui_ctx.set_style(style);

        Self {
            count: 0,
            video_open: true,
            hframes: HframeRegistry::new(),
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.hframes.aware({
            egui::Window::new("None").show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Empty");
                });
            })
        });

        self.hframes.aware({
            egui::Window::new("Devtools").show(ctx, |ui| {
                let video_toggle_text = if self.video_open {
                    "Force close video"
                } else {
                    "Open video"
                };
                if ui.button(video_toggle_text).clicked() {
                    self.video_open = !self.video_open;
                }
                ui.horizontal(|ui| {
                    ui.label("Counter controls: ");
                    if ui.button("+").clicked() {
                        self.count += 1;
                    }
                    if ui.button("-").clicked() {
                        self.count -= 1;
                    }
                });
            })
        });

        self.hframes.show_window(
            ctx,
            "counter",
            "Web Counter",
            &COUNTER_TEMPLATE.replace("{count}", &self.count.to_string()),
        );

        self.hframes.show_window(ctx, "iframe", "Iframe", IFRAME);

        if self.video_open {
            self.hframes.show_window(ctx, "video", "Video", VIDEO);
        }

        self.hframes.show_window(ctx, "yt", "YT", YT);

        self.hframes.sync(ctx);
    }
}
