pub struct Color {
    pub e: [f32; 3],
}

impl Color {
    pub fn new() -> Self {
        Self { e: [0.0; 3] }
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }
    
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    
    pub fn b(&self) -> f32 {
        self.e[2]
    }
}

impl Color {
    pub fn write_color (pixel: &Color) {
        let r: f32 = pixel.r();
        let g: f32 = pixel.g();
        let b: f32 = pixel.b();

        let ir: i32 = (255.999 * r) as i32;
        let ig: i32 = (255.999 * g) as i32;
        let ib: i32 = (255.999 * b) as i32;
        
        println!("{} {} {}\n", ir, ig, ib);
    }
}

