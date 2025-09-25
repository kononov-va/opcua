use std::{fs::OpenOptions, io::{Read, Write}};
use crc16::*;
use chrono::{DateTime, Local, TimeZone, Utc};

struct DeviceAnswer{
    message_time: DateTime<Local>,
    message_items: [f32; 12],
}

fn main() {
    let message: [u8; 7] = [1, 0x55, 0x4d, 0xff, 0xff, 0, 0];
    let mut answer: [u8; 33] = [0; 33];

    let mut file_desc = OpenOptions::new().read(true).write(true).open("/dev/ttyS3")
        .expect("failed to open ttyS3");
    file_desc.write(&message).expect("failed to write ttyS3");
    file_desc.read(&mut answer).expect("failed to read ttyS3");

    let f_answer = get_format_message(&answer).expect("CRC error");

    print!("{}; ", f_answer.message_time);
    for item in f_answer.message_items{
        print!("{}; ", item);
    }
}

fn get_format_message(buf: &[u8]) -> Result<DeviceAnswer, &'static str>{
    if u16::from_le_bytes([buf[31], buf[32]]) == State::<XMODEM>::calculate(&buf[0..31]) {
        let formated_answer = DeviceAnswer{
            message_time: Local::now(),
            message_items: [f32::from(u16::from_le_bytes([buf[5], buf[6]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[7], buf[8]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[9], buf[10]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[11], buf[12]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[13], buf[14]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[15], buf[16]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[17], buf[18]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[19], buf[20]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[21], buf[22]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[23], buf[24]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[25], buf[26]]))/32.0, 
                f32::from(u16::from_le_bytes([buf[27], buf[28]]))/32.0],
        };
        Ok(formated_answer)
    }
    else {
        Err("CRC error")
    }
}