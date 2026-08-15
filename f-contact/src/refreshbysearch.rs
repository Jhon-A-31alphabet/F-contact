use rusqlite::Connection;
use fltk_table::SmartTable;
use fltk::prelude::WidgetExt;
use crate::database;
use database::service::{Contacto, search};

pub fn refreshby_search(table: &mut SmartTable, conn: &Connection, vec: &Result<Vec<Contacto>, rusqlite::Error>) {
    table.clear();

    if let Ok(lista_usuarios) = vec {
        // Obtener usuario en la posicion 0
        if let Some(usuario_criterio) = lista_usuarios.get(0) {
            
            let criterio_busqueda = &usuario_criterio.nombre;   // <- se toma en cuenta el string de aqui para buscar...

            match search(&conn, criterio_busqueda) {
                Ok(contactos_buscados) => {
                    for (row_index, c) in contactos_buscados.iter().enumerate() {
                        let row = row_index as i32;

                        // Acceso directo a los campos de la estructura Usuario ubicada en database/service.rs
                        table.set_cell_value(row, 0, &c.id.to_string());
                        table.set_cell_value(row, 1, &c.nombre);
                        table.set_cell_value(row, 2, &c.categoria);
                        table.set_cell_value(row, 3, &c.telefono);
                        table.set_cell_value(row, 4, &c.organizacion);
                        table.set_cell_value(row, 5, &c.email);
                        table.set_cell_value(row, 6, &c.descripcion);
                    }
                }
                Err(e) => {
                    fltk::dialog::alert(0, 0, &format!("Error en la búsqueda: {}", e));
                }
            }
        } else {
            fltk::dialog::alert(0, 0, "Error la busqueda no pudo realizarse");
        }
    } else if let Err(e) = vec {
        fltk::dialog::alert(0, 0, &format!("Error en el Result de origen: {}", e));
    }


    table.redraw();
}

