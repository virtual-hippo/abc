use std::env;
mod solutions;


fn main() {
    let args: Vec<String> = env::args().collect();
    let problem_id = &args[1];
    match &problem_id[..] {
        // "1" => solutions::sol001(),
        // "2" => solutions::sol002(),
        // "3" => solutions::sol003(),
        "4" => solutions::sol004::solve(),
        // "5" => solutions::sol005(),
        // "6" => solutions::sol006(),
        // "7" => solutions::sol007(),
        // "8" => solutions::sol008(),
        // "9" => solutions::sol009(),
        "10" => solutions::sol010::solve(),
        _ => println!("The sol {} couldn't find.", problem_id)
    }
}