use super::*;
use freertos_rs::Mutex;
use prust_core::error::*;

const DEFAULT_TIMEOUT:u32 = 100;

lazy_static! {
    pub static ref RX:Mutex<Rx> = Mutex::new(Rx{}).unwrap();
    pub static ref TX:Mutex<Tx> = Mutex::new(Tx{}).unwrap();
}


pub struct Rx;
pub struct Tx;


impl Rx {

    pub fn recv_packet(&mut self, dest:&mut Vec<u8>) -> Result<(),prust_core::error::Error>{
        unsafe {
            let dest_ptr = dest.as_mut_ptr();
            if prust_recv_packet(dest_ptr, dest.capacity(),DEFAULT_TIMEOUT) == 0{
                let len =((*dest_ptr.offset(4) as u32)<<8) + (*dest_ptr.offset(5) as u32) + 7;
                dest.set_len(len as usize);
                Ok(())
            } else {
                Err(Error::HalTimoutError)  // TODO: Fix
            }
        }
    }
}

impl Tx{

    pub fn send_packet(&mut self, src:&mut Vec<u8>) -> Result<(),prust_core::error::Error>{
        unsafe{
            if prust_send_packet(src.as_mut_ptr(), src.len(),DEFAULT_TIMEOUT) == 0{
                Ok(())
            } else {
                Err(Error::HalTimoutError)  // TODO: Fix
            }
        }
    }
}