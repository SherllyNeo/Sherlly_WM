use regex::Regex;
use std::{time};
use curl::easy::Easy;
pub fn api_call(url: &str) -> Option<String> {
    let mut easy = Easy::new();
    let mut response_text = String::new();
    easy.url(url).unwrap();
          {
           let mut transfer = easy.transfer();
           transfer.write_function(|data| {
          response_text = String::from_utf8(Vec::from(data)).unwrap();

          Ok(data.len())
            }).unwrap();

       transfer.perform().unwrap();
              }
    let _second = time::Duration::from_secs(1);
    let re = Regex::new("\n| | ").unwrap();
    let  response_text = re.replace_all(&response_text,"");

    let regex_col = Regex::new(":").unwrap();

    let response_text = regex_col.replace_all(&response_text,": ").to_string();


   Some(format!("{response_text} |"))
}
