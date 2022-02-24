use ndarray::{Array, Array2, Array3};

pub fn canny(&mut rgb_image: Array3<u8>) -> Array2<bool> {
}

pub fn gaussian(gray_array: &Array2<f32>) -> Array2<f32> {
    let width = gray_array.shape()[1];
    let height = gray_array.shape()[0];
    let gauss_weight = arr2(&[
        [1./256., 1./64., 3./128., 1./64., 1./256.],
        [1./ 64., 1./16., 3./ 32., 1./16., 1./ 64.],
        [3./128., 3./32., 9./ 64., 3./32., 3./128.],
        [1./ 64., 1./16., 3./ 32., 1./16., 1./ 64.],
        [1./256., 1./64., 3./128., 1./64., 1./256.],
    ]);
}
