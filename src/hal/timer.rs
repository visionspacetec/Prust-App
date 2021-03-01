use super::prust_enable_timer;
use super::prust_disable_timer;

pub struct Tim2;

pub trait EnableTimer{
    fn enable();
}

pub trait DisableTimer{
    fn disable();
}

impl EnableTimer for Tim2{
    fn enable() {
        unsafe {prust_enable_timer();}
    }
}
impl DisableTimer for Tim2{
    fn disable() {
        unsafe {prust_disable_timer();}
    }
}

