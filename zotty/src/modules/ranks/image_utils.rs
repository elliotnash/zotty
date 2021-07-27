
pub fn format_descriminator(discriminator: impl ToString) -> String {
    let str = discriminator.to_string();
    match str.len() {
        1 => {format!("000{}", str)}
        2 => {format!("00{}", str)}
        3 => {format!("0{}", str)}
        _ => str
    }
}
