use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};
use image_hasher::{HashAlg, HasherConfig};

pub fn validate_supported_image(bytes: &[u8]) -> Result<(), &'static str> {
    let format = image::guess_format(bytes).map_err(|_| "Unsupported image format")?;

    if matches!(
        format,
        ImageFormat::Jpeg | ImageFormat::Png | ImageFormat::WebP
    ) {
        Ok(())
    } else {
        Err("Unsupported image format")
    }
}

pub fn decode_image(bytes: &[u8]) -> Option<DynamicImage> {
    ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()
}

pub fn phash(img: &DynamicImage) -> [u8; 8] {
    HasherConfig::with_bytes_type::<[u8; 8]>()
        .hash_size(8, 8)
        .hash_alg(HashAlg::Mean)
        .preproc_dct()
        .to_hasher()
        .hash_image(img)
        .into_inner()
}
