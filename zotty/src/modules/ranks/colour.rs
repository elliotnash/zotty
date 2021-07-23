use std::convert::TryFrom;
use cairo::Context;

#[derive(Debug)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}
impl Colour {
    pub fn from_alpha_hex(hex: u32) -> Colour {
        Colour {
            red: u8::try_from((hex >> 24) & 255).unwrap(),
            green: u8::try_from((hex >> 16) & 255).unwrap(),
            blue: u8::try_from((hex >> 8) & 255).unwrap(),
            alpha: u8::try_from(hex & 255).unwrap()
        }
    }
    pub fn from_hex(hex: i32) -> Colour {
        Colour {
            red: u8::try_from((hex >> 16) & 255).unwrap(),
            green: u8::try_from((hex >> 8) & 255).unwrap(),
            blue: u8::try_from(hex & 255).unwrap(),
            alpha: 255
        }
    }
    pub fn red_decimal(&self) -> f64 {f64::from(self.red)*0.00392156862}
    pub fn green_decimal(&self) -> f64 {f64::from(self.green)*0.00392156862}
    pub fn blue_decimal(&self) -> f64 {f64::from(self.blue)*0.00392156862}
    pub fn alpha_decimal(&self) -> f64 {f64::from(self.alpha)*0.00392156862}
}

pub fn set_colour(context: &Context, colour: Colour) {
    context.set_source_rgba(colour.red_decimal(), colour.green_decimal(), 
        colour.blue_decimal(), colour.alpha_decimal());
}

pub fn format_descriminator(discriminator: impl ToString) -> String {
    let str = discriminator.to_string();
    match str.len() {
        1 => {format!("000{}", str)}
        2 => {format!("00{}", str)}
        3 => {format!("0{}", str)}
        _ => str
    }
}
