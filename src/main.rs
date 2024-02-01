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
                    return;
                }
            };

            let output = convert_cel_to_fh(&f);
            let result = output.to_string();
            ui.set_result(result.into());
        }
    });

    ui.run()
}

// fn convert_input_string(s: &str) -> f64 {
//     let f = s.trim().parse::<f64>().unwrap();
//     return f;
// }

fn convert_cel_to_fh(input: &f64) -> f64 {
    let fh = input * 1.8;
    let output = fh + 32.00;
    return output;
}
