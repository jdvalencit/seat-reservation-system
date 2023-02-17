use crate::server::seat_manager::Seat;
use crate::utils::{MenuSelection, ResponseError};
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub enum Status {
    OK,
    ERR(ResponseError),
}

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub body: Option<Vec<Seat>>,
}

impl Response {
    pub fn get_response(request: &MenuSelection, seats: Vec<Seat>) -> (Self, Vec<Seat>) {
        match request {
            MenuSelection::DisplayAllSeats => Self::create_display_response(request, seats),
            MenuSelection::DisplayAvailableSeats => Self::create_display_response(request, seats),
            MenuSelection::DisplayReservedSeats(_username) => {
                Self::create_display_response(request, seats)
            }
            MenuSelection::ReserveSeat(username, seat_id) => {
                Self::create_reserve_response(seats, username.clone(), *seat_id)
            }
            MenuSelection::CancelReservation(username, seat_id) => {
                Self::create_cancel_response(seats, username.clone(), *seat_id)
            }
            MenuSelection::Checkout(username, seat_id) => {
                Self::create_checkout_response(seats, username.clone(), *seat_id)
            }
        }
    }

    fn create_display_response(request: &MenuSelection, seats: Vec<Seat>) -> (Self, Vec<Seat>) {
        let list = Seat::get_request_seats(request, seats.clone());

        if !list.is_empty() {
            let response = Response {
                status: Status::OK,
                body: Some(list.clone()),
            };

            (response, list)
        } else {
            let response = Response {
                status: Status::ERR(ResponseError::NothingToDisplay),
                body: None,
            };

            (response, seats)
        }
    }

    fn create_reserve_response(
        mut seats: Vec<Seat>,
        username: String,
        seat_id: i64,
    ) -> (Self, Vec<Seat>) {
        if (seat_id as usize) < seats.len() {
            let seat = &mut seats[(seat_id - 1) as usize];

            if seat.set_on_payment(username).is_ok() {
                let response = Response {
                    status: Status::OK,
                    body: None,
                };

                (response, seats)
            } else {
                let response = Response {
                    status: Status::ERR(ResponseError::CouldNotCheckout),
                    body: None,
                };

                (response, seats)
            }
        } else {
            let response = Response {
                status: Status::ERR(ResponseError::NotValidID),
                body: None,
            };

            (response, seats)
        }
    }

    fn create_cancel_response(
        mut seats: Vec<Seat>,
        username: String,
        seat_id: i64,
    ) -> (Self, Vec<Seat>) {
        if (seat_id as usize) < seats.len() {
            let seat = &mut seats[(seat_id - 1) as usize];

            if seat.set_canceled(username).is_ok() {
                let response = Response {
                    status: Status::OK,
                    body: None,
                };

                (response, seats)
            } else {
                let response = Response {
                    status: Status::ERR(ResponseError::CouldNotCancel),
                    body: None,
                };

                (response, seats)
            }
        } else {
            let response = Response {
                status: Status::ERR(ResponseError::NotValidID),
                body: None,
            };

            (response, seats)
        }
    }

    fn create_checkout_response(
        mut seats: Vec<Seat>,
        username: String,
        seat_id: i64,
    ) -> (Self, Vec<Seat>) {
        if (seat_id as usize) < seats.len() {
            let seat = &mut seats[(seat_id - 1) as usize];

            if seat.set_reserved(username).is_ok() {
                let response = Response {
                    status: Status::OK,
                    body: None,
                };

                (response, seats)
            } else {
                let response = Response {
                    status: Status::ERR(ResponseError::CouldNotReserve),
                    body: None,
                };

                (response, seats)
            }
        } else {
            let response = Response {
                status: Status::ERR(ResponseError::NotValidID),
                body: None,
            };

            (response, seats)
        }
    }

    pub fn write(self, client_st: &mut TcpStream) {
        let mut buf_writer = BufWriter::new(client_st);
        let mut data = String::new();
        if let Some(body) = self.body {
            data = Seat::format_seats(&body);
        }

        let response = format!("{:?}\r\n{data}\r\n\r\n", self.status);

        if buf_writer.write(response.as_bytes()).is_err() {
            println!("Error does not response");
        }

        if buf_writer.flush().is_err() {
            println!("Failed to flush stream responser");
        }
    }

    pub fn write_error(self, client_st: &mut TcpStream) {
        let mut buf_writer = BufWriter::new(client_st);
        let response = format!("{}\r\n\r\n", ResponseError::get_error_message(self.status));

        if buf_writer.write(response.as_bytes()).is_err() {
            println!("Error does not send");
        }

        if buf_writer.flush().is_err() {
            println!("Failed to flush stream responser");
        }
    }
}
