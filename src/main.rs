//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the builder with an imported glade file

// BELOW the smippest of READ fuction
  // pub fn read(&mut self, image:& ::core::Mat) -> Result<bool,String> {
  //   unsafe {
  //     let rv = ::sys::cv_highgui_cv_VideoCapture_read_Mat_image(self.as_raw_VideoCapture(), image.as_raw_Mat());
  //     if rv.error_msg as i32 != 0i32 {
  //         let v = CStr::from_ptr(rv.error_msg).to_bytes().to_vec();
  //         ::libc::free(rv.error_msg as *mut c_void);
  //         return Err(String::from_utf8(v).unwrap())
  //     }
  //     Ok(rv.result)
  //   }
  // }

extern crate opencv;

use opencv::highgui;
use opencv::core;
//use std::fs::File;
//use std::io::prelude::*;



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
    println!("{:?}", result);
     Ok(resutlofinterogation)
     
			}

fn main(){



    println!("{:?}", cameracapture().unwrap());

	
}
