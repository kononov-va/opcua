use std::{error::Error, io::{self, Read, Write}, time::Duration, env, fs::File};
use crc16::*;
use std::fmt;
//use fork::daemon;
use daemonize::Daemonize;

use std::path::PathBuf;
//use std::sync::Arc;

use opcua::server::prelude::*;
//use opcua::sync::Mutex;

struct DeviceAnswer{
    message_time: DateTime,//<Local>,
    message_items: Vec<f32>,
}


fn main() {
let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("root")
        .group("daemon") // Group name
        .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }

    start_ua_server();
}

fn start_ua_server(){
   let port = "/dev/ttyS3";

   opcua::console_logging::init();

    // Create an OPC UA server with sample configuration and default node set
    let mut server = Server::new(ServerConfig::load(&PathBuf::from("/etc/metran900/server.conf")).unwrap());

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write();
        address_space
            .register_namespace("urn:metran900-server")
            .unwrap()
    };

    // Add some variables of our own
    add_variables(&mut server, ns, String::from(port), 2);
    add_variables(&mut server, ns, String::from(port), 3);
    add_variables(&mut server, ns, String::from(port), 4);

    // Run the server. This does not ordinarily exit so you must Ctrl+C to terminate
    server.run();
}

fn add_variables(server: &mut Server, ns: u16, port: String, address: u8) {

    let v1_node = NodeId::new(ns, format!("{}v1", address));
    let v2_node = NodeId::new(ns, format!("{}v2", address));
    let v3_node = NodeId::new(ns, format!("{}v3", address));
    let v4_node = NodeId::new(ns, format!("{}v4", address));
    let v5_node = NodeId::new(ns, format!("{}v5", address));
    let v6_node = NodeId::new(ns, format!("{}v6", address));
    let v7_node = NodeId::new(ns, format!("{}v7", address));
    let v8_node = NodeId::new(ns, format!("{}v8", address));
    let v9_node = NodeId::new(ns, format!("{}v9", address));
    let v10_node = NodeId::new(ns, format!("{}v10", address));
    let v11_node = NodeId::new(ns, format!("{}v11", address));
    let v12_node = NodeId::new(ns, format!("{}v12", address));

    let address_space = server.address_space();

    {
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder(address.to_string(), address.to_string(), &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![
                Variable::new(&v1_node, format!("{}v1", address), format!("{}v1", address), 0f64),
                Variable::new(&v2_node, format!("{}v2", address), format!("{}v2", address), 0f64),
                Variable::new(&v3_node, format!("{}v3", address), format!("{}v3", address), 0f64),
                Variable::new(&v4_node, format!("{}v4", address), format!("{}v4", address), 0f64),
                Variable::new(&v5_node, format!("{}v5", address), format!("{}v5", address), 0f64),
                Variable::new(&v6_node, format!("{}v6", address), format!("{}v6", address), 0f64),
                Variable::new(&v7_node, format!("{}v8", address), format!("{}v7", address), 0f64),
                Variable::new(&v8_node, format!("{}v8", address), format!("{}v8", address), 0f64),
                Variable::new(&v9_node, format!("{}v9", address), format!("{}v9", address), 0f64),
                Variable::new(&v10_node, format!("{}v10", address), format!("{}v10", address), 0f64),
                Variable::new(&v11_node, format!("{}v11", address), format!("{}v11", address), 0f64),
                Variable::new(&v12_node, format!("{}v12", address), format!("{}v12", address), 0f64),
            ],
            &sample_folder_id,
        );
    }
    {
        // Store a counter and a flag in a tuple
        server.add_polling_action(3000, move || {
            let mut address_space = address_space.write();

            match get_device_message(address, port.clone(), Duration::from_millis(800), 1){
                Ok(data) => {

                    let _ = address_space.find_variable_mut(v1_node.clone()).unwrap().set_value_direct(data.message_items[0],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v2_node.clone()).unwrap().set_value_direct(data.message_items[1],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v3_node.clone()).unwrap().set_value_direct(data.message_items[2],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v4_node.clone()).unwrap().set_value_direct(data.message_items[3],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v5_node.clone()).unwrap().set_value_direct(data.message_items[4],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v6_node.clone()).unwrap().set_value_direct(data.message_items[5],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v7_node.clone()).unwrap().set_value_direct(data.message_items[6],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v8_node.clone()).unwrap().set_value_direct(data.message_items[7],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v9_node.clone()).unwrap().set_value_direct(data.message_items[8],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v10_node.clone()).unwrap().set_value_direct(data.message_items[9],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v11_node.clone()).unwrap().set_value_direct(data.message_items[10],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);
                    let _ = address_space.find_variable_mut(v12_node.clone()).unwrap().set_value_direct(data.message_items[11],
                         opcua::types::StatusCode::Good, &data.message_time, &data.message_time);

                },
                Err(error) => {
                    eprintln!("metran900: {address} {}", error);
                    let now = DateTime::now();

                    let _ = address_space.find_variable_mut(v1_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v2_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v3_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v4_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v5_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v6_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v7_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v8_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v9_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v10_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v11_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                    let _ = address_space.find_variable_mut(v12_node.clone()).unwrap().set_value_direct(0,
                         opcua::types::StatusCode::BadNoCommunication, &now, &now);
                },
            }
        } );
    };
}

#[derive(Debug)]
struct ErrorWrongAnswer {}

impl Error for ErrorWrongAnswer {
    
}

impl fmt::Display for ErrorWrongAnswer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid messge size")
    }
}

fn get_device_message(address: u8, serial_port: String, timeout: Duration, retry: u8) -> Result<DeviceAnswer, Box<dyn Error>>{
    let mut answer: [u8; 33] = [0; 33];

    let mut cnt: u8 = retry;
    while cnt > 0 {
        match ask_device(address, serial_port.clone(), &mut answer, timeout) {
            Ok(size) => {
                if size == 33{
                    match get_format_message(&answer) {
                        Ok(f_answer) => return Ok(f_answer),
                        Err(e) => return Err(Box::new(e)),
                    };
                }
                else {
                    return Err(Box::new(ErrorWrongAnswer {}));
                };
            },
            Err(ref e) if e.kind() == serialport::ErrorKind::Io(io::ErrorKind::TimedOut) => {cnt -= 1;},
            Err(e) => return Err(Box::new(e)),
        };
    };
    return Err(Box::new(TimeoutError {}));
}

fn ask_device(address: u8, serial_port: String, buf: &mut [u8], timeout: Duration) -> Result<usize, serialport::Error> {
    let mut message: [u8; 7] = [0, 0x55, 0x4d, 0xff, 0xff, 0, 0];
    message[0] = address;
 
    let mut port = match serialport::new(serial_port, 9600).timeout(timeout).open() {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    match port.write(&message) {
        Ok(_) => {},
        Err(e) => return Err(Into::into(e)), 
    }

    match port.read(buf) {
        Ok(size) => Ok(size),
        Err(e) => return Err(Into::into(e)),
    }
}

#[derive(Debug)]
struct CRCError {}

impl Error for CRCError {}

impl fmt::Display for CRCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CRC error")
    }
}

fn get_format_message(buf: &[u8]) -> Result<DeviceAnswer, CRCError>{
    if u16::from_le_bytes([buf[31], buf[32]]) == State::<XMODEM>::calculate(&buf[0..31]) {
        let formated_answer = DeviceAnswer{
            message_time: DateTime::now(),
            message_items: buf[5..29].chunks(2)
                .map(|x| f32::from(i16::from_le_bytes([x[0], x[1]]))/32.0).collect(),
        };
        Ok(formated_answer)

    }
    else {
        Err(CRCError {  })
    }
}

#[derive(Debug)]
struct TimeoutError {}

impl Error for TimeoutError {}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TimeoutError error")
    }
}
