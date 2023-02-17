use crate::server::connection::response::Status;

pub enum MenuSelection {
    DisplayAllSeats,
    DisplayAvailableSeats,
    DisplayReservedSeats(String),
    ReserveSeat(String, i64),
    CancelReservation(String, i64),
    Checkout(String, i64),
}

#[derive(PartialEq, Debug)]
pub enum ResponseError {
    NothingToDisplay,
    CouldNotReserve,
    CouldNotCancel,
    CouldNotCheckout,
    NotValidID,
    CouldNotReadRequest,
}

impl ResponseError {
    pub fn get_error_message(status: Status) -> String {
        match status {
            Status::ERR(Self::NothingToDisplay) => {
                "ERROR: There are no seats to display.\n".to_string()
            }
            Status::ERR(Self::CouldNotReserve) => "ERROR: Could not reserve seat.\n".to_string(),
            Status::ERR(Self::CouldNotCancel) => {
                "ERROR: Could not cancel reservation.\n".to_string()
            }
            Status::ERR(Self::CouldNotCheckout) => "ERROR: Could not checkout.\n".to_string(),
            Status::ERR(Self::NotValidID) => {
                "ERROR: Specified seat is not a valid id.\n".to_string()
            }
            Status::ERR(Self::CouldNotReadRequest) => {
                "ERROR: Could not read request.\n".to_string()
            }
            _ => "ERROR: Unknown.\n".to_string(),
        }
    }
}
