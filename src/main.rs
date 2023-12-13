// extern crate vlc;
use vlc::{Instance, Media, MediaPlayer};
use std::thread;
use fltk::{app, utils};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt};
use fltk::window::Window;

fn main() {
    println!("Hello, world!");
    println!("Version : {}", vlc::version());
    println!("Compiler : {}", vlc::compiler());
    // Create an instance
    let instance = Instance::new().unwrap();

    // Create a media from a file
    let md = Media::new_location(&instance, "http://ltxryfw.ynavc.com/upload/images/SITE_ltxryfw_site/202205/11/%E4%B9%A0%E8%BF%91%E5%B9%B3%EF%BC%9A%E5%9C%A8%E5%BA%86%E7%A5%9D%E4%B8%AD%E5%9B%BD%E5%85%B1%E4%BA%A7%E4%B8%BB%E4%B9%89%E9%9D%92%E5%B9%B4%E5%9B%A2%E6%88%90%E7%AB%8B100%E5%91%A8%E5%B9%B4%E5%A4%A7%E4%BC%9A%E4%B8%8A%E7%9A%84%E8%AE%B2%E8%AF%9D%EF%BC%88%E5%AE%9E%E5%86%B5%EF%BC%89.mp4").unwrap();
    // let md = Media::new_path(&instance, "path_to_a_media_file.ogg").unwrap();
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();
    //
    // // Wait for 10 seconds
    // thread::sleep(::std::time::Duration::from_secs(10));

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

    wind.end();
    wind.show();

    #[cfg(target_os = "linux")]
    mdp.set_xwindow(handle as u32);
    // For Windows
    #[cfg(target_os = "windows")]
    mdp.set_hwnd(handle);
    // For MacOS
    #[cfg(target_os = "macos")]
    mdp.set_nsobject(utils::content_view(&wind) as _);

    app.run().unwrap();
}