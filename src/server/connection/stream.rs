use crate::server::connection::request::read_from_stream;
use crate::server::connection::response::{Response, Status};
use crate::server::seat_manager::Seat;
use crate::server::utils::multithread::ThreadPool;
use crate::utils::{MenuSelection, ResponseError};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;

pub fn tcp_connect(client_st: &mut TcpStream, seats: Vec<Seat>) -> Vec<Seat> {
    if let Ok(request) = read_from_stream(client_st) {
        let (response, list) = Response::get_response(&request, seats.clone());

        if matches!(response.status, Status::OK) {
            response.write(client_st);
        } else {
            response.write_error(client_st);
        }

        if matches!(request, MenuSelection::CancelReservation(_, _))
            || matches!(request, MenuSelection::ReserveSeat(_, _))
            || matches!(request, MenuSelection::Checkout(_, _))
        {
            return list;
        }
    } else {
        let response = Response {
            status: Status::ERR(ResponseError::CouldNotReadRequest),
            body: None,
        };

        response.write_error(client_st);
    }

    seats
}

pub fn handle_connection(listener: TcpListener, pool: ThreadPool, mut seats: Vec<Seat>) {
    for stream in listener.incoming() {
        let seats_clone = seats.clone();
        let (tx, rx) = mpsc::channel::<Vec<Seat>>();

        match stream {
            Ok(mut stream) => {
                pool.execute(move || {
                    let seats_cleaned = Seat::clean_expired_ttl_seats(seats_clone);
                    let current_seats = tcp_connect(&mut stream, seats_cleaned);
                    tx.send(current_seats).expect("Send data");
                });
            }

            Err(_) => println!("Stream does not capture."),
        }

        let current_seats = rx.recv().unwrap();
        seats = current_seats;
    }
}
