use crate::prelude::*;
use bmp::Image;

fn load_image() -> Result<Image> {
    let image = bmp::open(&config::CONFIG.assets.icon)?;
    Ok(image)
}

pub fn load_icon() -> Result<iced::window::Icon> {
    load_image().and_then(|image| {
        let width = image.get_width();
        let height = image.get_height();
        let mut list = Vec::new();
        for x in 0..width {
            for y in 0..height {
                let px = image.get_pixel(x, y);
                list.push(px.r);
                list.push(px.g);
                list.push(px.b);
                if px == bmp::Pixel::new(u8::MAX, u8::MAX, u8::MAX) {
                    list.push(0);
                } else {
                    list.push(u8::MAX);
                }
            }
        }
        let icon = iced::window::icon::from_rgba(list, width, height)?;
        Ok(icon)
    })
}
