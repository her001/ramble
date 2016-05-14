use std::path::PathBuf;

pub struct Identity {
	name: String,
	cert: PathBuf,
	key: PathBuf,
	comment: String,
}
