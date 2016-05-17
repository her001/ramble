use mumble;
use openssl::ssl::{SslContext, SslMethod, SslStream};
use openssl::x509::X509FileType;
use protobuf::core::Message;
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};

use identity::Identity;
use connection::error::Error;

pub mod error;

pub struct Connection {
	pub server: SocketAddr,
	pub ident: Identity,
	tcp: SslStream<TcpStream>,
}

impl Connection {
	pub fn join_server<T: ToSocketAddrs>(srv: T, id: Identity) -> Result<Connection, Error> {
		let mut srv = try!(srv.to_socket_addrs());
		let srv = srv.next().unwrap();
		let mut context: SslContext = try!(SslContext::new(SslMethod::Tlsv1));
		try!(context.set_certificate_file(&id.cert, X509FileType::PEM));
		try!(context.set_private_key_file(&id.key, X509FileType::PEM));
		try!(context.check_private_key());
		try!(context.set_cipher_list("EECDH+AESGCM:EDH+aRSA+AESGCM:DHE-RSA-AES256-SHA:DHE-RSA-AES128-SHA:AES256-SHA:AES128-SHA"));
		let stream = try!(TcpStream::connect(srv));
		let tls_stream = try!(SslStream::connect(&context, stream));
		
		let mut con = Connection {
			server: srv,
			ident: id,
			tcp: tls_stream,
		};
		
		let mut message = mumble::Version::new();
		message.set_version(0130);
		message.set_release("Ramble".to_string());
		try!(con.write_message(message));
		
		let mut message = mumble::Authenticate::new();
		message.set_username(con.ident.name.clone());
		try!(con.write_message(message));
		
		Ok(con)
	}
	
	fn write_message<T: Message>(&mut self, msg: T) -> Result<(), Error> {
		try!(msg.write_to_writer(&mut self.tcp));
		Ok(())
	}
}
