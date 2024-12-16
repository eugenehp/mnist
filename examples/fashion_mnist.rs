use image::*;
use mnist::*;
use ndarray::prelude::*;
use show_image::{create_window, event, WindowOptions};

#[show_image::main]
fn main() {
    let (trn_size, _rows, _cols) = (50_000, 28, 28);

    // Deconstruct the returned Mnist struct.
    let Mnist {
        trn_img, trn_lbl, ..
    } = MnistBuilder::new()
        .use_fashion_data() // Comment out this and the changed `.base_path()` to run on the original MNIST
        .base_path("data/fashion/") // Comment out this and `use_fashion_data()` to run on the original MNIST
        .label_format_digit()
        .training_set_length(trn_size)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .download_and_extract()
        .finalize();

    let item_num = 3;
    return_item_description_from_number(trn_lbl[item_num]);

    let train_data = Array3::from_shape_vec((50_000, 28, 28), trn_img)
        .expect("Error converting images to Array3 struct")
        .mapv(|x| x as f32 / 256.);

    let image = bw_ndarray2_to_rgb_image(train_data.slice(s![item_num, .., ..]).to_owned());
    let window_options = WindowOptions::new().set_size(Some([100, 100]));
    let window = create_window("image", window_options).unwrap();
    window.set_image("test_result", image).unwrap();

    // Wait for the window to be closed or Escape to be pressed.
    for event in window.event_channel().map_err(|e| e.to_string()).unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if !event.is_synthetic
                && event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                println!("Escape pressed!");
                break;
            }
        }
    }
}

fn return_item_description_from_number(val: u8) {
    let description = match val {
        0 => "T-shirt/top",
        1 => "Trouser",
        2 => "Pullover",
        3 => "Dress",
        4 => "Coat",
        5 => "Sandal",
        6 => "Shirt",
        7 => "Sneaker",
        8 => "Bag",
        9 => "Ankle boot",
        _ => panic!("An unrecognized label was used..."),
    };
    println!(
        "Based on the '{}' label, this image should be a: {} ",
        val, description
    );
}

fn bw_ndarray2_to_rgb_image(arr: Array2<f32>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (width, height) = (arr.ncols(), arr.ncols());
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let val = (arr[[y, x]] * 255.) as u8;
            img.put_pixel(x as u32, y as u32, image::Rgb([val, val, val]))
        }
    }
    img
}
