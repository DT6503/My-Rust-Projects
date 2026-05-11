use std::io;

fn main() {
    println!("Hello, this app is for rectangles");

    loop {
        println!("\n--- New Calculation (Type 'exit' to quit) ---");
        println!("Please enter the width:");

        let mut width_input = String::new(); 
        io::stdin()
            .read_line(&mut width_input)
            .expect("I can't read the width!");

  
        if width_input.trim() == "exit" { break; }

        let width: f64 = match width_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Please enter a valid number.");
                continue; 
            }
        };


        let length = loop {
            println!("Please enter the length:");
            let mut l_input = String::new();
            
            io::stdin()
                .read_line(&mut l_input)
                .expect("I can't read the length!");

            match l_input.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(_) => println!("Invalid input. Enter one more time!"),
            }
        };

        let area = width * length;
        let perimeter = 2.0 * (width + length);

        println!("\nРезультаты для прямоугольника {} x {}:", width, length);
        println!("Площадь: {:.2}", area);
        println!("Периметр: {:.2}", perimeter);
    }
}