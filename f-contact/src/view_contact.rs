
use std::vec;
use std::rc::Rc;

use fltk::{

    button::Button, draw::width, enums::{Align, FrameType}, group::Group, input::{Input, MultilineInput}, prelude::*, table, window::Window,
};
use fltk::dialog;
use fltk_table::SmartTable;


use crate::database::service::{delete_contact, init_db, update_contact};
use crate::refresh_table;


pub fn view_and_edit_contact_win(contacto: &crate::database::service::Contacto,mut table: &mut SmartTable){
    // 1 & 3: Ventana de 550x450 con su título
    let mut view_and_edit_w = Window::default().with_size(600,500).with_label("Visualizacion de contacto");

    // 4: Todos los widgets encerrados en un frame de tipo embossed frame
    let mut marco_visualizacion = Group::new(10, 10, 530, 430, "");
    marco_visualizacion.set_frame(FrameType::EmbossedFrame);

    // 5: Botones superiores (Label con los íconos internos de FLTK)
    let mut btn_editar = Button::new(20, 20, 30, 30, "@filesaveas");
    let mut _btn_guardar = Button::new(60, 20, 30, 30, "@filesave");
    let mut _btn_borrar = Button::new(100,20,30,30,"Borrar");

    // 2: Inputs (Dejamos 150px de margen a la izquierda para los labels)

    let ancho_input = 360;
    let alto_input = 25;
    let x_input = 160;

    let mut inp_codigo = Input::new(x_input, 60, ancho_input, alto_input, "Codigo:");
    let mut inp_nombre = Input::new(x_input, 95, ancho_input, alto_input, "Nombre:");
    let mut inp_categoria = Input::new(x_input, 130, ancho_input, alto_input, "Categoria:");
    let mut inp_telefono = Input::new(x_input, 165, ancho_input, alto_input, "Numero de telefono:");
    let mut inp_organizacion = Input::new(x_input, 200, ancho_input, alto_input, "Organizacion:");
    let mut inp_correo = Input::new(x_input, 235, ancho_input, alto_input, "Correo:");

    let mut table_clone__ = table.clone();
    let mut table_clone_eliminar = table.clone();

    // Ajuste opcional para alinear mejor los textos a la izquierda del input
    inp_codigo.set_align(Align::Left);
    inp_nombre.set_align(Align::Left);
    inp_categoria.set_align(Align::Left);
    inp_telefono.set_align(Align::Left);
    inp_organizacion.set_align(Align::Left);
    inp_correo.set_align(Align::Left);


    let mut txt_adicional = MultilineInput::new(15, 280, 520, 140, "");

    // Habilitar el modo lectura inicialmente para todo
    inp_codigo.set_readonly(true);
    inp_nombre.set_readonly(true);
    inp_categoria.set_readonly(true);
    inp_telefono.set_readonly(true);
    inp_organizacion.set_readonly(true);
    inp_correo.set_readonly(true);
    txt_adicional.set_readonly(true);


    
 let categorias = vec![
        "Trabajo", 
        "Fax del trabajo", 
        "Amistad", 
        "Familiar", 
        "Empresa", 
        "Servicio", 
        "Casa", 
        "Movil", 
        "Vendedor", 
        "Radio"
    ];


    // Llenado de los inputs con los datos extraídos
    inp_codigo.set_value(&&contacto.id.to_string());
    inp_nombre.set_value(&contacto.nombre);
    
    // Parseo seguro del string de la BD a número para buscar en el diccionario
    let cat_index = contacto.categoria.parse::<usize>().unwrap_or(0);
    let cat_string = categorias.get(cat_index).unwrap_or(&"Desconocido");
    inp_categoria.set_value(cat_string);
    
    inp_telefono.set_value(&contacto.telefono);
    inp_organizacion.set_value(&contacto.organizacion);
    inp_correo.set_value(&contacto.email);
    txt_adicional.set_value(&contacto.descripcion);
   

    marco_visualizacion.end();
    view_and_edit_w.end();
   

    view_and_edit_w.show();
    view_and_edit_w.make_modal(true);



    let path = "db/contacts.db";
    let  conn__ = match init_db(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error al inicializar la base de datos: {}", e);
            return;
        }
    };

    let inp_codigo_clone = inp_codigo.value(); // clonado para ser usado en btn_guardar
    let txt_adicional_clone = txt_adicional.clone();
    let inp_telefono_clone = inp_telefono.clone();
    let inp_categoria_clone = inp_categoria.clone();
    let inp_correo_clone = inp_correo.clone();
    let inp_nombre_clone = inp_nombre.clone();
    let inp_organizacion_clone = inp_organizacion.clone();

    let inp_codigo_borrar =inp_codigo.value();
    let  conn_shared__ = Rc::new(conn__);
    let conn_shared_borrar = Rc::clone(&conn_shared__)
;



    btn_editar.set_callback(move |_| {
        // Se habilitan todos para escritura, excepto el código
        inp_nombre.set_readonly(false);
        inp_categoria.set_readonly(true);
        inp_telefono.set_readonly(false);
        inp_organizacion.set_readonly(false);
        inp_correo.set_readonly(false);
        txt_adicional.set_readonly(false);

    });


    _btn_guardar.set_callback(move |_|{

        match update_contact(&conn_shared__,inp_codigo_clone.parse().unwrap() , &inp_nombre_clone.value(), &inp_categoria_clone.value(), &inp_telefono_clone.value(), &txt_adicional_clone.value(), &inp_organizacion_clone.value(), &inp_correo_clone.value())
        {
        Ok(_) => {
                dialog::message(0, 0, "¡Contacto editado exitosamente");
                refresh_table::refresh_table_(&mut table_clone__, &conn_shared__);

            }
            Err(e)=> {
                    dialog::alert(0, 0, &format!("Error al guardar el contacto:\n{}", e));

            }
        }
    });


    _btn_borrar.set_callback(move|_|{
        match delete_contact(&conn_shared_borrar,inp_codigo_borrar.parse().unwrap()) {
            Ok(_) => {
                dialog::message(0, 0, "¡Contacto Borrado exitosamente");
                refresh_table::refresh_table_(&mut table_clone_eliminar, &conn_shared_borrar);

            }Err(e) => {
                dialog::alert(0, 0,&format!("Error al guardar el contacto:\n{}", e));
            }
        }
    });
}


