use cairo::{ ImageSurface, Format, Context, LineCap };
use std::{f64::consts::PI, convert::TryFrom, io::BufWriter};

//TODO fix formatting on long usernames

#[derive(Debug)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}
impl Colour {
    fn from_hex(hex: i32) -> Colour {
        Colour {
            red: u8::try_from(hex/(256_i32.pow(2))).unwrap(),
            green: u8::try_from((hex/256) % 256).unwrap(),
            blue: u8::try_from(hex % 256).unwrap()
        }
    }
    fn red_decimal(&self) -> f64 {f64::from(self.red)*0.00392156862}
    fn green_decimal(&self) -> f64 {f64::from(self.green)*0.00392156862}
    fn blue_decimal(&self) -> f64 {f64::from(self.blue)*0.00392156862}
}

pub fn generate_rank_card(username: &str, user_discriminator: u16, 
        rank: i32, level: i32, xp: i32) -> BufWriter<Vec<u8>> {
    
    // get level xp with calculation
    let level_xp = super::get_level_xp(level);

    // create surface and paint a rounded rectangle with nord1
    let surface = ImageSurface::create(Format::ARgb32, 750, 900)
        .expect("Couldn’t create a surface!");
    let context = Context::new(&surface);
    set_colour(&context, Colour::from_hex(0x2e3440));
    draw_rounded_rec(&context, 0_f64, 0_f64, 750_f64, 900_f64, 80_f64);
    context.fill();

    // calculate center of arc
    let xc = 750_f64*0.5;
    let yc = 900_f64*0.5 + 75_f64;

    // paint progress arc
    draw_progress_arc(&context, xc, yc, xp, level_xp);

    // add username text
    draw_username_text(&context, xc, 120_f64, username, user_discriminator);

    // add xp text
    draw_xp_text(&context, xc, yc, xp, level_xp);

    // add rank text
    draw_rank_text(&context, xc, yc-125_f64, rank);

    // add level text
    draw_level_text(&context, xc, yc+125_f64, level);

    // write to buffer and return
    let mut writer = BufWriter::with_capacity(70_000, Vec::<u8>::new());
    surface.write_to_png(&mut writer)
        .expect("Couldn’t write to BufWriter");
    writer
}

fn set_colour(context: &Context, colour: Colour) {
    context.set_source_rgb(colour.red_decimal(), colour.green_decimal(), colour.blue_decimal());
}

fn draw_progress_arc(context: &Context, xc: f64, yc: f64, xp: i32, level_xp: i32) {
    set_colour(context, Colour::from_hex(0x8fbcbb));
    context.set_line_width(20.0);
    context.set_line_cap(LineCap::Round);

    let start_angle = 0.5 * PI;
    let end_angle = start_angle + (2_f64*PI) * (f64::from(xp)/f64::from(level_xp));

    context.arc(xc, yc, xc-100.0, start_angle, end_angle);
    context.stroke();
}

fn draw_username_text(context: &Context, xc: f64, yc: f64, username: &str, user_discriminator: u16) {
    // set font size
    context.set_font_size(80_f64);
    // get text extents of both parts
    let discriminator_string = format!("#{}", user_discriminator);
    let username_extents = context.text_extents(&username);
    let discriminator_extents = context.text_extents(&discriminator_string);
    // if bigger than screen, rescale
    let total_width = username_extents.width + 8_f64 + discriminator_extents.width;
    let scale = if total_width > 650_f64 {
        650_f64 / total_width
    } else {1_f64};
    // rescale everything based off that scale
    context.set_font_size(80_f64*scale);
    let username_extents = context.text_extents(&username);
    let discriminator_extents = context.text_extents(&discriminator_string);
    // draw username
    set_colour(context, Colour::from_hex(0xd8dee9));
    context.move_to(xc-(0.5*(username_extents.width+discriminator_extents.width))-(8_f64*scale), yc+(0.5*discriminator_extents.height));
    context.text_path(&username);
    context.fill();
    // draw discriminator
    set_colour(context, Colour::from_hex(0xc2b59b));
    context.move_to(xc-(0.5*(discriminator_extents.width-username_extents.width)), yc+(0.5*discriminator_extents.height));
    context.text_path(&discriminator_string);
    context.fill();
}

fn draw_xp_text(context: &Context, xc: f64, yc: f64, xp: i32, level_xp: i32) {
    set_colour(context, Colour::from_hex(0xedeff3));
    context.set_font_size(60_f64);
    let xp_text = format!("{}/{}", format_i32(xp), format_i32(level_xp));
    let text_extents = context.text_extents(&xp_text);
    context.move_to(xc-(0.5*text_extents.width), yc+(0.5*text_extents.height));
    context.text_path(&xp_text);
    context.fill();
}

fn draw_rank_text(context: &Context, xc: f64, yc: f64, rank: i32) {
    set_colour(context, Colour::from_hex(0xedeff3));
    context.set_font_size(100_f64);
    let rank_text = format!("#{}", rank);
    let text_extents = context.text_extents(&rank_text);
    context.move_to(xc-(0.5*text_extents.width), yc+(0.5*text_extents.height));
    context.text_path(&rank_text);
    context.fill();
}

fn draw_level_text(context: &Context, xc: f64, yc: f64, level: i32) {
    set_colour(context, Colour::from_hex(0xd8dee9));
    context.set_font_size(60_f64);
    let level_text = format!("level {}", level);
    let text_extents = context.text_extents(&level_text);
    context.move_to(xc-(0.5*text_extents.width), yc+(0.5*text_extents.height));
    context.text_path(&level_text);
    context.fill();
}

fn draw_rounded_rec(context: &Context, x1: f64, y1: f64,
        x2: f64, y2: f64, radius: f64) {

    let degrees = PI / 180.0;

    context.new_sub_path();
    context.arc(x2 - radius, y1 + radius, radius, -90_f64 * degrees, 0_f64 * degrees);
    context.arc(x2 - radius, y2 - radius, radius, 0_f64 * degrees, 90_f64 * degrees);
    context.arc(x1 + radius, y2 - radius, radius, 90_f64 * degrees, 180_f64 * degrees);
    context.arc(x1 + radius, y1 + radius, radius, 180_f64 * degrees, 270_f64 * degrees);
    context.close_path();
}

fn format_i32(num: i32) -> String {
    if num < 1000 {
        format!("{}", num)
    } else if num < 10_000 {
        format!("{:.2}K", f64::from(num) / 1000.0)
    } else if num < 100_000 {
        format!("{:.1}K", f64::from(num) / 1000.0)
    } else {
        format!("{:.0}K", f64::from(num) / 1000.0)        
    }
}
