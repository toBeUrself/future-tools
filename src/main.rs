use anyhow::{Result, Ok};

use crate::services::{Services, SymbolBase, account::{get_account_balance, get_account_positions, get_leverage_brackets, get_symbol_list}};

pub mod services;
pub mod deserialize_number_from_string;

const GET_BRACKET_LIST: &'static str = "/user/v1/leverage/bracket/list";
const GET_BALANCE_DETAIL: &'static str = "/user/v1/balance/detail?coin=usdt&underlyingType=U_BASED";
const GET_SYMBOL_LIST: &'static str = "/market/v2/public/symbol/list?isPredict=true&isDelivery=true&version=";
const GET_POSITION_LIST: &'static str = "/user/v1/position/list?isPredict=true&isDelivery=true&welfareAccountVersion=true";
const TOKEN: &'static str = "eyJhbGciOiJSUzI1NiJ9.eyJhY2NvdW50SWQiOiIxMzUzNTUxOTcwMzA0Iiwic3ViIjoiMTIzNDU2NzhAZ21haWwuY29tIiwic2NvcGUiOiJhdXRoIiwibGFzdEF1dGhUaW1lIjoxNzAyNTM4MzQ1MTkyLCJzaWduVHlwZSI6IlVQIiwiYWNjb3VudExldmVsIjoiMSIsInVzZXJOYW1lIjoiMTIzNDU2NzhAZ21haWwuY29tIiwiZXhwIjoxNzA1MTMwMzQ1LCJkZXZpY2UiOiJ3ZWIiLCJ1c2VySWQiOjEzNTM1NTE5NzAzMDQsInVzZXJDb2RlIjoiODgyZDE5OGEzMTg4NjI4YjljMGI1OGI4ZmJkYzk0NzgifQ.hImpO5rWwvQuSLRecDSr8PywFlA70OPOHXdC0MxEkeuM8ZZJDmYJQnq2tkAdcbU0asieYUhyIBbSYoZP6nwXNx_6yMXtpc9l2K7wRWgc7uzzRtbmrEE9tgaY2numwHdcgAqQ5bNKqDs_nYqWXKJrQyyihb1NFYit_iedyB28R3I";

fn main() -> Result<()> {
    println!("Hello, world!");
    
    let ubase = "/fapi";
    let cbase = "/dapi";
    let current_symbol = "btc_usdt";
    
    let services = Services::builder(TOKEN, SymbolBase::UBase(ubase.to_string()));

    let balance = get_account_balance(&services, GET_BALANCE_DETAIL)?;
    let positions = get_account_positions(&services, GET_POSITION_LIST)?;
    let leverage_brackets = get_leverage_brackets(&services, GET_BRACKET_LIST)?;
    let symbol_list = get_symbol_list(&services, GET_SYMBOL_LIST)?;

    // println!("{:#?} {:#?} {:#?} {:#?}", balance, positions, leverage_brackets, symbol_list);

    let current_symbol_bracket = leverage_brackets.get_symbol_bracket(current_symbol);
    let current_symbol_config = symbol_list.get_symbol_config(current_symbol);

    println!("current symbol: {:?}", current_symbol);
    println!("current_symbol_config: {:#?}", current_symbol_config);
    println!("current_symbol_bracket: {:#?}", current_symbol_bracket);


    Ok(())
}
