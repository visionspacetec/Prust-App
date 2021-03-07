use super::*;
// prust_core sp
use prust_core::{
    error::*,
    sp::*,
    sp::{
        services::{
            service_3::service_3_1::*, service_3::service_3_25::*, service_3::service_3_27::*,
            service_8::*,
        },
        tc::*,
        tm::*,
    },
    *,
};

// Give aliases
type Tc3_1 = SpacePacket<TcPacket<Service3_1>>;
type Tc3_27 = SpacePacket<TcPacket<Service3_27>>;
type Tm3_25 = SpacePacket<TmPacket<Service3_25>>;
type Tc8_1 = SpacePacket<TcPacket<Service8_1>>;


// Data structure utilities
use alloc::{string::String, vec::Vec};
use hashbrown::HashMap;

// REPORT_ID -> (PACKET,PERIODIC_REPORT_ENABLED)
lazy_static! {
    pub static ref HK_REPORTS: Mutex<HashMap<u8, (Tc3_1, bool)>> =
        Mutex::new(HashMap::new()).unwrap();
    static ref PERIODIC_BUF:Mutex<Vec<u8>> = Mutex::new(Vec::with_capacity(1024)).unwrap();
}

pub mod func_man;
/// Utility module for the temporary problem
pub mod utils;
use func_man::{functions::*, *};
use utils::*;
use hal::*;
use hal::uart::*;
const BUF_LEN:usize = 1024;

// Function reads the packet and parses it and sends parsed packet.
pub fn handle_packets() -> ! {
    /* FUNCTION MAP AREA START */
    let funcs: HashMap<FuncId, fn(&Vec<u8>) -> Result<(), Error>> = prust_core::map!(
        create_func_id("turn_led") => pre_turn_led as fn(&Vec::<u8>)->Result<(),Error>,
        create_func_id("set_led") => pre_set_led as fn(&Vec::<u8>)->Result<(),Error>
    );
    /* FUNCTION MAP AREA END */
    gpio_output::Vst104UserEn::set(1, true);
    gpio_output::Vst104UserEn::set(3, true);
    gpio_output::Vst104UserEn::set(4, true);
    
    /* TIMER SETUP */
    let t =Timer::new(Duration::infinite()).set_auto_reload(true).create(
        |_| { 
            if let Ok(mut m) = PERIODIC_BUF.lock(Duration::ms(WRITE_TIMEOUT)) { 
                generate_periodic_report(&mut m);
                if let Ok(mut tx) = TX.lock(Duration::ms(WRITE_TIMEOUT)){
                    tx.send_packet(&mut m).unwrap_or_default();
                } 
            };
        }
    ).unwrap();
    t.stop(Duration::infinite()).unwrap_or_default();
    t.change_period(Duration::infinite(), Duration::ticks(1000)).unwrap_or_default();
    /* TIMER END */

    let mut buffer:Vec<u8> = Vec::with_capacity(BUF_LEN);
    loop {

        if let Ok(mut rx) = RX.lock(Duration::infinite()){
            if let Err(_ignore) = rx.recv_packet(&mut buffer){
                continue;
            }
        } else{
            continue;
        }
        
        let mut report_bytes: Vec<u8> = Vec::new();        
        let mes_type = match prust_core::sp::get_service_type(&buffer[0..buffer.len()]) {
            Ok(m) => m,
            _ => continue, // TODO: Give warning
        };

        if mes_type == (8, 1) {
            /* TC[8,1] PERFORM A FUNCTION START */

            // checking if the packet given is in correct format or not
            let space_packet = match Tc8_1::from_bytes(&buffer[0..buffer.len()]) {
                Ok(sp) => sp,
                Err(_) => {
                    continue;
                }
            };
            // in case of error after executing the func
            if let Err(e) = space_packet.exec_func(&funcs) {
                let (err_code, err_data) = error::get_err_code_n_data(e);
                let err_report =
                    match SpacePacket::new_service_1_8(&space_packet, 0, 0, err_code, err_data) {
                        Ok(s) => s,
                        Err(_) => continue, // TODO: Give a warning
                    };
                report_bytes.extend(err_report.to_bytes().iter());
            } else {
                let exec_report = match SpacePacket::new_service_1_7(&space_packet, 42, 0) {
                    Ok(s) => s,
                    Err(_) => continue, // TODO: Give warning
                };
                report_bytes.extend(exec_report.to_bytes().iter());
            }

        /* TC[8,1] PERFORM A FUNCTION END */
        } else if mes_type == (3, 1) {
            /* TC[3,1] CREATE A HOUSEKEEPING PARAMETER REPORT STRUCTURE START */

            let space_packet = match Tc3_1::from_bytes(&buffer[0..buffer.len()]) {
                Ok(sp) => sp,
                Err(_) => continue, // TODO: Give warning
            };
            let exec_report = SpacePacket::<_>::new_service_1_7(&space_packet, 42, 0).unwrap();
            // TODO: Give error in case of duplicate
            {
                {
                    let mut hk = HK_REPORTS.lock(Duration::infinite()).unwrap();
                    &hk.insert(space_packet.hk_id(), (space_packet, false));
                }
            }

            report_bytes.extend(exec_report.to_bytes().iter());

        /* TC[3,1] CREATE A HOUSEKEEPING PARAMETER REPORT STRUCTURE END*/
        } else if mes_type == (3, 27) {
            /* TC[3,27] GENERATE A ONE SHOT REPORT FOR HOUSEKEEPING PARAMETER REPORT STRUCTURES START*/

            let space_packet = match Tc3_27::from_bytes(&buffer[0..buffer.len()]) {
                Ok(sp) => sp,
                Err(_) => continue, // TODO: Give warning
            };
            
            // TODO urgent
            generate_one_shot_report(&space_packet, &mut report_bytes);

            let exec_report = SpacePacket::new_service_1_7(&space_packet, 42, 0).unwrap();
            report_bytes.extend(exec_report.to_bytes().iter());

        /* TC[3,27] GENERATE A ONE SHOT REPORT FOR HOUSEKEEPING PARAMETER REPORT STRUCTURES END*/
        } else if mes_type == (3, 5) || mes_type == (3, 6) {
            /* TC[3,5/6] ENABLE OR DISABLE PERIODIC GENERATION OF THE HOUSEKEEPING PARAMETER REPORT*/
            let space_packet = match SpacePacket::from_bytes_service_3_5x6(&buffer[0..buffer.len()]) {
                Ok(sp) => sp,
                Err(_) => continue, // TODO: Give warning
            };
            let hk_params = space_packet.get_report_parameter_ids();
            let mut toggled_timer = false;
            // TODO URGENT
            if let Ok(mut hk_reports) = HK_REPORTS.lock(Duration::ms(WRITE_TIMEOUT))
            {
                for i in hk_params.iter() {
                    if let Some(ent) = hk_reports.get_mut(i) {
                        if ent.1 != (mes_type.1 == 5) {
                            // toggle if you have to else do nothing
                            ent.1 = mes_type.1 == 5;
                            toggled_timer = true;
                        }
                    }
                }
            }
            
            // if enabled listen to timer
            if toggled_timer {
                // Listening or Unlistening to timer in critical section
                if mes_type.1 == 5 {
                    t.start(Duration::infinite()).unwrap_or_default();
                } else {
                    t.stop(Duration::infinite()).unwrap_or_default();
                }
            }
        } else {
            continue;
        }
        
        if let Ok(mut tx) = TX.lock(Duration::infinite()){
            tx.send_packet(&mut report_bytes).unwrap_or_default();
        }
    }
}

const WRITE_TIMEOUT:u32 = 1000;