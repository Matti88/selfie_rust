// A selfie with OpenCV in Rust
//
//


extern crate opencv;

use opencv::highgui;
use opencv::core;


// Function takes a picture and save the image as "/tmp/test.jpg"
// the function will print 
fn cameracapture()  -> Result<bool, String> {

    let window = "/tmp/test.jpg";
    let mut values = opencv::types::VectorOfint::new();
    try!(highgui::named_window(window,1));
    let mut cam = try!(highgui::VideoCapture::device(1));
    let opened = try!(highgui::VideoCapture::is_opened(&cam));
    if ! opened {
        println!("Using different camera");
        cam = try!(highgui::VideoCapture::device(-1));
    }
    let mut frame = try!(core::Mat::new());	
    let resutlofinterogation = try!(cam.read(&mut frame));
    let result = opencv::highgui::imwrite(window, &mut frame , &mut values).unwrap();
    println!("True if the operations were succesful: {:?}", result);
     Ok(resutlofinterogation)
     
}

fn main(){
    println!("{:?}", cameracapture().unwrap());
}
