use std::fs;
use ocr_json_common::TextBox;
use image::{Rgba};
use imageproc::drawing::{
    draw_filled_rect_mut
};
use imageproc::rect::Rect;
use std::env;
use std::path::Path;

fn main() {
    let img_file_name = if env::args().count() >= 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a target file path for image")
    };
    let json_file_name = if env::args().count() >= 3 {
      env::args().nth(2).unwrap()
    } else{
      panic!("Please enter a target file path for json")
    };
    let json = fs::read_to_string(json_file_name).expect("There was an error reading the file.");
    let ocr_rects: Vec<TextBox> = serde_json::from_str(&json).unwrap();
    let image_path = Path::new(&img_file_name);
    let white = Rgba([255u8, 255u8, 255u8, 255u8]);

    let mut img = image::open(image_path).unwrap();

    // run OCR on each word bounding box
    for rect in &ocr_rects {
        draw_filled_rect_mut(&mut img, Rect::at(rect.x, rect.y).of_size(rect.width, rect.height), white);
    }

    img.save("out.png").unwrap();
}
