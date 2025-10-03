use std::{error::Error, io::{self, Read, Write}, time::Duration};
use crc16::*;
use chrono::{Local};//DateTime, 
use std::fmt;

use std::path::PathBuf;
use std::sync::Arc;

use opcua::server::prelude::*;
use opcua::sync::Mutex;

struct DeviceAnswer{
    message_time: DateTime,//<Local>,
    message_items: Vec<f32>,
}


fn main() {
    let port = "/dev/ttyS3";

   opcua::console_logging::init();

    // Create an OPC UA server with sample configuration and default node set
    let mut server = Server::new(ServerConfig::load(&PathBuf::from("../server.conf")).unwrap());

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write();
        address_space
            .register_namespace("urn:metran900-server")
            .unwrap()
    };

    // Add some variables of our own
    add_variables(&mut server, ns, String::from(port));

    // Run the server. This does not ordinarily exit so you must Ctrl+C to terminate
    server.run();
}

fn add_variables(server: &mut Server, ns: u16, port: String) {

    let v1_node = NodeId::new(ns, "v1");
    let v2_node = NodeId::new(ns, "v2");
    let v3_node = NodeId::new(ns, "v3");
    let v4_node = NodeId::new(ns, "v4");
    let v5_node = NodeId::new(ns, "v5");
    let v6_node = NodeId::new(ns, "v6");
    let v7_node = NodeId::new(ns, "v7");
    let v8_node = NodeId::new(ns, "v8");
    let v9_node = NodeId::new(ns, "v9");
    let v10_node = NodeId::new(ns, "v10");
    let v11_node = NodeId::new(ns, "v11");
    let v12_node = NodeId::new(ns, "v12");
    let address_space = server.address_space();

    {
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("1", "1", &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![
                Variable::new(&v1_node, "v1", "v1", 0f64),
                Variable::new(&v2_node, "v2", "v2", 0f64),
                Variable::new(&v3_node, "v3", "v3", 0f64),
                Variable::new(&v4_node, "v4", "v4", 0f64),
                Variable::new(&v5_node, "v5", "v5", 0f64),
                Variable::new(&v6_node, "v6", "v6", 0f64),
                Variable::new(&v7_node, "v7", "v7", 0f64),
                Variable::new(&v8_node, "v8", "v8", 0f64),
                Variable::new(&v9_node, "v9", "v9", 0f64),
                Variable::new(&v10_node, "v10", "v10", 0f64),
                Variable::new(&v11_node, "v11", "v11", 0f64),
                Variable::new(&v12_node, "v12", "v12", 0f64),
            ],
            &sample_folder_id,
        );
    }
    {
        // Store a counter and a flag in a tuple
        //let data = Arc::new(Mutex::new((DeviceAnswer)));//
        server.add_polling_action(300, move || {
            //let mut data = data.lock();
            if let Ok(data) = get_device_message(1, port.clone(), Duration::from_millis(500), 3){

                let mut address_space = address_space.write();

                let _ = address_space.set_variable_value(v1_node.clone(), data.message_items[0], 
                    &data.message_time, &data.message_time);
                let _ = address_space.set_variable_value(v2_node.clone(), data.message_items[1], 
                    &data.message_time, &data.message_time);               
                let _ = address_space.set_variable_value(v3_node.clone(), data.message_items[2], 
                    &data.message_time, &data.message_time);
                let _ = address_space.set_variable_value(v4_node.clone(), data.message_items[3], 
                    &data.message_time, &data.message_time);    
                let _ = address_space.set_variable_value(v5_node.clone(), data.message_items[4], 
                    &data.message_time, &data.message_time);   
                let _ = address_space.set_variable_value(v6_node.clone(), data.message_items[5], 
                    &data.message_time, &data.message_time);   
                let _ = address_space.set_variable_value(v7_node.clone(), data.message_items[6], 
                    &data.message_time, &data.message_time);           
                let _ = address_space.set_variable_value(v8_node.clone(), data.message_items[7], 
                    &data.message_time, &data.message_time);
                let _ = address_space.set_variable_value(v9_node.clone(), data.message_items[8], 
                    &data.message_time, &data.message_time);               
                let _ = address_space.set_variable_value(v10_node.clone(), data.message_items[9], 
                    &data.message_time, &data.message_time);
                let _ = address_space.set_variable_value(v11_node.clone(), data.message_items[10], 
                    &data.message_time, &data.message_time);    
                let _ = address_space.set_variable_value(v12_node.clone(), data.message_items[11], 
                    &data.message_time, &data.message_time);                                                                                                                                                       
            }

        });
    }
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
                .map(|x| f32::from(u16::from_le_bytes([x[0], x[1]]))/32.0).collect(),
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
