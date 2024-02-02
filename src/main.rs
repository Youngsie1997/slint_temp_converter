slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = TempConvWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         if ui.get_counter() < 99 {
    //             ui.set_counter(ui.get_counter() + 1);
    //         }
    //     }
    // });

    ui.on_request_convert_temp({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let s = string.trim();
            let f = match s.parse::<f64>() {
                Ok(f) => f,
                Err(err) => {
                    dbg!(err);
                    ui.set_error_text_value("Please enter a valid temperature ex: 37.85".into());
                    ui.invoke_show_error_popup();
                    return;
                }
            };

            //        let output = convert_cel_to_fh(f);
            let output = convert_fh_to_cel(f);
            let result = output.to_string();
            ui.set_result(result.into());
        }
    });

    ui.on_selected_input_unit({
        let ui_handle = ui.as_weak();
        move |unit| {
            let ui = ui_handle.unwrap();
            let unit = unit.trim();
            match unit {
                "C" => {
                    ui.set_output_unitlist(ui.get_fk_unit_list());
                    ui.set_output_current_value("F".into());
                }
                "F" => {
                    ui.set_output_unitlist(ui.get_ck_unit_list());
                    ui.set_output_current_value("C".into());
                }
                "K" => {
                    ui.set_output_unitlist(ui.get_fc_unit_list());
                    ui.set_output_current_value("F".into());
                }
                _ => return,
            }
        }
    });

    ui.on_selected_output_unit({
        let ui_handle = ui.as_weak();
        move |unit| {
            let ui = ui_handle.unwrap();
            let unit = unit.trim();
            match unit {
                "C" => {
                    if ui.get_input_unit_current_value() == unit {
                        ui.set_input_unit_current_value("F".into());
                    }
                }
                "F" => {
                    if ui.get_input_unit_current_value() == unit {
                        ui.set_input_unit_current_value("C".into());
                    }
                }
                "K" => {
                    if ui.get_input_unit_current_value() == unit {
                        ui.set_input_unit_current_value("F".into());
                    }
                }
                _ => return,
            }
        }
    });

    ui.run()
}

// fn convert_input_string(s: &str) -> f64 {
//     let f = s.trim().parse::<f64>().unwrap();
//     return f;
// }

fn convert_cel_to_fh(input: f64) -> f64 {
    return input * 1.8 + 32.00;
}

fn convert_cel_to_kelvin(input: f64) -> f64 {
    return input + 273.15;
}

fn convert_fh_to_cel(input: f64) -> f64 {
    return (input - 32.00) / 1.8;
}

fn convert_fh_to_kelvin(input: f64) -> f64 {
    return convert_fh_to_cel(input) + 273.15;
}

fn convert_kelvin_to_cel(input: f64) -> f64 {
    return input - 273.15;
}

fn convert_kelvin_to_fh(mut input: f64) -> f64 {
    input = convert_kelvin_to_cel(input);
    return convert_cel_to_fh(input);
}
