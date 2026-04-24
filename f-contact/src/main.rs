use fltk::{app, prelude::*, window::Window, button::Button, input::Input, group::{Pack, PackType},menu::Choice,enums::FrameType, frame::Frame};
use fltk_table::{SmartTable, TableOpts};

fn main() {


    let app = app::App::default();
    let (screen_w, screen_h) = app::screen_size();

    let mut wind = Window::new(0, 0, screen_w as i32 - 250, screen_h  as i32 - 200, "Contactos"); // el tamano dependera de la resolucion
    //de la pantalla del usuario //

    let pack_x = 115;
    let pack_y = 40;
    let btn_size = 50;
    let input_w = 600;

    let height = btn_size;
    let (center_frame_w,center_frame_h) =(screen_w as i32 -300 ,screen_h as i32 -250 ); //tamano del frame

    let mut center_frame = Frame::default().with_size(center_frame_w,center_frame_h).center_of(&wind);
    center_frame.set_frame(FrameType::EngravedBox);



    let mut h_pack = Pack::new(pack_x, pack_y, btn_size + 30 + input_w + btn_size, height, "");
    h_pack.set_type(PackType::Horizontal);
    h_pack.set_spacing(20);

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
    let mut cat_label =Frame::new(0,0,80,height,"Categorias");
    cat_label.set_label_size(18);

    let mut choice = Choice::new(0, 0, 120, height - 10, ""); // ancho 120, altura ajustada al pack
    choice.add_choice("|Trabajo|Fax del trabajo|Amistad|Familiar|Empresa|Servicio|Casa|Movil|Vendedor|Radio");  // aqui van las categorias
	choice.set_value(0);
	choice.set_text_size(12);

	choice.set_callback(move|e|{
	e.set_value(0);
	e.redraw();

	});




    h_pack.end(); // cierre del pack -------.


    let mut table = SmartTable::default()
		.with_size(1000,450)
		.center_of_parent()
		.with_opts(TableOpts{
		rows: 30,
        cols: 4,
        editable: false,
        ..Default::default()
		});
		(&mut table).set_col_header_height(32);
		(&mut table).set_col_width(1,250);
				(&mut table).set_col_width(0,250);
						(&mut table).set_col_width(2,250);
								(&mut table).set_col_width(3,250);







    wind.end();
    wind.show();
    app.run().unwrap();
}

