use std::io::Read;

fn main(){
    println!("Welcome to globa.com
    To enable us serve you better, Kindly tell us who you are
    Enter 1. if admin
    Enter 2. if project manager
    Enter 3. if Employee
    Enter 4. if Customer
    Enter 5. if Vendor
    Enter 6. To quit");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice:u32= input.trim().parse().expect("Failed to read line");

    if choice == 1 {
        admin();
    } else if choice == 2 {
        project_manager();
    } else if choice == 3 {
        employee();
    } else if choice == 4 {
        customer();
    } else if choice == 5 {
        vendor();
    } else {
        println!("You offer cannot be proceeded");
    }
    
}
fn admin() {
    let mut file = std::fs::File::open("globacom_db.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn project_manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn customer() {
    let mut file = std::fs::File::open("customerinfo_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}