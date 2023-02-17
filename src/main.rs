use seat_reservation_system::client::connection::connect;
use seat_reservation_system::client::terminal::{build_request, get_menu_selection, get_user_info};
use seat_reservation_system::config::{IP_SERVERS, NUM_THREADS};
use seat_reservation_system::server::connection::stream::handle_connection;
use seat_reservation_system::server::seat_manager::Seat;
use seat_reservation_system::server::user::User;
use seat_reservation_system::server::utils::multithread::ThreadPool;
use std::net::TcpListener;
use std::{process, thread, time};

fn main() {
    let run_mode = std::env::args().nth(1);

    match run_mode {
        Some(mode) => {
            match mode.as_str() {
                "SERVER" => match TcpListener::bind(IP_SERVERS) {
                    Ok(listener) => {
                        let thread_pool = ThreadPool::new(NUM_THREADS);
                        let seats_vec = Seat::create_seats();
                        handle_connection(listener, thread_pool, seats_vec);
                    }
                    Err(_) => println!("Failed to listen in {}", IP_SERVERS),
                },
                "CLIENT" => {
                    let user_info = get_user_info();
                    let user = User::default(user_info.0, Some(user_info.1), Some(user_info.2));
                    loop {
                        let selection = get_menu_selection(&user);
                        let request = build_request(selection);

                        if connect(request).is_err() {
                            println!("Failed to send request to server");
                        }

                        let sleep_time = time::Duration::from_secs(1);
                        thread::sleep(sleep_time);
                    }
                } //Inicializar el cliente y el connection
                _ => {
                    println!("Non Valid Run mode.");
                    process::exit(0);
                }
            }
        }

        None => {
            println!("Run mode not specified. Use cargo run <mode>");
            println!("Valid run modes are CLIENT and SERVER")
        }
    }
}
