mod icon_path;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Image};

use crate::icon_path::icon_path;

#[derive(serde::Deserialize)]
struct ValueWrapper<T> {
    value: T
}

#[derive(serde::Deserialize)]
struct WeatherNearest {
    areaName: [ValueWrapper<String>; 1],
    region: [ValueWrapper<String>; 1]
}

#[derive(serde::Deserialize)]
struct WeatherCurrent {
    temp_F: String,
    FeelsLikeF: String,
    cloudcover: String,
    humidity: String,
    precipInches:String,
    visibilityMiles: String,
    weatherDesc: [ValueWrapper<String>; 1],
    windspeedMiles: String
}

#[derive(serde::Deserialize)]
struct Weather {
    current_condition: [WeatherCurrent; 1],

    nearest_area: [WeatherNearest; 1]
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let resp = reqwest::blocking::get("https://wttr.in/MyLocation?format=j1")?
        .json::<Weather>()?;
    let application = Application::new(
        Some("com.github.rust-ui-rundown.rust-ui-gtk"),
        Default::default(),
    ).expect("stuff no worky");

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Wttr.in weather application");
        window.set_default_size(500, 500);

        let container = GtkBox::new(gtk::Orientation::Vertical, 5);

        let formatted_label = format!("\n{}", resp.nearest_area[0].region[0].value);
        let label = Label::new(None);
        label.set_markup(&format!("<b>{}</b>\ntemp in F: {}\nhumidity: {}\n", formatted_label.as_str(), resp.current_condition[0].temp_F, resp.current_condition[0].humidity));
        println!("{}", resp.current_condition[0].weatherDesc[0].value);

        let button = Button::with_label("Click me!");

        container.set_spacing(20);
        container.add(&label);
        container.add(&button);
        window.add(&container);

        button.connect_clicked(move |_| {
            &label.set_label("Hello, World!");
        });

        window.show_all();
    });

    application.run(&[]);

    Ok(())
}

