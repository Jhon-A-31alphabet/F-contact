use fltk::{app, prelude::*, window::Window};

fn main() {
    let app = app::App::default();

    let (screen_w,screen_h) = app::screen_size();

    let mut wind = Window::new(0, 0, screen_w as i32, screen_h as i32, "Contactos_");

    wind.end();
    wind.show();
    app.run().unwrap();
}
