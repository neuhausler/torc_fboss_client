// The MIT License (MIT)
//
// Copyright (c) 2015 AT&T
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

// Simple example client to modify routes
//

extern crate torc_fboss_client;

use torc_fboss_client::api;
use std::env;

fn main() {
    let mut fboss = "127.0.0.1:5909".to_string();

    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        fboss = args[1].clone();
    }
    api::sync_routes(&fboss);

    let routes = api::get_routes(&fboss);
    for route in &routes {
        println!("Route {} --> {}", route.from, route.to);
    }

    api::add_route(&fboss, "10.250.3.94/32", "172.31.1.2");

    let routes = api::get_routes(&fboss);
    for route in &routes {
        println!("Route {} --> {}", route.from, route.to);
    }

    api::delete_route(&fboss, "10.250.3.94/32");

    let routes = api::get_routes(&fboss);
    for route in &routes {
        println!("Route {} --> {}", route.from, route.to);
    }

}
