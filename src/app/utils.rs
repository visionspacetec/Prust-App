use super::*;
use byteorder::{BigEndian, ByteOrder};

pub fn generate_one_shot_report(tc: &Tc3_27, report: &mut Vec<u8>) {
    let hk_structs = tc.get_hk_report_struct_ids();
    if let Ok(hk_reports) = HK_REPORTS.lock(Duration::ms(100)){
        for i in hk_structs.iter() {
            // TODO: Doesn't check if invalid id
            match hk_reports.get(i) {
                Some((sp, _)) => {
                    let mut res = Vec::<u8>::new();
                    for &p in sp.get_params().iter() {
                        match p {
                            0 => {
                                res.push(0);
                                res.push(0);
                                let len = res.len();
                                BigEndian::write_u16(
                                    &mut res[len - 2..len],
                                    adc::ADC1.lock(Duration::infinite()).and_then(
                                        |mut adc| Ok(adc.read())
                                    ).unwrap_or_default(),
                                );
                            }
                            _ => continue,
                        }
                    }

                    let tm3_25 = Tm3_25::new_service_3_25(42, 0, *i, res.to_vec());
                    match tm3_25 {
                        Err(e) => {
                            let (err_code, err_data) = error::get_err_code_n_data(e);
                            let err_report =
                                SpacePacket::<_>::new_service_1_8(sp, 0, 0, err_code, err_data)
                                    .unwrap();
                            report.extend(err_report.to_bytes().iter());
                        }
                        Ok(tm) => {
                            report.extend(tm.to_bytes());
                        }
                    };
                }

                None => {}
            }
        }
    }
}

pub fn generate_periodic_report(report: &mut Vec<u8>) {
    {
        if let Ok(hk_reports) = HK_REPORTS.lock(Duration::ms(10000)){
            for ent in hk_reports.iter() {
                // TODO: Doesn't check if invalid id
                match ent {
                    (struct_id, (sp, true)) => {
                        let mut res = Vec::<u8>::new();
                        for &p in sp.get_params().iter() {
                            // PAREMETERS ARE MATCHED HERE
                            match p {
                                0 => {
                                    res.push(0);
                                    res.push(0);
                                    let len = res.len();
                                    BigEndian::write_u16(
                                        &mut res[len - 2..len],
                                        adc::ADC1.lock(Duration::ms(100)).and_then(
                                            |mut adc| Ok(adc.read())
                                        ).unwrap_or_default(),
                                    );
                                }
                                _ => continue,
                            }
                        }
    
                        let tm3_25 = Tm3_25::new_service_3_25(42, 0, *struct_id, res.to_vec());
                        match tm3_25 {
                            Err(e) => {
                                let (err_code, err_data) = error::get_err_code_n_data(e);
                                let err_report =
                                    SpacePacket::<_>::new_service_1_8(sp, 0, 0, err_code, err_data)
                                        .unwrap();
                                report.extend(err_report.to_bytes().iter());
                            }
                            Ok(tm) => {
                                report.extend(tm.to_bytes());
                            }
                        };
                    }
    
                    _ => {}
                }
            }
        }
    }
}
