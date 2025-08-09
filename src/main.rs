mod drawing;
mod preprocessing;
mod recognition;

use drawing::App;

fn main() {
    println!("--- Number Recognizer ---");
    println!("Draw a number (0-9) with your mouse.");
    println!("Press 'C' to clear the canvas.");
    println!("Press 'Enter' to recognize the number.");
    println!("-------------------------");

    let mut app = App::new();
    
    app.run(&recognition::recognize_digit);
}