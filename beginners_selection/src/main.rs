use std::env;
mod sol;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem_id = &args[1];
    match &problem_id[..] {
        "1" => sol::call_solve1(),
        "2" => sol::call_solve2(),
        "3" => sol::call_solve3(),
        "4" => sol::call_solve4(),
        "5" => sol::call_solve5(),
        "6" => sol::call_solve6(),
        "7" => sol::call_solve7(),
        "8" => sol::call_solve8(),
        "9" => sol::call_solve9(),
        "10" => sol::call_solve10(),
        _ => println!("The sol {} couldn't find.", problem_id)
    }
}
