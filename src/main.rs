mod win;

fn main() {
    let info = win::get_information();

    println!("HELLOOOOO");
    println!("The info of the current system is listed below:");
    dbg!(info);
}
