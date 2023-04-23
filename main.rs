use std::io;

fn main() {
    let mut input = String::new();

    loop {
        println!("Choose a Language (tr-en): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if input == "en" || input == "tr" {
            break;
        } else {
            println!("Please choose an available language.");
        }
    }

    println!("You chose: {}", input);

    loop {
        let input3 = &input;
        let mut input2 = String::new();
        print!(": ");
        io::stdin()
            .read_line(&mut input2)
            .expect("Please write it again.");
        input2 = input2.trim().to_string();
        if input3 == "tr" {
            println!("İstemez, teşekkürler.");
        } else if input3 == "en" {
            println!("No, thanks.");
        }
    }
}
