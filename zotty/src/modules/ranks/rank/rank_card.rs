use skia_safe::{Canvas, ClipOp, Codec, Color, Data, Font, FontStyle, Image, Paint, PaintCap, PaintStyle, Path, Picture, Pixmap, Point, RRect, Rect, SamplingOptions, Surface, TextBlob, Typeface};
use serenity::model::prelude::User;
use tracing::log::warn;
use std::sync::RwLock;
use std::{f32::consts::PI, fs::File, io::{BufWriter, BufReader, Cursor}};
use once_cell::sync::Lazy;

use super::super::colour::{Colour, format_descriminator};
use crate::database::DBUser;
use crate::CONFIG;


static BLANK_SURFACE: Lazy<RwLock<Surface>> = Lazy::new(|| RwLock::new(load_image_surface()));
static TYPEFACE: Lazy<RwLock<Typeface>> = Lazy::new(|| RwLock::new(Typeface::from_name(&CONFIG.get().unwrap().modules.ranks.font_family, FontStyle::normal())
    .unwrap_or_else(|| {
        warn!("Failed to load typeface '{}'", &CONFIG.get().unwrap().modules.ranks.font_family);
        warn!("Loading default typeface");
        Typeface::default()
    })));

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
    surface.canvas().draw_image_rect(avatar, None, rect, &paint);
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
    // create path and draw bar
    let bar_path = Path::new();
    bar_path.move_to(Point::new(x1, y));
    bar_path.line_to(Point::new(x1+bar_length, y));
    surface.canvas().draw_path(&bar_path, &paint);
}

fn draw_username_text(surface: &mut Surface, x1: f32, x2: f32, y_bottom: f32, username: &str, user_discriminator: u16) {
    let width = x2 - x1;
    let xc = 0.5 * (x1 + x2);
    // create font
    let font_size = 50_f32;
    let mut font = Font::new(TYPEFACE.read().unwrap().clone(), font_size);
    // get bounds of both parts
    let discriminator_string = format!("#{}", format_descriminator(user_discriminator));
    let username_blob = TextBlob::new(username, &font).unwrap();
    let discriminator_blob = TextBlob::new(&discriminator_string, &font).unwrap();
    // if bigger than screen, rescale
    let total_width = username_blob.bounds().width() + 8. + discriminator_blob.bounds().width();
    let scale = if total_width > width {
        width / total_width
    } else {1.};
    // rescale everything based off that scale
    font.set_size(scale*font_size);
    let username_blob = TextBlob::new(username, &font).unwrap();
    let discriminator_blob = TextBlob::new(&discriminator_string, &font).unwrap();
    let yc = y_bottom-(discriminator_blob.bounds().height());
    // draw username
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::from_rgb(216, 222, 233));
    surface.canvas().draw_text_blob(
        &username_blob,
        Point::new(
            xc-(0.5*(username_blob.bounds().width()+discriminator_blob.bounds().width()))-(8.*scale),
            yc+(0.5*discriminator_blob.bounds().height())
        ),
        &paint
    );
    // draw discriminator
    paint.set_color(Color::from_rgb(194, 181, 155));
    surface.canvas().draw_text_blob(
        &discriminator_blob,
        Point::new(
            xc-(0.5*(discriminator_blob.bounds().width()-username_blob.bounds().width())),
            yc+(0.5*discriminator_blob.bounds().height())
        ),
        &paint
    );
}

fn draw_xp_text(surface: &mut Surface, xc: f32, yc: f32, xp: i32, level_xp: i32) -> f32 {
    let paint = Paint::default();
    paint.set_color(Color::from_rgb(237, 239, 243));
    let font = Font::new(TYPEFACE.read().unwrap().clone(), 30.);
    let seperation = 8_f32;
    // format text
    let top_text = format_i32(xp);
    let bottom_text = format_i32(level_xp);
    // get text blobs
    let top_blob = TextBlob::new(&top_text, &font).unwrap();
    let bottom_blob = TextBlob::new(&bottom_text, &font).unwrap();
    // draw top text
    surface.canvas().draw_text_blob(
        top_blob,
        Point::new(
            xc-(0.5*top_blob.bounds().width()),
            yc-seperation
        ),
        &paint
    );
    // draw bottom text
    surface.canvas().draw_text_blob(
        bottom_blob,
        Point::new(
            xc-(0.5*bottom_blob.bounds().width()),
            yc+bottom_blob.bounds().height()+seperation
        ),
        &paint
    );
    // draw seperator
    let half_width = 0.5 * top_blob.bounds().width().max(bottom_blob.bounds().width());
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(2.);
    let mut path = Path::new();
    path.move_to(Point::new(xc-half_width, yc));
    path.line_to(Point::new(xc+half_width, yc));
    surface.canvas().draw_path(&path, &paint);
    half_width
}

fn draw_rank_text(surface: &mut Surface, xc: f32, yc: f32, rank: i32) {
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgb(237, 239, 243));
    let top_size = 25_f32;
    let bottom_size = 75_f32;
    let seperation = 8_f32;
    let font = Font::new(TYPEFACE.read().unwrap().clone(), top_size);
    // format text
    let top_text = "RANK";
    let bottom_text = format!("#{}", rank);
    // get text blobs
    let top_blob = TextBlob::new(top_text, &font).unwrap();
    font.set_size(bottom_size);
    let bottom_blob = TextBlob::new(bottom_text, &font).unwrap();
    // get total height
    let half_height = 0.5 * (top_blob.bounds().height() + seperation + bottom_blob.bounds().height());
    // draw top
    surface.canvas().draw_text_blob(
        top_blob,
        Point::new(
            xc-(0.5*top_blob.bounds().width())+1.,
            (yc-half_height)+top_blob.bounds().height()
        ),
        &paint
    );
    // draw bottom
    surface.canvas().draw_text_blob(
        bottom_blob,
        Point::new(
            xc-(0.5*bottom_blob.bounds().width()),
            yc+half_height
        ),
        &paint
    );
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