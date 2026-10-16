//! Módulo encargado de recargar y redibujar la tabla principal con todos los registros de contactos.

use rusqlite::Connection;
use rust_i18n::t;

use fltk_table::{SmartTable};
use crate::database;
use database::service::{fetch_all,count_rows};
use fltk::prelude::WidgetExt;

/// Consulta la base de datos y llena la tabla de la interfaz con la totalidad de los contactos.
///
/// # Parámetros
/// - `table`: Referencia mutable a la tabla `SmartTable` que se va a actualizar.
/// - `conn`: Referencia a la conexión activa con la base de datos SQLite.
///
/// # Retorno
/// - No retorna ningún valor (`()`).
///
/// # Funciones y Módulos Utilizados
/// - `table.clear()`: Limpia las celdas y contenido previo de la tabla.
/// - `count_rows`: Cuenta el total de filas existentes en la base de datos.
/// - `fetch_all`: Obtiene todos los contactos guardados desde SQLite.
/// - `table.set_cell_value()`: Escribe el valor textual de un campo en una celda específica.
/// - `table.redraw()`: Vuelve a renderizar la tabla en la pantalla.
/// - `fltk::dialog::alert`: Despliega una alerta gráfica en caso de fallo en la consulta.
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
            fltk::dialog::alert(0, 0, &t!("errors.refresh_table", error = e.to_string()));
        }
    }table.redraw();  // redibujar la tabla
    }
    