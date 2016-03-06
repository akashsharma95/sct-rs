extern crate libc;
extern crate x11;
use std::os::raw::c_uint;
use std::os::raw::c_void;
use x11::xlib;
use x11::xrandr;
use std::ptr::{
    null,
    null_mut,
};
use std::env;

struct WhitePoints {
    r: f64,
    g: f64,
    b: f64
}
const WHITEPOINTS: [WhitePoints; 20] = [
     WhitePoints { r: 1.00000000,  g: 0.18172716,  b: 0.00000000 }, /* 1000K */
     WhitePoints { r: 1.00000000,  g: 0.42322816,  b: 0.00000000 },
     WhitePoints { r: 1.00000000,  g: 0.54360078,  b: 0.08679949 },
     WhitePoints { r: 1.00000000,  g: 0.64373109,  b: 0.28819679 },
     WhitePoints { r: 1.00000000,  g: 0.71976951,  b: 0.42860152 },
     WhitePoints { r: 1.00000000,  g: 0.77987699,  b: 0.54642268 },
     WhitePoints { r: 1.00000000,  g: 0.82854786,  b: 0.64816570 },
     WhitePoints { r: 1.00000000,  g: 0.86860704,  b: 0.73688797 },
     WhitePoints { r: 1.00000000,  g: 0.90198230,  b: 0.81465502 },
     WhitePoints { r: 1.00000000,  g: 0.93853986,  b: 0.88130458 },
     WhitePoints { r: 1.00000000,  g: 0.97107439,  b: 0.94305985 },
     WhitePoints { r: 1.00000000,  g: 1.00000000,  b: 1.00000000 }, /* 6500K */
     WhitePoints { r: 0.95160805,  g: 0.96983355,  b: 1.00000000 },
     WhitePoints { r: 0.91194747,  g: 0.94470005,  b: 1.00000000 },
     WhitePoints { r: 0.87906581,  g: 0.92357340,  b: 1.00000000 },
     WhitePoints { r: 0.85139976,  g: 0.90559011,  b: 1.00000000 },
     WhitePoints { r: 0.82782969,  g: 0.89011714,  b: 1.00000000 },
     WhitePoints { r: 0.80753191,  g: 0.87667891,  b: 1.00000000 },
     WhitePoints { r: 0.78988728,  g: 0.86491137,  b: 1.00000000 }, /* 10000K */
     WhitePoints { r: 0.77442176,  g: 0.85453121,  b: 1.00000000 }
];

pub fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(null());
        if display == null_mut() {
            panic!("Not able to open display");
        }
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);
        let res: *mut xrandr::XRRScreenResources = xrandr::XRRGetScreenResourcesCurrent(display, root);
        let args: Vec<String> = env::args().collect();
        let mut temp: u32  = 6500;
        if args.len() > 1 {
            temp = args[1].parse()
                          .unwrap();
        }
        if temp < 1000 || temp > 10000 {
            temp = 6500;
        }
        let mut brightness: f64 = 1.0;
        if args.len() > 2 {
            brightness = args[2].parse().unwrap();
        }
        temp -= 1000;
        let ratio: f64 = (temp % 500) as f64 / 500f64;
        let gammar: f64 = brightness * (WHITEPOINTS[(temp/500) as usize].r * (1f64 - ratio) + WHITEPOINTS[(temp/500+1) as usize].r * ratio);
        let gammag: f64 = brightness * (WHITEPOINTS[(temp/500) as usize].g * (1f64 - ratio) + WHITEPOINTS[(temp/500+1) as usize].g * ratio);
        let gammab: f64 = brightness * (WHITEPOINTS[(temp/500) as usize].b * (1f64 - ratio) + WHITEPOINTS[(temp/500+1) as usize].b * ratio);
        // println!("{}", gammar);
        // println!("{}", gammag);
        // println!("{}", gammab);
        for c in 0..(*res).ncrtc {
            let crtcxid = (*res).crtcs; // res->crtcs[c];

            let size = xrandr::XRRGetCrtcGammaSize(display, *crtcxid); // crtcxid used here
            let crtc_gamma: *mut xrandr::XRRCrtcGamma = xrandr::XRRAllocGamma(size);
            for i in 0..size {
                let g: f64 = (65535f64 * i as f64) / size as f64;
                *(*crtc_gamma).red = (g * gammar) as u16; // crtc_gamma->red[i] = g * gammar;
                *(*crtc_gamma).green = (g * gammag) as u16; // crtc_gamma->green[i] = g * gammag;
                *(*crtc_gamma).blue = (g * gammab) as u16; // crtc_gamma->blue[i] = g * gammab;
            }
            xrandr::XRRSetCrtcGamma(display, *crtcxid, crtc_gamma); // crtcxid used here
            xlib::XFree(crtc_gamma as *mut c_void);
        }
    }
}
