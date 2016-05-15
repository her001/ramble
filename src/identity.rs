use std::path::PathBuf;

pub struct Identity {
	pub name: String,
	pub cert: PathBuf,
	pub key: PathBuf,
	pub comment: String,
}
