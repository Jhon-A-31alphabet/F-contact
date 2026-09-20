//! Módulo para actualizar la tabla de la interfaz gráfica a partir de resultados de una búsqueda de contactos.

use rusqlite::Connection;
use fltk_table::SmartTable;
use fltk::prelude::WidgetExt;
use crate::database;
use database::service::{Contacto, search};

/// Refresca el contenido de la tabla mostrando únicamente los contactos que coinciden con la búsqueda.
///
/// # Parámetros
/// - `table`: Referencia mutable a la tabla `SmartTable`.
/// - `conn`: Referencia a la conexión con la base de datos SQLite.
/// - `vec`: Resultado con el vector de contactos obtenido de una consulta preliminar.
///
/// # Retorno
/// - No retorna ningún valor (`()`).
///
/// # Funciones y Módulos Utilizados
/// - `table.clear()`: Vacía la tabla antes de cargar la búsqueda.
/// - `search`: Busca en la base de datos los contactos que coincidan con el nombre especificado.
/// - `table.set_cell_value()`: Rena cada celda con las propiedades del objeto `Contacto`.
/// - `table.redraw()`: Fuerza el rediseño visual de la tabla actualizada.
/// - `fltk::dialog::alert`: Notifica al usuario en caso de error en la búsqueda o resultado inválido.
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