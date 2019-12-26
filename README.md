# deb-diversion

[![Crates.io](https://img.shields.io/crates/v/deb-diversion)](https://crates.io/crates/deb-diversion)

Futures codec for Debian's dpkg diversion file.

## Example

Generates equivalent output to `dpkg-divert --list`.

```rust
#[macro_use]
extern crate fomat_macros;

const DIVERSIONS: &str = "/var/lib/dpkg/diversions";

use async_std::fs::File;
use deb_diversion::*;
use futures::{executor, prelude::*};
use futures_codec::FramedRead;
use std::str;

fn main() {
    executor::block_on(async_main());
}

async fn async_main() {
    let file = File::open(DIVERSIONS).await.unwrap();

    let mut frames = FramedRead::new(file, DiversionDecoder::default());

    while let Some(event) = frames.next().await {
        let event = event.unwrap();

        pintln!(
            "diversion of "
            (str::from_utf8(&event.of).unwrap())
            " to "
            (str::from_utf8(&event.to).unwrap())
            " by "
            (str::from_utf8(&event.by).unwrap())
        );
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions
