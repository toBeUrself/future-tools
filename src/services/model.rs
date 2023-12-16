use std::str::FromStr;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResult<T> {
  pub error: Option<String>,
  pub msg_info: String,
  pub return_code: u8,
  pub result: Option<T>,
}

#[derive(Debug, PartialEq)]
pub enum PositionType {
  CROSSED,
  ISOLATED,
  NOWAY,
}

impl From<&String> for PositionType {

  fn from(input: &String) -> Self {
    match input as &str {
      "CROSSED"  => PositionType::CROSSED,
      "ISOLATED" => PositionType::ISOLATED,
      _          => PositionType::NOWAY,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum PositionSide {
  LONG,
  SHORT,
}

impl FromStr for PositionSide {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "LONG"  => Ok(PositionSide::LONG),
      "SHORT" => Ok(PositionSide::SHORT),
      _       => Err(()),
    }
  }
}