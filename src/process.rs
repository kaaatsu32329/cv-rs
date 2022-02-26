use ndarray::{Array, Array2, arr2};
use std::ops::Mul;
/*
pub fn canny(rgb_image: &Array3<u8>) -> Array2<bool> {
}
*/
pub fn gaussian(gray_array: &Array2<f32>) -> Array2<f32> {
    let mut gauss_array = vec![];
    let width = gray_array.shape()[1] as usize;
    let height = gray_array.shape()[0] as usize;
    let gauss_weight = arr2(&[
        [1./256., 1./64., 3./128., 1./64., 1./256.],
        [1./ 64., 1./16., 3./ 32., 1./16., 1./ 64.],
        [3./128., 3./32., 9./ 64., 3./32., 3./128.],
        [1./ 64., 1./16., 3./ 32., 1./16., 1./ 64.],
        [1./256., 1./64., 3./128., 1./64., 1./256.],
    ]);
    for y in 0..height {
        for x in 0..width {
            if x < 3 || x > width - 4 || y < 3 || y > height - 4 {
                gauss_array.push(gray_array[[y, x]]);
            } else {
                gauss_array.push(gray_array.slice(s![y-2..y+3, x-2..x+3]).mul(&gauss_weight).sum());
            }
        }
    }
    Array::from_shape_vec((height, width), gauss_array).unwrap()
}

pub fn binarization(gray_array: &Array2<f32>, threshold: f32) -> Array2<bool> {
    let mut bin_array = vec![];
    let width = gray_array.shape()[1] as usize;
    let height = gray_array.shape()[0] as usize;
    for y in 0..height {
        for x in 0..width {
            if gray_array[[y,x]] >= threshold {
                bin_array.push(true);
            } else {
                bin_array.push(false);
            }
        }
    }
    Array::from_shape_vec((height, width), bin_array).unwrap()
}

pub fn harris_corner(gray_array: &Array2<f32>) { // -> Array2<bool> {
}
