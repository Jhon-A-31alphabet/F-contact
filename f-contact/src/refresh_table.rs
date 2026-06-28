use rusqlite::Connection;

use fltk_table::{SmartTable};
use crate::database;
use database::service::{fetch_all,count_rows};
use fltk::prelude::WidgetExt;

pub fn refresh_table_(table:&mut SmartTable,conn:&Connection )
{
    table.clear();
    table.set_rows(count_rows(&conn)+1);
    match fetch_all(&conn) {
        Ok(lista_contactos) => {
            table.set_rows(lista_contactos.len() as i32);

            //insertar en la tabla
            for (row_index, c) in lista_contactos.iter().enumerate() {
                let (id, nombre, categoria, descripcion, telefono, organizacion, email) = c;
                let row = row_index as i32;

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
            fltk::dialog::alert(0, 0, &format!("Error al actualizar tabla: {}", e));
        }
    }table.redraw();  // redibujar la tabla
    }

