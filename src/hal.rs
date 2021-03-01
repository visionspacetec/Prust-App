
use cty::*;
use alloc::vec::*;
extern "C" {

    fn prust_set_pins(idx: size_t, set:uint8_t)-> c_void;
    fn prust_read_adc() -> uint16_t;
    fn prust_enable_timer() -> c_void;
    fn prust_disable_timer() -> c_void;
    fn prust_recv_packet(dest:*mut uint8_t, dest_cap:size_t, timeout: uint32_t)-> c_int;
    fn prust_send_packet(dest:*mut uint8_t, dest_cap:size_t, timeout: uint32_t)-> c_int;
}


// UART Wrapper from c
pub mod uart;
// TIM Wrapper from c
pub mod timer;
// ADC Wrapper from c
pub mod adc;
// GpioOutput Wrapper from c
pub mod gpio_output;