//! Functions are defined here
use super::*;

/// Parses the arguments for turn_led
pub fn pre_turn_led(args: &Vec<u8>) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::InvalidArg);
    } else {
        turn_led(args[0] != 0)
    }
}

/// Uses user1_1 from SHARED_PER global variable.
pub fn turn_led(turn: bool) -> Result<(), Error> {
    gpio_output::Vst104UserPin::set(1, 1, turn);
    Ok(())
}
static LED_COUNT:usize=3;
/// Parses the arguments for turn_led
pub fn pre_set_led(args: &Vec<u8>) -> Result<(), Error> {
    if args.len() != 2 || usize::from(args[1]) > LED_COUNT{
        return Err(Error::InvalidArg);
    } else {
        set_led(args[0], args[1] != 0)
    }
}
/// Uses user1_1, user1_2 from SHARED_PER global variable.
pub fn set_led(led_no: u8, turn: bool) -> Result<(), Error> {
    let idx = match led_no {
        0 => (1,1),
        1 => (1,2),
        2 => (3,1),
        _ => (0,0)
    };
    // TODO: Do error propagation.
    if idx.0 > 0{
        gpio_output::Vst104UserPin::set(idx.0, idx.1, turn);
    }
    Ok(())
}
