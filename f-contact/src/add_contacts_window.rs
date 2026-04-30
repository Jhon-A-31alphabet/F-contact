use fltk::{prelude::*, *,group::Pack,
    enums::{Align},input::Input,button::Button};
use fltk::menu::Choice;



struct PackSize{
	_x : u32,
	_y : u32,
	pack_width: u64,
	pack_height:u64,
}




pub fn add_contact_window(){

	let  pack_size_ = PackSize
	{
	_x:160,
	_y : 70,
	pack_width :280,
	pack_height : 400
	};


    let mut win = window::Window::default().with_size(600,500).with_label("Agregar contactos");
    win.make_modal(true);

    let mut save_btn = Button::new(20, 10, 100, 50, "@filesave");
    save_btn.set_label_size(20);

    let mut cancel_btn = Button::new(130, 10, 100, 50, "Salir");
    cancel_btn.set_label_size(20);



    let mut pack = Pack::new(pack_size_._x.try_into().unwrap(),
    pack_size_._y.try_into().unwrap(),
    pack_size_.pack_width.try_into().unwrap(),
    pack_size_.pack_height.try_into().unwrap(), "");
    pack.set_spacing(20); // Espacio entre cada fila


    // 1. Nombre
    let mut inp_name = Input::default().with_size(0, 35).with_label("Nombre:");
    inp_name.set_align(Align::Left); // Label a la izquierda

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
