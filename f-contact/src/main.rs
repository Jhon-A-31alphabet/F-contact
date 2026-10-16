//! Punto de entrada principal de la aplicación para gestión de contactos construida con FLTK y SQLite.

use std::rc::Rc;
use fltk::{app, button::Button, dialog, enums::FrameType, frame::Frame, group::{Pack, PackType}, input::Input, menu::Choice, prelude::*, window::Window};
use fltk_table::{SmartTable, TableOpts};
use fltk::enums::CallbackTrigger;
use rust_i18n::t;

rust_i18n::i18n!("locales", fallback = "es");

mod i18n;

mod add_contacts_window;
use crate::add_contacts_window::add_contact_window;
mod view_contact;
use crate::view_contact::view_and_edit_contact_win;
use fltk_theme::{ColorTheme, color_themes};


mod database;
use crate::database::service::{init_db,fetch_all,count_rows,search,get_contact_by_category};
mod refresh_table;

mod refreshbysearch;

use include_dir::include_dir;
const __: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/src/db");


/// Función principal que arranca la aplicación gráfica, crea la ventana primaria y gestiona los eventos del sistema.
///
/// # Retorno
/// - No retorna ningún valor (`()`).
///
/// # Funciones y Módulos Utilizados
/// - `app::App::default()` / `app.run()`: Inicia y ejecuta el ciclo de eventos de FLTK.
/// - `ColorTheme::apply()`: Aplica un tema visual predefinido a la GUI.
/// - `Window::new()` / `Frame::default()` / `Pack::new()`: Estructuran la ventana principal y la disposición de los elementos.
/// - `Button::new()` / `Input::new()` / `Choice::new()`: Elementos interactivos para añadir, buscar y filtrar contactos.
/// - `SmartTable::default()`: Componente de tabla que despliega la lista general de contactos.
/// - `init_db`: Inicializa la base de datos SQLite en el archivo especificado.
/// - `fetch_all`: Recupera la lista completa de contactos almacenados.
/// - `search`: Realiza búsquedas por nombre o coincidencias de texto.
/// - `get_contact_by_category`: Filtra los registros según la categoría seleccionada en la lista desplegable.
/// - `get_contact_by_id`: Recupera un contacto específico al hacer clic en una celda de la tabla para editarlo.
fn main() {
    rust_i18n::set_locale("es");

	let db_path = "db/contacts.db";
    let conn = match init_db(db_path) {
        Ok(c) => {
            c
        }
        Err(e) => {
         eprintln!("{}", t!("errors.db_init", error = e.to_string()));
            return;
        }
    };





    let app = app::App::default();
    let (screen_w, screen_h) = app::screen_size();
    let color_theme = ColorTheme::new(color_themes::TAN_THEME);
    color_theme.apply(); // color del tema aplicado....

    let mut wind = Window::new(0, 0, screen_w as i32 - 250, screen_h  as i32 - 200, &*t!("app.title")); // el tamano dependera de la resolucion
    //de la pantalla del usuario //

    let (center_frame_w,center_frame_h) =(screen_w as i32 -300 ,screen_h as i32 -250 ); //tamano del frame

    let mut center_frame = Frame::default().with_size(center_frame_w,center_frame_h).center_of(&wind);
    center_frame.set_frame(FrameType::EmbossedFrame);

    let margin = 20;
    let pack_x = center_frame.x() + margin;
    let pack_y = center_frame.y() + margin;
    let available_width = center_frame_w - (margin * 2);

    let btn_size = 45;
    let spacing = 12;
    let height = btn_size;

    let lang_label_w = 70;
    let lang_choice_w = 110;

    let fixed_elements_width =
        (btn_size * 2) + 80 + 120 + lang_label_w + lang_choice_w + (spacing * 6);
    let input_w = available_width - fixed_elements_width;



    let mut h_pack = Pack::new(pack_x, pack_y, available_width, height, "");
    h_pack.set_type(PackType::Horizontal);
    h_pack.set_spacing(spacing);

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
    let mut cat_label =Frame::new(0,0,80,height,&*t!("main.categories"));
    cat_label.set_label_size(18);

    let mut choice = Choice::new(0, 0, 120, height - 10, ""); // ancho 120, altura ajustada al pack
    choice.add_choice(&i18n::category_choice_string());  // aqui van las categorias
	choice.set_value(0);
	choice.set_text_size(12);

    // idioma label
    let mut lang_label = Frame::new(0, 0, lang_label_w, height, &*t!("main.language"));
    lang_label.set_label_size(18);

    // selector de idioma
    let mut lang_choice = Choice::new(0, 0, lang_choice_w, height - 10, "");
    let lang_items = i18n::LOCALES.iter().map(|(_, name)| *name).collect::<Vec<_>>().join("|");
    lang_choice.add_choice(&lang_items);
    lang_choice.set_value(0);
    lang_choice.set_text_size(12);

    h_pack.end(); // cierre del pack -------.

    

    let table_y = pack_y + height + 15;
    let table_h = center_frame_h - (table_y - center_frame.y()) - margin;

    let mut table = SmartTable::default()
		.with_pos(pack_x, table_y)
		.with_size(available_width,table_h)
		.with_opts(TableOpts{
		rows: count_rows(&conn)+1,
        cols: 7,
        editable: false,
        ..Default::default()
		});
		(&mut table).set_col_header_height(32);
		let col_w = available_width / 4;
		(&mut table).set_col_width(0,col_w);
		(&mut table).set_col_width(1,col_w);
		(&mut table).set_col_width(2,col_w);
		(&mut table).set_col_width(3,col_w);

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
        dialog::alert(0, 0, &t!("errors.fetch_all", error = e.to_string()));
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
        let index = c.value();

        if index <= 0 {
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
                Err(e) => dialog::alert(0, 0, &t!("errors.load_all", error = e.to_string())),
            }
        } else if let Some(db_cat) = i18n::CATEGORY_DB_VALUES.get((index - 1) as usize) {
            // AQUÍ MANEJAMOS EL STRUCT "Usuario" QUE DEVUELVE get_contact_by_category()
            match get_contact_by_category(&conn_for_choice, db_cat) {
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
                Err(e) => dialog::alert(0, 0, &t!("errors.filter_category", error = e.to_string())),
            }
        }
    });

    // Selector de idioma: cambia el idioma activo y retraduce las etiquetas de la ventana principal.
    let mut wind_for_lang = wind.clone();
    let mut cat_label_for_lang = cat_label.clone();
    let mut choice_for_lang = choice.clone();
    let mut lang_label_for_lang = lang_label.clone();

    lang_choice.set_callback(move |c| {
        let index = c.value();
        if let Some((code, _)) = i18n::LOCALES.get(index as usize) {
            rust_i18n::set_locale(code);
        }

        wind_for_lang.set_label(&t!("app.title"));
        cat_label_for_lang.set_label(&t!("main.categories"));
        lang_label_for_lang.set_label(&t!("main.language"));

        choice_for_lang.clear();
        choice_for_lang.add_choice(&i18n::category_choice_string());
        choice_for_lang.set_value(0);
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
                                        dialog::message_default(&t!("errors.no_contact"));
                                    }
                                },
                                Err(_e) => dialog::message_default(&t!("errors.query_db"))
                            }
                        },
                        Err(_e) => {
                            dialog::message_default(&t!("errors.category_not_detected"));
                        }
                    }

                }
            }
        //}
    });

      





    app.run().unwrap();

}

