use app;

fn main() {
    let app = app::app::Transformer::default();
    let options = eframe::NativeOptions {
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}
