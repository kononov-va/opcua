use std::{fs::OpenOptions, io::{Read, Write}};
use crc16::*;

fn main() {
    let message: [u8; 7] = [1, 0x55, 0x4d, 0xff, 0xff, 0, 0];
    let mut answer: [u8; 33] = [0; 33];

    let mut file_desc = OpenOptions::new().read(true).write(true).open("/dev/ttyS3")
        .expect("failed to open ttyS3");
    file_desc.write(&message).expect("failed to write ttyS3");
    file_desc.read(&mut answer).expect("failed to read ttyS3");

    let message_crc16 = State::<AUG_CCITT>::calculate(&answer[0..30]);

    for item in answer
    {
        print!("{:X} ", item);
    }
    println!("\nAUG_CCITT {:X}", message_crc16);
    println!("CCITT_FALSE {:X}", State::<CCITT_FALSE>::calculate(&answer[0..30]));
    println!("CRC_A {:X}", State::<CRC_A>::calculate(&answer[0..30]));
    println!("DDS_110 {:X}", State::<DDS_110>::calculate(&answer[0..30]));
    println!("GENIBUS {:X}", State::<GENIBUS>::calculate(&answer[0..30]));
    println!("KERMIT {:X}", State::<KERMIT>::calculate(&answer[0..30]));
    println!("MCRF4XX {:X}", State::<MCRF4XX>::calculate(&answer[0..30]));
    println!("RIELLO {:X}", State::<RIELLO>::calculate(&answer[0..30]));
    println!("TMS37157 {:X}", State::<TMS37157>::calculate(&answer[0..30]));
    println!("XMODEM {:X}", State::<XMODEM>::calculate(&answer[0..30]));
    println!("X_25 {:X}", State::<X_25>::calculate(&answer[0..30]));
}
