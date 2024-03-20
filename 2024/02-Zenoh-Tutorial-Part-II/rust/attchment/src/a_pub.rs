mod a_sub;

use std::time::Duration;
use zenoh::prelude::r#async::*;
use zenoh::sample::Attachment;
use crc::{Crc, CRC_8_AUTOSAR};

#[async_std::main]
async fn main() {
    let z = zenoh::open(zenoh::config::default()).res().await.unwrap();
    let value = "This is a message whose integrity needs to be verified";
    pub const AUTOSAR: Crc<u8> = Crc::<u8>::new(&CRC_8_AUTOSAR);
    let mut checksum = vec!(0u8, 1);
    checksum[0] = AUTOSAR.checksum(value.as_bytes());
    let mut atch = Attachment::new();
    atch.insert("crc", checksum.as_slice());

    loop {
        let _ = z
            .put("demo/attachment", value)
            .with_attachment(atch.clone())
            .res().await;
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
