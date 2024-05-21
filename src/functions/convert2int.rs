pub fn convert2float(data_input: &String) -> f64 {
    let x = data_input.trim().parse::<f64>().unwrap();
    x
}