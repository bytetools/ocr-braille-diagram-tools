use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextBox {
  pub id: String,
  pub hint: String,
  pub confidence: u32,
  pub width: u32,
  pub height: u32,
  pub x: i32,
  pub y: i32,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
