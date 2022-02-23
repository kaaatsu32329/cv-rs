use ndarray::{Array, Array2, Array3};

pub fn get_mask(image: &T) -> Array2<bool> {
    let mut mask_array = vec![];
    let width = image.shape()[1] as usize;
    let height = image.shape()[0] as usize;
    #[allow(unused_variables)]
    for y in 0..width {
        for x in 0..height {
            mask_array.push(false);
        }
    }
    Array::from_shape_vec((height, width), mask_array).unwrap()
}
