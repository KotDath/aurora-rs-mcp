use image::GenericImageView;
use softbuffer::Context;
use softbuffer::Surface;
use std::num::NonZeroU32;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::AppInfo;
use crate::window::app_skia;

#[path = "application.rs"]
mod application;

pub fn show(app_info: AppInfo, event_loop: EventLoop<()>) {
    let app = application::WinitAppBuilder::with_init(
        move |elwt| {
            let app_info = app_info.clone();
            let window = application::make_window(&app_info, elwt, |w| w);
            let context = Context::new(window.clone()).unwrap();
            (app_info, window, context)
        },
        |_elwt, (_, window, context)| Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(|(app_info, window, _context), surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);
        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let size = window.inner_size();
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    surface
                        .resize(width, height)
                        .expect("Failed to resize surface");
                    // Get image from skia draw
                    let image = app_skia::main::draw(
                        app_info,
                        u32::from(width) as i32,
                        u32::from(height) as i32,
                    )
                    .expect("Failed to draw image");
                    // Update buffer
                    let mut buffer = surface.buffer_mut().unwrap();
                    let width = image.width() as usize;
                    for (x, y, pixel) in image.pixels() {
                        let red = pixel.0[0] as u32;
                        let green = pixel.0[1] as u32;
                        let blue = pixel.0[2] as u32;
                        let color = blue | (green << 8) | (red << 16);
                        buffer[y as usize * width + x as usize] = color;
                    }
                    // Redraw surface
                    buffer.present().unwrap();
                }
            }
            _ => {}
        }
    });
    application::run(event_loop, app);
}
