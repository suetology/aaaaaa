mod ubus_manager;

use std::sync::Mutex;
use rouille::Request;
use rouille::Response;
use crate::ubus_manager::UbusManager;

const LOGIN_ROUTE: &str = "/login";
const UPTIME_ROUTE: &str = "/uptime";

enum StatusCode {
    OK,
    BAD_REQUEST,
    UNAUTHORIZED
}

impl StatusCode {
    fn value(&self) -> u16 {
        match self {
            StatusCode::OK => 200,
            StatusCode::BAD_REQUEST => 400,
            StatusCode::UNAUTHORIZED => 401,
        }
    }
}

enum HandleResult {
    LoginRoute { username: String, password: String },
    UptimeRoute { session_id: String },
    Error { status_code: StatusCode, msg: String }
}

fn hadle_route(request: &Request) -> HandleResult {
    let route: String = request.url();
    
    if route == LOGIN_ROUTE { 
        return handle_login_route(request);
    } else if route == UPTIME_ROUTE { 
        return handle_uptime_route(request); 
    } 

    return HandleResult::Error { status_code: StatusCode::BAD_REQUEST, 
                                 msg: "Unknown route".to_string() }
}

fn handle_login_route(request: &Request) -> HandleResult {
    let mut username: String;
    let mut password: String;

    match request.get_param("username") {
        Some(value) => username = value,
        None => return HandleResult::Error { status_code: StatusCode::UNAUTHORIZED, 
                                             msg: "Login failed. Username is not provided".to_string() }
    };

    match request.get_param("password") {
        Some(value) => password = value,
        None => return HandleResult::Error { status_code: StatusCode::UNAUTHORIZED, 
                                             msg: "Login failed. Password is not provided".to_string() }
    };

    return HandleResult::LoginRoute { username, password };
}

fn handle_uptime_route(request: &Request) -> HandleResult {
    let mut session_id: String;

    match request.get_param("session_id") {
        Some(value) => session_id = value,
        None => return HandleResult::Error { status_code: StatusCode::UNAUTHORIZED, 
                                             msg: "Failed getting uptime. Authorization is required".to_string() }
    }
    
    return HandleResult::UptimeRoute { session_id };
}

fn main() {
    let ubus_manager = Mutex::new(UbusManager::new());

    rouille::start_server("localhost:8080", move |request| {
        match hadle_route(request) {
            HandleResult::LoginRoute { username, password } => {
                let result = ubus_manager.lock().unwrap().session_login(username, password);
                match result {
                    Ok(id) => return Response::text("\nRouille server: Logged in successfully (session id: ".to_string() + &id + ")\n")
                                                .with_status_code(StatusCode::OK.value()),
                    Err(msg) => return Response::text("\nRouille server: ".to_string() + &msg.to_string() + "\n")
                                                .with_status_code(StatusCode::BAD_REQUEST.value())
                };
                return Response::text("");
            },
            HandleResult::UptimeRoute { session_id } => {
                let result = ubus_manager.lock().unwrap().get_uptime(session_id);
                match result {
                    Ok(uptime) => return Response::text("\nRouille server: uptime = ".to_string() + &uptime.to_string() + "\n")
                                                    .with_status_code(StatusCode::OK.value()),
                    Err(msg) => return Response::text("\nRouille server: ".to_string() + &msg.to_string() + "\n")
                                                .with_status_code(StatusCode::BAD_REQUEST.value())
                }
            },
            HandleResult::Error { status_code, msg } => 
                return Response::text("\nRouille server: ".to_string() + &msg.to_string() + "\n").with_status_code(status_code.value())
        };
    });
}