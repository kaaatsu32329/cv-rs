extern crate gtk;
extern crate gdk_pixbuf;
extern crate rscam;
extern crate ndarray;

use gdk_pixbuf::{Pixbuf, PixbufLoader};
use ndarray::{Array, Array2, Array3};

pub fn cvtPIX2ARRAY(pixbufImage: &Pixbuf) -> Array3<i8> {
    let mut arrayImage = vec![];
    let width = pixbufImage.get_width();
    let height = pixbufImage.get_height();
    let pixels;
    unsafe {
        pixels = pixbufImage.get_pixels();
    }
    for element in pixels.chunks(3) {
        arrayImage.push(element[0]);
        arrayImage.push(element[1]);
        arrayImage.push(element[2]);
    }
    Array::from_vec(arrayImage).into_shape((height, width, 3)).unwrap()
}

//pub fn cvtRGB2GRAY(arrayImage: Array3<i8>) -> Array2<f32> {
//}

pub fn cvtRGB2HSV(rgbImage: Array3<i8>) -> Array3<i8> {
    let mut hsvImage = vec![];
    for rgb in rgbImage
}

pub fn cvtHSV2RGB(hsvImage: Array3<i8>) -> Array3<i8> {
}
