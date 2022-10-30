use clap::{Arg, Command};
use gtk::prelude::*;
use gtk::Builder;
use rand::{thread_rng, Rng};
use std::process::exit;

fn main() {
    let matches = Command::new("Zufall")
        .version("3.2.0-rolling")
        .about("Zufall, a tool to help on decisions.")
        .subcommand(
            Command::new("cli")
                .about("Use zufall in the shell")
                .arg(
                    Arg::new("FROM")
                        .help("lower limit of the number range")
                        .index(1)
                        .default_value("0"),
                )
                .arg(
                    Arg::new("TO")
                        .help("upper limit of the number range")
                        .index(2)
                        .default_value("1"),
                ),
        )
        .subcommand(Command::new("gui").about("Start in graphical mode"))
        .get_matches();

    match matches.subcommand_name() {
        Some("cli") => {
            let sc_matches = matches.subcommand_matches("cli").unwrap();
            let value_from: u128 = sc_matches
                .get_one::<String>("FROM")
                .map(|s| s.as_str())
                .unwrap()
                .parse()
                .expect("Failed to parse '<FROM>'");
            let value_to: u128 = sc_matches
                .get_one::<String>("TO")
                .map(|s| s.as_str())
                .unwrap()
                .parse()
                .expect("Failed to parse '<TO>'");

            cli(
                value_from,
                value_to
                    .checked_add(1)
                    .expect("Overflow, '<TO>' is to large"),
            );
        }
        Some("gui") => gui(),
        _ => println!("Please specify a subcommand (gui or cli). See: zufall --help"),
    };
}

fn cli(from: u128, to: u128) {
    if from >= to {
        println!("<FROM> must be smaller than <TO>");
        exit(50);
    }
    let output = thread_rng().gen_range(from..to);
    println!("random number: {}", output);
}

fn gui() {
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
