use super::*;

static BLANK_VEC: [u8; FUNC_ID_LEN] = [0_u8; FUNC_ID_LEN];
// A temp helper function
pub fn create_func_id(name: &'static str) -> FuncId {
    let mut res = String::from(name);
    res.push_str(&String::from_utf8_lossy(&BLANK_VEC[name.len()..]));
    let res = FuncId::from(&res).unwrap();
    res
}

pub mod functions;

pub fn init() {
    
}
