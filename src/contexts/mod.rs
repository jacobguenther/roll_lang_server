
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
	pub logged_in: bool,
	pub logout_success: bool,
	pub display_name: String,
}
impl Default for Base {
	fn default() -> Base {
		Base {
			logged_in: false,
			logout_success: false,
			display_name: "".to_owned(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
	pub base: Base,
	pub login_success: bool,
	pub deleted_account: bool,
	pub map_levels: Vec<MapLevel>,
}
impl Default for Index {
	fn default() -> Index {
		Index {
			base: Base::default(),
			login_success: false,
			deleted_account: false,
			map_levels: vec!(MapLevel::default()),
		}
	}
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MapLevel {
	name: String,
	height: u32,
	layers: Vec<LayerE>,
}
impl Default for MapLevel {
	fn default() -> MapLevel {
		MapLevel {
			name: "default".to_owned(),
			height: 0,
			layers: vec!(LayerE::Grid(GridLayer::default())),
		}
	}
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LayerE {
	Grid(GridLayer),
	Image(ImageLayer)
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GridLayer {
	scale: u32,
	grid_count: u32,
	grid_size: u32,
	horizontal_offset: i32,
	vertical_offset: i32,
}
impl Default for GridLayer {
	fn default() -> GridLayer {
		GridLayer {
			scale: 100,
			grid_count: 32,
			grid_size: 64,
			horizontal_offset: 0,
			vertical_offset: 0,
		}
	}
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageLayer {
	name: String,
	path: String,
	size: (u32, u32),
	scale: u32,
}
/*
#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
	layer_type: String,
	scale: u32,
}
impl Default for Layer {
	fn default() -> Layer {
		Layer {
			layer_type: "grid".to_owned(),
			scale: 100,
		}
	}
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
	pub base: Base,
	pub name: String,
	pub login_fail: bool,
}
impl Default for Login {
	fn default() -> Login {
		Login {
			base: Base::default(),
			name: String::new(),
			login_fail: false,
		}
	}
}
impl Login {
	pub fn new(name: &str) -> Login {
		Login {
			base: Base::default(),
			name: name.to_owned(),
			login_fail: false,
		}
	}
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccount {
	pub base: Base,

	pub name: String,
	pub email: String,

	pub password_too_short: bool,
	pub passwords_dont_match: bool,
	pub name_or_email_taken: bool,
	pub unknown_error: bool,
}
impl Default for CreateAccount {
	fn default() -> CreateAccount {
		CreateAccount {
			base: Base::default(),
			name: String::new(),
			email: String::new(),
			password_too_short: false,
			passwords_dont_match: false,
			name_or_email_taken: false,
			unknown_error: false,
		}
	}
}
impl CreateAccount {
	pub fn new(name: &str, email: &str) -> CreateAccount {
		CreateAccount {
			base: Base::default(),
			name: name.to_owned(),
			email: email.to_owned(),
			password_too_short: false,
			passwords_dont_match: false,
			name_or_email_taken: false,
			unknown_error: false,
		}
	}
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
	pub base: Base,

	pub name: String,
	pub email: String,

	pub update_name_wrong_password: bool,
	pub update_name_taken: bool,

	pub update_email_wrong_password: bool,
	pub update_email_taken: bool,

	pub update_password_wrong_password: bool,
	pub update_password_dont_match: bool,
	pub update_password_too_short: bool,
	pub update_password_too_long: bool,
	pub update_password_missing_characters: bool,

	pub delete_account_wrong_password: bool,
	pub changed_name: bool,
	pub changed_email: bool,
	pub changed_password: bool,
	pub unknown_error: bool,
}
impl Default for Account {
	fn default() -> Account {
		Account {
			base: Base::default(),
			name: String::new(),
			email: String::new(),
			update_name_wrong_password: false,
			update_name_taken: false,
			update_email_wrong_password: false,
			update_email_taken: false,
			update_password_wrong_password: false,
			update_password_dont_match: false,
			update_password_too_short: false,
			update_password_too_long: false,
			update_password_missing_characters: false,
			delete_account_wrong_password: false,
			changed_name: false,
			changed_email: false,
			changed_password: false,
			unknown_error: false,
		}
	}
}
impl Account {
	pub fn new(name: &str, email: &str) -> Account {
		Account {
			base: Base::default(),
			name: name.to_owned(),
			email: email.to_owned(),
			update_name_wrong_password: false,
			update_name_taken: false,
			update_email_wrong_password: false,
			update_email_taken: false,
			update_password_wrong_password: false,
			update_password_dont_match: false,
			update_password_too_short: false,
			update_password_too_long: false,
			update_password_missing_characters: false,
			delete_account_wrong_password: false,
			changed_name: false,
			changed_email: false,
			changed_password: false,
			unknown_error: false,
		}
	}
}