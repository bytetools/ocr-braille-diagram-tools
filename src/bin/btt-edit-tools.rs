use ocr_json_common::TextBox;
use serde_json;
use std::{
  env,
  cmp::min,
  process::Command,
  io::Write,
  fs::OpenOptions,
  fs,
};
use text_io::read;

// TODO: make more extensible
fn save_json(json: String, fname: &String) {
  let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(fname).expect("Unable to open file");
  file.write_all(json.as_bytes()).expect("unable to write to file");
}

// TODO: make more exensible!!!
fn reload_json(fname: &String) {
  Command::new("cargo")
    .arg("run")
    .arg("--bin")
    .arg("btt-label-ocr")
    .arg("./diagram.png")
    .arg(fname)
    .output()
    .expect("Failed to execute command");
}

fn new_id(ids: &Vec<String>) -> String {
  let mut new_id = 0;
  let mut new_sid = format!("{}", new_id);
  while ids.contains(&new_sid) {
    new_sid = format!("{}", new_id);
    new_id+=1;
  }
  new_sid
}

fn rem_rect(boxes: &mut Vec<TextBox>, id: String) {
  boxes.retain(|b| b.id != id);
}

fn new_rect(boxes: &mut Vec<TextBox>, id: String, x: String, y: String, w: String, h: String) {
  // TODO: unsafe
  boxes.push(TextBox {
    id,
    x: x.parse().unwrap(),
    y: y.parse().unwrap(),
    width: w.parse().unwrap(),
    height: h.parse().unwrap(),
    hint: String::new(),
    confidence: 0
  });
}

fn set_text(boxes: &mut Vec<TextBox>, xid: String, new_text: String) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == xid).unwrap().clone();
  boxes.retain(|b| b.id != xid);
  boxes.push(TextBox{
    id: bx.id,
    confidence: 0,
    hint: new_text.clone(),
    x: bx.x,
    y: bx.y,
    width: bx.width,
    height: bx.height,
  });
}

fn merge(boxes: &mut Vec<TextBox>, xid: String, yid: String) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == xid).unwrap();
  // TODO: unsafe
  let by = boxes.iter().find(|b| b.id == yid).unwrap();
  let y = min(bx.y, by.y);
  let x = min(bx.x, by.x);
  let w = (bx.x - by.x).abs() + (if x == bx.x {by.width as i32} else {bx.width as i32});
  let h = (bx.y - by.y).abs() + (if y == by.y {by.height as i32} else {bx.height as i32});
  let text = format!("{} {}", bx.hint, by.hint);
  let confi = 0;
  let id = bx.id.clone();
  boxes.retain(|b| b.id != xid && b.id != yid);
  boxes.push(TextBox {
    id,
    confidence: confi,
    hint: text,
    x,
    y,
    width: w as u32,
    height: h as u32,
  });
}

fn vsplit(boxes: &mut Vec<TextBox>, sid: String) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap();

  let w = bx.width;
  let x = bx.x;
  let h = bx.height/2;
  let y1 = bx.y;
  let y2 = bx.y + (bx.height as i32)/2;
  let hint = bx.hint.clone();
  let mut tsplit = hint.split("\n"); // tesseract likes to use newlines for some reason... use to our advantage
  let t1 = tsplit.next().unwrap_or("");
  let t2 = tsplit.next().unwrap_or("");
  let id1 = bx.id.clone();
  let ids: Vec<String> = boxes.iter().map(|b| b.id.clone()).collect();
  let id2 = new_id(&ids);
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox {
    x,
    y: y1,
    hint: t1.to_string().clone(),
    confidence: 0,
    id: id1,
    width: w,
    height: h,
  });
  boxes.push(TextBox {
    x,
    y: y2,
    hint: t2.to_string().clone(),
    confidence: 0,
    id: id2,
    width: w,
    height: h,
  });
}

fn hsplit(boxes: &mut Vec<TextBox>, sid: String) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap();

  let w = bx.width/2;
  let y = bx.y;
  let h = bx.height;
  let x1 = bx.x;
  let x2 = bx.x + (bx.width as i32)/2;
  let hint = bx.hint.clone();
  let mut tsplit = hint.split("\n"); // tesseract likes to use newlines for some reason... use to our advantage
  let t1 = tsplit.next().unwrap_or("");
  let t2 = tsplit.next().unwrap_or("");
  let id1 = bx.id.clone();
  let ids: Vec<String> = boxes.iter().map(|b| b.id.clone()).collect();
  let id2 = new_id(&ids);
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox {
    x: x1,
    y,
    hint: t1.to_string().clone(),
    confidence: 0,
    id: id1,
    width: w,
    height: h,
  });
  boxes.push(TextBox {
    x: x2,
    y,
    hint: t2.to_string().clone(),
    confidence: 0,
    id: id2,
    width: w,
    height: h,
  });
}

fn triml(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x + mw,
    width: bx.width - (mw as u32),
    y: bx.y,
    height: bx.height,
    confidence: 0,
  });
}
fn trimr(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x,
    width: bx.width - (mw as u32),
    y: bx.y,
    height: bx.height,
    confidence: 0,
  });
}
fn trimt(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x,
    width: bx.width,
    y: bx.y + mw,
    height: bx.height - (mw as u32),
    confidence: 0,
  });
}
fn trimb(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x,
    width: bx.width,
    y: bx.y,
    height: bx.height - (mw as u32),
    confidence: 0,
  });
}

fn movel(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x - mw,
    width: bx.width,
    y: bx.y,
    height: bx.height,
    confidence: 0,
  });
}
fn mover(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x + mw,
    width: bx.width,
    y: bx.y,
    height: bx.height,
    confidence: 0,
  });
}
fn moveu(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x,
    width: bx.width,
    y: bx.y - mw,
    height: bx.height,
    confidence: 0,
  });
}
fn moved(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x,
    width: bx.width,
    y: bx.y + mw,
    height: bx.height,
    confidence: 0,
  });
}

fn paddl(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x - mw,
    width: bx.width + (mw as u32),
    y: bx.y,
    height: bx.height,
    confidence: 0,
  });
}
fn paddr(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x,
    width: bx.width + (mw as u32),
    y: bx.y,
    height: bx.height,
    confidence: 0,
  });
}
fn paddt(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x,
    width: bx.width,
    y: bx.y - mw,
    height: bx.height + (mw as u32),
    confidence: 0,
  });
}
fn paddb(boxes: &mut Vec<TextBox>, sid: String, mw: i32) {
  // TODO: unsafe
  let bx = boxes.iter().find(|b| b.id == sid).unwrap().clone();
  boxes.retain(|b| b.id != sid);
  boxes.push(TextBox{
    id: bx.id,
    hint: bx.hint,
    x: bx.x,
    width: bx.width,
    y: bx.y,
    height: bx.height + (mw as u32),
    confidence: 0,
  });
}

fn main() {
    let json_fname = if env::args().count() == 2 {
      env::args().nth(1).unwrap()
    } else {
      panic!("Please enter a file path");
    };
    let json_str = fs::read_to_string(json_fname.clone()).expect("There was an error reading the provided file.");
    let mut boxes: Vec<TextBox> = serde_json::from_str(&json_str).unwrap();
    let mut line: String = String::new();
    println!("Type 'exit' to quit!");
    while line != "exit" {
      line = read!("{}\n");
      let mut split = line.split("|");
      let command = split.next();
      if command == Some("merge") {
        // TODO: not safe
        let one_id = split.next().unwrap();
        let two_id = split.next().unwrap();
        merge(&mut boxes, one_id.to_string(), two_id.to_string());
      } else if command == Some("vsplit") {
        // TODO: not safe
        let id = split.next().unwrap();
        vsplit(&mut boxes, id.to_string());
      } else if command == Some("hsplit") {
        // TODO: not safe
        let id = split.next().unwrap();
        hsplit(&mut boxes, id.to_string());
      } else if command == Some("triml") {
        // TODO: not safe
        let id = split.next().unwrap();
        let px = split.next().unwrap();
        triml(&mut boxes, id.to_string(), px.parse().unwrap());
      } else if command == Some("trimr") {
        // TODO: not safe
        let id = split.next().unwrap();
        let px = split.next().unwrap();
        trimr(&mut boxes, id.to_string(), px.parse().unwrap());
      } else if command == Some("trimt") {
        // TODO: not safe
        let id = split.next().unwrap();
        let px = split.next().unwrap();
        trimt(&mut boxes, id.to_string(), px.parse().unwrap());
      } else if command == Some("trimb") {
        // TODO: not safe
        let id = split.next().unwrap();
        let px = split.next().unwrap();
        trimb(&mut boxes, id.to_string(), px.parse().unwrap());
      } else if command == Some("text") {
        // TODO: not safe
        let id = split.next().unwrap();
        let new_text = split.next().unwrap();
        set_text(&mut boxes, id.to_string(), new_text.to_string());
      } else if command == Some("moveu") {
        // TODO: not safe
        let id = split.next().unwrap().to_string();
        let diff = split.next().unwrap().parse().unwrap();
        moveu(&mut boxes, id, diff);
      } else if command == Some("moved") {
        // TODO: not safe
        let id = split.next().unwrap().to_string();
        let diff = split.next().unwrap().parse().unwrap();
        moved(&mut boxes, id, diff);
      } else if command == Some("mover") {
        // TODO: not safe
        let id = split.next().unwrap().to_string();
        let diff = split.next().unwrap().parse().unwrap();
        mover(&mut boxes, id, diff);
      } else if command == Some("movel") {
        // TODO: not safe
        let id = split.next().unwrap().to_string();
        let diff = split.next().unwrap().parse().unwrap();
        movel(&mut boxes, id, diff);
      } else if command == Some("paddl") {
        // TODO: not safe
        let id = split.next().unwrap().to_string();
        let diff = split.next().unwrap().parse().unwrap();
        paddl(&mut boxes, id, diff);
      } else if command == Some("paddr") {
        // TODO: not safe
        let id = split.next().unwrap().to_string();
        let diff = split.next().unwrap().parse().unwrap();
        paddr(&mut boxes, id, diff);
      } else if command == Some("paddt") {
        // TODO: not safe
        let id = split.next().unwrap().to_string();
        let diff = split.next().unwrap().parse().unwrap();
        paddt(&mut boxes, id, diff);
      } else if command == Some("paddb") {
        // TODO: not safe
        let id = split.next().unwrap().to_string();
        let diff = split.next().unwrap().parse().unwrap();
        paddb(&mut boxes, id, diff);
      } else if command == Some("add") {
        // TODO: not safe
        let ids: Vec<String> = boxes.iter().map(|b| b.id.clone()).collect();
        let id = new_id(&ids);
        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let w = split.next().unwrap();
        let h = split.next().unwrap();
        new_rect(&mut boxes,
        id.to_string(),
        x.to_string(),
        y.to_string(),
        w.to_string(),
        h.to_string());
      } else if command == Some("save") {
        // TODO: unsafe
        let fname = split.next().unwrap();
        let json_out = serde_json::to_string(&boxes).unwrap();
        save_json(json_out, &fname.to_string());
        println!("Saved as {}", fname);
      } else if command == Some("show") {
        // TODO: VERY unsafe
        let id = split.next().unwrap();
        let bx = boxes.iter().find(|b| b.id == id).unwrap();
        let json = serde_json::to_string(bx).unwrap();
        println!("JSON: {}", json);
      } else if command == Some("rem") {
        // TODO: unsafe
        let id = split.next().unwrap();
        rem_rect(&mut boxes, id.to_string());
      } else if command == Some("exit") {
        continue;
      } else {
        println!("Invalid command.");
      }
      let json_out = serde_json::to_string(&boxes).unwrap();
      save_json(json_out, &json_fname);
      reload_json(&json_fname);
    }
}
