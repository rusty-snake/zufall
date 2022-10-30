use gtk::prelude::*;
use gtk::Builder;
use rand::{thread_rng, Rng};

fn main() {
    gtk::init().expect("Failed to initialize gtk");

    let builder = Builder::from_string(include_str!("zufall.glade"));

    let window: gtk::Window = builder.object("ZufallMainWindow").unwrap();
    let to_from_warning_var1: gtk::InfoBar = builder.object("ToFromWarning").unwrap();
    let to_from_warning_var2: gtk::InfoBar = builder.object("ToFromWarning").unwrap();
    let to_from_ok: gtk::Button = builder.object("ToFromOK").unwrap();
    let from_spin: gtk::SpinButton = builder.object("InFrom").unwrap();
    let to_spin: gtk::SpinButton = builder.object("InTo").unwrap();
    let output_box: gtk::Label = builder.object("Output").unwrap();
    let button: gtk::Button = builder.object("GetButton").unwrap();

    button.connect_clicked(move |_| {
        let value_from = from_spin.value_as_int();
        let value_to = to_spin.value_as_int() + 1;
        if value_from >= value_to {
            output_box.set_label("");
            to_from_warning_var1.show();
        } else {
            let output = &thread_rng().gen_range(value_from..value_to).to_string();
            output_box.set_label(output);
        }
    });

    to_from_ok.connect_clicked(move |_| {
        to_from_warning_var2.hide();
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
