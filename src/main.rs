use log :: {info, warn};
mod vec3mod;
use crate :: vec3mod :: Vec3; 

fn main() {
    /*

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

    */

    let mut vec1: Vec3 = Vec3 :: new();
    vec1.e[0] = 1.237;
    vec1.e[1] = 17.893;
    vec1.e[2] = -14.487;

    let mut vec2: Vec3 = Vec3 :: new();

    vec2.e[0] = 2.424;
    vec2.e[1] = -3.643;
    vec2.e[2] = 27.149;

    let mut vec3: Vec3 = Vec3 :: new();

    vec3 = vec1 + vec2;

    println!("vec1: {} {} {}", vec1.x(), vec1.y(), vec1.z());
    println!("vec2: {} {} {}", vec2.x(), vec2.y(), vec2.z());
    println!("Sum: ");
    println!("vec3: {} {} {}", vec3.x(), vec3.y(), vec3.z());
    vec3 = vec1 - vec2;
    println!("Sum: ");
    println!("vec3: {} {} {}", vec3.x(), vec3.y(), vec3.z());
    vec3 = vec1 * vec2;
    println!("vec3: {} {} {}", vec3.x(), vec3.y(), vec3.z());
    vec3 = vec1 * 2.0;
    println!("vec3: {} {} {}", vec3.x(), vec3.y(), vec3.z());
    vec3 = vec1 / 2.0;
    println!("vec3: {} {} {}", vec3.x(), vec3.y(), vec3.z());
}




