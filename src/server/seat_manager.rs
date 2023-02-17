use crate::config::{MAX_SEATS, PAYMENT_LIMIT_TIME};
use crate::utils::MenuSelection;
use std::io::{Error, ErrorKind};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Debug, Clone)]
pub enum SeatStatus {
    Available,
    OnPayment,
    Reserved,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Seat {
    id: usize,
    status: SeatStatus,
    reserve_username: Option<String>,
    reserve_date: Option<Duration>,
}

impl Seat {
    pub fn default(number: usize) -> Self {
        Seat {
            id: number,
            status: SeatStatus::Available,
            reserve_username: None,
            reserve_date: None,
        }
    }

    pub fn create_seats() -> Vec<Seat> {
        let mut seats = Vec::with_capacity(MAX_SEATS);

        for i in 1..(MAX_SEATS + 1) {
            seats.push(Seat::default(i));
        }

        seats
    }

    // Getters

    pub fn get_reservation_username(&self) -> Option<String> {
        self.reserve_username.clone()
    }

    pub fn get_reservation_date(&self) -> Option<Duration> {
        self.reserve_date
    }

    // Setters

    pub fn set_on_payment(&mut self, username: String) -> Result<(), Error> {
        if matches!(self.status, SeatStatus::OnPayment)
            || matches!(self.status, SeatStatus::Reserved)
        {
            return Err(Error::new(
                ErrorKind::Other,
                "Seat already waiting for payment or reserved.",
            ));
        }

        let date = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => time,
            Err(_) => return Err(Error::new(ErrorKind::Other, "Could not get System Time.")),
        };

        self.reserve_username = Some(username);
        self.reserve_date = Some(date);
        self.status = SeatStatus::OnPayment;

        Ok(())
    }

    pub fn set_canceled(&mut self, username: String) -> Result<(), Error> {
        if let Some(reservation_username) = self.get_reservation_username() {
            if username.eq_ignore_ascii_case(&reservation_username) {
                self.reset_reservation();
                Ok(())
            } else {
                Err(Error::new(
                    ErrorKind::Other,
                    "El usuario no corresponde con la reserva.",
                ))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Seat is not reserved."))
        }
    }

    pub fn set_reserved(&mut self, username: String) -> Result<(), Error> {
        if let Some(reservation_username) = self.get_reservation_username() {
            if username.eq_ignore_ascii_case(&reservation_username) {
                self.status = SeatStatus::Reserved;
                Ok(())
            } else {
                Err(Error::new(
                    ErrorKind::Other,
                    "El usuario no corresponde con la reserva.",
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Seat was not waiting for payment.",
            ))
        }
    }

    fn reset_reservation(&mut self) {
        self.status = SeatStatus::Available;
        self.reserve_username = None;
        self.reserve_date = None;
    }

    // Utilities

    pub fn get_request_seats(selection: &MenuSelection, seats: Vec<Seat>) -> Vec<Seat> {
        match selection {
            MenuSelection::DisplayAllSeats => seats,
            MenuSelection::DisplayAvailableSeats => {
                let mut list = Vec::new();

                for seat in seats {
                    if matches!(seat.status, SeatStatus::Available) {
                        list.push(seat);
                    }
                }

                list
            }
            MenuSelection::DisplayReservedSeats(username) => {
                let mut list = Vec::new();

                for seat in seats {
                    if matches!(seat.status, SeatStatus::Reserved) {
                        if let Some(reserve_username) = seat.get_reservation_username() {
                            if username.eq_ignore_ascii_case(&reserve_username) {
                                list.push(seat);
                            }
                        }
                    }
                }

                list
            }
            _ => seats,
        }
    }

    pub fn clean_expired_ttl_seats(mut seats: Vec<Seat>) -> Vec<Seat> {
        for seat in seats.iter_mut() {
            if matches!(seat.status, SeatStatus::OnPayment) && seat.ttl_check() {
                seat.reset_reservation();
            }
        }

        seats
    }

    pub fn ttl_check(&self) -> bool {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(x) => {
                if let Some(date) = self.get_reservation_date() {
                    x >= date + PAYMENT_LIMIT_TIME
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    pub fn format_seats(seats: &Vec<Seat>) -> String {
        let mut result = String::new();

        for seat in seats {
            let username = match seat.get_reservation_username() {
                Some(reserve_username) => reserve_username,
                None => "N/A".to_string(),
            };

            let status = match seat.status {
                SeatStatus::Available => "Available".to_string(),
                SeatStatus::OnPayment => "Waiting".to_string(),
                SeatStatus::Reserved => "Reserved".to_string(),
            };

            let format = format!("{}\t{:?}\t{:?}\n", seat.id, status, username);

            result.push_str(&format);
        }

        result
    }
}
