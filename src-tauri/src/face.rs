use dlib_face_recognition::*;
use image::{Rgb, RgbImage};

fn draw_rectangle(image: &mut RgbImage, rect: &Rectangle, colour: Rgb<u8>) {
    for x in rect.left..rect.right {
        image.put_pixel(x as u32, rect.top as u32, colour);
        image.put_pixel(x as u32, rect.bottom as u32, colour);
    }

    for y in rect.top..rect.bottom {
        image.put_pixel(rect.left as u32, y as u32, colour);
        image.put_pixel(rect.right as u32, y as u32, colour);
    }
}

fn draw_point(image: &mut RgbImage, point: &Point, colour: Rgb<u8>) {
    image.put_pixel(point.x() as u32, point.y() as u32, colour);
    image.put_pixel(point.x() as u32 + 1, point.y() as u32, colour);
    image.put_pixel(point.x() as u32 + 1, point.y() as u32 + 1, colour);
    image.put_pixel(point.x() as u32, point.y() as u32 + 1, colour);
}

#[test]
fn test() {
    let mut first_photo = image::open("../data/two_people.jpg").unwrap().to_rgb8();
    let matrix_photo_1 = ImageMatrix::from_image(&first_photo);

    let detector = FaceDetector::default();

    let face_locations: FaceLocations = detector.face_locations(&matrix_photo_1);

    let Ok(landmarks) = LandmarkPredictor::open("../data/shape_predictor_68_face_landmarks.dat") else {
        panic!("Unable to load landmark predictor!");
    };

    let red = Rgb([255, 0, 0]);

    for r in face_locations.iter() {
        draw_rectangle(&mut first_photo, r, red);

        let landmarks = landmarks.face_landmarks(&matrix_photo_1, r);

        for point in landmarks.iter() {
            draw_point(&mut first_photo, point, red);
        }
    }

    if let Err(e) = first_photo.save("../data/two_people_2.jpg") {
        println!("Error saving the image: {e}");
    } else {
        println!("Output image saved to");
    }
}
