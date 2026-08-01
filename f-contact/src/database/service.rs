#![allow(unused)]

use rusqlite::{Connection, Result, Statement, StatementStatus::RePrepare, ffi::sqlite3_prepare, params};

/// Inicializa la base de datos y crea la tabla contacts si no existe.
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

/// Inserta un nuevo contacto utilizando una sentencia preparada (Prepared Statement).
pub fn insert_contact(
    conn: &Connection,
    nombre: &str,
    categoria: &str, // nescesita el i32 para en base al indice el widget choice poder seleccionar  ---> add_contacts_windows.rs
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

pub type FilaContacto = (i64, String, String, String, String, String, String);

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

pub fn count_rows(conn: &Connection)-> i32 {  //retorna el numero de filas disponibles en la tabla...
    let sql = "SELECT COUNT(*) FROM contacts";
    
   
    match conn.query_row(sql, [], |row| row.get::<_, i64>(0)) {
        
        Ok(count) => count as i32,
        // Si la tabla no existe o falla la conexión, devolvemos 0 de forma segura
        Err(_) => 0, 
    }
}
#[derive(Debug)]
pub struct  Usuario{
    pub id: i64,
    pub nombre: String,
    pub categoria: String,
    pub descripcion: String,
    pub telefono :String,
    pub organizacion:String,
    pub email: String
}

pub fn search(conn:&Connection,nombre: &str)-> Result<Vec<Usuario>>{

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


        Ok(Usuario {
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

/// Actualiza un contacto existente filtrando por su `id`.
pub fn update_contact(
    conn: &Connection,
    id: i64,
    nombre: &str,
    categoria: &i32,
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

/// Obtiene un contacto específico por su ID y lo retorna en un vector,
pub fn get_contact_by_id(conn: &Connection, id: i64) -> Result<Vec<Usuario>> {
    let sql_query = "SELECT * FROM contacts WHERE id = ?1";
    let mut stmt = conn.prepare(sql_query)?;

    let iterador_fila = stmt.query_map([id], |row| {
        Ok(Usuario {
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
