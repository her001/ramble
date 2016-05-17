// Copyright 2016 Andrew Conrad
//
// This program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation; either version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program; if not, see <http://www.gnu.org/licenses>.
//
// Additional permission under GNU GPL version 3 section 7
//
// If you modify this Program, or any covered work, by linking or combining it with OpenSSL (or a modified version of that library), containing parts covered by the terms of Apache License 1.0, the licensors of this Program grant you additional permission to convey the resulting work. Corresponding Source for a non-source form of such a combination shall include the source code for the parts of OpenSSL used as well as that of the covered work.

extern crate ramble;

use std::path::PathBuf;

use ramble::connection::Connection;
use ramble::identity::Identity;

#[test]
fn connect() {
	let mut cert_path = PathBuf::new();
	cert_path.push("tests/test_cert.pem");
	let mut key_path = PathBuf::new();
	key_path.push("tests/test_key.pem");
	
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
