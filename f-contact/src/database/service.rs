use rusqlite::{params, Connection, Result};

/// Inicializa la base de datos y crea la tabla `contacts` si no existe.
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
    categoria: &i32,
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
