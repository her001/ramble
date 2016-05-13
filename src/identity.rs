use std::fs::File;

pub struct Identity {
	name: String,
	cert: File,
	key: File,
	comment: String,
}
