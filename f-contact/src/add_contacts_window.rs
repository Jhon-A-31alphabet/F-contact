use fltk::{prelude::*, *};

pub fn add_contact_window(){

    let mut win = window::Window::default().with_size(400,350).with_label("Agregar contactos");
    let _label = frame::Frame::default().with_label("Check this text:");
    win.end();
    win.show();


}
