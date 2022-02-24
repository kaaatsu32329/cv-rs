use ndarray::{Array, Array2, Array3};
use image::*;
use std::path::PathBuf;

// TODO: 後回しで行う。

enum ColorStyle {
    Rgb,
    Hsv,
    Gray,
}

struct ImageMat {
    image: Array3<f32>,
    style: ColorStyle,
}

impl ImageMat {
    fn open_image(path: &PathBuf) {
        let opened_image = open(path);
        let open_array = vec![];
        let width = opened_image.width() as usize;
        let height = opened_image.height() as usize;
        for y in 0..height as u32 {
            for x in 0..width as u32 {
                open_array.push(rgb_image.get_pixel(x, y)[0]);
                open_array.push(rgb_image.get_pixel(x, y)[1]);
                open_array.push(rgb_image.get_pixel(x, y)[2]);
            }
        }
        image = Array::from_shape_vec((height, width, 3 as usize), open_image).unwrap();
        style = ColorStyle::Rgb;
    }

    fn cvt_Color(target: ColorStyle) {
        match target {
            ColorStyle::Rgb => {}
            ColorStyle::Hsv => {}
            ColorStyle::Gray => {}
        }
    }
}
