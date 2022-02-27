use ndarray::{Array, Array2, arr2};
use std::ops::Mul;
/*
pub fn canny(rgb_image: &Array3<u8>) -> Array2<bool> {
}
*/
pub fn gaussian3x3(gray_array: &Array2<f32>) -> Array2<f32> {
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
            if x < 2 || x > width - 3 || y < 2 || y > height - 3 {
                gauss_array.push(gray_array[[y, x]]);
            } else {
                gauss_array.push(gray_array.slice(s![y-2..y+3, x-2..x+3]).mul(&gauss_weight).sum());
            }
        }
    }
    Array::from_shape_vec((height, width), gauss_array).unwrap()
}

pub fn gaussian(gray_array: &Array2<f32>, size: u8) -> Array2<f32> {
    if size%2 == 0 {
        panic!("Argument Error.");
    }
    let mut gauss_array = vec![];
    let width = gray_array.shape()[1] as usize;
    let height = gray_array.shape()[0] as usize;
    let mut gauss_weight: Array2<f32> = Array::zeros((size as usize, size as usize));
    for j in 0..size {
        for i in 0..size {
            gauss_weight[[j as usize, i as usize]] = (binomial_coefficient(size as u32, i as u32) * binomial_coefficient(size as u32, i as u32)) as f32;
        }
    }
    for y in 0..height {
        for x in 0..width {
            if x < ((size-1)/2) as usize || x >= width - ((size+1)/2) as usize || y < ((size-1)/2) as usize || y >= height - ((size+1)/2) as usize {
                gauss_array.push(gray_array[[y,x]]);
                // ToDo: 縁の処理の追加。現状はオリジナルの画素のまま。
            } else {
                gauss_array.push(gray_array.slice(s![y-((size-1)/2) as usize..y+((size+1)/2) as usize, x-((size-1)/2) as usize..x+((size+1)/2) as usize]).mul(&gauss_weight).sum() / gauss_weight.sum());
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
/*
pub fn harris_corner(gray_array: &Array2<f32>) -> Array2<bool> {
}*/

fn binomial_coefficient(n: u32, k: u32) -> u32 {
    if k == 0 || k == n {
        1
    } else {
        binomial_coefficient(n-1, k-1) * n / k
    }
}

pub fn gradient_x(gray_array: &Array2<f32>) -> Array2<f32> {
    let mut gradient_array = vec![];
    let width = gray_array.shape()[1] as usize;
    let height = gray_array.shape()[0] as usize;
    let grad = arr2(&[
            [-0.125, 0., 0.125],
            [-0.250, 0., 0.250],
            [-0.125, 0., 0.125],
        ]);
    for y in 0..height {
        for x in 0..width {
            if x == 0 {
                if y > 0 && y < height - 1 {
                    gradient_array.push(gray_array.slice(s![y-1..y+2, x..x+2]).mul(&grad.slice(s![.., 0..3;2])).sum());
                } else if y == 0 {
                    gradient_array.push(gray_array.slice(s![y..y+2, x..x+2]).mul(&grad.slice(s![0..3;2, 0..3;2])).sum());
                } else {
                    gradient_array.push(gray_array.slice(s![y-1..y+1, x..x+2]).mul(&grad.slice(s![0..3;2, 0..3;2])).sum());
                }
            } else if x > width - 2 {
                if y > 0 && y < height - 1 {
                    gradient_array.push(gray_array.slice(s![y-1..y+2, x-1..x+1]).mul(&grad.slice(s![.., 0..3;2])).sum());
                } else if y == 0 {
                    gradient_array.push(gray_array.slice(s![y..y+2, x-1..x+1]).mul(&grad.slice(s![0..3;2, 0..3;2])).sum());
                } else {
                    gradient_array.push(gray_array.slice(s![y-1..y+1, x-1..x+1]).mul(&grad.slice(s![0..3;2, 0..3;2])).sum());
                }
            } else if y == 0 {
                gradient_array.push(gray_array.slice(s![y..y+2, x-1..x+2]).mul(&grad.slice(s![0..3;2, ..])).sum());
            } else if y == height - 1 {
                gradient_array.push(gray_array.slice(s![y-1..y+1, x-1..x+2]).mul(&grad.slice(s![0..3;2, ..])).sum());
            } else {
                gradient_array.push(gray_array.slice(s![y-1..y+2, x-1..x+2]).mul(&grad).sum());
            }
        }
    }
    Array::from_shape_vec((height, width), gradient_array).unwrap()
}

pub fn gradient_y(gray_array: &Array2<f32>) -> Array2<f32> {
    let mut gradient_array = vec![];
    let width = gray_array.shape()[1] as usize;
    let height = gray_array.shape()[0] as usize;
    let grad = arr2(&[
            [-0.125, -0.250, -0.125],
            [ 0.   ,  0.   ,  0.   ],
            [ 0.125,  0.250,  0.125],
        ]);
    for y in 0..height {
        for x in 0..width {
            if x == 0 {
                if y > 0 && y < height - 1 {
                    gradient_array.push(gray_array.slice(s![y-1..y+2, x..x+2]).mul(&grad.slice(s![.., 0..3;2])).sum());
                } else if y == 0 {
                    gradient_array.push(gray_array.slice(s![y..y+2, x..x+2]).mul(&grad.slice(s![0..3;2, 0..3;2])).sum());
                } else {
                    gradient_array.push(gray_array.slice(s![y-1..y+1, x..x+2]).mul(&grad.slice(s![0..3;2, 0..3;2])).sum());
                }
            } else if x > width - 2 {
                if y > 0 && y < height - 1 {
                    gradient_array.push(gray_array.slice(s![y-1..y+2, x-1..x+1]).mul(&grad.slice(s![.., 0..3;2])).sum());
                } else if y == 0 {
                    gradient_array.push(gray_array.slice(s![y..y+2, x-1..x+1]).mul(&grad.slice(s![0..3;2, 0..3;2])).sum());
                } else {
                    gradient_array.push(gray_array.slice(s![y-1..y+1, x-1..x+1]).mul(&grad.slice(s![0..3;2, 0..3;2])).sum());
                }
            } else if y == 0 {
                gradient_array.push(gray_array.slice(s![y..y+2, x-1..x+2]).mul(&grad.slice(s![0..3;2, ..])).sum());
            } else if y == height - 1 {
                gradient_array.push(gray_array.slice(s![y-1..y+1, x-1..x+2]).mul(&grad.slice(s![0..3;2, ..])).sum());
            } else {
                gradient_array.push(gray_array.slice(s![y-1..y+2, x-1..x+2]).mul(&grad).sum());
            }
        }
    }
    Array::from_shape_vec((height, width), gradient_array).unwrap()
}
