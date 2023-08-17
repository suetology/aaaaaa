extern crate rouille;

fn main() {
    rouille::start_server("localhost:8080", move |request| {
        rouille::Response::text("Hello, Rouille!")
    });
}

