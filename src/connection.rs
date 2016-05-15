use openssl::ssl::*;
use std::net::{TcpStream, ToSocketAddrs};

use identity::Identity;

pub struct Connection<T: ToSocketAddrs> {
	pub server: T,
	pub ident: Identity,
	tcp: SslStream<TcpStream>,
}

//pub fn connect
