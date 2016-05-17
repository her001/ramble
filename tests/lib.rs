extern crate ramble;

use std::path::PathBuf;

use ramble::connection::Connection;
use ramble::identity::Identity;

#[test]
fn connect() {
	let mut cert_path = PathBuf::new();
	cert_path.push("cert.pem");
	let mut key_path = PathBuf::new();
	key_path.push("key.pem");
	
	let ident = Identity {
		name: "Test".to_string(),
		cert: cert_path,
		key: key_path,
		comment: "Test".to_string(),
	};
	
	let con = Connection::join_server("localhost:64738", ident);
	match con {
		Result::Ok(_) => (),
		Result::Err(err) =>
			panic!("Connection failed: {}", err),
	}
}
