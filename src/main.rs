use log :: {info, warn};
mod vec3mod;

fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;
    
    print!("P3\n{} {}\n255\n", image_height, image_width);

    for j in 0..image_height {
        // info!("Scanlines remaining: {}\n", image_height - j);
        for i in 0.. image_width {
            let r: f32 = (i as f32) / ((image_width - 1) as f32);
            let g: f32 = (j as f32) / ((image_height - 1) as f32);
            let b: f32 = 0.0;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;
            
            /*
            if j < 5 && i < 5 {
                print!("{} {} {} {} {} {}\n", r, g, b, ir, ig, ib);
            }
            */

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}


