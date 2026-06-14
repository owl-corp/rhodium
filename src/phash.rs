use image::DynamicImage;
use image::ImageReader;
use image_hasher::{HashAlg, HasherConfig};
use std::io::Cursor;

pub fn decode_image(bytes: &[u8]) -> Option<DynamicImage> {
    ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()
}

pub fn phash(img: &DynamicImage) -> [u8; 8] {
    let hasher = HasherConfig::with_bytes_type::<[u8; 8]>()
        .hash_size(8, 8)
        .hash_alg(HashAlg::Mean)
        .preproc_dct()
        .to_hasher();

    let hash = hasher.hash_image(img);

    let mut out = [0u8; 8];
    out.copy_from_slice(hash.as_bytes());
    out
}
