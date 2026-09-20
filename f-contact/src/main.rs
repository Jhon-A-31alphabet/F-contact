
use std::rc::Rc;
use fltk::{app, button::Button, dialog, enums::FrameType, frame::Frame, group::{Pack, PackType}, input::Input, menu::Choice, prelude::*, window::Window};
use fltk_table::{SmartTable, TableOpts};
use fltk::enums::CallbackTrigger;


mod add_contacts_window;
use crate::add_contacts_window::add_contact_window;
mod view_contact;
use crate::view_contact::view_and_edit_contact_win;
use fltk_theme::{ColorTheme, color_themes};


mod database;
use crate::database::service::{init_db,fetch_all,count_rows,search,get_contact_by_category};
mod refresh_table;

mod refreshbysearch;



fn main() {
	let db_path = "db/contacts.db";
    let conn = match init_db(db_path) {
        Ok(c) => {
            c
        }
        Err(e) => {
         eprintln!("Error al inicializar la base de datos: {}", e);
            return;
        }
    };





    let app = app::App::default();
    let (screen_w, screen_h) = app::screen_size();
    let color_theme = ColorTheme::new(color_themes::TAN_THEME);
    color_theme.apply(); // color del tema aplicado....

    let mut wind = Window::new(0, 0, screen_w as i32 - 250, screen_h  as i32 - 200, "Contactos"); // el tamano dependera de la resolucion
    //de la pantalla del usuario //

    let pack_x = 115;
    let pack_y = 40;
    let btn_size = 50;
    let input_w = 600;

    let height = btn_size;
    let (center_frame_w,center_frame_h) =(screen_w as i32 -300 ,screen_h as i32 -250 ); //tamano del frame

    let mut center_frame = Frame::default().with_size(center_frame_w,center_frame_h).center_of(&wind);
    center_frame.set_frame(FrameType::EmbossedFrame);



    let mut h_pack = Pack::new(pack_x, pack_y, btn_size + 30 + input_w + btn_size, height, "");
    h_pack.set_type(PackType::Horizontal);
    h_pack.set_spacing(20);

    // Boton para agregar
    let mut add_button = Button::new(0, 0, btn_size, height, "@+");
    add_button.set_label_size(20);


    // Input de busqueda   se pretende buscar contactos o palabras claves que coincidan

    let mut input = Input::new(0, -0, input_w, height,"");
    input.set_text_size(20);

    // boton para buscar .....................................
    let mut search_button = Button::new(0, -0, btn_size, height, "@search");
    search_button.set_label_size(18);

    //categoria label
    let mut cat_label =Frame::new(0,0,80,height,"Categorias");
    cat_label.set_label_size(18);

    let mut choice = Choice::new(0, 0, 120, height - 10, ""); // ancho 120, altura ajustada al pack
    choice.add_choice("Todos|Trabajo|Fax del trabajo|Amistad|Familiar|Empresa|Servicio|Casa|Movil|Vendedor|Radio");  // aqui van las categorias
	choice.set_value(0);
	choice.set_text_size(12);

    
	




    h_pack.end(); // cierre del pack -------.

    

    let mut table = SmartTable::default()
		.with_size(1000,390)
		.center_of_parent()
		.with_opts(TableOpts{
		rows: count_rows(&conn)+1,
        cols: 7,
        editable: false,
        ..Default::default()
		});
		(&mut table).set_col_header_height(32);
		(&mut table).set_col_width(1,250);
		(&mut table).set_col_width(0,250);
		(&mut table).set_col_width(2,250);
		(&mut table).set_col_width(3,250);

	    match fetch_all(&conn){
		Ok(lista_contactos)=>{
		for (row_index, c) in lista_contactos.iter().enumerate() {
            let (id, nombre, categoria, descripcion, telefono, organizacion, email) = c;
            let row = row_index as i32;
             table.set_rows(lista_contactos.len() as i32);


            // Insertar en cada columna dada la fila (numero del iterador)
            table.set_cell_value(row, 0, &id.to_string());
            table.set_cell_value(row, 1, nombre);
            table.set_cell_value(row, 2, categoria);
            table.set_cell_value(row, 3, telefono);
            table.set_cell_value(row, 4, organizacion);
            table.set_cell_value(row, 5, email);
            table.set_cell_value(row, 6, descripcion);

            }
        
		}
        
	Err(e) => {
        fltk::dialog::alert(0, 0, &format!("Error: {}", e));
        }
 }  




    wind.end();
    wind.show();

    let  conn_shared = Rc::new(conn);
    let mut table_clone = table.clone();
    
    let conn_for_btn = conn_shared.clone();
    let mut refreshed_table = table_clone.clone();




    let mut table_for_search = table.clone(); // Su propio clon de la tabla
    let conn_shared_clone = conn_shared.clone();
    let input_clone = input.clone();

    let mut table_to_fetchall = table.clone();  // clon del table para comparar si el el input esta vacio.......
    
    add_button.set_callback(move|_| {
        add_contact_window(&mut table_clone,&conn_for_btn);

         refreshed_table.append_empty_row("-");
    });

    let value = conn_shared.clone();

    search_button.set_callback(move|_|{

        if input.value().is_empty() {
                        let _ = fetch_all(&conn_shared_clone);
                        refresh_table::refresh_table_(&mut table_to_fetchall, &conn_shared_clone);

            
        }else{
            let p = search(&value,&input_clone.value());
            refreshbysearch::refreshby_search(&mut table_for_search, &conn_shared_clone, &p);


        }

    
    });


    // Clonamos las referencias para el Choice de categorías
    let mut table_for_choice = table.clone();
    let conn_for_choice = conn_shared.clone();

    choice.set_callback(move |c| {
        if let Some(selected_cat) = c.choice() {
            
            if selected_cat == "Todos" {
                // AQUÍ MANEJAMOS LA TUPLA QUE DEVUELVE fetch_all()
                match fetch_all(&conn_for_choice) {
                    Ok(lista_contactos) => {
                        table_for_choice.set_rows(lista_contactos.len() as i32);
                        
                        for (row_index, contacto) in lista_contactos.iter().enumerate() {
                            let (id, nombre, categoria, descripcion, telefono, organizacion, email) = contacto;
                            let row = row_index as i32;
                            
                            table_for_choice.set_cell_value(row, 0, &id.to_string());
                            table_for_choice.set_cell_value(row, 1, nombre);
                            table_for_choice.set_cell_value(row, 2, categoria);
                            table_for_choice.set_cell_value(row, 3, telefono);
                            table_for_choice.set_cell_value(row, 4, organizacion);
                            table_for_choice.set_cell_value(row, 5, email);
                            table_for_choice.set_cell_value(row, 6, descripcion);
                        }
                        table_for_choice.redraw();
                    }
                    Err(e) => fltk::dialog::alert(0, 0, &format!("Error al cargar todos: {}", e)),
                }

            } else {
                // AQUÍ MANEJAMOS EL STRUCT "Usuario" QUE DEVUELVE get_contact_by_category()
                match get_contact_by_category(&conn_for_choice, &selected_cat) {
                    Ok(lista_contactos) => {
                        table_for_choice.set_rows(lista_contactos.len() as i32);
                        
                        for (row_index, contacto) in lista_contactos.iter().enumerate() {
                            let row = row_index as i32;
                            
                            table_for_choice.set_cell_value(row, 0, &contacto.id.to_string());
                            table_for_choice.set_cell_value(row, 1, &contacto.nombre);
                            table_for_choice.set_cell_value(row, 2, &contacto.categoria);
                            table_for_choice.set_cell_value(row, 3, &contacto.telefono);
                            table_for_choice.set_cell_value(row, 4, &contacto.organizacion);
                            table_for_choice.set_cell_value(row, 5, &contacto.email);
                            table_for_choice.set_cell_value(row, 6, &contacto.descripcion);
                        }
                        table_for_choice.redraw();
                    }
                    Err(e) => fltk::dialog::alert(0, 0, &format!("Error al filtrar categoría: {}", e)),
                }
            }
        }
    });


    //clicks una vez presionado una celda... dada una celda cualquiera..

        table.set_trigger(CallbackTrigger::Release);

        let conn_para_celda = conn_shared.clone();
        let table_para_celda = table.clone();
        let mut table_editar_ver_cont = table.clone();



        table.set_callback(move |t|{
        if let fltk::table::TableContext::Cell = t.callback_context() {
            if fltk::app::event() == fltk::enums::Event::Released {
                
                
                    let row = t.callback_row();
                    let id_str = table_para_celda.cell_value(row, 0);
                    
                   

                    // Usamos trim() por si la celda tiene espacios en blanco invisibles
                    match id_str.trim().parse::<i64>() {
                        Ok(id) => {


                            match database::service::get_contact_by_id(&conn_para_celda, id) {
                                Ok(vector_resultados) => {
                                    if let Some(contacto) = vector_resultados.first() {
                                       //usa el parametro contacto para llenar los campos

                                        view_and_edit_contact_win(contacto,&mut table_editar_ver_cont);
                                    } else {
                                        dialog::message_default("Advertencia: La BD no devolvió ningún contacto con el ID leido");
                                    }
                                },
                                Err(_e) => dialog::message_default("Error al consultar la Base de Datos")
                            }
                        },
                        Err(_e) => {
                            dialog::message_default("categoria no detectada");
                        }
                    }

                }
            }
        //}
    });

      





    app.run().unwrap();

}

