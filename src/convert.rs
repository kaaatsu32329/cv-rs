use ndarray::{Array, Array2, Array3};
use image::{RgbImage, Rgb};

pub fn cvt_img2array(rgb_image: &RgbImage) -> Array3<u8> {
    let mut array_image = vec![];
    let width = rgb_image.width() as usize;
    let height = rgb_image.height() as usize;
    for y in 0..height as u32 {
        for x in 0..width as u32 {
            array_image.push(rgb_image.get_pixel(x, y)[0]);
            array_image.push(rgb_image.get_pixel(x, y)[1]);
            array_image.push(rgb_image.get_pixel(x, y)[2]);
        }
    }
    Array::from_shape_vec((height, width, 3 as usize), array_image).unwrap()
}

pub fn cvt_array2img(image_array: &Array3<u8>) -> RgbImage {
    let width = image_array.shape()[1] as u32;
    let height = image_array.shape()[0] as u32;
    let mut rgb_image = RgbImage::new(width, height);
    for x in 0..width as usize {
        for y in 0..height as usize {
            rgb_image.put_pixel(x as u32, y as u32, Rgb([image_array[[y, x, 0]], image_array[[y, x, 1]], image_array[[y, x, 2]]]));
        }
    }
    rgb_image
}

pub fn cvt_rgb2hsv(rgb_array: &Array3<u8>) -> Array3<f32> { // Cylinder model
    let mut hsv_array = vec![];
    let width = rgb_array.shape()[1] as usize;
    let height = rgb_array.shape()[0] as usize;
    let mut red; let mut blue; let mut green;
    for y in 0..height {
        for x in 0..width {
            red = rgb_array[[y, x, 0]];
            green = rgb_array[[y, x, 1]];
            blue = rgb_array[[y, x, 2]];
            let (max_element, min_element, which, same_all) = max_min(rgb_array[[y, x, 0]], rgb_array[[y, x, 1]], rgb_array[[y, x, 2]]);
            if same_all {
                hsv_array.push(0.);
            }
            else {
                match which {
                    1 => {
                        hsv_array.push((60. * ((green as f32 - blue as f32)) / (max_element - min_element) as f32 + 3600.) % 360.);
                    }
                    2 => {
                        hsv_array.push((60. * ((blue as f32 - red as f32)) / (max_element - min_element) as f32 + 3720.) % 360.);
                    }
                    3 => {
                        hsv_array.push((60. * ((red as f32 - green as f32)) / (max_element - min_element) as f32 + 3840.) % 360.);
                    }
                    _ => {
                        panic!("Error to convert RGB to HSV.");
                    }
                }
            }
            hsv_array.push(255. * (max_element - min_element) as f32 / max_element as f32);
            hsv_array.push(max_element as f32);
        }
    }
    Array::from_shape_vec((height, width, 3 as usize), hsv_array).unwrap()
}

pub fn cvt_hsv2rgb(hsv_array: &Array3<f32>) -> Array3<u8> {
    let mut rgb_array = vec![];
    let width = hsv_array.shape()[1] as usize;
    let height = hsv_array.shape()[0] as usize;
    let mut max: f32; let mut min: f32;
    for y in 0..height {
        for x in 0..width {
            max = hsv_array[[y,x,2]];
            min = hsv_array[[y,x,2]] - ((hsv_array[[y,x,1]] / 255.) * hsv_array[[y,x,2]]);
            match (hsv_array[[y,x,0]] / 60.) as u8 {
                0 => {
                    rgb_array.push(max as u8);
                    rgb_array.push(((hsv_array[[y,x,0]] as f32 / 60.) * (max - min) as f32 + min as f32) as u8);
                    rgb_array.push(min as u8);
                }
                1 => {
                    rgb_array.push((((120. - hsv_array[[y,x,0]] as f32) / 60.) * (max - min) as f32 + min as f32) as u8);
                    rgb_array.push(max as u8);
                    rgb_array.push(min as u8);
                }
                2 => {
                    rgb_array.push(min as u8);
                    rgb_array.push(max as u8);
                    rgb_array.push((((hsv_array[[y,x,0]] as f32 - 120.) / 60.) * (max - min) as f32 + min as f32) as u8);
                }
                3 => {
                    rgb_array.push(min as u8);
                    rgb_array.push((((240. - hsv_array[[y,x,0]] as f32) / 60.) * (max - min) as f32 + min as f32) as u8);
                    rgb_array.push(max as u8);
                }
                4 => {
                    rgb_array.push((((hsv_array[[y,x,0]] as f32 - 240.) / 60.) * (max - min) as f32 + min as f32) as u8);
                    rgb_array.push(min as u8);
                    rgb_array.push(max as u8);
                }
                5 => {
                    rgb_array.push(max as u8);
                    rgb_array.push(min as u8);
                    rgb_array.push((((360. - hsv_array[[y,x,0]] as f32) / 60.) * (max - min) as f32 + min as f32) as u8);
                }
                _ => {
                    panic!("!!!");
                }
            }
        }
    }
    Array::from_shape_vec((height, width, 3 as usize), rgb_array).unwrap()
}

pub fn cvt_rgb2gray(rgb_array: &Array3<u8>) -> Array2<f32> {
    let mut gray_array = vec![];
    let width = rgb_array.shape()[1] as usize;
    let height = rgb_array.shape()[0] as usize;
    for y in 0..height {
        for x in 0..width {
            gray_array.push(0.299 * rgb_array[[y, x, 0]] as f32 + 0.587 * rgb_array[[y, x, 1]] as f32 + 0.114 * rgb_array[[y, x, 2]] as f32);
        }
    }
    Array::from_shape_vec((height, width), gray_array).unwrap()
}

pub fn cvt_gray2rgb(gray_array: &Array2<f32>) -> Array3<u8> {
    let mut rgb_array = vec![];
    let width = gray_array.shape()[1] as usize;
    let height = gray_array.shape()[0] as usize;
    for y in 0..height {
        for x in 0..width {
            rgb_array.push(gray_array[[y, x]] as u8);
            rgb_array.push(gray_array[[y, x]] as u8);
            rgb_array.push(gray_array[[y, x]] as u8);
        }
    }
    Array::from_shape_vec((height, width, 3), rgb_array).unwrap()
}

pub fn cvt_bin2rgb(bin_array: &Array2<bool>) -> Array3<u8> {
    let mut rgb_array = vec![];
    let width = bin_array.shape()[1] as usize;
    let heigth = bin_array.shape()[0] as usize;
    for y in 0..heigth {
        for x in 0..width {
            if bin_array[[y,x]] {
                rgb_array.push(255);
                rgb_array.push(255);
                rgb_array.push(255);
            } else {
                rgb_array.push(0);
                rgb_array.push(0);
                rgb_array.push(0);
            }
        }
    }
    Array::from_shape_vec((heigth, width, 3 as usize), rgb_array).unwrap()
}

pub fn mask(width: usize, height: usize) -> Array2<bool> {
    let mut mask_array = vec![];
    #[allow(unused_variables)]
    for y in 0..width {
        for x in 0..height {
            mask_array.push(false);
        }
    }
    Array::from_shape_vec((height, width), mask_array).unwrap()
}

fn max_min(alpha: u8, beta: u8, gamma: u8) -> (u8, u8, u8, bool) {
    if alpha == beta && alpha == gamma {
        return (alpha, alpha, 0, true);
    } else if alpha >= beta && alpha > gamma {
        if beta > gamma {
            return (alpha, gamma, 1, false);
        } else {
            return (alpha, beta, 1, false);
        }
    } else if beta >= gamma && beta > alpha {
        if gamma > alpha {
            return (beta, alpha, 2, false);
        } else {
            return (beta, gamma, 2, false);
        }
    } else if gamma >= alpha && gamma > beta {
        if alpha > beta {
            return (gamma, beta, 3, false);
        } else {
            return (gamma, alpha, 3, false);
        }
    } else {
        panic!("Error");
    }
}
