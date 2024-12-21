mod od;
use od::*;
use opencv::{core::MatTraitConstManual, highgui, imgcodecs, Result};
use std::io;
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
    #[allow(unused)]
    const THRESHOLD: f32 = 1.0_f32;

    let mut window_name = String::new();

    println!();
    println!("Enter the window name(Can be truncated):");

    io::stdin()
        .read_line(&mut window_name)
        .expect("Failed to read line");

    let window_name = window_name.trim();

    let window = Window::get_window(window_name);

    let mut frame = Vec::new();

    while highgui::wait_key(1)? < 0 {
        /* let start = Instant::now(); */

        frame.clear();

        window
            .capture_image()
            .unwrap()
            .write_to(
                &mut io::Cursor::new(&mut frame),
                xcap::image::ImageFormat::Bmp,
            )
            .expect("Failed to write image");

        if let Ok(img) = imgcodecs::imdecode(&frame.as_bytes(), imgcodecs::IMREAD_GRAYSCALE) {
            let result = vision::detect_image(&img);
            println!("{:?}", &result.data_typed::<f32>().unwrap());

            highgui::imshow("Computer Vision", &img)?;
        } else {
            eprintln!("Failed to decode image");
        }

        /* println!("{} FPS", 1.0 / start.elapsed().as_secs_f32()); */
    }
    Ok(())
}
