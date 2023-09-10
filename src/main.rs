use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let args_lv1 = &args[1];
    match args_lv1.as_str() {
        "init" => {
            println!("Level 1 is hi");
           let str = "hi , he";
           parse_to_json(&str)
            
        }

        _ => {
            println!("Unknown argument");
        }
    }
   
}


fn parse_to_json(json_text : &str){
    let parsed_curl: Vec<&str> = json_text.split(',').collect();
    println!("{}", parsed_curl[1]);
}