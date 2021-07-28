use skia_safe::{Codec, Data, Surface, Typeface};
use std::fs;

pub fn load_image_surface(bg_image: BackgroundImage) -> Surface {
    let path = match bg_image {
        BackgroundImage::Rank => {"rank.png"}
        BackgroundImage::Leaderboard => {"leaderboard.png"}
    };
    let data = fs::read(path).unwrap();
    let skdata = Data::new_copy(&*data);
    let mut codec = Codec::from_data(skdata).unwrap();
    let image = codec.get_image(None, None).unwrap();
    let mut surface = Surface::new_raster_n32_premul((image.dimensions().width, image.dimensions().height)).expect("no surface!");
    surface.canvas().draw_image(image, (0, 0), None);
    surface.canvas().save();
    surface
}

pub enum BackgroundImage{
    Rank,
    Leaderboard
}

pub fn load_typeface(fontweight: FontWeight) -> Typeface {
    let path = match fontweight {
        FontWeight::Light => {"font-light.otf"}
        FontWeight::Regular => {"font.otf"}
    };
    let data = fs::read(path).unwrap();
    let skdata = Data::new_copy(&*data);
    Typeface::from_data(skdata, 0).unwrap()
}

pub enum FontWeight{
    Light,
    Regular
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
