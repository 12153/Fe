use crate::theme::Theme;

pub trait Style {
    fn style(&self, theme: &Theme) -> String;
}

// example style implementation 
pub struct Button;

impl Style for Button {
    fn style(&self, theme: &Theme) -> String {
        format! {
            "backgroundColor: {}",
            theme.palette.secondary.main,
        }
    }
}
