use skia_safe::{
    ClipOp,
    Codec,
    Color,
    Data,
    Font,
    SamplingOptions,
    FilterMode,
    MipmapMode,
    Paint,
    PaintCap,
    PaintStyle,
    Path,
    Point,
    RRect,
    Rect,
    Surface,
    EncodedImageFormat,
    utils::text_utils::Align
};
use serenity::model::prelude::User;

use super::super::image_utils::format_descriminator;
use crate::{
    database::DBUser,
    modules::ranks::image_utils::{
        FontWeight,
        load_typeface,
        BackgroundImage,
        load_image_surface
    }
};

pub async fn generate_rank_card(user: User, db_user: DBUser, rank: i32) -> Data {

    let avatar_url = user.static_avatar_url().unwrap_or(user.default_avatar_url())
        .replace("webp", "png").replace("1024", "256");

    let avatar =  reqwest::get(avatar_url).await.expect("Failed to download avatar").bytes().await.unwrap().to_vec();

    tokio::task::spawn_blocking(move || {
        generate(&*avatar, &user.name, user.discriminator, rank, db_user.level, db_user.xp)
    }).await.unwrap()
}

fn generate(avatar: &[u8], username: &str, user_discriminator: u16, 
        rank: i32, level: i32, xp: i32) -> Data {
    
    // get level xp with calculation
    let level_xp = super::super::get_level_xp(level);

    // clone surface from blank surface
    let mut surface = load_image_surface(BackgroundImage::Rank);

    let width = surface.width() as f32;
    let height = surface.height() as f32;

    // create base rectangle
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb(238, 46, 52, 64));
    paint.set_style(PaintStyle::Fill);
    let margin = 40_f32;
    let left_margin = 250_f32;
    let rect = Rect::new(margin, margin, width-margin, height-margin);
    surface.canvas().draw_round_rect(rect, 25., 25., &paint);

    //draw avatar
    draw_avatar(&mut surface, 0.5 * (left_margin+10.-margin), 0.5 * height, 190., margin, avatar);

    //draw progress bar
    let progress_margin = 30_f32;
    let progress_thickness = 30_f32;
    draw_progress_bar(&mut surface, left_margin+progress_margin, width-margin-progress_margin, 
        height-(margin+progress_margin), progress_thickness, xp, level_xp);

    //draw username
    let username_magin = 15_f32;
    draw_username_text(&mut surface, left_margin+progress_margin, width-(margin+progress_margin),
        height-(margin+progress_margin+username_magin), username, user_discriminator);

    //draw xp text
    let xp_xc = 0.5 * (left_margin+ (width-margin));
    let xp_xy = margin+60.;
    let xp_half_width = draw_xp_text(&mut surface, xp_xc, xp_xy, xp, level_xp);

    //draw rank text
    let rank_xc = 0.5 * (left_margin+(xp_xc-xp_half_width));
    draw_rank_text(&mut surface, rank_xc, xp_xy, rank);

    //draw level text
    let level_xc = 0.5 * (xp_xc+(width-margin));
    draw_level_text(&mut surface, level_xc, xp_xy, level);

    // write to buffer and return
    surface.image_snapshot().encode_to_data(EncodedImageFormat::PNG)
        .expect("Failed to encode rank card to png")
}

fn draw_avatar(surface: &mut Surface, xc: f32, yc: f32, size: f32, left_margin: f32, avatar_data: &[u8]) -> f32 {
    // create image from byte slice
    let skdata = Data::new_copy(avatar_data);
    let mut codec = Codec::from_data(skdata).unwrap();
    let avatar = codec.get_image(None, None)
        .expect("Failed to read avatar");
    // create rect to place image
    let avatar_x = left_margin + xc - 0.5 * size;
    let avatar_y = yc - 0.5 * size;
    let rect = Rect::new(avatar_x, avatar_y, avatar_x+size, avatar_y+size);
    // create rounded clipping mask and apply
    surface.canvas().save();
    let crrect = RRect::new_rect_xy(rect, 18., 18.);
    surface.canvas().clip_rrect(crrect, ClipOp::Intersect, true);
    //draw avatar on canvas
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    let sampling = SamplingOptions::new(FilterMode::Linear, MipmapMode::Nearest);
    surface.canvas().draw_image_rect_with_sampling_options(avatar, None, rect, sampling, &paint);
    // reset clipping mask
    surface.canvas().restore();
    // return the amount of space used 
    //TODO remove probably
    left_margin + size
}

//TODO move functions that take Surface to impl block if possible

fn draw_progress_bar(surface: &mut Surface, x1: f32, x2: f32, y: f32, thickness: f32, xp: i32, level_xp: i32) {
    // set colour and paint style
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_rgb(67, 76, 94));
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(thickness);
    paint.set_stroke_cap(PaintCap::Round);
    // create path and draw backing
    let mut backing_path = Path::new();
    backing_path.move_to(Point::new(x1, y));
    backing_path.line_to(Point::new(x2, y));
    surface.canvas().draw_path(&backing_path, &paint);
    // calculate length to draw
    let progress = xp as f32 / level_xp as f32;
    let bar_length = (x2-x1) * progress;
    // set new colour
    paint.set_color(Color::from_rgb(136, 192, 208));
    // create path and draw bar
    let mut bar_path = Path::new();
    bar_path.move_to(Point::new(x1, y));
    bar_path.line_to(Point::new(x1+bar_length, y));
    surface.canvas().draw_path(&bar_path, &paint);
}

fn draw_username_text(surface: &mut Surface, x1: f32, x2: f32, y_bottom: f32, username: &str, user_discriminator: u16) {
    let width = x2 - x1;
    let xc = 0.5 * (x1 + x2);
    // create paint
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::from_rgb(216, 222, 233));
    // create font
    let font_size = 50_f32;
    let seperator = 8_f32;
    let mut font = Font::new(load_typeface(FontWeight::Regular), font_size);
    // get bounds of both parts
    let discriminator_string = format!("#{}", format_descriminator(user_discriminator));
    let username_bounds = font.measure_str(&username, Some(&paint)).1;
    let discriminator_bounds = font.measure_str(&discriminator_string, Some(&paint)).1;
    // if bigger than screen, rescale
    let total_width = username_bounds.width() + seperator + discriminator_bounds.width();
    let scale = if total_width > width {
        width / total_width
    } else {1.};
    font.set_size(scale*font_size);
    let username_bounds = font.measure_str(&username, Some(&paint)).1;
    let discriminator_bounds = font.measure_str(&discriminator_string, Some(&paint)).1;
    // calculate height
    let max_top = username_bounds.top.max(discriminator_bounds.top);
    let half_height = 0.5 * (0.-max_top);
    // draw username
    surface.canvas().draw_str_align(
        username,
        Point::new(
            xc-(0.5*(username_bounds.width()+discriminator_bounds.width())), y_bottom - half_height
        ),
        &font,
        &paint,
        Align::Left
    );
    // draw discriminator
    paint.set_color(Color::from_rgb(194, 181, 155));
    surface.canvas().draw_str_align(
        &discriminator_string,
        Point::new(
            xc-(0.5*(discriminator_bounds.width()-username_bounds.width())), y_bottom - half_height
        ),
        &font,
        &paint,
        Align::Left
    );
}

fn draw_xp_text(surface: &mut Surface, xc: f32, yc: f32, xp: i32, level_xp: i32) -> f32 {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_rgb(237, 239, 243));
    let font = Font::new(load_typeface(FontWeight::Light), 30.);
    let seperation = 8_f32;
    let extra_extend = 4_f32;
    // format text
    let top_text = format_i32(xp);
    let bottom_text = format_i32(level_xp);
    // get text bounds
    let top_bounds = font.measure_str(&top_text, Some(&paint)).1;
    let bottom_bounds = font.measure_str(&bottom_text, Some(&paint)).1;
    // draw top text
    surface.canvas().draw_str_align(
        top_text,
        Point::new(
            xc, yc - seperation
        ),
        &font,
        &paint,
        Align::Center
    );
    // draw bottom text
    surface.canvas().draw_str_align(
        bottom_text,
        Point::new(
            xc, yc + seperation + bottom_bounds.height()
        ),
        &font,
        &paint,
        Align::Center
    );
    // draw seperator
    let half_width = 0.5 * top_bounds.width().max(bottom_bounds.width());
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(2.);
    let mut path = Path::new();
    path.move_to(Point::new(xc-half_width-extra_extend, yc));
    path.line_to(Point::new(xc+half_width+extra_extend, yc));
    surface.canvas().draw_path(&path, &paint);
    half_width
}

fn draw_rank_text(surface: &mut Surface, xc: f32, yc: f32, rank: i32) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_rgb(237, 239, 243));
    let top_size = 25_f32;
    let bottom_size = 75_f32;
    let seperation = 8_f32;
    let top_font = Font::new(load_typeface(FontWeight::Light), top_size);
    let mut bottom_font = Font::new(load_typeface(FontWeight::Regular), bottom_size);
    bottom_font.set_subpixel(true);
    // format text
    let top_text = "RANK";
    let bottom_text = format!("#{}", rank);
    // get text bounds
    let top_bounds = top_font.measure_str(top_text, Some(&paint)).1;
    let bottom_bounds = bottom_font.measure_str(&bottom_text, Some(&paint)).1;
    // get total height
    let half_height = 0.5 * (top_bounds.height() + seperation + bottom_bounds.height());
    // draw top
    surface.canvas().draw_str_align(
        top_text,
        Point::new(
            xc, yc - half_height + top_bounds.height()
        ),
        &top_font,
        &paint,
        Align::Center
    );
    // draw bottom
    surface.canvas().draw_str_align(
        &bottom_text,
        Point::new(
            xc, yc+half_height
        ),
        &bottom_font,
        &paint,
        Align::Center
    );
}

fn draw_level_text(surface: &mut Surface, xc: f32, yc: f32, level: i32) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_rgb(237, 239, 243));
    let top_size = 25_f32;
    let bottom_size = 75_f32;
    let seperation = 8_f32;
    let top_font = Font::new(load_typeface(FontWeight::Light), top_size);
    let bottom_font = Font::new(load_typeface(FontWeight::Regular), bottom_size);
    // format text
    let top_text = "LEVEL";
    let bottom_text = format!("{}", level);
    // get text bounds
    let top_bounds = top_font.measure_str(top_text, Some(&paint)).1;
    let bottom_bounds = bottom_font.measure_str(&bottom_text, Some(&paint)).1;
    // get total height
    let half_height = 0.5* (top_bounds.height()+seperation+bottom_bounds.height());
    // draw top
    surface.canvas().draw_str_align(
        top_text,
        Point::new(
            xc, yc - half_height + top_bounds.height()
        ),
        &top_font,
        &paint,
        Align::Center
    );
    // draw bottom
    surface.canvas().draw_str_align(
        &bottom_text,
        Point::new(
            xc, yc+half_height
        ),
        &bottom_font,
        &paint,
        Align::Center
    );
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