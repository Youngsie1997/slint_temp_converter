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
        move |input_string, unit| {
            let ui = ui_handle.unwrap();
            let s = input_string.trim();
            let temp: f64 = convert_input_string(s);
            if unit == "C" {
                ui.set_result(convert_cel_to_fh(temp).into());
            } else {
                ui.set_result(convert_fh_to_cel(temp).into());
            }
        }
    });

    ui.run()
}

fn convert_input_string(s: &str) -> f64 {
    let f = s.trim().parse::<f64>().unwrap();
    return f;
}

fn convert_cel_to_fh(input: f64) -> String {
    let fh = input * 1.8;
    let output = fh + 32.00;
    return output.to_string();
}

fn convert_fh_to_cel(input: f64) -> String {
    let cel = input - 32.00;
    let output = cel / 1.8; //This is ugly I should consider making the temp variable mutatable in
                            //the first place as this requires copying into two locations in memory needlessly
    return output.to_string();
}
