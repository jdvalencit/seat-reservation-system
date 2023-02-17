use crate::server::user::User;
use crate::utils::MenuSelection;
use std::io;
use std::io::Write;
use std::process;

pub fn get_user_info() -> (String, String, i64) {
    print!("Insert Username: ");
    io::stdout().flush().unwrap();
    let username = get_input();
    print!("Insert Mail: ");
    io::stdout().flush().unwrap();
    let mail = get_input();
    print!("Insert Phone Number: ");
    io::stdout().flush().unwrap();
    let phone = get_integer_input();
    println!();

    println!("------------ YOUR ACCOUNT INFO ------------");
    println!("Username: {}", username);
    println!("Mail: {}", mail);
    println!("Phone: {}\n", phone);

    (username, mail, phone)
}

pub fn get_menu_selection(user: &User) -> MenuSelection {
    loop {
        print_menu();

        let selection = get_integer_input();
        let username = user.get_username();

        if let Some(selection) = {
            match selection {
                1 => Some(MenuSelection::DisplayAllSeats),
                2 => Some(MenuSelection::DisplayAvailableSeats),
                3 => Some(MenuSelection::DisplayReservedSeats(username.clone())),
                4 => {
                    println!("Insert num of the seat you'd like to reserve: ");
                    Some(MenuSelection::ReserveSeat(
                        username.clone(),
                        get_integer_input(),
                    ))
                }
                5 => {
                    //Se debe realizar petición para consultar las sillas reservadas por el usuario
                    println!("Insert num of the seat you'd like to cancel: ");
                    Some(MenuSelection::CancelReservation(
                        username.clone(),
                        get_integer_input(),
                    ))
                }
                6 => {
                    println!("Insert num of the seat you'd like to pay: ");
                    Some(MenuSelection::Checkout(
                        username.clone(),
                        get_integer_input(),
                    ))
                }
                7 => process::exit(0),
                _ => {
                    println!("Non-Valid Input");
                    None
                }
            }
        } {
            return selection;
        }
    }
}

pub fn build_request(selection: MenuSelection) -> String {
    match selection {
        MenuSelection::DisplayAllSeats => "DisplayAllSeats\r\n\r\n".to_string(),
        MenuSelection::DisplayAvailableSeats => "DisplayAvailableSeats\r\n\r\n".to_string(),
        MenuSelection::DisplayReservedSeats(username) => {
            format!("DisplayReservedSeats {username}\r\n\r\n")
        }
        MenuSelection::ReserveSeat(username, seat_id) => {
            format!("ReserveSeat {username} {seat_id}\r\n\r\n")
        }
        MenuSelection::CancelReservation(username, seat_id) => {
            format!("CancelReservation {username} {seat_id}\r\n\r\n")
        }
        MenuSelection::Checkout(username, seat_id) => {
            format!("Checkout {username} {seat_id}\r\n\r\n")
        }
    }
}

fn print_menu() {
    println!("------------ SELECT AN OPTION ------------");
    println!("1: Display All Seats");
    println!("2: Display Available Seats");
    println!("3: Display My Reserved Seats");
    println!("4: Reserve Seat");
    println!("5: Cancel Reservation");
    println!("6: Checkout");
    println!("7: Exit");
}

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR");
    input.pop();
    input
}

fn get_integer_input() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR");
    input.pop();

    if let Ok(result) = input.trim().parse::<i64>() {
        return result;
    }

    -1
}
