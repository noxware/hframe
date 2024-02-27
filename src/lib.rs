//! Show HTML content "inside" your [egui](https://github.com/emilk/egui) rendered application. "hframe" stands for "HTML Frame".
//!
//! > **Note:** `hframe` only works when the application is compiled to WebAssembly and run in a browser. But you can still make a desktop up by leveraging [Tauri](https://tauri.app/).
//! >
//! > [egui-tauri-template](https://github.com/noxware/egui-tauri-template)
//! > will serve as good foundation if you need to target both platforms.
//!
//! ## Simple example
//!
//! ```rust
//! use hframe::Aware;
//!
//! const IFRAME: &str = r#"
//! <iframe src="https://www.example.com/"></iframe>
//! "#;
//!
//! pub struct App;
//!
//! impl App {
//!     pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
//!         Self
//!     }
//! }
//!
//! impl eframe::App for App {
//!     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//!         egui::Window::new("Egui Rendered Window")
//!             .show(ctx, |ui| {
//!                 ui.centered_and_justified(|ui| {
//!                     ui.label("This window can be shown on top of the iframe thanks to `.aware()`");
//!                 });
//!             })
//!             .aware();
//!
//!         hframe::HtmlWindow::new("Iframe Window").content(IFRAME).show(ctx);
//!
//!         hframe::sync(ctx);
//!     }
//! }
//! ```
//!
//! For a more complete example see [demo's app.rs](https://github.com/noxware/hframe/blob/master/examples/demo/src/app.rs).
//!
//! ## How does it work?
//!
//! `hframe` renders the HTML content on top of the canvas where `egui` is rendering. Web content is absolute positioned following specifc areas of the canvas. To be
//! abale to show egui windows on top of the HTML content, `hframe` uses different set
//! of HTML masking/cliping techniques combined with other smart logic to produce the
//! illusion of the HTML content being "inside" the egui rendered application.
//!
//! This approach has been chosen after many experiments with alternative techniques
//! which were not able to provide the same level of integration and flexibility.
//!
//! Additionally, some tracking is performed to provide an immediate mode like public
//! API. HTML will only be re-rendered if you change the initially provided content. This
//! can be useful if you want to change the HTML content in a controlled and reactive
//! manner without giving up the immediate mode API.
//!
//! ## Limitations
//!
//! - The current implementation assumes that the canvas takes the whole screen and
//!   it is not scaled. The default configuration of the [eframe_template](https://github.com/emilk/eframe_template) will work.
//! - Currently the API only provides a way to create egui windows with HTML "inside"
//!   but doesn't provide a way to put bare HTML content in other places.
//! - Multiple masking strategies are provided bacause all browsers have problems with
//!   one or the other. The `Auto` strategy tries to guess the best one for the current
//!   browser but it may choose a wrong strategy for unknown browsers.

mod aware;
mod html_window;
mod html_window_state;
pub(crate) mod mask_strategies;
mod registry;
mod utils;

pub use html_window::*;
pub use html_window_state::*;
pub use registry::*;
