use fltk::{
    app,
    enums::Event,
    prelude::{WidgetBase, WidgetExt, GroupExt, WindowExt},
    window::{GlutWindow, Window},
};

use image::io::Reader as ImageReader;
use speedy2d::{GLRenderer, color::*, dimen::Vector2, image::*, font::*, shape::*};
use dyn_clone::DynClone;
use std::cell::RefCell;
use std::rc::Rc;
use fltk::button::Button;
use fltk::draw::font;
use fltk::enums::{FrameType};
use fltk::frame::Frame;
use image::{DynamicImage, GenericImageView};
use fltk::enums::Color as fltkColor;
use fltk::image as fltkImage;
use winapi::um::wingdi::Rectangle;

fn main() {
    let app = app::App::default();
    app.set_visual(fltk::enums::Mode::Alpha).unwrap();


    let wind_size_w = 900;
    let wind_size_h = 500;
    let mut main_win = Window::new(0, 0, wind_size_w, wind_size_h, "Speedy 2d Resizeable");
    let mut win = GlutWindow::default().with_size(wind_size_w / 2, wind_size_h / 2).with_pos(0, 0);

    let mut frame = Frame::new(0, 20, 300, 380, "");
    frame.set_color(fltkColor::Blue);
    win.end();
    main_win.make_resizable(true);
    main_win.end();
    main_win.show();


    win.make_current();
    frame.set_frame(FrameType::EngravedBox);

    gl::load_with(|s| win.get_proc_address(s));
    // let mut renderer = unsafe { GLRenderer::new_for_current_context(((win.width()/1) as u32, (win.height()/1) as u32)) }.unwrap();

    let mut renderer = unsafe {
        GLRenderer::new_for_gl_context((640, 480), |s| win.get_proc_address(s))
    }.unwrap();
    let rect_viewport = Vector2::new((main_win.width() / 1) as u32, (main_win.height() / 1) as u32);
    renderer.set_viewport_size_pixels(rect_viewport);
    println!("win width,height:{},{}", win.width(), win.height());


    win.draw(
        {
            move |selfwin| {
                println!("hello");

                let rect_viewport = Vector2::new((main_win.width() / 1) as u32, (main_win.height() / 1) as u32);
                renderer.set_viewport_size_pixels(rect_viewport); // need to set viewport here
                println!("viewport:{:?}", rect_viewport);

                renderer.draw_frame(|graphics| {
                    let path = "mesh-1_640.png";
                    let loaded_img: DynamicImage = ImageReader::open(path).unwrap().decode().unwrap();
                    let has_alpha = has_alpha(&loaded_img);
                    let ci_bytes = match has_alpha {
                        true => loaded_img.into_rgba8().into_raw(),
                        false => loaded_img.into_rgb8().into_raw(),
                    };
                    let img_type = match has_alpha {
                        true => speedy2d::image::ImageDataType::RGBA,
                        false => speedy2d::image::ImageDataType::RGB,
                    };
                    let (img_width, img_height) = ImageReader::open(path).unwrap().decode().unwrap().dimensions();

                    let to_draw_image = match graphics.create_image_from_raw_pixels(
                        img_type,
                        ImageSmoothingMode::Linear,
                        Vector2::new(img_width, img_height),
                        &ci_bytes,
                    ) {
                        Ok(img) => img,
                        Err(e) => panic!("Error creating image {}", e)
                    };
                    let rect_pos = Vector2::new(0, 0);

                    let rect_pos_2 = Vector2::new(main_win.width(), main_win.height());
                    println!("selfwin:{},{}", selfwin.width().clone(), selfwin.height().clone());
                    println!("rect_pos_2:{:?}", rect_pos_2);
                    println!("main_win:{},{}", main_win.width(), main_win.height());

                    let rect = speedy2d::shape::Rectangle::new(
                        Vector2::new(rect_pos.x as f32, rect_pos.y as f32),
                        Vector2::new(rect_pos_2.x as f32, rect_pos_2.y as f32),
                        // Vector2::new((img_width-200) as f32, (img_height-50) as f32),
                    );
                    graphics.draw_rectangle_image(rect, &to_draw_image);
                    let font_size = 16 as f32;
                    let helvetica_bytes = include_bytes!("../HelveticaNeue/HelveticaNeue-Medium.otf");
                    let helvetica_font = speedy2d::font::Font::new(helvetica_bytes).unwrap();
                    let offset_text = helvetica_font.layout_text(&format!("Offset: x: {}, y: {}", format!("{:.1$}", rect_pos_2.x.to_string(), 5), format!("{:.1$}", rect_pos_2.y.to_string(), 5)), font_size, TextOptions::new());
                    graphics.draw_text((360.0, selfwin.height() as f32 - 20.0), Color::WHITE, &offset_text);
                    graphics.draw_text((rect_pos_2.x as f32, rect_pos_2.y as f32), Color::WHITE, &helvetica_font.layout_text(&*format!("x2{:?}", rect_pos_2), font_size, TextOptions::new()));
                    graphics.draw_text((rect_pos.x as f32, rect_pos.y as f32), Color::RED, &helvetica_font.layout_text(&*format!("x1{:?}", rect_pos), font_size, TextOptions::new()));
                    graphics.draw_text((0 as f32, -60 as f32), Color::RED, &helvetica_font.layout_text(&*format!("x1{:?}", -60), font_size, TextOptions::new()));
                });
            }
        });

    // win.resize_callback(move |selfwin, _x, _y, w, h| {});

    app.run().unwrap();
}

fn has_alpha(img: &DynamicImage) -> bool {
    img.color().has_alpha()
}

