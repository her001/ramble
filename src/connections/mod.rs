// Copyright 2016 Andrew Conrad
//
// This program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation; either version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program; if not, see <http:// www.gnu.org/licenses>.
//
// Additional permission under GNU GPL version 3 section 7
//
// If you modify this Program, or any covered work, by linking or combining it with OpenSSL (or a modified version of that library), containing parts covered by the terms of Apache License 1.0, the licensors of this Program grant you additional permission to convey the resulting work. Corresponding Source for a non-source form of such a combination shall include the source code for the parts of OpenSSL used as well as that of the covered work.

use mumble;
use openssl::ssl::{SslContext, SslMethod, SslStream};
use openssl::x509::X509FileType;
use protobuf::core::Message;
use protobuf::repeated::RepeatedField;
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};

use identity::Identity;
use connections::error::Error;

pub mod error;

pub struct Connection {
	server: SocketAddr,
	ssl_context: SslContext,
	tcp: SslStream<TcpStream>,
}

impl Connection {
	pub fn connect<T: ToSocketAddrs>(srv: T) -> Result<Connection, Error> {
		let mut srv = try!(srv.to_socket_addrs());
		let srv = srv.next().unwrap();
		let mut context: SslContext = try!(SslContext::new(SslMethod::Tlsv1));
		try!(context.set_cipher_list("EECDH+AESGCM:EDH+aRSA+AESGCM:DHE-RSA-AES256-SHA:DHE-RSA-AES128-SHA:AES256-SHA:AES128-SHA"));
		let tls_stream = try!(tcp_connect(srv, &context));
		
		let con = Connection {
			server: srv,
			ssl_context: context,
			tcp: tls_stream,
		};
		
		Ok(con)
	}
}

pub struct JoinedConnection {
	server: SocketAddr,
	ssl_context: SslContext,
	tcp: SslStream<TcpStream>,
	identity: Identity,
}

impl JoinedConnection {
	pub fn join(con: Connection, ident: Identity) -> Result<JoinedConnection, Error> {
		let mut context = con.ssl_context;
		try!(context.set_certificate_file(&ident.cert, X509FileType::PEM));
		try!(context.set_private_key_file(&ident.key, X509FileType::PEM));
		try!(context.check_private_key());
		let tls_stream = try!(tcp_connect(con.server, &context));
		
		let mut j_con = JoinedConnection {
			server: con.server,
			ssl_context: context,
			tcp: tls_stream,
			identity: ident,
		};
		
		let mut message = mumble::Version::new();
		message.set_version(0130);
		message.set_release("Ramble".to_string());
		message.set_os("Unknown".to_string());
		message.set_os_version("Unknown".to_string());
		try!(j_con.write_message(message));

		let mut message = mumble::Authenticate::new();
		let mut tokens = RepeatedField::new();
		tokens.push("".to_string());
		message.set_username(j_con.identity.name.clone());
		message.set_password("".to_string());
		message.set_tokens(tokens);
		try!(j_con.write_message(message));
		
		
		Ok(j_con)
	}
	
	fn write_message<T: Message>(&mut self, msg: T) -> Result<(), Error> {
		try!(msg.write_to_writer(&mut self.tcp));
		Ok(())
	}
}

fn tcp_connect(srv: SocketAddr, context: &SslContext) -> Result<SslStream<TcpStream>, Error> {
	let stream = try!(TcpStream::connect(srv));
	let tls_stream = try!(SslStream::connect(context, stream));
	Ok(tls_stream)
}
