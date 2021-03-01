use super::*;

pub struct  Vst104UserEn;
//IDEA: Make this checks on compile time ? or use macros.
impl Vst104UserEn{
    // TODO: Change return values.
    pub fn set(no:usize, val:bool){
        /* REGION TO REFACTOR*/
        let idx = match no {
            1 => 3,
            3 => 4,
            4 => 5,
            _ => -1
        };
        if idx != -1{
            unsafe {prust_set_pins(idx as size_t, val as u8)};
        };
        /* REGION TO REFACTOR END*/
    }
}
pub struct Vst104UserPin;


impl Vst104UserPin{
    // TODO: Change return values.
    pub fn set(no1:usize, no2:usize, val:bool){
        /* REGION TO REFACTOR*/
        let idx = match (no1,no2) {
            (1,1) => 0,
            (1,2) => 1,
            (3,1) => 2,
            _ => -1
        };
        if idx != -1{
            unsafe {prust_set_pins(idx as size_t, val as u8)};
        };
        /* REGION TO REFACTOR END*/
    }
}