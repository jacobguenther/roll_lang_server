// File: src/models.rs

pub mod player;
pub mod r#macro;

pub struct Settings {
	pub theme: String,
}
impl Default for Settings {
	fn default() -> Settings {
		Settings {
			theme: String::from("light"),
		}
	}
}