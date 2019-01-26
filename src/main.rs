use std::io::BufRead;

// check for negative positive numers
// max one decimal comma
fn isNum(s:&str) -> bool{
    // let s=
    // if s[0] == '-' {
    //     s[1..]
    // } else {
    //     s[0..]
    // }
    let mut first = true;
    let mut decimal = 0;
    for c in s.chars() {
        decimal += if (c == '.' ) || ( c == ',') {1} else {0};
        if ! ( (first && c =='-') || c.is_digit(10) || (decimal <=1 )){
            return false
        }
        first = false;
    }
    true
}


/// Simple conversion tool to generate a json array with all lines from a CSV (fields separeated by ; )
/// Headers in first Line
/// All other lines as an array
fn main() {
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
                       let esc = if isNum(item){
                           ""
                        }else{
                            "\""
                        };
                       print!("{}  \"{}\":{}{}{}",newlineescape,title,esc,item,esc);
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
