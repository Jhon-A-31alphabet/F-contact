//! Módulo para la visualización, edición y eliminación de los detalles de un contacto existente.

use std::rc::Rc;

use fltk::{

    button::Button, enums::{Align, FrameType}, group::Group, input::{Input, MultilineInput}, prelude::*, window::Window,
};
use fltk::dialog;
use fltk_table::SmartTable;
use rust_i18n::t;


use crate::database::service::{delete_contact, init_db, update_contact};
use crate::{i18n, refresh_table};

/// Muestra una ventana modal para visualizar, editar o eliminar la información detallada de un contacto.
///
/// # Parámetros
/// - `contacto`: Referencia al contacto obtenido de la base de datos.
/// - `table`: Referencia mutable a la tabla `SmartTable` de la interfaz para refrescarla tras cambios.
///
/// # Retorno
/// - No retorna ningún valor (`()`).
///
/// # Funciones y Módulos Utilizados
/// - `Window::default()` / `.make_modal()`: Crea la ventana flotante de vista/edición.
/// - `Group::new()` / `FrameType::EmbossedFrame`: Define el marco contenedor visual para los widgets.
/// - `Button::new()`: Crea los botones de la barra superior (editar, guardar, borrar).
/// - `Input::new()` / `MultilineInput::new()`: Campos de entrada para desplegar y modificar datos del contacto.
/// - `init_db`: Inicializa/conecta la base de datos SQLite para realizar consultas.
/// - `update_contact`: Actualiza los registros del contacto en la base de datos SQLite.
/// - `delete_contact`: Elimina el registro del contacto en la base de datos.
/// - `refresh_table_`: Vuelve a cargar y dibujar la tabla principal con los datos actualizados.
/// - `dialog::message` / `dialog::alert`: Muestran ventanas emergentes con avisos o mensajes de error.
pub fn view_and_edit_contact_win(contacto: &crate::database::service::Contacto, table: &mut SmartTable){
    // 1 & 3: Ventana de 550x450 con su título
    let mut view_and_edit_w = Window::default().with_size(600,500).with_label(&t!("view.title"));

    // 4: Todos los widgets encerrados en un frame de tipo embossed frame
    let mut marco_visualizacion = Group::new(10, 10, 530, 430, "");
    marco_visualizacion.set_frame(FrameType::EmbossedFrame);

    // 5: Botones superiores (Label con los íconos internos de FLTK)
    let mut btn_editar = Button::new(20, 20, 30, 30, "@filesaveas");
    let mut _btn_guardar = Button::new(60, 20, 30, 30, "@filesave");
    let mut _btn_borrar = Button::new(100,20,50,30,&*t!("view.delete"));

    // 2: Inputs (Dejamos 150px de margen a la izquierda para los labels)

    let ancho_input = 360;
    let alto_input = 25;
    let x_input = 160;

    let mut inp_codigo = Input::new(x_input, 60, ancho_input, alto_input, &*t!("view.code"));
    let mut inp_nombre = Input::new(x_input, 95, ancho_input, alto_input, &*t!("view.name"));
    let mut inp_categoria = Input::new(x_input, 130, ancho_input, alto_input, &*t!("view.category"));
    let mut inp_telefono = Input::new(x_input, 165, ancho_input, alto_input, &*t!("view.phone"));
    let mut inp_organizacion = Input::new(x_input, 200, ancho_input, alto_input, &*t!("view.organization"));
    let mut inp_correo = Input::new(x_input, 235, ancho_input, alto_input, &*t!("view.email"));

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


    
    // Valor estable almacenado en la base de datos (se conserva sin traducir).
    let categoria_db = contacto.categoria.clone();

    // Llenado de los inputs con los datos extraídos
    inp_codigo.set_value(&&contacto.id.to_string());
    inp_nombre.set_value(&contacto.nombre);

    // Se muestra la etiqueta traducida de la categoría para el idioma activo.
    inp_categoria.set_value(&i18n::category_label_from_db(&categoria_db));

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
            eprintln!("{}", t!("errors.db_init", error = e.to_string()));
            return;
        }
    };

    let inp_codigo_clone = inp_codigo.value(); // clonado para ser usado en btn_guardar
    let txt_adicional_clone = txt_adicional.clone();
    let inp_telefono_clone = inp_telefono.clone();
    let categoria_db_clone = categoria_db.clone();
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

        match update_contact(&conn_shared__,inp_codigo_clone.parse().unwrap() , &inp_nombre_clone.value(), &categoria_db_clone, &inp_telefono_clone.value(), &txt_adicional_clone.value(), &inp_organizacion_clone.value(), &inp_correo_clone.value())
        {
        Ok(_) => {
                dialog::message(0, 0, &t!("view.updated"));
                refresh_table::refresh_table_(&mut table_clone__, &conn_shared__);

            }
            Err(e)=> {
                    dialog::alert(0, 0, &t!("view.save_error", error = e.to_string()));

            }
        }
    });


    _btn_borrar.set_callback(move|_|{
        match delete_contact(&conn_shared_borrar,inp_codigo_borrar.parse().unwrap()) {
            Ok(_) => {
                dialog::message(0, 0, &t!("view.deleted"));
                refresh_table::refresh_table_(&mut table_clone_eliminar, &conn_shared_borrar);

            }Err(e) => {
                dialog::alert(0, 0,&t!("view.save_error", error = e.to_string()));
            }
        }
    });
}