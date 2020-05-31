use gio::{ApplicationExt, ApplicationExtManual };

use gtk::prelude::*;
use gtk::{
    MenuItem, Grid, Label, ComboBoxText, TextView, TextBuffer, ScrolledWindow,
    MenuBar, Menu, RadioButton, ApplicationWindow, WindowPosition, CheckMenuItem, Entry, LinkButton
};

use std::sync::{Arc, Mutex};
use std::env::args;

use umwerter::konstanten::UmwertungsTabelle;
use umwerter::errors::UmwerterError;


macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


pub struct Content;

impl Content {
    fn new(window : &ApplicationWindow) -> Grid {
        let current_umwerter = Arc::new(Mutex::new(UmwertungsTabelle::Iso18265A1));

        let mut row = 0;
        let container = Grid::new();

        
        // Units
        let cb_source_units_label = Label::new("Source Unit");
        cb_source_units_label.set_xalign(0.0);
        let cb_source_units_value = ComboBoxText::new();

        let cb_destination_units_label = Label::new("Destination Unit");
        cb_destination_units_label.set_xalign(0.0);
        let cb_destination_units_value = ComboBoxText::new();

        
        // Result
        let content = TextBuffer::new(None);
        let content_view = TextView::new_with_buffer(&content);
        let content_scroller = ScrolledWindow::new(None, None);


        // Menu
        let menu_bar = MenuBar::new();
        
        let app_menu_label = MenuItem::new_with_label("Program");
        let app_menu = Menu::new();
        let app_menu_quit_item = MenuItem::new_with_label("Quit");
        app_menu_quit_item.connect_activate(clone!(window => move |_| {
            window.destroy();
        }));

        window.connect_delete_event(clone!(window => move |_, _| {
            window.destroy();
            Inhibit(false)
        }));

        
        app_menu.append(&app_menu_quit_item);
        app_menu_label.set_submenu(Some(&app_menu));
        
        let options_menu_label = MenuItem::new_with_label("Options");
        let options_menu = Menu::new();
        let options_menu_verbose_item = CheckMenuItem::new_with_label("Verbose");
        options_menu_verbose_item.set_active(true);        
        options_menu.append(&options_menu_verbose_item);
        options_menu_label.set_submenu(Some(&options_menu));
        
        menu_bar.append(&app_menu_label);
        menu_bar.append(&options_menu_label);

        container.attach(&menu_bar, 0, row, 2, 1);
        row += 1;


        // Conversion tables
        let mut last_rb : Option<RadioButton> = None;
        for table in UmwertungsTabelle::bezeichner().iter() {            
            let rb = if let Some(widget) = last_rb  {
                RadioButton::new_with_label_from_widget(&widget, table)
            } else {
                RadioButton::new_with_label(table)
            };

            // The only thing that you need to be aware of is that you will need to increment
            // reference counters when passing GTK objects into closures to program your UI,
            // which is done by cloning the object.
            
            rb.connect_clicked(clone!
                               (cb_source_units_value, cb_destination_units_value,
                                current_umwerter => move |widget| {
                                    
                let umwerter = UmwertungsTabelle::bezeichner_to_enum(&widget.get_label().unwrap()[..]);
                if let Some(umwerter_trait) = umwerter {
                    {
                        let einheiten = &umwerter::bestimme_einheiten(&umwerter_trait);
                        Content::update_einheiten(&cb_source_units_value, einheiten);
                        Content::update_einheiten(&cb_destination_units_value, einheiten);
                    }
                    let mut data = current_umwerter.lock().unwrap();
                    *data = umwerter_trait;
                }
            }));

            container.attach(&rb, 0, row, 2, 1);
            last_rb = Some(rb);

            row += 1;
        }

        let einheiten = &umwerter::bestimme_einheiten(&UmwertungsTabelle::Iso18265A1);
        Content::update_einheiten(&cb_source_units_value, einheiten);
        Content::update_einheiten(&cb_destination_units_value, einheiten);

        // Value Field
        let e_input_label = Label::new("Value");
        e_input_label.set_xalign(0.0);
        let e_input_value = Entry::new();


        // Conversion action
        let lb_current_dest = LinkButton::new("Convert");
        lb_current_dest.connect_clicked(clone!(
            e_input_value,
            cb_source_units_value, cb_destination_units_value,
            current_umwerter,
            options_menu_verbose_item,
            content => move |_| {
                let eingabe = e_input_value.get_text();
                if let Some(s) = eingabe.clone() {
                    if s == "" {
                        content.set_text("No value to convert");
                        return;
                    }
                }
                
                let parse_ergebnis = eingabe.clone().unwrap().parse::<f64>();
                if parse_ergebnis.is_err() {
                    content.set_text(&format!("Invalid value: {}", eingabe.unwrap()));
                    return;
                }

                if cb_source_units_value.get_active_text().is_none() {
                    content.set_text("Please select source unit");
                    return;
                }
            
                if cb_destination_units_value.get_active_text().is_none() {
                    content.set_text("Please select destination unit");
                    return;
                }

                let umwerter_tabelle = *current_umwerter.lock().unwrap();
                match umwerter::werte_um(
                    parse_ergebnis.unwrap(),
                    &cb_source_units_value.get_active_text().unwrap(),
                    &cb_destination_units_value.get_active_text().unwrap(),
                    &umwerter_tabelle
                ) {
                    Ok(erg) => {
                        let bezeichner = UmwertungsTabelle::enum_to_kurzbezeichner(umwerter_tabelle);

                        if options_menu_verbose_item.get_active() {
                            content.set_text(
                                &format!("{0:.2} {1} - {2} - {3}",
                                         erg,
                                         &cb_destination_units_value.get_active_text().unwrap(),
                                         bezeichner,
                                         &cb_source_units_value.get_active_text().unwrap()
                                )
                            );
                        } else {
                            content.set_text(&format!("{0:.2}", erg));
                        }
                    },
                    Err(e) => {
                        match e {
                            UmwerterError::QuellWertAusserhalbUmwertungsnorm(wert) |
                            UmwerterError::ZielWertAusserhalbUmwertungsnorm(wert) =>
                                content.set_text(&format!("Conversion {0} {1} -> {2} not defined", wert,
                                                          &cb_source_units_value.get_active_text().unwrap(),
                                                          &cb_destination_units_value.get_active_text().unwrap())),
                            _ => {},
                        }
                    }
                }
            }));

        container.attach(&cb_source_units_label, 0, row, 1, 1);
        container.attach(&cb_source_units_value, 1, row, 1, 1);
        row += 1;

        container.attach(&cb_destination_units_label, 0, row, 1, 1);
        container.attach(&cb_destination_units_value, 1, row, 1, 1);
        row += 1;
        
        container.attach(&e_input_label, 0, row, 1, 1);
        container.attach(&e_input_value, 1, row, 1, 1);
        row += 1;

        container.attach(&lb_current_dest, 0, row, 2, 1);
        row += 1;

        content_scroller.add(&content_view);
        content_scroller.set_vexpand(true);
        content_scroller.set_hexpand(true);
        container.attach(&content_scroller, 0, row, 2, 1);

        container
    }

    fn update_einheiten(cb : &ComboBoxText, einheiten : &[&str]) {
        cb.remove_all();
        einheiten.iter().for_each(|s| {
            cb.append_text(s);
        });
    }
}


fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("18265 Hardness Converter");
    window.set_position(WindowPosition::Center);
    window.set_size_request(300, 360);

    let content = Content::new(&window);
    window.add(&content);

    window.show_all();
    
}

fn main() {
    let application = gtk::Application::new("com.cybernetics.umwerter", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
