use cairo::{ ImageSurface, FontFace, FontSlant, FontWeight, Context, LineCap };
use serenity::model::prelude::User;
use std::{f64::consts::PI, fs::File, io::{BufWriter, BufReader, Cursor}};

use super::super::colour::{Colour, set_colour, format_descriminator};
use crate::database::DBUser;
use crate::CONFIG;

pub async fn generate_rank_card(user: User, db_user: DBUser, rank: i32) -> BufWriter<Vec<u8>> {

    let avatar_url = user.static_avatar_url().unwrap_or(user.default_avatar_url())
        .replace("webp", "png").replace("1024", "256");

    let avatar =  reqwest::get(avatar_url).await.expect("Failed to download avatar").bytes().await.unwrap().to_vec();

    let reader = BufReader::with_capacity(150_000, Cursor::new(avatar));

    tokio::task::spawn_blocking(move || {
        generate(reader, &user.name, user.discriminator, rank, db_user.level, db_user.xp)
    }).await.unwrap()
}

fn generate(avatar: BufReader<Cursor<Vec<u8>>>, username: &str, user_discriminator: u16, 
        rank: i32, level: i32, xp: i32) -> BufWriter<Vec<u8>> {
    
    // get level xp with calculation
    let level_xp = super::super::get_level_xp(level);

    // create surface from rank card
    let mut file = File::open("rank.png")
        .expect("Couldn’t open input file.");

    let base = ImageSurface::create_from_png(&mut file)
        .expect("Couldn't create a surface!");

    let width = f64::from(base.get_width());
    let height = f64::from(base.get_height());
    
    let context = Context::new(&base);

    // create base rectangle
    set_colour(&context, Colour::from_alpha_hex(0x2E3440EE));
    let margin = 40_f64;
    let left_margin = 250_f64;
    draw_rounded_rec(&context, margin, margin, width-margin, height-margin, 25_f64);
    context.fill();

    //draw avatar
    draw_avatar(&context, 0.5 * (left_margin+10_f64-margin), 0.5 * height, 190_f64, margin, avatar);

    //draw progress bar
    let progress_margin = 30_f64;
    let progress_thickness = 30_f64;
    draw_progress_bar(&context, left_margin+progress_margin, width-margin-progress_margin, 
        height-(margin+progress_margin), progress_thickness, xp, level_xp);

    //draw username 
    let username_magin = 15_f64;
    draw_username_text(&context, left_margin+progress_margin, width-(margin+progress_margin),
        height-(margin+progress_margin+username_magin), username, user_discriminator);

    //draw xp text
    let xp_xc = 0.5 * (left_margin+ (width-margin));
    let xp_xy = margin+60_f64;
    let xp_half_width = draw_xp_text(&context, xp_xc, xp_xy, xp, level_xp);

    //draw rank text
    let rank_xc = 0.5 * (left_margin+(xp_xc-xp_half_width));
    draw_rank_text(&context, rank_xc, xp_xy, rank);

    //draw level text
    let level_xc = 0.5 * (xp_xc+(width-margin));
    draw_level_text(&context, level_xc, xp_xy, level);

    // write to buffer and return
    let mut writer = BufWriter::with_capacity(350_000, Vec::<u8>::new());
    base.write_to_png(&mut writer)
        .expect("Couldn’t write to BufWriter");
    writer
}

fn draw_avatar(context: &Context, xc: f64, yc: f64, size: f64, left_margin: f64, mut image: BufReader<Cursor<Vec<u8>>>) -> f64 {
    // create image from bufreader
    let avatar_source = ImageSurface::create_from_png(&mut image)
        .expect("Failed to read avatar");
    // calculate scale from image size
    let scale = size / f64::from(avatar_source.get_width());
    // paint background for transparent avatars
    set_colour(&context, Colour::from_hex(0x2E3440));
    draw_rounded_rec(&context, 
        left_margin + xc - 0.5*(scale*f64::from(avatar_source.get_width())), 
        yc - 0.5*(scale*f64::from(avatar_source.get_height())), 
        left_margin + xc + 0.5*(scale*f64::from(avatar_source.get_width())), 
        yc + 0.5*(scale*f64::from(avatar_source.get_height())),
        20_f64);
    context.fill();
    //scale entire canvas to scale image
    context.scale(scale, scale);
    //calculate x, y relative to scale
    let avatar_x = left_margin/scale + xc/scale - 0.5 * f64::from(avatar_source.get_width());
    let avatar_y = yc/scale - 0.5 * f64::from(avatar_source.get_height());
    //add avatar to canvas
    context.set_source_surface(&avatar_source, avatar_x, avatar_y);
    //reset scale
    context.scale(1_f64/scale, 1_f64/scale);
    //draw clipping mask for avatar
    draw_rounded_rec(&context, 
        left_margin + xc - 0.5*(scale*f64::from(avatar_source.get_width())), 
        yc - 0.5*(scale*f64::from(avatar_source.get_height())), 
        left_margin + xc + 0.5*(scale*f64::from(avatar_source.get_width())), 
        yc + 0.5*(scale*f64::from(avatar_source.get_height())),
        20_f64);
    // clip and paint
    context.clip();
    context.paint();
    // make sure to reset clipping mask so others can draw
    context.reset_clip();
    // return the amount of space used
    left_margin + scale*f64::from(avatar_source.get_width())
}

fn draw_progress_bar(context: &Context, x1: f64, x2: f64, y: f64, thickness: f64, xp: i32, level_xp: i32) {
    set_colour(context, Colour::from_hex(0x434C5E));
    context.set_line_width(thickness);
    context.set_line_cap(LineCap::Round);
    // draw backing
    context.move_to(x1, y);
    context.line_to(x2, y);
    context.stroke();
    // calculate length to draw
    let progress = f64::from(xp) / f64::from(level_xp);
    let bar_length = (x2-x1) * progress;
    // draw bar
    context.move_to(x1, y);
    set_colour(context, Colour::from_hex(0x88C0D0));
    context.line_to(x1+bar_length, y);

    context.stroke();
}

fn draw_username_text(context: &Context, x1: f64, x2: f64, y_bottom: f64, username: &str, user_discriminator: u16) {
    let width = x2 - x1;
    let xc = 0.5 * (x1 + x2);
    // set font size
    let font_size = 50_f64;
    context.set_font_size(font_size);
    let font = FontFace::toy_create(&CONFIG.get().unwrap().modules.ranks.font_family, 
        FontSlant::Normal, FontWeight::Normal);
    context.set_font_face(&font);
    // get text extents of both parts
    let discriminator_string = format!("#{}", format_descriminator(user_discriminator));
    let username_extents = context.text_extents(&username);
    let discriminator_extents = context.text_extents(&discriminator_string);
    // if bigger than screen, rescale
    let total_width = username_extents.width + 8_f64 + discriminator_extents.width;
    let scale = if total_width > width {
        width / total_width
    } else {1_f64};
    // rescale everything based off that scale
    context.set_font_size(font_size*scale);
    let username_extents = context.text_extents(&username);
    let discriminator_extents = context.text_extents(&discriminator_string);
    let yc = y_bottom-(discriminator_extents.height);
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

fn draw_xp_text(context: &Context, xc: f64, yc: f64, xp: i32, level_xp: i32) -> f64 {
    set_colour(context, Colour::from_hex(0xedeff3));
    context.set_font_size(30_f64);
    let font = FontFace::toy_create(&CONFIG.get().unwrap().modules.ranks.font_family, 
        FontSlant::Normal, FontWeight::Normal);
    context.set_font_face(&font);
    let seperation = 8_f64;
    // format text
    let top_text = format_i32(xp);
    let bottom_text = format_i32(level_xp);
    // get text extents
    let top_extents = context.text_extents(&top_text);
    let bottom_extents = context.text_extents(&bottom_text);
    // draw top text
    context.move_to(xc-(0.5*top_extents.width), yc-seperation);
    context.text_path(&top_text);
    // draw bottom text
    context.move_to(xc-(0.5*bottom_extents.width), yc+bottom_extents.height+seperation);
    context.text_path(&bottom_text);
    context.fill();
    // draw seperator
    let half_width = 0.5 * top_extents.width.max(bottom_extents.width);
    context.set_line_width(2_f64);
    context.move_to(xc-half_width, yc);
    context.line_to(xc+half_width, yc);
    context.stroke();
    half_width
}

fn draw_rank_text(context: &Context, xc: f64, yc: f64, rank: i32) {
    set_colour(context, Colour::from_hex(0xedeff3));
    let font = FontFace::toy_create(&CONFIG.get().unwrap().modules.ranks.font_family, 
        FontSlant::Normal, FontWeight::Normal);
    context.set_font_face(&font);
    let bottom_size = 75_f64;
    let top_size = 25_f64;
    let seperation = 8_f64;
    // format text
    let top_text = "RANK";
    let bottom_text = format!("#{}", rank);
    // get text extents
    context.set_font_size(top_size);
    let top_extents = context.text_extents(top_text);
    context.set_font_size(bottom_size);
    let bottom_extents = context.text_extents(&bottom_text);
    // get total height
    let half_height = 0.5* (top_extents.height+seperation+bottom_extents.height);
    // draw top
    context.set_font_size(top_size);
    context.move_to(xc-(0.5*top_extents.width)+1_f64, (yc-half_height)+top_extents.height);
    context.text_path(top_text);
    context.fill();
    // draw bottom
    context.set_font_size(bottom_size);
    context.move_to(xc-(0.5*bottom_extents.width), yc+half_height);
    context.text_path(&bottom_text);
    context.fill();
}

fn draw_level_text(context: &Context, xc: f64, yc: f64, level: i32) {
    set_colour(context, Colour::from_hex(0xedeff3));
    let font = FontFace::toy_create(&CONFIG.get().unwrap().modules.ranks.font_family, 
        FontSlant::Normal, FontWeight::Normal);
    context.set_font_face(&font);
    let bottom_size = 75_f64;
    let top_size = 25_f64;
    let seperation = 8_f64;
    // format text
    let top_text = "LEVEL";
    let bottom_text = format!("{}", level);
    // get text extents
    context.set_font_size(top_size);
    let top_extents = context.text_extents(top_text);
    context.set_font_size(bottom_size);
    let bottom_extents = context.text_extents(&bottom_text);
    // get total height
    let half_height = 0.5* (top_extents.height+seperation+bottom_extents.height);
    // draw top
    context.set_font_size(top_size);
    context.move_to(xc-(0.5*top_extents.width), (yc-half_height)+top_extents.height);
    context.text_path(top_text);
    context.fill();
    // draw bottom
    context.set_font_size(bottom_size);
    context.move_to(xc-(0.5*bottom_extents.width), yc+half_height);
    context.text_path(&bottom_text);
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