use regex::Regex;
use std::{thread,time};
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
    let second = time::Duration::from_secs(1);
    let re = Regex::new("\n").unwrap();
    let mut response_text = re.replace_all(&response_text,"");
    let regex = Regex::new(concat!(
                     "[",
                             "\u{01F600}-\u{01F64F}", // emoticons
                                "\u{01F300}-\u{01F5FF}", 
                                                      //
                                                      //        symbols & pictographs
                             "\u{01F680}-\u{01F6FF}",
                                       //                 // transport & map symbols
                             "\u{01F1E0}-\u{01F1FF}",
                                                      //                         // flags (iOS)
                             "\u{002702}-\u{0027B0}",
                             "\u{0024C2}-\u{01F251}",
                              "]+",
                                 ))
                              .unwrap();
  
    let response_text = regex.replace_all(&response_text,"").to_string();
    // thread::sleep(second);
    
    
   Some(format!("{response_text} |"))
}
