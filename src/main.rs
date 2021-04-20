use fltk::{app, button::*, enums::*, frame::*, group::*, prelude::*, window::*};
mod sidebar;
mod account;
/*

Created:0.0.1
updated:0.0.1

description:

 */


fn main() -> Result<(), Box<dyn std::error::Error>> {

    //create app & widgets
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);

    let mut wind = Window::default()
        .with_size(600, 400)
        .center_screen()
        .with_label("Starlite Business");


    wind.make_resizable(true);
    wind.set_color(Color::White);
    wind.end();
    wind.show();


    //run the app
    Ok(app.run()?)
}