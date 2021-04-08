use cairo::{ImageSurface, FontFace, FontSlant, FontWeight, Context};
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
    let num_entries = 10_f64;
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
    set_colour(&context, Colour::from_alpha_hex(0x2E3440FA));
    draw_rounded_rec(&context, x1, y1, x2, y2, 15_f64);
    context.fill();
    // draw avatar
    let avatar_margin = 5_f64;
    let avatar_size = (y2-y1)-(avatar_margin*2_f64);
    let avatar_xc = x1+avatar_margin+(avatar_size/2_f64);
    let yc = (y1+y2)/2_f64;
    draw_avatar(context, avatar_xc, yc, avatar_size, user.avatar);
    // draw rank text
    let rank_margin = 15_f64;
    let rank_x = x1+avatar_margin+avatar_size+rank_margin;
    draw_rank_text(context, rank_x, yc, user.rank);
    // draw username + discriminator
    let username_margin = 75_f64;
    let username_x = rank_x+username_margin;
    draw_username_text(context, username_x, yc, &user.user.name, user.user.discriminator);
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

fn draw_rank_text(context: &Context, x1: f64, yc: f64, rank: i32) {
    set_colour(context, Colour::from_hex(0xedeff3));
    let font = FontFace::toy_create(&CONFIG.get().unwrap().modules.ranks.font_family, 
        FontSlant::Normal, FontWeight::Normal);
    context.set_font_face(&font);
    context.set_font_size(35_f64);
    // format text
    let rank_text = format!("#{}", rank);
    // get text extents
    let rank_extents = context.text_extents(&rank_text);
    // draw text
    context.move_to(x1, yc+(0.5*rank_extents.height)-1_f64);
    context.text_path(&rank_text);
    context.fill();
}

fn draw_username_text(context: &Context, x1: f64, yc: f64, username: &str, user_discriminator: u16) {
    // set font size
    let font_size = 35_f64;
    let margin = 6_f64;
    context.set_font_size(font_size);
    let font = FontFace::toy_create(&CONFIG.get().unwrap().modules.ranks.font_family, 
        FontSlant::Normal, FontWeight::Normal);
    context.set_font_face(&font);
    // get formatted strings
    let username_string = format!("{}", username);
    let discriminator_string = format!("#{}", user_discriminator);
    // get text extents
    let username_extents = context.text_extents(&username_string);
    let discriminator_extents = context.text_extents(&discriminator_string);
    // draw username
    set_colour(context, Colour::from_hex(0xd8dee9));
    context.move_to(x1, yc+(0.5*discriminator_extents.height)-1_f64);
    context.text_path(&username_string);
    context.fill();
    // draw discriminator
    set_colour(context, Colour::from_hex(0xc2b59b));
    context.move_to(x1+username_extents.width+margin, yc+(0.5*discriminator_extents.height)-1_f64);
    context.text_path(&discriminator_string);
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
