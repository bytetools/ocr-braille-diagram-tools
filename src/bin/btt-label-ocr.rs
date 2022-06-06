use std::fs;
use ocr_json_common::TextBox;
use image::{Rgba};
use imageproc::drawing::{
    draw_hollow_rect_mut,
    draw_text_mut,
};
use imageproc::rect::Rect;
use std::env;
use std::path::Path;
use rusttype::{Font, Scale};

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
    let red = Rgba([255u8, 0u8, 0u8, 255u8]);

    let font_data: &[u8] = include_bytes!("../../fonts/DejaVuSansMono.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).expect("Error loading font.");

    let mut img = image::open(image_path).unwrap();

    // run OCR on each word bounding box
    for rect in &ocr_rects {
        draw_hollow_rect_mut(&mut img, Rect::at(rect.x, rect.y).of_size(rect.width, rect.height), red);
        let y: u32 = rect.y as u32;
        let x: u32 = (rect.x-25) as u32;
        let text = String::from(rect.id.clone());
        //let text = "⠨⠙⠕⠃⠗⠕⠙⠕⠱⠇⠊";
        draw_text_mut(&mut img, red, x, y, Scale::uniform(20.0), &font, &text);
    }

    img.save("out.png").unwrap();
}
