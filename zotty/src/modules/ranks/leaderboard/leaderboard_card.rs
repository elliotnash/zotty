use skia_safe::{
    Codec,
    Paint,
    Rect,
    RRect,
    Data,
    Font,
    Point,
    Surface,
    EncodedImageFormat,
    SamplingOptions,
    FilterMode,
    MipmapMode,
    PaintStyle,
    Color,
    ClipOp,
    utils::text_utils::Align
};
use serenity::{http::CacheHttp, model::{id::UserId, prelude::User}};
use std::f64::consts::PI;

use futures::{stream, StreamExt};

use super::super::image_utils::format_descriminator;
use crate::{
    database::DBUser,
    modules::ranks::image_utils::{
        BackgroundImage,
        load_image_surface,
        load_typeface,
        FontWeight
    }
};
use crate::CONFIG;

#[derive(Debug)]
struct LUser {
    avatar: Vec<u8>,
    rank: i32,
    user: User,
    db_user: DBUser
}

pub async fn generate_leaderboard_card(cache_http: impl CacheHttp+Clone, db_users: Vec<DBUser>, starting_rank: i32) -> Data {

    let lusers: Vec<LUser> = stream::iter(db_users).enumerate()
        .map(|(i, db_user)| {
            let cache_http = &cache_http;
            async move {
                let user_id = db_user.user_id.parse::<UserId>().unwrap();
                let user = user_id.to_user(cache_http).await.unwrap();
                let avatar_url = user.static_avatar_url().unwrap_or(user.default_avatar_url())
                    .replace("webp", "png").replace("1024", "64");
                LUser{
                    avatar: reqwest::get(avatar_url).await.expect("Failed to download avatar").bytes().await.unwrap().to_vec(),
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

fn generate(users: Vec<LUser>) -> Data {

    // create surface from rank card
    let mut surface = load_image_surface(BackgroundImage::Leaderboard);

    let width = surface.width() as f32;
    let height = surface.height() as f32;
    let margin = 7.5;
    let num_entries = 10.;
    let entry_height = (height-margin)/num_entries-margin;
    let username_margin = 25. + (users.last().unwrap().rank.to_string().len() as f32 *25.);

    let mut y1 = margin;

    for user in users {
        draw_user_entry(&mut surface, user, margin, y1, width-margin, y1+entry_height, username_margin);
        y1 += entry_height+margin;
    }

    // write to buffer and return
    surface.image_snapshot().encode_to_data(EncodedImageFormat::PNG)
        .expect("Failed to encode rank card to png")
}

fn draw_user_entry(surface: &mut Surface, user: LUser, x1: f32, y1: f32, x2: f32, y2: f32, username_margin: f32) {
    // create base rectangle
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb(250, 46, 52, 64));
    paint.set_style(PaintStyle::Fill);
    let rect = Rect::new(x1, y1, x2, y2);
    surface.canvas().draw_round_rect(rect, 15., 15., &paint);
    // draw avatar
    let avatar_margin = 5.;
    let avatar_size = (y2-y1)-(avatar_margin*2.);
    let avatar_xc = x1+avatar_margin+(avatar_size*0.5);
    let yc = (y1+y2)*0.5;
    draw_avatar(surface, avatar_xc, yc, avatar_size, &*user.avatar);
    // draw rank text
    let rank_margin = 15.;
    let rank_x = x1+avatar_margin+avatar_size+rank_margin;
    draw_rank_text(surface, rank_x, yc, user.rank);
    // draw username + discriminator
    let username_x = rank_x+username_margin;
    draw_username_text(surface, username_x, yc, &user.user.name, user.user.discriminator);
    // draw level text
    let level_margin = 20.;
    let level_x = x2-level_margin;
    draw_level_text(surface, level_x, yc, user.db_user.level);

}

fn draw_avatar(surface: &mut Surface, xc: f32, yc: f32, size: f32, mut avatar_data: &[u8]) {
    // create image from byte slice
    let skdata = Data::new_copy(avatar_data);
    let mut codec = Codec::from_data(skdata).unwrap();
    let avatar = codec.get_image(None, None)
        .expect("Failed to read avatar");
    // create rect to place image
    let half_size = size * 0.5;
    let rect = Rect::new(xc-half_size, yc-half_size, xc+half_size, yc+half_size);
    // create rounded clipping mask and apply
    surface.canvas().save();
    let crrect = RRect::new_rect_xy(rect, 10., 10.);
    surface.canvas().clip_rrect(crrect, ClipOp::Intersect, true);
    //draw avatar on canvas
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    let sampling = SamplingOptions::new(FilterMode::Linear, MipmapMode::Nearest);
    surface.canvas().draw_image_rect_with_sampling_options(avatar, None, rect, sampling, &paint);
    // reset clipping mask
    surface.canvas().restore();
}

fn draw_rank_text(surface: &mut Surface, x1: f32, yc: f32, rank: i32) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_rgb(237, 239, 243));
    let font = Font::new(load_typeface(FontWeight::Regular), 35.);
    // format text
    let rank_text = format!("#{}", rank);
    // get text bounds
    let rank_bounds = font.measure_str(&rank_text, Some(&paint)).1;
    // draw text
    surface.canvas().draw_str_align(
        &rank_text,
        Point::new(
            x1, yc+(-0.5*rank_bounds.top)
        ),
        &font,
        &paint,
        Align::Left
    );
}

fn draw_username_text(surface: &mut Surface, x1: f32, yc: f32, username: &str, user_discriminator: u16) {
    // set paint
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_rgb(216, 222, 233));
    // set font size
    let font_size = 35.;
    let margin = 6.;
    let font = Font::new(load_typeface(FontWeight::Regular), font_size);
    // get formatted strings
    let username_string = format!("{}", username);
    let discriminator_string = format!("#{}", format_descriminator(user_discriminator));
    // get text bounds
    let username_bounds = font.measure_str(&username_string, Some(&paint)).1;
    let discriminator_bounds = font.measure_str(&discriminator_string, Some(&paint)).1;
    // draw username
    surface.canvas().draw_str_align(
        &username_string,
        Point::new(
            x1, yc+(-0.5*username_bounds.top)
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
            x1+username_bounds.width()+margin, yc+(-0.5*discriminator_bounds.top)
        ),
        &font,
        &paint,
        Align::Left
    );
}

fn draw_level_text(surface: &mut Surface, x1: f32, yc: f32, level: i32) {
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgb(237, 239, 243));
    let font = Font::new(load_typeface(FontWeight::Regular), 35.);
    // format text
    let level_text = level.to_string();
    // get text bounds
    let level_bounds = font.measure_str(&level_text, Some(&paint)).1;
    // draw text
    surface.canvas().draw_str_align(
        &level_text,
        Point::new(
            x1-level_bounds.width(), yc+(-0.5*level_bounds.top)
        ),
        &font,
        &paint,
        Align::Left
    );
}
