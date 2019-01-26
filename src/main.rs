use std::io::BufRead;
use regex::Regex;

/// Simple conversion tool to generate a json array with all lines from a CSV (fields separeated by ; )
/// Headers in first Line
/// All other lines as an array
fn main() {
    let re_isnum = Regex::new(r"^(-?\d+)[\.,]?(\d*)$").unwrap(); 
    let heading = std::io::stdin().lock().lines().next().unwrap().unwrap() ;
    let mut headers:Vec<String> = Vec::new();
    for i in heading.split(';'){
        headers.push(i.to_string());
    }

    println!("[");
    let mut newlineouterescape = " ";
    for x in  std::io::stdin().lock().lines() {
        println!("{}{{",newlineouterescape);
        newlineouterescape = ",\r\n ";
        match x{
               Err(_) => {} //last
               Ok(x) => {
                   let mut i =0;
                   let mut h = headers.iter();
                   let mut newlineescape = "";
                   for item in x.split(';') {
                       let title = match h.next(){
                            Some(x) => {x.to_string()},
                            None => { format!("default{}",i)}
                       };
                       let escapeditem = match re_isnum.captures(item){
                           None => {format!("\"{}\"",item)},
                           Some(m) =>  {
                                        let mut s:String = String::new();
                                        s.push_str( m.get(1).map_or("", |m| m.as_str()));
                                        match m.get(2) {
                                            Some(decimals) =>{
                                                let t = decimals.as_str();
                                                if t.len()>0 {
                                                    s.push('.');
                                                    s.push_str(t);
                                                }
                                            }
                                            None => {}
                                        }
                                        s
                            }

                       };
                       print!("{}  \"{}\" : {}",newlineescape,title,escapeditem);
                       i += 1;
                       newlineescape = ",\r\n";
                   }
             }
        }
        print!("\r\n }}");

    }
    println!("\r\n]");

}
// unit tests 
