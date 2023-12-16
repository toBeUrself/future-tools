use anyhow::{Result, bail};
use std::{str::FromStr, fmt::Display};
use serde::{Serialize, Deserialize, Deserializer};

use super::{model::{PositionType}, Services};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
  pub coin: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub wallet_balance: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub available_balance: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub bonus: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub coupon: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub crossed_margin: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub isolated_margin: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub open_order_margin_frozen: f64,
}

#[derive(Debug)]
pub struct PositionList {
  pub position_list: Option<Vec<Position>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
  pub auto_margin: bool,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub available_close_size: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub close_order_size: f64,
  pub contract_type: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub entry_price: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub isolated_margin: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub leverage: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub open_order_margin_frozen: f64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub open_order_size: f64,
  pub position_side: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub position_size: f64,
  pub position_type: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub realized_profit: f64,
  pub symbol: String,
}

#[derive(Debug)]
pub struct BracketList {
  pub bracket_wrapper: Option<Vec<BracketWrapper>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BracketWrapper {
  pub symbol: String,
  pub leverage_brackets: Vec<LeverageBracket>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
  pub bracket: u16,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub maint_margin_rate: f32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub start_margin_rate: f32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub min_leverage: u16,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub max_leverage: u16,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub max_nominal_value: f64,
  pub symbol: String,
}

impl BracketList {
  pub fn get_symbol_bracket(&self, symbol: &str) -> Result<&Vec<LeverageBracket>> {
    if let Some(wrapper) = &self.bracket_wrapper {
      if let Some(result) = wrapper.into_iter().find(|wrap| wrap.symbol == symbol) {
        Ok(&result.leverage_brackets)
      } else {
        bail!("not find")
      }
    } else {
      bail!("not find")
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolList {
  pub time: u64,
  pub version: String,
  pub symbols: Vec<SymbolItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SymbolItem {
  pub id: u16,
  pub symbol: String,
  pub is_display: bool,
  pub base_coin: String,
  pub quote_coin: String,
  pub base_coin_precision: u16,
  pub contract_type: String,
  pub init_leverage: u16,
  pub init_position_type: String,
  pub underlying_type: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub contract_size: f32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub liquidation_fee: f32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub maker_fee: f32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub taker_fee: f32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub market_take_bound: f32,
}

impl SymbolList {
  pub fn get_symbol_config(&self, symbol: &str) -> Result<&SymbolItem> {
    if let Some(item) = self.symbols.as_slice().into_iter().find(|item| item.symbol == symbol) {
      Ok(item)
    } else {
      bail!("not find symbol config")
    }
  }
}

impl PositionList {
  // ∑除本合约全仓外其他全仓仓位未实现盈亏✅
  pub fn get_crossed_float_profit(&self) -> f64 {
    if let Some(list) = &self.position_list {
      let crossed_float_profit = 0.0;
      for p in list.iter() {
        let Position {
          symbol: s,
          position_side: ps,
          position_type: pt,
          entry_price,
          position_size,
          contract_type: ct,
          ..
        } = p;

        match PositionType::from(pt) {
          PositionType::CROSSED => {
            if *position_size > 0.0 {

            }
          },
          _ => (),
        }
      }

      return 0.0;
    } else {
      return 0.0;
    }
  }
}

// dex = 余额 - 所有逐仓占用保证金 - 委托保证金 + ∑除本合约全仓外其他全仓仓位未实现盈亏✅

pub fn get_account_balance(services: &Services, url: &str) -> Result<Balance> {
  let response = services.get::<Balance>(url)?;
    
  if let Some(balance) = response {
    Ok(balance)
  } else {
    bail!("没有请求到账户信息")
  }
}

pub fn get_account_positions(services: &Services, url: &str) -> Result<PositionList> {
  let response = services.get::<Vec<Position>>(url)?;

  Ok(PositionList {
    position_list: response,
  })
}

pub fn get_leverage_brackets(services: &Services, url: &str) -> Result<BracketList> {
  let response = services.get::<Vec<BracketWrapper>>(url)?;

  Ok(BracketList {
    bracket_wrapper: response,
  })
}

pub fn get_symbol_list(services: &Services, url: &str) -> Result<SymbolList> {
  let response = services.get::<SymbolList>(url)?;

  if let Some(symbol_list) = response {
    Ok(symbol_list)
  } else {
    bail!("没有请求到交易对列表")
  }
}



pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: FromStr + Deserialize<'de>,
  <T as FromStr>::Err: Display,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum StringOrInt<T> {
      String(String),
      Number(T),
  }

  match StringOrInt::<T>::deserialize(deserializer)? {
    StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
    StringOrInt::Number(n) => Ok(n),
  }
}