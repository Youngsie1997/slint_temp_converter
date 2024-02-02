use slint::format;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = TempConvWindow::new()?;

    ui.on_request_convert_temp({
        let ui_handle = ui.as_weak();
        move |value, unit| {
            let ui = ui_handle.unwrap();
            let s = value.trim();
            let f = match s.parse::<f64>() {
                Ok(f) => f,
                Err(err) => {
                    dbg!(err);
                    ui.set_error_text_value("Please enter a valid temperature ex: 37.85".into());
                    ui.invoke_show_error_popup();
                    return;
                }
            };
            match unit.trim() {
                "C" => {
                    let fh = convert_cel_to_fh(&f);
                    let kelvin = convert_cel_to_kelvin(&f);
                    let result = format!("{:.2} F", fh);
                    let result2 = format!("{:.2} K", kelvin);
                    ui.set_result(result);
                    ui.set_result2(result2);
                }
                "F" => {
                    let cel = convert_fh_to_cel(&f);
                    let kelvin = convert_fh_to_kelvin(&f);
                    let result = format!("{:.2} C", cel);
                    let result2 = format!("{:.2} K", kelvin);
                    ui.set_result(result);
                    ui.set_result2(result2);
                }
                "K" => {
                    let cel = convert_kelvin_to_cel(&f);
                    let fh = convert_kelvin_to_fh(&f);
                    let result = format!("{:.2} C", cel);
                    let result2 = format!("{:.2} F", fh);
                    ui.set_result(result);
                    ui.set_result2(result2);
                }
                _ => {
                    return;
                }
            }
        }
    });

    ui.run()
}

fn convert_cel_to_fh(input: &f64) -> f64 {
    return input * 1.8 + 32.00;
}

fn convert_cel_to_kelvin(input: &f64) -> f64 {
    return input + 273.15;
}

fn convert_fh_to_cel(input: &f64) -> f64 {
    return (input - 32.00) / 1.8;
}

fn convert_fh_to_kelvin(input: &f64) -> f64 {
    return convert_fh_to_cel(input) + 273.15;
}

fn convert_kelvin_to_cel(input: &f64) -> f64 {
    return input - 273.15;
}

fn convert_kelvin_to_fh(input: &f64) -> f64 {
    let cel = convert_kelvin_to_cel(input);
    return convert_cel_to_fh(&cel);
}
