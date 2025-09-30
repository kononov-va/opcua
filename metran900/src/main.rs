use std::{error::Error, fs::OpenOptions, io::{self, Read, Write}};
use crc16::*;
use chrono::{DateTime, Local};
use std::fmt;
use std::time::Duration;
use tokio::time::error::Elapsed;

struct DeviceAnswer{
    message_time: DateTime<Local>,
    message_items: Vec<f32>,
}

fn main() {
    //let f_answer: DeviceAnswer;

 //   for i in 1..3{
    let f_answer = get_device_message(1, String::from("/dev/ttyS3"), 
            Duration::from_millis(500), 3)
            .unwrap_or_else(|error| {panic!("{error}")});

    println!("{}; ", f_answer.message_time);
    for item in f_answer.message_items{
        print!("{}; ", item);
    }
    print!("\n");
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

fn get_device_message(address: u8, serial_port: String, timeout: Duration, retry: i8) -> Result<DeviceAnswer, Box<dyn Error>>{
    let mut answer: [u8; 33] = [0; 33];
    let mut read_result: Result<usize, io::Error>;

    let mut cnt = retry;
    while cnt > 0 {
        read_result = match tokio::runtime::Runtime::new().unwrap().
            block_on(async {tokio::time::timeout(timeout, async {
            ask_device(address, serial_port.clone(), &mut answer) }).await })
        {
            Ok(res) => {cnt = 0; res},
            Err(_) =>  continue,
        };
        cnt -= 1;
    }

    let read_size = match read_result {
        Ok(size) => size,
        Err(e) => return Err(Box::new(e)),
    };

    if read_size == 33{
        match get_format_message(&answer) {
            Ok(f_answer) => Ok(f_answer),
            Err(e) => Err(Box::new(e)),
        }
    }
    else {
        Err(Box::new(ErrorWrongAnswer {}))
    }
}

fn ask_device(address: u8, serial_port: String, buf: &mut [u8]) -> Result<usize, io::Error>{
    let mut message: [u8; 7] = [0, 0x55, 0x4d, 0xff, 0xff, 0, 0];
    message[0] = address;

    let mut file_desc = match OpenOptions::new().read(true).write(true).open(serial_port){
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    file_desc.write(&message)?;
    
    match file_desc.read(buf) {
        Ok(size) => Ok(size),
        Err(e) => Err(e),
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
            message_time: Local::now(),
            message_items: buf[5..29].chunks(2)
                .map(|x| f32::from(u16::from_le_bytes([x[0], x[1]]))/32.0).collect(),
        };
        Ok(formated_answer)

    }
    else {
        Err(CRCError {  })
    }
}