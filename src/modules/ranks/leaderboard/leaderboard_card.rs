use cairo::{ ImageSurface, FontFace, FontSlant, FontWeight, Context, LineCap };
use serenity::{http::CacheHttp, model::{id::UserId, prelude::User}};
use std::{f64::consts::PI, fs::File, io::{BufWriter, BufReader, Cursor}};

use futures::{stream, StreamExt};

use super::super::colour::{Colour, set_colour};
use crate::database::DBUser;
use crate::CONFIG;

#[derive(Debug)]
struct LUser {
    avatar: BufReader<Cursor<Vec<u8>>>,
    rank: i32,
    user: User,
    db_user: DBUser
}

pub async fn generate_leaderboard_card(cache_http: impl CacheHttp+Clone, db_users: Vec<DBUser>, starting_rank: i32) -> BufWriter<Vec<u8>> {

    let lusers: Vec<LUser> = stream::iter(db_users).enumerate()
        .map(|(i, db_user)| {
            let cache_http = &cache_http;
            async move {
                let user_id = db_user.user_id.parse::<UserId>().unwrap();
                let user = user_id.to_user(cache_http).await.unwrap();
                let avatar_url = user.static_avatar_url().unwrap_or(user.default_avatar_url())
                    .replace("webp", "png").replace("1024", "64");
                let avatar =  reqwest::get(avatar_url).await.expect("Failed to download avatar").bytes().await.unwrap().to_vec();
                let reader = BufReader::with_capacity(150_000, Cursor::new(avatar));
                LUser{
                    avatar: reader,
                    rank: (i as i32)+1+starting_rank,
                    user,
                    db_user
                }
            }
        }).buffered(10).collect().await;

    tokio::task::spawn_blocking(move || {
        generate(lusers)
    }).await.unwrap()
}

fn generate(users: Vec<LUser>) -> BufWriter<Vec<u8>> {

    // create surface from rank card
    let mut file = File::open("leaderboard.png")
        .expect("Couldn’t open input file.");

    let base = ImageSurface::create_from_png(&mut file)
        .expect("Couldn't create a surface!");
    
    let context = Context::new(&base);

    let width = f64::from(base.get_width());
    let height = f64::from(base.get_height());
    let margin = 7.5;
    let num_entries = users.len() as f64;
    let entry_height = (height-margin)/num_entries-margin;

    let mut y1 = margin;

    for user in users {
        draw_user_entry(&context, user, margin, y1, width-margin, y1+entry_height);
        y1 += entry_height+margin;
    }

    // write to buffer and return
    let mut writer = BufWriter::with_capacity(3500_000, Vec::<u8>::new());
    base.write_to_png(&mut writer)
        .expect("Couldn’t write to BufWriter");
    writer
}

fn draw_user_entry(context: &Context, user: LUser, x1: f64, y1: f64, x2: f64, y2: f64) {
    // create base rectangle
    set_colour(&context, Colour::from_alpha_hex(0x3B4252DD));
    draw_rounded_rec(&context, x1, y1, x2, y2, 15_f64);
    context.fill();
    // draw avatar ontop
    let avatar_margin = 5_f64;
    let avatar_size = (y2-y1)-(avatar_margin*2_f64);
    let avatar_xc = x1+avatar_margin+(avatar_size/2_f64);
    let yc = (y1+y2)/2_f64;
    draw_avatar(context, avatar_xc, yc, avatar_size, user.avatar);
}

fn draw_avatar(context: &Context, xc: f64, yc: f64, size: f64, mut image: BufReader<Cursor<Vec<u8>>>) {
    // create image from bufreader
    let avatar_source = ImageSurface::create_from_png(&mut image)
        .expect("Failed to read avatar");
    // calculate scale from image size
    let scale = size / f64::from(avatar_source.get_width());
    // paint background for transparent avatars
    set_colour(&context, Colour::from_hex(0x2E3440));
    draw_rounded_rec(&context, 
        xc - 0.5*(scale*f64::from(avatar_source.get_width())), 
        yc - 0.5*(scale*f64::from(avatar_source.get_height())), 
        xc + 0.5*(scale*f64::from(avatar_source.get_width())), 
        yc + 0.5*(scale*f64::from(avatar_source.get_height())),
        10_f64);
    context.fill();
    //scale entire canvas to scale image
    context.scale(scale, scale);
    //calculate x, y relative to scale
    let avatar_x = xc/scale - 0.5 * f64::from(avatar_source.get_width());
    let avatar_y = yc/scale - 0.5 * f64::from(avatar_source.get_height());
    //add avatar to canvas
    context.set_source_surface(&avatar_source, avatar_x, avatar_y);
    //reset scale
    context.scale(1_f64/scale, 1_f64/scale);
    //draw clipping mask for avatar
    draw_rounded_rec(&context, 
        xc - 0.5*(scale*f64::from(avatar_source.get_width())), 
        yc - 0.5*(scale*f64::from(avatar_source.get_height())), 
        xc + 0.5*(scale*f64::from(avatar_source.get_width())), 
        yc + 0.5*(scale*f64::from(avatar_source.get_height())),
        10_f64);
    // clip and paint
    context.clip();
    context.paint();
    // make sure to reset clipping mask so others can draw
    context.reset_clip();
}

fn draw_progress_bar(context: &Context, x1: f64, x2: f64, y: f64, thickness: f64, xp: i32, level_xp: i32) {
    set_colour(context, Colour::from_hex(0x2E3440));
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
    let discriminator_string = format!("#{}", user_discriminator);
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