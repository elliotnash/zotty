use skia_safe::{Canvas, Codec, Color, Data, Image, Paint, PaintStyle, Path, Picture, Pixmap, RRect, Rect, SamplingOptions, Surface, ClipOp, PaintCap, Point};
use serenity::model::prelude::User;
use std::sync::RwLock;
use std::{f32::consts::PI, fs::File, io::{BufWriter, BufReader, Cursor}};
use once_cell::sync::Lazy;

use super::super::colour::{Colour, format_descriminator};
use crate::database::DBUser;
use crate::CONFIG;


static BLANK_SURFACE: Lazy<RwLock<Surface>> = Lazy::new(|| RwLock::new(load_image_surface()));

fn load_image_surface() -> Surface {
    let data = std::fs::read("rank.png").unwrap();
    let skdata = Data::new_copy(&*data);
    let mut codec = Codec::from_data(skdata).unwrap();
    let image = codec.get_image(None, None).unwrap();
    let mut surface = Surface::new_raster_n32_premul((image.dimensions().width, image.dimensions().height)).expect("no surface!");
    let mut paint = Paint::default();
    surface.canvas().draw_image(image, (0, 0), None);
    surface.canvas().save();
    surface
}

pub async fn generate_rank_card(user: User, db_user: DBUser, rank: i32) -> BufWriter<Vec<u8>> {

    let avatar_url = user.static_avatar_url().unwrap_or(user.default_avatar_url())
        .replace("webp", "png").replace("1024", "256");

    let avatar =  reqwest::get(avatar_url).await.expect("Failed to download avatar").bytes().await.unwrap().to_vec();

    tokio::task::spawn_blocking(move || {
        generate(&*avatar, &user.name, user.discriminator, rank, db_user.level, db_user.xp)
    }).await.unwrap()
}

fn generate(avatar: &[u8], username: &str, user_discriminator: u16, 
        rank: i32, level: i32, xp: i32) -> BufWriter<Vec<u8>> {
    
    // get level xp with calculation
    let level_xp = super::super::get_level_xp(level);

    // clone surface from blank surface
    let surface = BLANK_SURFACE.read().unwrap().clone();

    let width = surface.width() as f32;
    let height = surface.height() as f32;

    // create base rectangle
    let mut paint = Paint::default();
    paint.set_color(Color::from_argb(238, 46, 52, 64));
    paint.set_style(PaintStyle::Fill);
    // set_colour(&context, Colour::from_alpha_hex(0x2E3440EE));
    let margin = 40_f32;
    let left_margin = 250_f32;
    let rect = Rect::new(margin, margin, width-margin, height-margin);
    surface.canvas().draw_round_rect(rect, 25., 25., &paint);
    //draw_rounded_rec(&context, margin, margin, width-margin, height-margin, 25_f64);

    //draw avatar
    draw_avatar(&mut surface, 0.5 * (left_margin+10.-margin), 0.5 * height, 190., margin, avatar);

    //draw progress bar
    let progress_margin = 30_f32;
    let progress_thickness = 30_f32;
    draw_progress_bar(&mut surface, left_margin+progress_margin, width-margin-progress_margin, 
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

fn draw_avatar(surface: &mut Surface, xc: f32, yc: f32, size: f32, left_margin: f32, mut avatar_data: &[u8]) -> f32 {
    // create image from byte slice
    let skdata = Data::new_copy(avatar_data);
    let mut codec = Codec::from_data(skdata).unwrap();
    let avatar = codec.get_image(None, None)
        .expect("Failed to read avatar");
    // calculate scale from image size
    let scale = size / (avatar.width() as f32);
    // create rect to place image
    let avatar_x = left_margin + xc - 0.5 * size;
    let avatar_y = yc - 0.5 * size;
    let rect = Rect::new(avatar_x, avatar_y, avatar_x+size, avatar_y+size);
    // create rounded clipping mask and apply
    surface.canvas().save();
    let crrect = RRect::new_rect_xy(rect, 20., 20.);
    surface.canvas().clip_rrect(crrect, ClipOp::Intersect, true);
    //draw avatar on canvas
    let paint = Paint::default();
    surface.canvas().draw_image_rect(avatar, None, rect, paint);
    // reset clipping mask
    surface.canvas().restore();
    // return the amount of space used 
    //TODO remove probably
    left_margin + size
}

//TODO move functions that take Surface to impl block if possible

fn draw_progress_bar(surface: &mut Surface, x1: f32, x2: f32, y: f32, thickness: f32, xp: i32, level_xp: i32) {
    // set_colour(context, Colour::from_hex(0x434C5E));
    // set colour and paint style
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgb(67, 76, 94));
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(thickness);
    paint.set_stroke_cap(PaintCap::Round);
    // create path and draw backing
    let backing_path = Path::new();
    backing_path.move_to(Point::new(x1, y));
    backing_path.line_to(Point::new(x2, y));
    surface.canvas().draw_path(&backing_path, &paint);
    // calculate length to draw
    let progress = xp as f32 / level_xp as f32;
    let bar_length = (x2-x1) * progress;
    // set new colour
    paint.set_color(Color::from_rgb(136, 192, 208));
    // set_colour(context, Colour::from_hex(0x88C0D0));
    // create path and draw bar
    let bar_path = Path::new();
    bar_path.move_to(Point::new(x1, y));
    bar_path.line_to(Point::new(x1+bar_length, y));
    surface.canvas().draw_path(&bar_path, &paint);
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