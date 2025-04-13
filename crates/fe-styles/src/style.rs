use crate::theme::Theme;

pub trait Style {
    fn style(&self, theme: &Theme) -> String;
}

struct Button;

impl Style for Button {
    fn style(&self, _theme: &Theme) -> String {
        let m: String = String::new();

        return m
    }
}
