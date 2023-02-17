pub struct User {
    username: String,
    mail: Option<String>,
    phone: Option<i64>,
    reserved_seats: Vec<i32>,
}

impl User {
    pub fn default(username: String, mail: Option<String>, phone: Option<i64>) -> Self {
        let reserved_seats = vec![];

        User {
            username,
            mail,
            phone,
            reserved_seats,
        }
    }

    pub fn display_full_info(&self) {
        println!("------------ BEGIN INFO ------------");
        println!("Username: {}", self.username);
        println!("Mail: {:?}", self.mail);
        println!("Phone: {:?}", self.phone);
        println!("Reserved Seats: {:?}", self.reserved_seats);
        println!("------------- END INFO -------------");
    }

    pub fn display_seats_info(&self) {
        println!("------------ BEGIN INFO ------------");
        println!(
            "Currently, you have {} seats reserved",
            self.reserved_seats.len()
        );
        if !self.reserved_seats.is_empty() {
            println!("Those are: ");
            for seat in self.reserved_seats.iter() {
                println!("- Seat #{}", seat);
            }
        }
        println!("------------- END INFO -------------");
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }
}
