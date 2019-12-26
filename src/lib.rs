use bytes::BytesMut;
use futures_codec::Decoder;
use std::io;

pub const DIVERSIONS: &str = "/var/lib/dpkg/diversions";

pub struct Diversion {
    pub of: BytesMut,
    pub to: BytesMut,
    pub by: BytesMut,
}

#[derive(Default)]
pub struct DiversionDecoder;

impl Decoder for DiversionDecoder {
    type Item = Diversion;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(of_pos) = line(&src) {
            if let Some(to_pos) = line(&src[of_pos + 1..]) {
                if let Some(by_pos) = line(&src[of_pos + to_pos + 2..]) {
                    let of = src.split_to(of_pos);
                    let _ = src.split_to(1);
                    let to = src.split_to(to_pos);
                    let _ = src.split_to(1);
                    let by = src.split_to(by_pos);
                    let _ = src.split_to(1);
                    return Ok(Some(Diversion { of, to, by }));
                }
            }
        }

        Ok(None)
    }

    fn decode_eof(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(of_pos) = line(&src) {
            if let Some(to_pos) = line(&src[of_pos + 1..]) {
                let of = src.split_to(of_pos);
                let _ = src.split_to(1);
                let to = src.split_to(to_pos);
                let _ = src.split_to(1);
                let by = src.split_to(src.len() - 1);
                return Ok(Some(Diversion { of, to, by }));
            }
        }

        Ok(None)
    }
}

fn line(source: &[u8]) -> Option<usize> {
    memchr::memchr(b'\n', &source)
}
