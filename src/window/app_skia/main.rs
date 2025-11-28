use image::DynamicImage;

use skia_safe::{Color, EncodedImageFormat, Font, FontMgr, FontStyle, Paint, TextBlob};

use crate::AppInfo;

pub fn draw(
    app_info: &AppInfo,
    width: i32,
    height: i32,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    // Create surface
    let mut surface = skia_safe::surfaces::raster_n32_premul((width, height)).unwrap();
    let canvas = surface.canvas();

    // Get font
    let typeface = FontMgr::new()
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();

    // Get paint
    let paint = &mut Paint::default();

    // Set background color
    canvas.clear(Color::from_rgb(50, 50, 50));

    let shift = 90;

    // Title
    paint.set_color(Color::from_rgb(246, 87, 0));
    let font_size = 40.0;
    let text = &app_info.title;
    let x = width / 2 - (text.len() as i32 * font_size as i32 / 4);
    let y = (height / 2) - shift;
    let font = Font::from_typeface(&typeface, font_size);
    let blob = TextBlob::from_str(text, &font).unwrap();
    let _ = canvas.draw_text_blob(&blob, (x - 5, y), paint);

    // D-Bus
    paint.set_color(Color::WHITE);
    let font_size = 24.0;
    let text = &app_info.model;
    let x = width / 2 - (text.len() as i32 * font_size as i32 / 4);
    let y = (height / 2 + 40 + 40) - shift;
    let font = Font::from_typeface(&typeface, font_size);
    let blob = TextBlob::from_str(text, &font).unwrap();
    let _ = canvas.draw_text_blob(&blob, (x - 10, y), paint);

    // C lib
    paint.set_color(Color::WHITE);
    let font_size = 24.0;
    let text = &app_info.location;
    let x = width / 2 - (text.len() as i32 * font_size as i32 / 4);
    let y = (height / 2 + (40 + 40) + 24 + 20) - shift;
    let font = Font::from_typeface(&typeface, font_size);
    let blob = TextBlob::from_str(text, &font).unwrap();
    let _ = canvas.draw_text_blob(&blob, (x - 10, y), paint);

    // C++ clear
    paint.set_color(Color::WHITE);
    let font_size = 24.0;
    let text = &app_info.format;
    let x = width / 2 - (text.len() as i32 * font_size as i32 / 4);
    let y = (height / 2 + (40 + 40) + (24 + 20) + 24 + 20) - shift;
    let font = Font::from_typeface(&typeface, font_size);
    let blob = TextBlob::from_str(text, &font).unwrap();
    let _ = canvas.draw_text_blob(&blob, (x - 15, y), paint);

    // C++ Qt
    paint.set_color(Color::WHITE);
    let font_size = 24.0;
    let text = &format!("Is online: {}", app_info.is_online);
    let x = width / 2 - (text.len() as i32 * font_size as i32 / 4);
    let y = (height / 2 + (40 + 40) + (24 + 20) + (24 + 20) + 24 + 20) - shift;
    let font = Font::from_typeface(&typeface, font_size);
    let blob = TextBlob::from_str(text, &font).unwrap();
    let _ = canvas.draw_text_blob(&blob, (x, y), paint);

    // Get image
    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let binding = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .expect("Fail get image");
    Ok(image::load_from_memory(binding.as_bytes())?)
}
