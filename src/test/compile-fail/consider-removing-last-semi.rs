// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn f() -> String {  //~ ERROR E0269
    0u8;
    "bla".to_string();  //~ HELP consider removing this semicolon
}

fn g() -> String {  //~ ERROR E0269
    "this won't work".to_string();
    "removeme".to_string(); //~ HELP consider removing this semicolon
}

fn main() {}
