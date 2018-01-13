// Copyright (c) 2013-2017 Sandstorm Development Group, Inc. and contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;

extern crate capnp;
#[macro_use] extern crate capnp_rpc;
//extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_curl;
extern crate curl;

pub mod http_capnp {
  include!(concat!(env!("OUT_DIR"), "/http_capnp.rs"));
}

pub mod client;
pub mod server;

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 3 {
        match &args[1][..] {
            "client" => return client::main(),
            "server" => return server::main(),
            _ => ()
        }
    }

    println!("usage: {} [client | server] ADDRESS", args[0]);
}
