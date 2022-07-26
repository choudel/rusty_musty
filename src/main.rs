fn main() {
    {
        let my_string = String::from("Rust is fantastic!");
        println!(
            "the string is {}",
            my_string.replace("fantastic!", "Awsome")
        );
    }
    {
        let my_string = String::from("The weather is\nnice\noutside mate!");
        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }
    {
        let my_string= String::from("Leave+a+like+if+you+enjoyed!");
        let tokens: Vec<&str>=my_string.split("+").collect();
        println!("{}",my_string);
        println!("At index 2: {}",tokens[2]);
    }
    {
        let my_string= String::from("  My name is choudel skk   ");
        println!("after trim: {}",my_string.trim());
    }
    {
        let my_string = String::from("dcode on tube");

        match my_string.chars().nth(4){
            Some(c)=>println!("Character at index 4: {}",c),
            None =>println!("no char in index 4")
        }
    }
}
