
use std::vec;

use fltk::{

    button::Button,
    enums::{Align, FrameType},
    group::Group,
    input::{Input, MultilineInput},
    prelude::*,
    window::Window,
};



pub fn view_and_edit_contact_win(contacto: &crate::database::service::Contacto){
    // 1 & 3: Ventana de 550x450 con su título
    let mut view_and_edit_w = Window::default().with_size(600,500).with_label("Visualizacion de contacto");

    // 4: Todos los widgets encerrados en un frame de tipo embossed frame
    let mut marco_visualizacion = Group::new(10, 10, 530, 430, "");
    marco_visualizacion.set_frame(FrameType::EmbossedFrame);

    // 5: Botones superiores (Label con los íconos internos de FLTK)
    let mut btn_editar = Button::new(20, 20, 30, 30, "@filesaveas");
    let _btn_guardar = Button::new(60, 20, 30, 30, "@filesave");

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

// La posicion de cada elemento en el vector sirve para representar el numero de cada categoria
//antes de  inyectar datos a la tabla SQL.
    
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
    inp_codigo.set_value(&contacto.id.to_string());
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


    btn_editar.set_callback(move |_| {
        // Se habilitan todos para escritura, excepto el código
        inp_nombre.set_readonly(false);
        inp_categoria.set_readonly(false);
        inp_telefono.set_readonly(false);
        inp_organizacion.set_readonly(false);
        inp_correo.set_readonly(false);
        txt_adicional.set_readonly(false);
    });
}


