// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast exec-env directive doesn't work for check-fast
// compile-flags: --cfg ndebug
// exec-env:RUST_LOG=conditional-debug-macro-off=4

fn main() {
    // only fails if debug! evaluates its argument.
    debug2!("{:?}", { if true { fail2!() } });
}
