# hframe

## About

Show HTML content "inside" your [egui](https://github.com/emilk/egui) rendered application.

## Simple example

```rust
use hframe::Aware;

const IFRAME: &str = r#"
<iframe src="https://www.example.com/"></iframe>
"#;

pub struct App;

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Egui Rendered Window")
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("This window can be shown on top of the iframe thanks to `.aware()`");
                });
            })
            .aware();

        hframe::HtmlWindow::new("Iframe Window").content(IFRAME).show(ctx);

        hframe::sync(ctx);
    }
}
```

For a more complete example see [demo's app.rs](https://github.com/noxware/hframe/blob/master/examples/demo/src/app.rs).

## Running the demo

> Ensure you have `trunk` installed with `cargo install --locked trunk`.

```
git clone https://github.com/noxware/hframe
cd hframe/examples/demo
trunk serve --open
```
