use gtk::{
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::{GtkWindowExt, WidgetExt},
};

const APP_NAME: &str = "Alternate";
const APP_DOMAIN: &str = "br.dev.diegogarcia.gnome";

const APP_ID: &str = "br.dev.diegogarcia.gnome.Alternate";
const APP_PATH: &str = "br/dev/diegogarcia/gnome/Alternate";

fn main() -> gtk::glib::ExitCode {
    let app = adw::Application::builder() //
        .application_id(APP_ID)
        .build();

    app.connect_activate(&build_main_window);

    app.run()
}

fn build_main_window(app: &adw::Application) {
    // APP_ID.to_string()

    let window: adw::ApplicationWindow =
        gtk::Builder::from_file(format!("{}/data/ui/compiled/window.ui", APP_ID)) //
            .object("root")
            .unwrap();

    window.set_application(Some(app));
    window.set_title(Some(APP_NAME));

    window.add_css_class("devel");
    window.present();
}
