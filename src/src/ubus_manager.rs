use std::process::Command;
use std::path::Path;
use std::os::unix::net::UnixStream;
use ubus::{ Connection, BlobMsgData };
use serde_json::{ Value, json };

const DEFAULT_TIMEOUT: u32 = 300;

pub struct UbusManager {
	connection: Option<Connection<UnixStream>>
}

impl UbusManager  {
	pub fn new() -> UbusManager {
		let mut ubus_manager = UbusManager {
			connection: None
		};
		
		let socket = Path::new("/var/run/ubus.sock");

		match Connection::connect(&socket) {
			Ok(connection) => ubus_manager.connection = Some(connection), 
			_ => { }
		}
		return ubus_manager;
	}

	pub fn session_login(&mut self, username: String, password: String) -> Result<String, &'static str> {
		let login_args = format!(r#"{{
			"username": "{username}",
			"password": "{password}",
			"timeout": {DEFAULT_TIMEOUT}
		    }}"#);
		let output = Command::new("ubus").args(&["call", "session", "login", &login_args]).output().expect("Failed executing session login command");

		if output.status.success() {
			let stdout = String::from_utf8_lossy(&output.stdout);
			let parsed: Value = serde_json::from_str(&stdout.to_string()).expect("Failed to parse ubus output");
			if let Some(session_id) = parsed["ubus_rpc_session"].as_str() {
				return Ok(session_id.to_string());
			}
		} 
		return Err("Failed executing session login command");
	}

	pub fn get_uptime(&mut self, session_id: String) -> Result<u64, &'static str> {
		let result = self.authenticate(&session_id);
		match result {
			Err(_) => return Err("Authentication failed"),
			Ok(_) => {}
		}

		let output = Command::new("ubus").args(&["call", "system", "info"]).output().expect("Failed executing system info command");
		if output.status.success() {
			let stdout = String::from_utf8_lossy(&output.stdout);
			let parsed: Value = serde_json::from_str(&stdout.to_string()).expect("Failed to parse ubus output");

			if let Some(uptime) = parsed["uptime"].as_u64() {
				return Ok(uptime);
			}
		}
		return Err("Failed getting uptime");
	}

	fn authenticate(&mut self, session_id: &str) -> Result<(), ()> {
		let args = format!(r#"{{
			"ubus_rpc_session": "{session_id}"
		    }}"#);
		let output = Command::new("ubus").args(&["call", "session", "get", &args]).output().expect("Failed executing session get command");
	
		if output.status.success() {
			return Ok(());
		}

		return Err(());
	}

	fn get_ubus_obj_id(&mut self, name: &str) -> Option<u32> {
		let mut result = None;

		self.connection.as_mut()?.lookup(
			|obj| {
				if name == obj.path {
					result = Some(obj.id);
				}
			},
			|_| {}
		).unwrap();
	
		return result;
	}
}