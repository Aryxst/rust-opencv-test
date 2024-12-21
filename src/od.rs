pub mod vision {
    use opencv::{
        core::{min_max_loc, Point, ToInputArray, CV_32FC1},
        imgcodecs::{imread, IMREAD_GRAYSCALE},
        imgproc::{self, TM_CCORR_NORMED},
        prelude::*,
    };

    pub fn detect_image(haystack: &Mat) -> Mat {
        let needle = imread("./src/needle_spawn_sign.png", IMREAD_GRAYSCALE)
            .expect("Failed to read needle image");

        let zero = Mat::zeros(
            haystack.rows() - needle.rows() + 1,
            haystack.cols() - needle.cols() + 1,
            CV_32FC1,
        )
        .unwrap();

        let mut result = zero.to_mat().unwrap();

        imgproc::match_template(
            &haystack.input_array().unwrap(),
            &needle.input_array().unwrap(),
            &mut result,
            TM_CCORR_NORMED,
            &Mat::default(),
        )
        .expect("Failed to match template");

        result
    }
    pub fn get_loc(result: &Mat) -> (f64, f64, Point, Point) {
        let mut min_val: f64 = 0.0;
        let mut max_val: f64 = 0.0;
        let mut min_loc = Point::new(0, 0);
        let mut max_loc = Point::new(0, 0);
        let mask = Mat::default();

        let _ = min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &mask,
        );

        (min_val, max_val, min_loc, max_loc)
    }
}
