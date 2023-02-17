use crate::utils::MenuSelection;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::net::TcpStream;

pub fn read_from_stream(mut client_st: &TcpStream) -> Result<MenuSelection, Error> {
    let mut buf_reader = BufReader::new(&mut client_st);
    let mut request = String::new();
    buf_reader.read_line(&mut request)?;
    let splitted_request: Vec<&str> = request.split_whitespace().collect();

    match splitted_request[0] {
        "DisplayAllSeats" => Ok(MenuSelection::DisplayAllSeats),
        "DisplayAvailableSeats" => Ok(MenuSelection::DisplayAvailableSeats),
        "DisplayReservedSeats" => {
            let username = splitted_request[1].to_string();

            Ok(MenuSelection::DisplayReservedSeats(username))
        }
        "ReserveSeat" => {
            let username = splitted_request[1].to_string();
            let seat_id = splitted_request[2].parse::<i64>().unwrap();

            Ok(MenuSelection::ReserveSeat(username, seat_id))
        }
        "CancelReservation" => {
            let username = splitted_request[1].to_string();
            let seat_id = splitted_request[2].parse::<i64>().unwrap();

            Ok(MenuSelection::CancelReservation(username, seat_id))
        }
        "Checkout" => {
            let username = splitted_request[1].to_string();
            let seat_id = splitted_request[2].parse::<i64>().unwrap();

            Ok(MenuSelection::Checkout(username, seat_id))
        }
        _ => Err(Error::new(ErrorKind::Other, "Could not process request.")),
    }
}
