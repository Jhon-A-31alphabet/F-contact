use fltk::{prelude::*, *,group::Pack,
    enums::{Align},input::Input,button::Button};
use fltk::menu::Choice;


pub fn add_contact_window(){

    let mut win = window::Window::default().with_size(600,500).with_label("Agregar contactos");
    win.make_modal(true);

    let mut guardar_btn = Button::new(20, 10, 100, 50, "@filesave");
    guardar_btn.set_label_size(20);

    let mut cancelar_btn = Button::new(130, 10, 100, 50, "Salir");
    cancelar_btn.set_label_size(20);



    let mut pack = Pack::new(160, 70, 280, 400, "");
    pack.set_spacing(20); // Espacio entre cada fila


    // 1. Nombre
    let mut inp_nombre = Input::default().with_size(0, 35).with_label("Nombre:");
    inp_nombre.set_align(Align::Left); // Label a la izquierda

    // 2. Descripción
    let mut inp_desc = Input::default().with_size(0, 35).with_label("Descripción:");
    inp_desc.set_align(Align::Left);

    // 3. Número de teléfono
    let mut inp_tel = Input::default().with_size(0, 35).with_label("Número de teléfono:");
    inp_tel.set_align(Align::Left);

    // 4. Organización
    let mut inp_org = Input::default().with_size(0, 35).with_label("Organización:");
    inp_org.set_align(Align::Left);

    // 5. Email
    let mut inp_email = Input::default().with_size(0, 35).with_label("Email:");
    inp_email.set_align(Align::Left);

    // 6. Categoría (Choice)
    let mut cho_cat = Choice::default().with_size(0, 35).with_label("Categoría:");
    cho_cat.set_align(Align::Left);
    cho_cat.add_choice("Personal|Trabajo|Estudiante|Otros");
    cho_cat.set_value(0);



    pack.end();




    win.end();
    win.show();


}
