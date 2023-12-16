use anyhow::{Result, Ok};
use reqwest::{self, header::{self, HeaderValue}};

use self::model::ApiResult;

pub mod model;
pub mod account;

pub const BASE_URL: &'static str = "http://www.xt-qa.com";

pub enum SymbolBase {
  UBase(String),
  CBase(String),
}

pub struct Services {
  pub symbol_base: SymbolBase,
  pub client: reqwest::blocking::Client,
}

impl Services {
  fn get_whole_api_url(&self, path: &str) -> String {
    let base_str: &str;
    match &self.symbol_base {
      SymbolBase::UBase(ubase) => base_str = ubase,
      SymbolBase::CBase(cbase) => base_str = cbase,
    }

    format!("{}{}{}", BASE_URL, base_str, path)
  }
}

impl Services{
  pub fn get<T>(&self, url: &str) -> Result<Option<T>> 
  where
    T: for<'de> serde::Deserialize<'de> 
  {
    let whole_url = self.get_whole_api_url(url);

    let response = self.client.get(whole_url).send()?;
    let response_text = response.text()?;
    let response_json: ApiResult<T> = serde_json::from_str(&response_text.to_owned())?;

    Ok(response_json.result)
  }
}

impl Services {
  pub fn builder(token: &str, symbol_base: SymbolBase) -> Self {
    let mut headers = header::HeaderMap::new();
  
    let auth = format!("Bearer {}", token);
    let mut auth_value = header::HeaderValue::from_str(&auth).unwrap();
    auth_value.set_sensitive(true);
  
    headers.insert(header::AUTHORIZATION, auth_value);
    headers.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"));
    headers.insert(header::COOKIE, HeaderValue::from_static("OUTFOX_SEARCH_USER_ID_NCOO=1302301671.6555967; _scid=771de793-9ace-4411-a1d7-7f88dcde4ab0; sensorsdata2015jssdkcross=%7B%22distinct_id%22%3A%223607980540571%22%2C%22first_id%22%3A%22187fa4d2eb2545-046666666666668-1c525634-1296000-187fa4d2eb398d%22%2C%22props%22%3A%7B%22%24latest_traffic_source_type%22%3A%22%E7%9B%B4%E6%8E%A5%E6%B5%81%E9%87%8F%22%2C%22%24latest_search_keyword%22%3A%22%E6%9C%AA%E5%8F%96%E5%88%B0%E5%80%BC_%E7%9B%B4%E6%8E%A5%E6%89%93%E5%BC%80%22%2C%22%24latest_referrer%22%3A%22%22%7D%2C%22identities%22%3A%22eyIkaWRlbnRpdHlfbG9naW5faWQiOiIzNjA3OTgwNTQwNTcxIiwiJGlkZW50aXR5X2Nvb2tpZV9pZCI6IjE4N2ZhNGQyZWIyNTQ1LTA0NjY2NjY2NjY2NjY2OC0xYzUyNTYzNC0xMjk2MDAwLTE4N2ZhNGQyZWIzOThkIn0%3D%22%2C%22history_login_id%22%3A%7B%22name%22%3A%22%24identity_login_id%22%2C%22value%22%3A%223607980540571%22%7D%2C%22%24device_id%22%3A%22187fa4d2eb2545-046666666666668-1c525634-1296000-187fa4d2eb398d%22%7D; currency=usd; theme=light; countryId=1; clientCode=1702451634939lk2rjj62eqc6tfKVOop; _ga=GA1.1.1634851696.1683357236; _scid_r=771de793-9ace-4411-a1d7-7f88dcde4ab0; token=eyJhbGciOiJSUzI1NiJ9.eyJhY2NvdW50SWQiOiIxMzUzNTUxOTcwMzA0Iiwic3ViIjoiMTIzNDU2NzhAZ21haWwuY29tIiwic2NvcGUiOiJhdXRoIiwibGFzdEF1dGhUaW1lIjoxNzAyNTM4MzQ1MTkyLCJzaWduVHlwZSI6IlVQIiwiYWNjb3VudExldmVsIjoiMSIsInVzZXJOYW1lIjoiMTIzNDU2NzhAZ21haWwuY29tIiwiZXhwIjoxNzA1MTMwMzQ1LCJkZXZpY2UiOiJ3ZWIiLCJ1c2VySWQiOjEzNTM1NTE5NzAzMDQsInVzZXJDb2RlIjoiODgyZDE5OGEzMTg4NjI4YjljMGI1OGI4ZmJkYzk0NzgifQ.hImpO5rWwvQuSLRecDSr8PywFlA70OPOHXdC0MxEkeuM8ZZJDmYJQnq2tkAdcbU0asieYUhyIBbSYoZP6nwXNx_6yMXtpc9l2K7wRWgc7uzzRtbmrEE9tgaY2numwHdcgAqQ5bNKqDs_nYqWXKJrQyyihb1NFYit_iedyB28R3I; lang=cn; _ga_CY0DPVC3GS=GS1.1.1702616817.187.0.1702616817.0.0.0; _ga_MK8XKWK7DV=GS1.1.1702616817.186.0.1702616817.0.0.0; JSESSIONID=17F82C0129F4CCD005538C04C59D6C84"));

    println!("{:?}", headers);

    let client = reqwest::blocking::Client::builder()
                .default_headers(headers)
                .build().unwrap();
    
    Services {
      symbol_base,
      client,
    }
  }
}

