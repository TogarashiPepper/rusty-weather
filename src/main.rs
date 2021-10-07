use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Image, Label};

#[derive(serde::Deserialize)]
struct ValueWrapper<T> {
	value: T,
}

#[derive(serde::Deserialize)]
struct WeatherNearest {
	areaName: [ValueWrapper<String>; 1],
	region: [ValueWrapper<String>; 1],
}

#[derive(serde::Deserialize)]
struct WeatherCurrent {
	temp_F: String,
	FeelsLikeF: String,
	cloudcover: String,
	humidity: String,
	precipInches: String,
	visibilityMiles: String,
	weatherDesc: [ValueWrapper<String>; 1],
	windspeedMiles: String,
}

#[derive(serde::Deserialize)]
struct Weather {
	current_condition: [WeatherCurrent; 1],

	nearest_area: [WeatherNearest; 1],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let resp = reqwest::blocking::get("https://wttr.in/Paris?format=j1")?.json::<Weather>()?;
	let application = Application::new(
		Some("com.github.rust-ui-rundown.rust-ui-gtk"),
		Default::default(),
	)
	.expect("stuff no worky");

	application.connect_activate(
		move |app| {
			let window = ApplicationWindow::new(app);
			window.set_title("Wttr.in weather application");
			window.set_default_size(
				500, 500,
			);

			let menu = gio::Menu::new();
			menu.append(
				Some("Quit"),
				Some("app.quit"),
			);
			app.set_app_menu(Some(
				&menu,
			));

			let container = GtkBox::new(
				gtk::Orientation::Vertical,
				5,
			);

			let formatted_label = format!(
				"\n{}",
				resp.nearest_area[0].region[0].value
			);
			let label = Label::new(None);
			label.set_markup(
				&format!(
					"<b>{}, {}</b>\ntemp in F: {}\nhumidity: {}\nprecip: {}in\nwind speed in miles: {}\n",
					formatted_label.as_str(),
					resp.nearest_area[0].areaName[0].value,
					resp.current_condition[0].temp_F,
					resp.current_condition[0].humidity,
                    resp.current_condition[0].precipInches,
                    resp.current_condition[0].windspeedMiles
				),
			);

			let button = Button::with_label("Click me!");

			container.set_spacing(20);
			container.add(&label);
			container.add(&button);
			window.add(&container);

			button.connect_clicked(
				move |_| {
					&label.set_label("Hello, World!");
				},
			);

			window.show_all();
		},
	);

	application.run(&[]);

	Ok(())
}
