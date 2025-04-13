
pub struct Theme {
    pub palette: Palette,
}

pub struct Palette {
    pub primary: Swatch,
    pub secondary: Swatch,
    pub danger: Swatch,
    pub error: Swatch,
    pub info: Swatch,
    pub text: Swatch,
}

pub struct Swatch {
    pub dark: &'static str,
    pub main: &'static str,
    pub light: &'static str,
    pub contrast: &'static str,
}

pub static LIGHT_THEME: Theme = Theme {
    palette: Palette {
        primary: Swatch {
            dark: "#0D47A1",
            main: "#2196F3",
            light: "#BBDEFB",
            contrast: "#FFFFFF",
        },
        secondary: Swatch {
            dark: "#004D40",
            main: "#009688",
            light: "#B2DFDB",
            contrast: "#FFFFFF",
        },
        danger: Swatch {
            dark: "#B71C1C",
            main: "#F44336",
            light: "#FFCDD2",
            contrast: "#FFFFFF",
        },
        error: Swatch {
            dark: "#C62828",
            main: "#E53935",
            light: "#FFCDD2",
            contrast: "#FFFFFF",
        },
        info: Swatch {
            dark: "#01579B",
            main: "#03A9F4",
            light: "#B3E5FC",
            contrast: "#FFFFFF",
        },
        text: Swatch {
            dark: "#212121",
            main: "#333333",
            light: "#757575",
            contrast: "#FFFFFF",
        },
    },
};
