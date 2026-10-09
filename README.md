# 📖 f-contact (FLTK CONTACT)

Una aplicación de escritorio rápida y eficiente para la gestión de contactos, desarrollada completamente en **Rust**. Utiliza la biblioteca **FLTK** (`fltk-rs`) para una interfaz gráfica ligera y multiplataforma, y **SQLite** (`rusqlite`) para el almacenamiento seguro y local de los datos. Pensado en el minimalismo y para usarse en hardware modesto.

## 🖼️ Capturas de Pantalla

| Vista Principal | Agregar Contactos | Visualización y Edición |
| :---: | :---: | :---: |
| ![Vista Principal](screenshot/Captura%20desde%202026-10-09%2015-43-42.png) | ![Agregar Contactos](screenshot/Captura%20desde%202026-10-09%2015-41-28.png) | ![Visualización de Contacto](screenshot/Captura%20desde%202026-10-09%2015-44-02.png) |

---

## 🚀 Preparar el proyecto

Para poder trabajar con este proyecto, necesitas configurar tu entorno de desarrollo. Gracias a que el proyecto utiliza las características `bundled` en su `Cargo.toml` tanto para FLTK como para SQLite, el proceso descarga y compila estas librerías automáticamente, pero aún requiere herramientas básicas de compilación y cabeceras gráficas en tu sistema.

1. **Instalar Rust y Cargo:**
   Si aún no tienes Rust instalado, descárgalo usando `rustup`:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
 

2. **Dependencias del Sistema (Herramientas de compilación y X11):**
   Debes tener CMake y un compilador de C/C++ instalado en tu sistema.

   * **Windows:** Instala las herramientas de compilación de Visual Studio (C++ build tools) o MSYS2.
   * **macOS:** Instala las herramientas de línea de comandos de Apple y CMake.
     ```bash
     xcode-select --install
     brew install cmake
     ```
   * **Linux (Debian/Ubuntu):**
     ```bash
     sudo apt install build-essential cmake libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev
     ```
   * **Linux (Fedora):**
     ```bash
     sudo dnf groupinstall "Development Tools"
     sudo dnf install cmake libX11-devel libXext-devel libXft-devel libXinerama-devel libXcursor-devel libXrender-devel libXfixes-devel pango-devel mesa-libGL-devel
     ```
   * **Linux (Arch Linux):**
     ```bash
     sudo pacman -S base-devel cmake libx11 libxext libxft libxinerama libxcursor libxrender libxfixes pango
     ```

3. **Estructura de Directorios (Importante):**
   La aplicación guarda la base de datos en la ruta local `db/contacts.db`. **Debes crear la carpeta `db`** en la raíz del proyecto antes de ejecutar la aplicación por primera vez:
   ```bash
   mkdir db
   ```

## 📦 Como traer las dependencias

Rust utiliza **Cargo** como gestor de paquetes. Las dependencias principales (`fltk`, `fltk-table`, `fltk-theme`, `rusqlite`) ya están configuradas en el archivo `Cargo.toml`.

Para descargar todas las dependencias sin compilar, ejecuta:
```bash
cargo fetch
```
*(También puedes usar `just setup` si tienes Just instalado, como se explica más abajo).*

## 🛠️ Como compilarlo

Puedes compilar el proyecto utilizando Cargo o mediante el ejecutor de tareas Just.

**Usando Cargo directamente:**
* **Modo Desarrollo (Debug):** Ideal para pruebas rápidas.
  ```bash
  cargo build
  ```
* **Modo Producción (Release):** Genera el binario optimizado para el usuario final.
  ```bash
  cargo build --release
  ```

## ⚡ Como usar los comandos Just (Justfile)

Este proyecto utiliza [Just](https://github.com/casey/just), un práctico ejecutor de comandos (similar a `make`). Para instalarlo, ejecuta: `cargo install just`.

El archivo `justfile` incluido en el proyecto expone los siguientes comandos para facilitar el desarrollo:

* **`just setup`**: Descarga e instala las dependencias del proyecto partiendo del `Cargo.toml`.
  * *Ejemplo:* `just setup`

* **`just build`**: Se mueve al directorio del código fuente y compila el proyecto.
  * *Ejemplo:* `just build`

* **`just run`**: Ejecuta el proyecto compilado desde el directorio objetivo (`target`).
  * *Ejemplo:* `just run`

* **`just update`**: Actualiza las dependencias del proyecto a sus versiones más recientes permitidas en el `Cargo.toml`.
  * *Ejemplo:* `just update`

* **`just deploy <commit_message>`**: Crea un commit con el mensaje proporcionado y despliega los cambios a la rama `main` en el repositorio remoto.
  * *Ejemplo:* `just deploy "Agregada la funcionalidad de búsqueda de contactos"`

* **`just pull`**: Trae los últimos cambios de la rama remota actual y los fusiona automáticamente con tu entorno local.
  * *Ejemplo:* `just pull`

## 📚 Como generar documentación

Este proyecto contiene comentarios de documentación exhaustivos que explican la arquitectura del software. Para generar y abrir la documentación HTML del proyecto en tu navegador, ejecuta:

```bash
cargo doc --no-deps --open
```

## 🤝 Guía de contribución

¡Toda contribución es bienvenida! Sigue estos pasos para aportar al proyecto:

1. **Haz un Fork** del repositorio.
2. **Clona tu Fork** en tu máquina local.
3. **Crea una rama nueva** para tu funcionalidad (`git checkout -b feature/nueva-mejora`).
4. **Realiza tus cambios** y verifica que el código compila (`just build`).
5. **Haz Commit y Push** de tus cambios usando los comandos del justfile (`just deploy "Descripción de tu mejora"`).
6. **Abre un Pull Request (PR)** en el repositorio original detallando tu aportación.
