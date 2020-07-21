use std::io;

fn far_to_cel(fah: i32) {
    let result = (fah - 32) * 5/9;
    println!("{} Fahrenheit is {} Celsius",fah, result);
}
fn cel_to_far(cel: i32) {
    let result = (cel * 9/5) + 32;
    println!("{} Celsius is {} Fahrenheit",cel, result);
}


fn main() {
    println!("|---------Temperature Converter---------|");
    println!("          ---------------------          ");
    
    loop {
	    println!("1: Fahrenheit to Celsius");
	    println!("2: Celsius to Fahrenheit");
	    println!("3: Exit");
	    println!("Enter your Choice: ");
	
	    let mut input = String::new();
	
	    io::stdin()
	        .read_line(&mut input)
	        .expect("Failed to read line");
	
	    let input: u32 = match input.trim().parse() {
	        Ok(num) => num,
	        Err(_) => continue,
	    };
	
	    if input == 1 {
	        println!("Enter the Fahrenheit: ");
	        let mut fah = String::new();
	        io::stdin()
	            .read_line(&mut fah)
	            .expect("Failed to read line");
	        let fah: i32 = fah.trim().parse()
	            .expect("Enter a valid Fahrenheit");
	        far_to_cel(fah);
	    } else if input == 2 {
	        println!("Enter the Celsius: ");
	        let mut cel = String::new();
	        io::stdin()
	            .read_line(&mut cel)
	            .expect("Failed to read line");
	        let cel: i32 = cel.trim().parse()
	            .expect("Enter a valid Celsius");
	        cel_to_far(cel);
	    } else {
	        println!("-----------Thank You-----------");
	        break;
    		}
    	}

}
