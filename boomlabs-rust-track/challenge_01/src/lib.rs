#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub enum Color {
    RGB { red: u8, green: u8, blue: u8 },
    HSV { hue: u16, saturation: u8, value: u8 },
}

impl Color {
    /// Constructs a new value from variant `RGB` with the given red, green, and blue values.
    ///
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color::RGB { red, green, blue }
    }

    /// Constructs a new value from variant `HSV` with the given values.
    ///
    /// In case hue is above 360 ​​or saturation or value is above 100, we expect you to `panic!` with
    /// whatever message you choose.
    ///
    pub fn new_hsv(hue: u16, saturation: u8, value: u8) -> Color {
        if hue > 360 || saturation > 100 || value > 100 {
            panic!("hue, saturation, and value must be between 0 and 100");
        }
        Color::HSV {
            hue,
            saturation,
            value,
        }
    }
}

impl Color {
    /// If `self` is `RGB`, then you return its `red`, `green`, `blue` components in that order.
    /// Otherwise, `panic!` with whatever message you choose.
    ///
    pub fn unwrap_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::RGB { red, green, blue } => (*red, *green, *blue),
            _ => panic!("Not RGB"),
        }
    }

    /// If `self` is `HSV` then you return its `hue`, `saturation`, `value` components in this
    /// row. Otherwise, `panic!` with whatever message you choose.
    ///
    pub fn unwrap_hsv(&self) -> (u16, u8, u8) {
        match self {
            Color::HSV {
                hue,
                saturation,
                value,
            } => (*hue, *saturation, *value),
            _ => panic!("Not HSV"),
        }
    }
}

impl Color {
    /// In case the variant of `self` is `RGB`, we expect a string with the content `#rrggbb`, where
    /// the red, green, and blue components are formatted in hexadecimal, and each is
    /// exactly two lowercase (zero-padded) characters.
    ///
    /// If the variant is `HSV`, we expect a string `hsv(h,s%,v%)` where the numbers are typed in
    /// the decimal system, no leading zeros, no spaces after the commas, the second two ending in
    /// `%`.
    ///
    pub fn to_string(&self) -> String {
        match self {
            Color::RGB { red, green, blue } => format!("#{:02x}{:02x}{:02x}", red, green, blue),
            Color::HSV {
                hue,
                saturation,
                value,
            } => format!("hsv({},{}%,{}%)", hue, saturation, value),
        }
    }
}

impl Color {
    /// Inverts color componentwise -- for each of the values, the difference with the maximum is taken.
    ///
    pub fn invert(&self) -> Self {
        match self {
            Color::RGB { red, green, blue } => Color::RGB {
                red: 255 - red,
                green: 255 - green,
                blue: 255 - blue,
            },
            Color::HSV {
                hue,
                saturation,
                value,
            } => Color::HSV {
                hue: 360 - hue,
                saturation: 100 - saturation,
                value: 100 - value,
            },
        }
    }
}
