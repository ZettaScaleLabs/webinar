use zenoh::prelude::r#async::*;
use crc::{Crc, CRC_8_AUTOSAR};

#[async_std::main]
async fn main() {
    let z = zenoh::open(zenoh::config::default()).res().await.unwrap();
    let sub = z.declare_subscriber("demo/attachment").res().await.unwrap();
    pub const AUTOSAR: Crc<u8> = Crc::<u8>::new(&CRC_8_AUTOSAR);
    while let Ok(s) = sub.recv_async().await {
        let payload = s.payload.contiguous();
        let cs = AUTOSAR.checksum(payload.as_ref());
        let checks = match &s.attachment {
            Some(a) => {
                let r = "crc".as_bytes();
                if let Some(checksum) = a.get(&r) {
                    if checksum.contiguous()[0] == cs {
                        true
                    } else { false }
                } else { false }
            },
            None => {
                false
            }
        };
        if checks {
            println!("Received valid data: {:?} -- {}", String::from_utf8_lossy(&payload), cs);
        } else {
            println!("Received corrupted data");
        }
    }
}
