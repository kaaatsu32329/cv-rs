use ndarray::{Array, Array2, Array3};

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

pub fn cvt_rgb2hsv(rgb_array: &Array3<u8>) -> Array3<f32> { // Cylinder model
    let mut hsv_array = vec![];
    let width = rgb_array.shape()[1] as usize;
    let height = rgb_array.shape()[0] as usize;
    let mut hue: f32;
    let mut red; let mut blue; let mut green;
    for y in 0..height {
        for x in 0..width {
            red = rgb_array[[y, x, 0]];
            green = rgb_array[[y, x, 1]];
            blue = rgb_array[[y, x, 2]];
            let (max_element, min_element, which, same_all) = max_min(red, blue, green);
            if same_all {
                hsv_array.push(0.);
                hsv_array.push(0.);
            }
            else {
                if which == 1 {
                    hue = (60. * (green as f32 - blue as f32)) / (max_element - min_element) as f32;
                    if hue < 0. { hue += 360.; }
                    hsv_array.push(hue);
                    hsv_array.push((max_element as f32 - min_element as f32) / max_element as f32 * 255.);
                } else if which == 2 {
                    hue = (60. * (blue - red) as f32) / (max_element - min_element) as f32 + 120.;
                    if hue < 0. { hue += 360.; }
                    hsv_array.push(hue);
                    hsv_array.push((max_element as f32 - min_element as f32) / max_element as f32 * 255.);
                } else if which == 3 {
                    hue = (60. * (red - green) as f32) / (max_element - min_element) as f32 + 240.;
                    if hue < 0. { hue += 360.; }
                    hsv_array.push(hue);
                    hsv_array.push((max_element - min_element) as f32 / max_element as f32 * 255.);
                }
            }
            hsv_array.push(max_element as f32);
        }
    }
    Array::from_shape_vec((height, width, 3 as usize), hsv_array).unwrap()
}

pub fn cvt_hsv2rgb(hsv_array: &Array3<f32>) -> Array3<u8> {
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

fn max_min(alpha: u8, beta: u8, gumma: u8) -> (u8, u8, u8, bool) {
    if alpha == beta || alpha == gumma {
        return (alpha, alpha, 0, true)
    } else if alpha > beta || alpha > gumma {
        if beta > gumma {
            return (alpha, gumma, 1, false)
        } else {
            return (alpha, beta, 1, false)
        }
    } else if beta > gumma || beta > alpha {
        if gumma > alpha {
            return (beta, alpha, 2, false)
        } else {
            return (beta, gumma, 2, false)
        }
    } else if gumma > alpha || gumma > beta {
        if alpha > beta {
            return (gumma, beta, 3, false)
        } else {
            return (gumma, alpha, 3, false)
        }
    }
}
