extern crate leptess;

use ocr_json_common::TextBox;
use leptess::{leptonica, tesseract};
use std::env;
use std::path::Path;

/* TODO: preprox here */

fn main() {
    let mut ocr_rects = Vec::new();
    let file_name = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a target file path")
    };
    let image_path = Path::new(&file_name);
    let mut api = tesseract::TessApi::new(Some("/usr/share/tessdata/"), "eng").unwrap();
    let pix = leptonica::pix_read(image_path).unwrap();
    api.set_image(&pix);

    // detect bounding boxes for words
    let boxes = api
        .get_component_images(leptess::capi::TessPageIteratorLevel_RIL_WORD, true)
        .unwrap();

    let mut boxid = 0;
    // run OCR on each word bounding box
    for b in &boxes {
        api.set_rectangle(&b);
        let text = api.get_utf8_text().unwrap();
        let confi = api.mean_text_conf();
        let bref = b.as_ref();
        /*
        println!(
            "[X: {}, Y: {}, W: {}, H: {}]: confidence: {}, text: {}",
            bref.x, bref.y, bref.w, bref.h, confi, text
        );*/
        ocr_rects.push(TextBox {
          id: format!("{}", boxid),
          hint: text,
          confidence: confi as u32,
          x: bref.x,
          y: bref.y,
          height: bref.h as u32,
          width: bref.w as u32,
        });
        boxid += 1;
    }

    let json = serde_json::to_string(&ocr_rects).unwrap();
    println!("{}", json);
}
