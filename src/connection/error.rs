use std::error;
use std::fmt;
use std::io;
use openssl::ssl;
use protobuf;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Ssl(ssl::error::SslError),
	Protobuf(protobuf::error::ProtobufError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Io(ref err) => write!(f, "IO Error: {}", err),
			Error::Ssl(ref err) => write!(f, "SSL Error: {}", err),
			Error::Protobuf(ref err) => write!(f, "Protobuf Error: {}", err),
		}
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Io(ref err) => err.description(),
			Error::Ssl(ref err) => err.description(),
			Error::Protobuf(ref err) => err.description(),
		}
	}
	
	fn cause(&self) -> Option<&error::Error> {
		match *self {
			Error::Io(ref err) => Some(err),
			Error::Ssl(ref err) => Some(err),
			Error::Protobuf(ref err) => Some(err),
		}
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

impl From<ssl::error::SslError> for Error {
	fn from(err: ssl::error::SslError) -> Error {
		Error::Ssl(err)
	}
}

impl From<protobuf::error::ProtobufError> for Error {
	fn from(err: protobuf::error::ProtobufError) -> Error {
		Error::Protobuf(err)
	}
}
