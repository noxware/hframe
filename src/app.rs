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
            egui::Window::new("Counter Controls").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("+").clicked() {
                        self.count += 1;
                    }
                    if ui.button("-").clicked() {
                        self.count -= 1;
                    }
                });
            })
        });

        self.hframes.show(
            ctx,
            "counter",
            "Web Counter",
            &COUNTER_TEMPLATE.replace("{count}", &self.count.to_string()),
        );

        self.hframes.show(ctx, "iframe", "Iframe", IFRAME);
        self.hframes.show(ctx, "video", "Video", VIDEO);
        self.hframes.show(ctx, "yt", "YT", YT);

        self.hframes.sync(ctx);
    }
}
