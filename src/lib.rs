#[macro_use(s)]
extern crate ndarray;

#[macro_use]
extern crate ndarray_linalg;

mod convert;
mod process;

pub use crate::convert::*;
pub use crate::process::*;

#[cfg(test)]
mod tests {
    use image::{open};
    use crate::convert::*;
    use rand::Rng;

    #[test]
    fn convert_check() {
        let sample_image = open("./sample/noise.jpeg").unwrap().into_rgb8();
        let rgb_sample = cvt_img2array(&sample_image);
        let width = rgb_sample.shape()[1] as usize;
        let height = rgb_sample.shape()[0] as usize;
        let hsv_sample = cvt_rgb2hsv(&rgb_sample);
        let re_rgb_sample = cvt_hsv2rgb(&hsv_sample);
        let mut rng = rand::thread_rng();
        let (x, y) = (rng.gen_range(0..width), rng.gen_range(0..height));
        assert_eq!(re_rgb_sample[[y, x, 0]], rgb_sample[[y, x, 0]]);
        assert_eq!(re_rgb_sample[[y, x, 1]], rgb_sample[[y, x, 1]]);
        assert_eq!(re_rgb_sample[[y, x, 2]], rgb_sample[[y, x, 2]]);
    }
}
