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
