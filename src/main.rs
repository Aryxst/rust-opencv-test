mod od;
use opencv::{
    boxed_ref::BoxedRef,
    core::{Point2i, ToInputArray, _InputArray},
    highgui, imgcodecs, imgproc,
    prelude::*,
    Result,
};
use std::{io, time::Instant};
use xcap::{image::EncodableLayout, Window};

trait WindowExtra {
    fn get_window(name: &str) -> Window;
}

impl WindowExtra for Window {
    fn get_window(name: &str) -> Window {
        Window::all()
            .unwrap()
            .into_iter()
            .find(|window| window.app_name().to_lowercase().contains(name))
            .expect("Unable to find window")
    }
}

fn main() -> Result<()> {
    let mut frame = Vec::new();
    let mut window_name = String::new();

    println!();
    println!("Enter the window name(Can be truncated):");

    io::stdin()
        .read_line(&mut window_name)
        .expect("Failed to read line");

    let window_name = window_name.trim();

    let window = Window::get_window(window_name);

    while highgui::wait_key(1)? < 0 {
        let start = Instant::now();

        frame.clear();

        window
            .capture_image()
            .unwrap()
            .write_to(
                &mut io::Cursor::new(&mut frame),
                xcap::image::ImageFormat::Bmp,
            )
            .expect("Failed to write image");

        if let Ok(img) = imgcodecs::imdecode(&frame.as_bytes(), imgcodecs::IMREAD_COLOR) {
            // Panics
            detect_image(
                &Mat::from_bytes::<Point2i>(frame.as_bytes())
                    .unwrap()
                    .input_array()
                    .unwrap(),
            );

            highgui::imshow("Computer Vision", &img)?;
        } else {
            eprintln!("Failed to decode image");
        }

        println!("{} FPS", 1.0 / start.elapsed().as_secs_f32());
    }
    Ok(())
}

fn detect_image(haystack: &BoxedRef<'_, _InputArray>) {
    let mut result = Mat::default();
    let needle = imgcodecs::imread("needle_spawn_sign.png", imgcodecs::IMREAD_UNCHANGED)
        .expect("Failed to read needle image");

    imgproc::match_template(
        &haystack.input_array().unwrap(),
        &needle.input_array().unwrap(),
        &mut result,
        imgproc::TM_CCORR_NORMED,
        &needle.input_array().unwrap(),
    )
    .unwrap();

    println!("{:?}", result);
}
