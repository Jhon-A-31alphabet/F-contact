//! Utilidades de internacionalización (i18n) y definición estable de las categorías.

use rust_i18n::t;

// Pares `(código, nombre nativo)` de los idiomas soportados por la interfaz.
pub const LOCALES: [(&str, &str); 5] = [
    ("es", "Español"),
    ("en", "English"),
    ("fr", "Français"),
    ("pt", "Português"),
    ("ja", "日本語"),
];

//claves asociadas a traduccion

pub const CATEGORY_KEYS: [&str; 10] = [
    "work",
    "work_fax",
    "friendship",
    "family",
    "company",
    "service",
    "home",
    "mobile",
    "vendor",
    "radio",
];

/// Valores internos (estables) que se almacenan en la base de datos por cada categoría.
///
/// Se mantienen en español para no romper los registros ya existentes; la interfaz
/// siempre muestra la etiqueta traducida al idioma activo.
pub const CATEGORY_DB_VALUES: [&str; 10] = [
    "Trabajo",
    "Fax del trabajo",
    "Amistad",
    "Familiar",
    "Empresa",
    "Servicio",
    "Casa",
    "Movil",
    "Vendedor",
    "Radio",
];

//  ..... Devuelve la etiqueta traducida de una categoría a partir de su clave.
pub fn category_label(key: &str) -> String {
    t!(format!("categories.{}", key)).to_string()
}

// Devuelve la lista de etiquetas traducidas de todas las categorías.
pub fn category_labels() -> Vec<String> {
    CATEGORY_KEYS.iter().map(|k| category_label(k)).collect()
}

/// Devuelve la etiqueta traducida de una categoría a partir de su valor almacenado en la base de datos.
pub fn category_label_from_db(db_value: &str) -> String {
    match CATEGORY_DB_VALUES.iter().position(|v| *v == db_value) {
        Some(index) => category_label(CATEGORY_KEYS[index]),
        None => t!("view.unknown").to_string(),
    }
}

/// traduce la seleccion de categorias en elmenu principal
pub fn category_choice_string() -> String {
    let mut items = vec![t!("main.all").to_string()];
    items.extend(category_labels());
    items.join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traducciones_disponibles_en_todos_los_idiomas() {
        rust_i18n::set_locale("es");
        assert_eq!(category_label("work"), "Trabajo");
        rust_i18n::set_locale("en");
        assert_eq!(category_label("work"), "Work");
        assert_eq!(category_label_from_db("Empresa"), "Company");
        rust_i18n::set_locale("fr");
        assert_eq!(category_label("work"), "Travail");
        rust_i18n::set_locale("pt");
        assert_eq!(category_label("work"), "Trabalho");
        rust_i18n::set_locale("ja");
        assert_eq!(category_label("work"), "仕事");
        rust_i18n::set_locale("es");
    }
}
