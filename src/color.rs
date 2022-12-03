/// Color struct with rgba values from 0 to 1.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        }
    }
}

impl AsRef<[f32; 4]> for Color {
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Color as *const [f32; 4]) }
    }
}

impl Color {
    pub fn rgb(r: u32, g: u32, b: u32) -> Self {
        Self {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: 1.,
        }
    }

    /// Converts a hexidecimal number rgb or rgba to a color struct.
    pub fn from_hex(hex: u32) -> Self {
        let mut color = Self::default();
        if hex <= 0xffffff {
            color.r = ((hex >> 16) & 0xff) as f32 / 255.;
            color.g = ((hex >> 8) & 0xff) as f32 / 255.;
            color.b = (hex & 0xff) as f32 / 255.;
            color.a = 1.;
        } else {
            color.r = ((hex >> 24) & 0xff) as f32 / 255.;
            color.g = ((hex >> 16) & 0xff) as f32 / 255.;
            color.b = ((hex >> 8) & 0xff) as f32 / 255.;
            color.a = (hex & 0xff) as f32 / 255.;
        }
        color
    }
}
