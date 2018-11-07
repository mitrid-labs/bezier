[![Travis branch](https://img.shields.io/travis/mitrid-labs/bezier/master.svg)](https://travis-ci.org/mitrid-labs/bezier)
[![Coveralls github branch](https://img.shields.io/coveralls/github/mitrid-labs/bezier/master.svg)](https://coveralls.io/github/mitrid-labs/bezier?branch=master)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/mitrid-labs/bezier/blob/master/LICENSE)

Bezier is a demo implementation of a Bitcoin-like blockchain using the [Mitrid Core](https://github.com/mitrid-labs/mitrid-core) library.
<br>
<br>

## Install

To install the Bezier library, add in your Cargo.toml:


```toml
# Cargo.toml

[dependencies]
bezier_lib = "^0.1"
```

and in the root of your crate:

```rust

extern crate bezier_lib;
```

## Run

Bezier is still not runnable.

## Testing

To build the tests you need `libsodium` and `libclang` installed on your system.
If the dependencies are resolved, then you can run the tests by typing `cargo test` on your terminal.

## [License](LICENSE)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
