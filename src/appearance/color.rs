pub use nannou::color::named::*;
use std::marker::PhantomData;

// use nannou::color::LinSrgba;
use nannou::color::Rgb;
pub use nannou::color::Rgba as Color;

pub const DEFAULT_FILL_COLOR: Color = RED_D;
pub const DEFAULT_STROKE_COLOR: Color = Color {
    color: Rgb {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const DARK_BLUE: Color = Color {
    color: Rgb {
        red: 35.0 / 255.0,
        green: 17.0 / 255.0,
        blue: 142.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const DARK_BROWN: Color = Color {
    color: Rgb {
        red: 139.0 / 255.0,
        green: 69.0 / 255.0,
        blue: 19.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const LIGHT_BROWN: Color = Color {
    color: Rgb {
        red: 205.0 / 255.0,
        green: 133.0 / 255.0,
        blue: 63.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const BLUE_E: Color = Color {
    color: Rgb {
        red: 28.0 / 255.0,
        green: 117.0 / 255.0,
        blue: 138.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const BLUE_D: Color = Color {
    color: Rgb {
        red: 41.0 / 255.0,
        green: 171.0 / 255.0,
        blue: 202.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const BLUE_C: Color = Color {
    color: Rgb {
        red: 88.0 / 255.0,
        green: 196.0 / 255.0,
        blue: 221.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const BLUE_B: Rgb = Rgb {
    red: 156.0 / 255.0,
    green: 220.0 / 255.0,
    blue: 235.0 / 255.0,
    standard: PhantomData,
};
pub const BLUE_A: Color = Color {
    color: Rgb {
        red: 199.0 / 255.0,
        green: 233.0 / 255.0,
        blue: 241.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const TEAL_E: Color = Color {
    color: Rgb {
        red: 73.0 / 255.0,
        green: 168.0 / 255.0,
        blue: 143.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const TEAL_D: Color = Color {
    color: Rgb {
        red: 85.0 / 255.0,
        green: 193.0 / 255.0,
        blue: 167.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const TEAL_C: Color = Color {
    color: Rgb {
        red: 92.0 / 255.0,
        green: 208.0 / 255.0,
        blue: 179.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const TEAL_B: Color = Color {
    color: Rgb {
        red: 118.0 / 255.0,
        green: 221.0 / 255.0,
        blue: 192.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const TEAL_A: Color = Color {
    color: Rgb {
        red: 172.0 / 255.0,
        green: 234.0 / 255.0,
        blue: 215.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GREEN_E: Color = Color {
    color: Rgb {
        red: 105.0 / 255.0,
        green: 156.0 / 255.0,
        blue: 82.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GREEN_D: Color = Color {
    color: Rgb {
        red: 119.0 / 255.0,
        green: 176.0 / 255.0,
        blue: 93.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GREEN_C: Color = Color {
    color: Rgb {
        red: 131.0 / 255.0,
        green: 193.0 / 255.0,
        blue: 103.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GREEN_B: Color = Color {
    color: Rgb {
        red: 166.0 / 255.0,
        green: 207.0 / 255.0,
        blue: 140.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GREEN_A: Color = Color {
    color: Rgb {
        red: 201.0 / 255.0,
        green: 226.0 / 255.0,
        blue: 174.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const YELLOW_E: Color = Color {
    color: Rgb {
        red: 232.0 / 255.0,
        green: 193.0 / 255.0,
        blue: 28.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const YELLOW_D: Color = Color {
    color: Rgb {
        red: 244.0 / 255.0,
        green: 211.0 / 255.0,
        blue: 69.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const YELLOW_C: Color = Color {
    color: Rgb {
        red: 255.0 / 255.0,
        green: 255.0 / 255.0,
        blue: 0.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const YELLOW_B: Color = Color {
    color: Rgb {
        red: 255.0 / 255.0,
        green: 234.0 / 255.0,
        blue: 148.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const YELLOW_A: Color = Color {
    color: Rgb {
        red: 255.0 / 255.0,
        green: 241.0 / 255.0,
        blue: 182.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GOLD_E: Color = Color {
    color: Rgb {
        red: 199.0 / 255.0,
        green: 141.0 / 255.0,
        blue: 70.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GOLD_D: Color = Color {
    color: Rgb {
        red: 225.0 / 255.0,
        green: 161.0 / 255.0,
        blue: 88.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GOLD_C: Color = Color {
    color: Rgb {
        red: 240.0 / 255.0,
        green: 172.0 / 255.0,
        blue: 95.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GOLD_B: Color = Color {
    color: Rgb {
        red: 249.0 / 255.0,
        green: 183.0 / 255.0,
        blue: 117.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GOLD_A: Color = Color {
    color: Rgb {
        red: 247.0 / 255.0,
        green: 199.0 / 255.0,
        blue: 151.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const RED_E: Color = Color {
    color: Rgb {
        red: 207.0 / 255.0,
        green: 80.0 / 255.0,
        blue: 68.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const RED_D: Color = Color {
    color: Rgb {
        red: 230.0 / 255.0,
        green: 90.0 / 255.0,
        blue: 76.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const RED_C: Color = Color {
    color: Rgb {
        red: 252.0 / 255.0,
        green: 98.0 / 255.0,
        blue: 85.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const RED_B: Color = Color {
    color: Rgb {
        red: 255.0 / 255.0,
        green: 128.0 / 255.0,
        blue: 128.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const RED_A: Color = Color {
    color: Rgb {
        red: 247.0 / 255.0,
        green: 161.0 / 255.0,
        blue: 163.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const MAROON_E: Color = Color {
    color: Rgb {
        red: 148.0 / 255.0,
        green: 66.0 / 255.0,
        blue: 79.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const MAROON_D: Color = Color {
    color: Rgb {
        red: 162.0 / 255.0,
        green: 77.0 / 255.0,
        blue: 97.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const MAROON_C: Color = Color {
    color: Rgb {
        red: 197.0 / 255.0,
        green: 95.0 / 255.0,
        blue: 115.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const MAROON_B: Color = Color {
    color: Rgb {
        red: 236.0 / 255.0,
        green: 147.0 / 255.0,
        blue: 171.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const MAROON_A: Color = Color {
    color: Rgb {
        red: 236.0 / 255.0,
        green: 171.0 / 255.0,
        blue: 193.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const PURPLE_E: Color = Color {
    color: Rgb {
        red: 100.0 / 255.0,
        green: 65.0 / 255.0,
        blue: 114.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const PURPLE_D: Color = Color {
    color: Rgb {
        red: 113.0 / 255.0,
        green: 85.0 / 255.0,
        blue: 130.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const PURPLE_C: Color = Color {
    color: Rgb {
        red: 154.0 / 255.0,
        green: 114.0 / 255.0,
        blue: 172.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const PURPLE_B: Color = Color {
    color: Rgb {
        red: 177.0 / 255.0,
        green: 137.0 / 255.0,
        blue: 198.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const PURPLE_A: Color = Color {
    color: Rgb {
        red: 202.0 / 255.0,
        green: 163.0 / 255.0,
        blue: 232.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const WHITE: Color = Color {
    color: Rgb {
        red: 255.0 / 255.0,
        green: 255.0 / 255.0,
        blue: 255.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const BLACK: Color = Color {
    color: Rgb {
        red: 0.0 / 255.0,
        green: 0.0 / 255.0,
        blue: 0.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const LIGHT_GRAY: Color = Color {
    color: Rgb {
        red: 187.0 / 255.0,
        green: 187.0 / 255.0,
        blue: 187.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const LIGHT_GREY: Color = Color {
    color: Rgb {
        red: 187.0 / 255.0,
        green: 187.0 / 255.0,
        blue: 187.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GRAY: Color = Color {
    color: Rgb {
        red: 136.0 / 255.0,
        green: 136.0 / 255.0,
        blue: 136.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GREY: Color = Color {
    color: Rgb {
        red: 136.0 / 255.0,
        green: 136.0 / 255.0,
        blue: 136.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const DARK_GREY: Color = Color {
    color: Rgb {
        red: 68.0 / 255.0,
        green: 68.0 / 255.0,
        blue: 68.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const DARK_GRAY: Color = Color {
    color: Rgb {
        red: 68.0 / 255.0,
        green: 68.0 / 255.0,
        blue: 68.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const DARKER_GREY: Color = Color {
    color: Rgb {
        red: 34.0 / 255.0,
        green: 34.0 / 255.0,
        blue: 34.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const DARKER_GRAY: Color = Color {
    color: Rgb {
        red: 34.0 / 255.0,
        green: 34.0 / 255.0,
        blue: 34.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GREY_BROWN: Color = Color {
    color: Rgb {
        red: 115.0 / 255.0,
        green: 99.0 / 255.0,
        blue: 87.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const PINK: Color = Color {
    color: Rgb {
        red: 209.0 / 255.0,
        green: 71.0 / 255.0,
        blue: 189.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const LIGHT_PINK: Color = Color {
    color: Rgb {
        red: 220.0 / 255.0,
        green: 117.0 / 255.0,
        blue: 205.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const GREEN_SCREEN: Color = Color {
    color: Rgb {
        red: 0.0 / 255.0,
        green: 255.0 / 255.0,
        blue: 0.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
pub const ORANGE: Color = Color {
    color: Rgb {
        red: 255.0 / 255.0,
        green: 134.0 / 255.0,
        blue: 47.0 / 255.0,
        standard: PhantomData,
    },
    alpha: 1.0,
};
