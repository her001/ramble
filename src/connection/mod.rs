use openssl::ssl::{SslContext, SslMethod, SslStream};
use openssl::x509::X509FileType;
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
		
		//mumble auth
		
		Ok(Connection {
			server: srv,
			ident: id,
			tcp: tls_stream,
		})
	}
}
