pub fn new(
    stage: &Vec<Vec<bool>>
) {
    print!("\x1B[2J");
    for parent in stage {
        let mut text = String::new();
        for child in parent {
            text = format!("{}{}", text, if child == &true { "#" } else { "." });
        }
        println!("{}", text);
    }
}

