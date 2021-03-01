use freertos_rs::Mutex;

use super::*;
//CH14!
pub struct Adc1;

lazy_static! {
    pub static ref ADC1:Mutex<Adc1> = Mutex::new(Adc1{}).unwrap();
}

impl Adc1{
    pub fn read(&mut self)-> u16{
        unsafe {prust_read_adc()}       
    }
}