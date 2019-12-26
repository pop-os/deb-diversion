//! Display packages which are installed.

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
