use ndarray::*;
use std::ops::Mul;

pub trait GetValue {
    fn get_max(&self) -> f32;
}

impl GetValue for ArrayBase<OwnedRepr<f32>, Ix2> {
    fn get_max(&self) -> f32 {
        let width = self.shape()[1] as usize;
        let height = self.shape()[0] as usize;
        let mut max = 0.;
        for y in 0..height {
            for x in 0..width {
                max = if max < self[[y, x]] as f32 {
                    self[[y,x]]
                } else {
                    max
                }
            }
        }
        max
    }
}

pub fn canny_edge(gray_array: &Array2<f32>) -> Array2<bool> {
    let width = gray_array.shape()[1] as usize;
    let height = gray_array.shape()[0] as usize;

    let gaussed_array = gaussian3x3(&gray_array);
    let i_x = gradient_x(&gaussed_array);
    let i_y = gradient_y(&gaussed_array);
    let hyp_array = image_hypotenuse(&i_x, &i_y).unwrap();
    let hyp_array = &hyp_array / hyp_array.get_max() * 255.;
    let mut theta = arctan2(&i_y, &i_x).unwrap();
    for y in 0..height {
        for x in 0..width {
            theta[[y,x]] = angle_discretization(theta[[y,x]]);
        }
    }

    edge_or_not(&hyp_array, &theta).unwrap()
}

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

pub fn harris_corner(gray_array: &Array2<f32>) -> Array2<bool> {
    let mut pixels = vec![];
    let width = gray_array.shape()[1] as usize;
    let height = gray_array.shape()[0] as usize;
    let mut corner_array = vec![];

    // Make false mask.
    #[allow(unused_variables)]
    for y in 0..height {
        for x in 0..width {
            corner_array.push(false);
        }
    }

    let gaussed_array = gaussian(&gray_array, 5);
    let i_x = gradient_x(&gaussed_array);
    let i_y = gradient_y(&gaussed_array);
    let i_xx = i_x.clone().mul(&i_x);
    let i_yy = i_y.clone().mul(&i_y);
    let i_xy = i_x.mul(&i_y);
    for j in 1..height-1 {
        for i in 1..width-1 {
            let sum_xx = i_xx.slice(s![j-1..j+2, i-1..i+2]).sum();
            let sum_yy = i_yy.slice(s![j-1..j+2, i-1..i+2]).sum();
            let sum_xy = i_xy.slice(s![j-1..j+2, i-1..i+2]).sum();
            let r = (sum_xx*sum_yy) - sum_xy*sum_xy - 0.05*(sum_xx+sum_yy)*(sum_xx+sum_yy);
            if r > 10000. {
                pixels.push((i,j,r));
            }
        }
    }
    pixels.sort_by(|a,b| b.2.partial_cmp(&a.2).unwrap());
    let mut corner_pixels = vec![];
    corner_pixels.push((pixels[0].0, pixels[0].1));
    let distance = 10f32;
    for m in 0..pixels.len() {
        for n in 0..corner_pixels.len() {
            if (pixels[m].0 as f32 - corner_pixels[n].0 as f32).abs() <= distance && (pixels[m].1 as f32 - corner_pixels[n].1 as f32).abs() <= distance {
                break;
            } else {
                corner_array[pixels[m].1 * width + pixels[m].0] = true;
            }
        }
    }
    Array::from_shape_vec((height, width), corner_array).unwrap()
}

fn binomial_coefficient(n: u32, k: u32) -> u32 {
    if k == 0 || k == n {
        1
    } else {
        binomial_coefficient(n-1, k-1) * n / k
    }
}

fn image_hypotenuse(gray_array1: &Array2<f32>, gray_array2: &Array2<f32>) -> Result<Array2<f32>, String> {
    let width = gray_array1.shape()[1] as usize;
    let height = gray_array1.shape()[0] as usize;
    if width != gray_array2.shape()[1] || height != gray_array2.shape()[0] {
        Err("Error! Array sizes vary.".to_string())
    } else {
        let mut hyp_array = vec![];
        for y in 0..height {
            for x in 0..width {
                hyp_array.push((gray_array1[[y,x]].powf(2.0) + gray_array2[[y,x]].powf(2.0)).sqrt());
            }
        }
        Ok(Array::from_shape_vec((height, width), hyp_array).unwrap())
    }
}

fn arctan2(gray_array1: &Array2<f32>, gray_array2: &Array2<f32>) -> Result<Array2<f32>, String> {
    let width = gray_array1.shape()[1] as usize;
    let height = gray_array1.shape()[0] as usize;
    if width != gray_array2.shape()[1] || height != gray_array2.shape()[0] {
        Err("Error! Array sizes vary.".to_string())
    } else {
        let mut arctan_array = vec![];
        for y in 0..height {
            for x in 0..width {
                arctan_array.push(gray_array1[[y,x]].atan2(gray_array2[[y,x]]));
            }
        }
        Ok(Array::from_shape_vec((height, width), arctan_array).unwrap())
    }
}

fn angle_discretization(mut angle: f32) -> f32 {
    if angle < 0. { angle += 180.; }
    if (angle >= 22.5) && (angle < 67.5) {
        return 45.
    } else if (angle >= 67.5) && (angle < 112.5) {
        return 90.
    } else if (angle >= 112.5) && (angle < 157.5) {
        return 135.
    } else {
        return 0.
    }
}

fn edge_or_not(strength_array: &Array2<f32>, angle_array: &Array2<f32>) -> Result<Array2<bool>, String> {
    let width = strength_array.shape()[1] as usize;
    let height = strength_array.shape()[0] as usize;
    if width != angle_array.shape()[1] || height != angle_array.shape()[0] {
        Err("Error! Array sizes vary.".to_string())
    } else {
        let mut bool_array = vec![];
        for y in 0..height {
            for x in 0..width {
                if x>0 && x<width-1 && y>0 && y<height-1 && strength_array[[y,x]] > 10. {
                    match angle_array[[y,x]] as u8 {
                        45 => {
                            bool_array.push(strength_array[[y,x]] > strength_array[[y,x+1]] && strength_array[[y,x]] > strength_array[[y,x-1]]);
                        }
                        90 => {
                            bool_array.push(strength_array[[y,x]] > strength_array[[y-1,x-1]] && strength_array[[y,x]] > strength_array[[y+1,x+1]]);
                        }
                        135 => {
                            bool_array.push(strength_array[[y,x]] > strength_array[[y-1,x-1]] && strength_array[[y,x]] > strength_array[[y+1,x+1]]);
                        }
                        0 => {
                            bool_array.push(strength_array[[y,x]] > strength_array[[y-1,x]] && strength_array[[y,x]] > strength_array[[y+1,x]]);
                        }
                        _ => {
                            bool_array.push(strength_array[[y,x]] > strength_array[[y-1,x]] && strength_array[[y,x]] > strength_array[[y+1,x]]);
                            // Log
                        }
                    }
                } else {
                    bool_array.push(false);
                }
            }
        }
        Ok(Array::from_shape_vec((height, width), bool_array).unwrap())
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
