use std::fs;
use ocr_json_common::TextBox;
use image::{Rgba};
use imageproc::drawing::{
    draw_text_mut,
};
use std::env;
use std::path::Path;
use rusttype::{Font, Scale};
use louis::Louis;
use louis::modes::DOTS_UNICODE;

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
    let font_size_str = if env::args().count() >= 4 {
      env::args().nth(3).unwrap()
    } else {
      "20.0".to_string()
    };
    let font_size = font_size_str.parse().unwrap();
    let json = fs::read_to_string(json_file_name).expect("There was an error reading the file.");
    let ocr_rects: Vec<TextBox> = serde_json::from_str(&json).unwrap();
    let image_path = Path::new(&img_file_name);
    let black = Rgba([0u8, 0u8, 0u8, 255u8]);

    let font_data: &[u8] = include_bytes!("../../fonts/UBraille.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).expect("Error loading font.");

    let mut img = image::open(image_path).unwrap();

    let brl = Louis::new().unwrap();

    // run OCR on each word bounding box
    for rect in &ocr_rects {
        let text = rect.hint.clone();
        let brl_text = brl.translate_simple("en_US.tbl", &text, false, DOTS_UNICODE);
        println!("[{}]: {}", rect.id, brl_text);
        draw_text_mut(&mut img, black, rect.x.try_into().unwrap(), rect.y.try_into().unwrap(), Scale::uniform(font_size), &font, &brl_text);
    }

    img.save("out.png").unwrap();
}
