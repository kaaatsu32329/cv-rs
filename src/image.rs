use image::{ImageResult, Bgr, Rgb, Luma};
use image::io::Reader as ImageReader;
use ndarray::{Array2, Array3};

pub struct NdImage<T>(pub T);

pub enum ColorStyle {
    Bgr,
    Rgb,
    Luma,
}

struct Mask {
    width: u16,
    height: u16,
    pixels: Array2<u8>,
}

struct Image {
    style: u8,
    width: u16,
    height: u16,
    pixels: Array3<u8>,
}

pub fn open_file(style: ColorStyle, path: impl AsRef<std::path::Path>) -> ImageResult<Array3<u8>> {
    match style {
        ColorStyle::Bgr => {
            let image = image::open(path)?.to_rgb8();
            let image: NdImage = NdImage(&image).into();
            image.to_owned()
        }
        ColorStyle::Rgb => {
            let image = image::open(path)?.to_rgb8();
            let image: NdImage =
        }
        ColorStyle::Luma => {}
    }
    Ok(image)
}

pub fn cvtColor(&mut image: image, style: ColorStyle) {
    match style {
        ColorStyle::Bgr => {}
        ColorStyle::Rgb => {}
        ColorStyle::Luma => {}
        _ => {}
    }
}
