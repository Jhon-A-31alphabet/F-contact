#![allow(unused)]
use fltk::{
    prelude::*,
    *,
    group::Pack,
    enums::Align,
    input::Input,
    button::Button,
};
use fltk::menu::Choice;
use fltk::enums::FrameType;
use fltk::group::Group;
use fltk::dialog;
use fltk_table::SmartTable;
use rusqlite::Connection;

use crate::{database, refresh_table};
use database::service::{init_db, insert_contact};

// struct para dimensiones
struct PackSize {
    _x: u32,
    _y: u32,
    pack_width: u64,
    pack_height: u64,
}

// table: &mut SmartTable, conn_ : &Connection (según tu firma original)
pub fn add_contact_window(mut table: &mut SmartTable, conn_: &Connection) {
    let mut table_clone_ = table.clone();

    let pack_size_ = PackSize {
        _x: 160,
        _y: 70,
        pack_width: 280,
        pack_height: 400,
    };

    // Inicia la conexión (aunque ya recibes conn_ por parámetro, mantengo tu estilo)
    let db_path_ = "contacts.db";
    let conn = match init_db(db_path_) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error al inicializar la base de datos: {}", e);
            return;
        }
    };

    let mut win = window::Window::default()
        .with_size(600, 500)
        .with_label("Agregar contactos");
    win.make_modal(true);

    let mut frame_add_contact = Group::new(10, 10, 530, 430, "");
    frame_add_contact.set_frame(FrameType::EmbossedFrame);

    let mut save_btn = Button::new(20, 20, 30, 30, "@filesave");
    save_btn.set_label_size(20);

    let mut cancel_btn = Button::new(60, 20, 30, 30, "@returnarrow");
    cancel_btn.set_label_size(20);

    let mut pack = Pack::new(
        pack_size_._x.try_into().unwrap(),
        pack_size_._y.try_into().unwrap(),
        pack_size_.pack_width.try_into().unwrap(),
        pack_size_.pack_height.try_into().unwrap(),
        "",
    );
    pack.set_spacing(20);

    // 1. Nombre
    let mut inp_name = Input::default().with_size(0, 35).with_label("Nombre:");
    inp_name.set_align(Align::Left);

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
    let categorias = ["Trabajo","Fax del trabajo","Amistad","Familiar","Empresa","Servicio","Casa","Movil","Vendedor","Radio"];

    let mut cho_cat = Choice::default().with_size(0, 35).with_label("Categoría:");
    cho_cat.set_align(Align::Left);
    cho_cat.add_choice("Trabajo|Fax del trabajo|Amistad|Familiar|Empresa|Servicio|Casa|Movil|Vendedor|Radio");
    cho_cat.set_value(0);

    pack.end();
    frame_add_contact.end();

    win.end();
    win.show();

    // Guardar contacto: convertir el value() (índice) al string de la opción
    save_btn.set_callback(move |_| {
        // FLTK suele devolver un i32/usize como índice.
        let idx = cho_cat.value() as usize;

        // String final a guardar:
        let categoria_str = categorias.get(idx).copied().unwrap_or(categorias[0]);

        match insert_contact(
            &conn,
            &inp_name.value(),
            categoria_str, 
            &inp_desc.value(),
            &inp_tel.value(),
            &inp_org.value(),
            &inp_email.value(),
        ) {
            Ok(_) => {
                dialog::message(0, 0, "¡Contacto guardado exitosamente!");
                refresh_table::refresh_table_(&mut table_clone_, &conn);
            }
            Err(e) => {
                dialog::alert(0, 0, &format!("Error al guardar el contacto:\n{}", e));
            }
        }
    });

    // (opcional) si quieres cerrar el modal con cancelar
    cancel_btn.set_callback(move |_| {
        win.hide();
    });
}





