use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;
use image::{GenericImageView, Pixel};
//use image::{GenericImageView, Pixel, FilterType};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::str;
use std::time::{Instant};

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> Vec<u8> {
    // Set start on the timer
    let start = Instant::now();
    // Process the input image data
    let mut img_pre = image::load_from_memory(image_data).unwrap();
    // Image is resized to 300px X 300px
    let mut img = img_pre.resize(300, 300, image::imageops::FilterType::Gaussian);
    // Flatten image
    let mut flat_img: Vec<f32> = Vec::new();
    for (_x, _y, rgb) in img.pixels() {
        flat_img.push(rgb[2] as f32);
        flat_img.push(rgb[1] as f32);
        flat_img.push(rgb[0] as f32);
    }
    // Measure time to process input image
    println!("Loaded image in ... {:?}", start.elapsed());

    // Open and read the TFLite label input data
    let file = File::open("labelmap.txt").unwrap();
    let reader = BufReader::new(file);
    // Create dict/map from the label data
    let mut map = HashMap::new();
    // Process the lines of labelmap.txt
    let mut i: u32 = 0;
    for line in reader.lines() {
        if i != 0 {
            if line.unwrap() != "???" {
                let mut a = HashMap::new();
                a.insert("id", i.to_string());
                a.insert("name", line.unwrap());
                map.insert(i.to_string(), a);
                a.clear();
                i = i + 1;
            }
        }
    }
    println!("Labels: {:?}", map);

    /* TODO
    // Load TFLite model data
    let model_data: &[u8] = include_bytes!("detect.tflite");
    let labels = include_str!("labelmap.txt");

    let mut session = ssvm_tensorflow_interface::Session::new(model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
    session.add_input("input", &flat_img, &[1, 300, 300, 3]);
    //session.add_output("MobilenetV2/Predictions/Softmax");
    session.run();
    let res_vec: Vec<f32> = session.get_output("MobilenetV2/Predictions/Softmax");
    println!("{:?}", res_vec);
    // Parse results.
    let mut iter = 0;
    let mut box_vec: Vec<[f32; 4]> = Vec::new();
    while (iter * 4) < res_vec.len() {
        box_vec.push([
            res_vec[4 * iter + 1], // x1
            res_vec[4 * iter],     // y1
            res_vec[4 * iter + 3], // x2
            res_vec[4 * iter + 2], // y2
        ]);
        iter += 1;
    }
    println!("Parsed results in ... {:?}", start.elapsed());

    println!("Drawing box: {} results ...", box_vec.len());
    let line = Pixel::from_slice(&[0, 255, 0, 0]);
    for i in 0..box_vec.len() {
        let xy = box_vec[i];
        let x1: i32 = xy[0] as i32;
        let y1: i32 = xy[1] as i32;
        let x2: i32 = xy[2] as i32;
        let y2: i32 = xy[3] as i32;
        let rect = Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32);
        draw_hollow_rect_mut(&mut img, rect, *line);
    }
    */
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(80u8)).expect("Unable to write");
    println!("Drawn on image in ... {:?}", start.elapsed());

    return buf;
}
