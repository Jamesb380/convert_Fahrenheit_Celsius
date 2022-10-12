
use std::io;

fn main() {
    println!("Convert Temperature");
    println!("Type Celsius to convert to Celsius and Fahrenheit to convert to Fahrenheit");
   
    let mut opt = String::new();
    io::stdin().read_line( &mut opt).expect("unable to read input");
    let opt = opt.trim();
    let mut t = String::new();

    if opt == "Celsius"{
        println!("Please enter Fahrenheit temperature");
        io::stdin().read_line(&mut t).expect("Failed to readline");
        let t: f32 = match t.trim().parse(){
            Ok(num) => num,
            Err(_) => 9.99,
        };
        let c = cels(t);
        println!("This is in celsius {} degrees", c);
    }
    else if opt =="Fahrenheit"  {

        println!("Please enter Celsius temperature");
        io::stdin().read_line(&mut t).expect("Failed to readline");
        let t: f32 = match t.trim().parse(){
            Ok(num) => num,
            Err(_) => 8.88,
        };

        let f: f32 = fahren(t);
        println!("This is in fahrenheit {} degrees", f);   
    }
    else {
        eprintln!("incorrect option selected")
    }
      

}

fn cels(t: f32) -> f32 {
    const FACTOR: f32 = 0.55555556;
    (t - 32.0) * FACTOR
}

fn fahren(t: f32)-> f32{
    const FACTOR: f32 = 1.8;
    t * FACTOR + 32.0
}