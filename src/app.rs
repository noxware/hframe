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

#[derive(Default)]
pub struct App {
    counter_open: bool,
    iframe_open: bool,
    yt_open: bool,
    count: i32,
    video_open: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let style = egui::Style {
            visuals: egui::Visuals::dark(),
            ..Default::default()
        };

        cc.egui_ctx.set_style(style);

        Self {
            video_open: true,
            counter_open: true,
            iframe_open: true,
            yt_open: true,
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        hframe::aware(ctx, {
            egui::Window::new("None").show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Empty");
                });
            })
        });

        hframe::aware(ctx, {
            egui::Window::new("Devtools").show(ctx, |ui| {
                ui.label(format!(
                    "Mask Strategy: {}",
                    hframe::mask_strategy_meta(ctx).name
                ));
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

        hframe::Window::new(
            "counter",
            "Web Counter",
            &COUNTER_TEMPLATE.replace("{count}", &self.count.to_string()),
        )
        .open(&mut self.counter_open)
        .show(ctx);

        hframe::Window::new("iframe", "Iframe", IFRAME)
            .open(&mut self.iframe_open)
            .show(ctx);

        if self.video_open {
            hframe::Window::new("video", "Video", VIDEO).show(ctx);
        }

        hframe::Window::new("yt", "YT", YT)
            .open(&mut self.yt_open)
            .show(ctx);

        hframe::sync(ctx);
    }
}
