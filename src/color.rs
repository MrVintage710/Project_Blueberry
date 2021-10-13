#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub const CLEAR : Color = Color(0, 0, 0, 0);

    pub fn blend(&mut self, color : Color) {
        //Blending A over B where A is the other color and B is current Color
        let alpha_a : f64 = color.3 as f64 / 255.0;
        let alpha_b : f64 = self.3 as f64 / 255.0;
        let alpha = alpha_a + alpha_b * (1.0 - alpha_a);

        let r : u8 = ((color.0 as f64 * alpha_a + self.0 as f64 * alpha_b * (1.0 - alpha_a)) / alpha) as u8;
        let g : u8 = ((color.1 as f64 * alpha_a + self.1 as f64 * alpha_b * (1.0 - alpha_a)) / alpha) as u8;
        let b : u8 = ((color.2 as f64 * alpha_a + self.2 as f64 * alpha_b * (1.0 - alpha_a)) / alpha) as u8;
        let a = alpha_a + alpha_b * (1.0 - alpha_a);


        self.0 = r;
        self.1 = g;
        self.2 = b;
        self.0 = (alpha * 255.0) as u8;
    }
}