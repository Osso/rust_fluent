use std::io::prelude::*;
use std::net;
use std::io;
use std::convert::From;

extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::Encodable;

extern crate time;

pub struct Fluentd<A: net::ToSocketAddrs> {
    pub address: A,
    pub stream: net::TcpStream,
}

#[derive(Debug)]
pub enum FluentError {
    DecodeError(json::EncoderError),
    IoError(io::Error),
}

impl From<io::Error> for FluentError {
    fn from(err: io::Error) -> FluentError {
        FluentError::IoError(err)
    }
}

impl From<json::EncoderError> for FluentError {
    fn from(err: json::EncoderError) -> FluentError {
        FluentError::DecodeError(err)
    }
}


impl <A: net::ToSocketAddrs> Fluentd<A> {
    pub fn new<'a>(address: A) -> Result<Fluentd<A>, FluentError> {
        let stream = net::TcpStream::connect(&address).unwrap();
        let client = Fluentd {
            address: address,
            stream: stream,
        };
        Ok(client)
    }

    pub fn write<'a, B: Encodable> (&mut self, tag: &'a str, object: &B) -> Result<(), FluentError> {
        let tag = try!(json::encode(&tag));
        let now = time::now();
        let record = try!(json::encode(object));
        let message = format!("[{},{},{}]", tag, now.to_timespec().sec, record);

        let _ = self.stream.write(&message.into_bytes());
        Ok(())
    }
}
