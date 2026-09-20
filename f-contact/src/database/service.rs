//! Módulo de servicio para la gestión de base de datos SQLite y operaciones CRUD de contactos.

#![allow(unused)]

use rusqlite::{Connection, Result, Statement, StatementStatus::RePrepare, ffi::sqlite3_prepare, params};

/// Inicializa la base de datos en la ruta indicada y crea la tabla `contacts` si aún no existe.
///
/// # Parámetros
/// - `db_path`: Ruta relativa o absoluta del archivo de base de datos SQLite.
///
/// # Retorno
/// - `Result<Connection>`: Devuelve la conexión activa (`Connection`) o un error (`rusqlite::Error`).
///
/// # Funciones y Módulos Utilizados
/// - `Connection::open()`: Abre o crea el archivo SQLite.
/// - `conn.execute()`: Ejecuta la sentencia DDL `CREATE TABLE IF NOT EXISTS`.
pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Se ejecuta al iniciar para garantizar que el esquema exista.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            categoria TEXT,
            telefono TEXT,
            descripcion TEXT,
            organizacion TEXT,
            email TEXT
        )",
        [], // Sin parámetros para esta consulta
    )?;

    Ok(conn)
}

/// Inserta un nuevo registro de contacto en la base de datos.
///
/// # Parámetros
/// - `conn`: Conexión activa a la base de datos SQLite.
/// - `nombre`: Nombre completo del contacto.
/// - `categoria`: Categoría del contacto.
/// - `telefono`: Número telefónico.
/// - `descripcion`: Notas o información adicional.
/// - `organizacion`: Empresa o entidad.
/// - `email`: Correo electrónico.
///
/// # Retorno
/// - `Result<usize>`: Retorna el número de filas insertadas (1) o un error de SQLite.
///
/// # Funciones y Módulos Utilizados
/// - `conn.prepare()`: Precompila la sentencia SQL de inserción con marcadores posicionales (`?1`, `?2`, etc.).
/// - `stmt.execute()`: Ejecuta la inserción vinculando los parámetros recibidos.
pub fn insert_contact(
    conn: &Connection,
    nombre: &str,
    categoria: &str, 
    telefono: &str,
    descripcion: &str,
    organizacion: &str,
    email: &str,
) -> Result<usize> {
    // La sentencia se prepara, lo cual precompila el SQL y lo hace seguro y eficiente.
    let mut stmt = conn.prepare(
        "INSERT INTO contacts (nombre, categoria, telefono, descripcion, organizacion, email)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )?;

    // Ejecutamos pasando los parámetros exactos.
    stmt.execute(params![
        nombre,
        categoria,
        telefono,
        descripcion,
        organizacion,
        email
    ])
}

/// Alias de tipo que define una tupla con los campos crudos de una fila de contacto `(id, nombre, categoria, descripcion, telefono, organizacion, email)`.
pub type FilaContacto = (i64, String, String, String, String, String, String);

/// Obtiene todos los contactos guardados en la tabla `contacts`.
///
/// # Parámetros
/// - `conn`: Conexión activa a SQLite.
///
/// # Retorno
/// - `Result<Vec<FilaContacto>>`: Un vector con tuplas `FilaContacto` o un error si la consulta falla.
///
/// # Funciones y Módulos Utilizados
/// - `conn.prepare()`: Prepara la sentencia `SELECT * FROM contacts`.
/// - `stmt.query_map()`: Mapea cada fila devuelta por la consulta SQL a la tupla `FilaContacto`.
pub fn fetch_all(conn:&Connection) -> Result<Vec<FilaContacto>> {

    let mut stmt = conn.prepare("SELECT * FROM contacts")?;

    let rows_iter = stmt.query_map([],|row|{

    let id: i64 = row.get(0)?;
        let nombre: String = row.get(1)?;
        let categoria: String = row.get(2)?;
        let descripcion: String = row.get(3)?;
        let telefono: String = row.get(4)?;
        let organizacion: String = row.get(5)?;
        let email: String = row.get(6)?;

        Ok((id, nombre, categoria, descripcion, telefono, organizacion, email))

    })?;

    rows_iter.collect()

}

/// Cuenta el número total de filas registradas en la tabla de contactos.
///
/// # Parámetros
/// - `conn`: Conexión activa a la base de datos.
///
/// # Retorno
/// - `i32`: Retorna la cantidad total de registros o `0` si ocurre algún error.
///
/// # Funciones y Módulos Utilizados
/// - `conn.query_row()`: Ejecuta la consulta SQL `SELECT COUNT(*)` y extrae el resultado entero.
pub fn count_rows(conn: &Connection)-> i32 {  //retorna el numero de filas disponibles en la tabla...
    let sql = "SELECT COUNT(*) FROM contacts";
    
   
    match conn.query_row(sql, [], |row| row.get::<_, i64>(0)) {
        
        Ok(count) => count as i32,
        // Si la tabla no existe o falla la conexión, devolvemos 0 de forma segura
        Err(_) => 0, 
    }
}


/// Estructura de datos que representa a un contacto dentro del sistema.
#[derive(Debug)]
pub struct  Contacto{
    /// Identificador único autoincremental en la base de datos.
    pub id: i64,
    /// Nombre del contacto.
    pub nombre: String,
    /// Categoría asignada.
    pub categoria: String,
    /// Descripción o notas sobre el contacto.
    pub descripcion: String,
    /// Número de teléfono.
    pub telefono :String,
    /// Nombre de la empresa u organización.
    pub organizacion:String,
    /// Dirección de correo electrónico.
    pub email: String
}

/// Busca contactos en la base de datos cuyo nombre empiece con el texto proporcionado.
///
/// # Parámetros
/// - `conn`: Conexión activa a SQLite.
/// - `nombre`: Prefijo o texto a buscar.
///
/// # Retorno
/// - `Result<Vec<Contacto>>`: Lista de estructuras `Contacto` encontradas o un error.
///
/// # Funciones y Módulos Utilizados
/// - `conn.prepare()`: Prepara la consulta SQL usando el operador `LIKE ?1`.
/// - `stmt.query_map()`: Construye instancias de la estructura `Contacto` por cada fila coincidente.
pub fn search(conn:&Connection,nombre: &str)-> Result<Vec<Contacto>>{

    let sql_like_query = "SELECT * FROM contacts WHERE nombre LIKE ?1 ";
    
    let mut stmt_ =match conn.prepare(sql_like_query){
           Ok(statement)=>statement,
           Err(error)=> return Err((error))
    };
    let mut buscar_nombre =format!("{}%",nombre);

   let mut buscar_fila=stmt_.query_map([buscar_nombre], |row|{
        
        let buscar_id:i64 = row.get(0)?;
        let buscar_nombre:String =row.get(1)?; 
        let buscar_categoria :String = row.get(2)?;
        let buscar_descripcion:String = row.get(3)?;
        let buscar_telefono: String = row.get(4)?;
        let buscar_organizacion :String = row.get(5)?;
        let buscar_email : String = row.get(6)?;


        Ok(Contacto {
            id: buscar_id,
            nombre: buscar_nombre,
            categoria:buscar_categoria,
            descripcion:buscar_descripcion,
            telefono: buscar_telefono,
            organizacion: buscar_organizacion,
            email:buscar_email


        })
    })?;
   

    //buscar_fila.collect();
    let mut resultados = Vec::new();
    for usuario in buscar_fila{
        resultados.push(usuario?);
    }
    Ok((resultados))
}






/// Actualiza los datos de un contacto existente filtrado por su `id`.
///
/// # Parámetros
/// - `conn`: Conexión activa a la base de datos.
/// - `id`: Identificador único del contacto a modificar.
/// - `nombre`: Nuevo nombre.
/// - `categoria`: Nueva categoría.
/// - `telefono`: Nuevo número telefónico.
/// - `descripcion`: Nueva descripción.
/// - `organizacion`: Nueva organización.
/// - `email`: Nuevo correo.
///
/// # Retorno
/// - `Result<usize>`: Cantidad de registros modificados o error de SQLite.
///
/// # Funciones y Módulos Utilizados
/// - `conn.prepare()`: Prepara la sentencia `UPDATE contacts SET ... WHERE id = ?7`.
/// - `stmt.execute()`: Ejecuta la consulta de actualización con los parámetros proporcionados.
pub fn update_contact(
    conn: &Connection,
    id: i64,
    nombre: &str,
    categoria: &str,
    telefono: &str,
    descripcion: &str,
    organizacion: &str,
    email: &str,
) -> Result<usize> {
    // Sentencia preparada para el UPDATE.
    let mut stmt = conn.prepare(
        r#"
        UPDATE contacts
        SET nombre = ?1,
            categoria = ?2,
            telefono = ?3,
            descripcion = ?4,
            organizacion = ?5,
            email = ?6
        WHERE id = ?7
        "#
    )?;

    // Los parámetros deben pasarse exactamente en el orden de los placeholders (?1 al ?7).
    stmt.execute(params![
        nombre,
        categoria,
        telefono,
        descripcion,
        organizacion,
        email,
        id
    ])
}

/// Obtiene los datos de un contacto específico buscando por su `id`.
///
/// # Parámetros
/// - `conn`: Conexión activa a SQLite.
/// - `id`: Identificador del contacto.
///
/// # Retorno
/// - `Result<Vec<Contacto>>`: Vector que contiene el contacto hallado o un vector vacío si no existe.
///
/// # Funciones y Módulos Utilizados
/// - `conn.prepare()`: Prepara la consulta `SELECT * FROM contacts WHERE id = ?1`.
/// - `stmt.query_map()`: Convierte el registro coincidente en una instancia de `Contacto`.
pub fn get_contact_by_id(conn: &Connection, id: i64) -> Result<Vec<Contacto>> {
    let sql_query = "SELECT * FROM contacts WHERE id = ?1";
    let mut stmt = conn.prepare(sql_query)?;

    let iterador_fila = stmt.query_map([id], |row| {
        Ok(Contacto {
            id: row.get(0)?,
            nombre: row.get(1)?,
            categoria: row.get(2)?,
            descripcion: row.get(3)?,
            telefono: row.get(4)?,
            organizacion: row.get(5)?,
            email: row.get(6)?,
        })
    })?;

    let mut resultados = Vec::new();
    for usuario in iterador_fila {
        resultados.push(usuario?);
    }
    
    Ok(resultados)
}



/// Obtiene la lista de contactos que pertenecen a una categoría específica.
///
/// # Parámetros
/// - `conn`: Conexión activa a SQLite.
/// - `categoria`: Nombre exacto de la categoría a filtrar.
///
/// # Retorno
/// - `Result<Vec<Contacto>>`: Vector con los contactos pertenecientes a la categoría dada.
///
/// # Funciones y Módulos Utilizados
/// - `conn.prepare()`: Prepara la consulta `SELECT * FROM contacts WHERE categoria = ?1`.
/// - `stmt.query_map()`: Recorre y mapea las filas devueltas a objetos `Contacto`.
pub fn get_contact_by_category(conn: &Connection, categoria: &str) -> Result<Vec<Contacto>> {
    let sql_query = "SELECT * FROM contacts WHERE categoria = ?1";
    let mut stmt = conn.prepare(sql_query)?;

    let iterador_fila = stmt.query_map([categoria], |row| {
        Ok(Contacto {
            id: row.get(0)?,
            nombre: row.get(1)?,
            categoria: row.get(2)?,
            descripcion: row.get(3)?,
            telefono: row.get(4)?,
            organizacion: row.get(5)?,
            email: row.get(6)?,
        })
    })?;

    let mut resultados = Vec::new();
    for usuario in iterador_fila {
        resultados.push(usuario?);
    }
    
    Ok(resultados)
}


/// Elimina un contacto de la base de datos utilizando su `id`.
///
/// # Parámetros
/// - `conn`: Conexión activa a SQLite.
/// - `id`: Identificador único del contacto a borrar.
///
/// # Retorno
/// - `Result<usize>`: Cantidad de registros eliminados (1) o un error si falla.
///
/// # Funciones y Módulos Utilizados
/// - `conn.prepare()`: Prepara la consulta `DELETE FROM contacts WHERE id = ?1`.
/// - `stmt.execute()`: Borra el registro de la base de datos.
pub fn delete_contact(conn: &Connection,id :i64)-> Result<usize>{
    let sql_query = "DELETE  FROM contacts WHERE id = ?1";
    let mut stmt = conn.prepare(sql_query)?;
    stmt.execute(params![id])

}