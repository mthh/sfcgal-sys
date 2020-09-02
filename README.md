# sfcgal-sys

[![Build Status Travis](https://travis-ci.org/mthh/sfcgal-sys.svg?branch=master)](https://travis-ci.org/mthh/sfcgal-sys)
[![Crates.io](https://img.shields.io/crates/v/sfcgal-sys.svg)](https://crates.io/crates/sfcgal-sys)

Rust low-level FFI bindings to [`SFCGAL`](http://oslandia.github.io/SFCGAL/) C API.  
Don't use this crate directly, prefer it's higher-level wrapper : [sfcgal-rs](https://github.com/mthh/sfcgal-rs).


## Internals

This crate contains a few lines of C code (compiled as a static library with `cc` crate) wrapping SFCGAL C API in order to replace the error and warning handlers (which use `printf` by default).  
It expects SFCGAL to be installed as a system library and that you have the header file for it's C API.
Then bindgen is run to generate these low-levels bindings.

In addition to all the `sfcgal_` types and functions, this crate expose :
- a Rust `initialize` function: it calls `sfcgal_init()` function then it calls a custom `w_sfcgal_init_handlers()` function which replace the error and warning handlers from `printf` to a char buffer. That `initialize` function internally uses `std::sync::ONCE_INIT` to ensure it's only called once.
- two C functions `w_sfcgal_get_last_error` and `w_sfcgal_get_last_warning` which repectively reads the buffer containing the error message and the buffer containing the warning message.

In the future it could probably be improved by not requiring SFCGAL to be installed as a system library.


## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
