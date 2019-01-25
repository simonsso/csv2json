use std::io::{self, Read};
use std::io::BufRead;
use regex::Regex;
use std::u8;
fn main() {
    
    let re_txdata = Regex::new(r"SpiTransaction::read\((.*)\)").unwrap(); 

    for x in io::stdin().lock().lines() {
        match x{
               Err(_) => {} //last
               Ok(x) => {
                    if re_txdata.is_match(&x){
                        let y = re_txdata.captures(&x).unwrap().get(1).map_or("", |m| m.as_str());
                        wdata.push(format!("{}",y));
                        // println!("//deleted {} //::{}::",x,y);

                    }
    println!("Hello, world!");
             }
        }
    }
}
