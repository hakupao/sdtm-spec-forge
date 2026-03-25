use rust_xlsxwriter::Format;
use rust_xlsxwriter::Color;
use rust_xlsxwriter::FormatBorder;
use rust_xlsxwriter::FormatAlign;

/// Header style: dark blue background, white bold text
pub fn header_format() -> Format {
    Format::new()
        .set_background_color(Color::RGB(0x003366))
        .set_font_color(Color::White)
        .set_bold()
        .set_font_size(10.0)
        .set_border(FormatBorder::Thin)
        .set_text_wrap()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

/// Normal cell style
pub fn cell_format() -> Format {
    Format::new()
        .set_font_size(10.0)
        .set_border(FormatBorder::Thin)
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
}

/// Custom variable highlight: light yellow background
pub fn custom_var_format() -> Format {
    Format::new()
        .set_background_color(Color::RGB(0xFFFFCC))
        .set_font_size(10.0)
        .set_border(FormatBorder::Thin)
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
}

/// Tree indent format with specific level
pub fn tree_indent_format(level: u8) -> Format {
    Format::new()
        .set_font_size(10.0)
        .set_indent(level)
        .set_border(FormatBorder::Thin)
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
}

/// Title format for cover page
pub fn title_format() -> Format {
    Format::new()
        .set_font_size(20.0)
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

/// Subtitle format
pub fn subtitle_format() -> Format {
    Format::new()
        .set_font_size(14.0)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

/// Section header within a sheet
pub fn section_header_format() -> Format {
    Format::new()
        .set_font_size(11.0)
        .set_bold()
        .set_background_color(Color::RGB(0xD9E2F3))
        .set_border(FormatBorder::Thin)
}

/// Info label on cover page
pub fn info_label_format() -> Format {
    Format::new()
        .set_font_size(11.0)
        .set_bold()
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
}

/// Info value on cover page
pub fn info_value_format() -> Format {
    Format::new()
        .set_font_size(11.0)
        .set_align(FormatAlign::Left)
        .set_align(FormatAlign::VerticalCenter)
}

/// Info label on cover page with borders (for B7:C10)
pub fn info_label_bordered_format() -> Format {
    Format::new()
        .set_font_size(11.0)
        .set_bold()
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_border(FormatBorder::Thin)
}

/// Info value on cover page with borders (for B7:C10)
pub fn info_value_bordered_format() -> Format {
    Format::new()
        .set_font_size(11.0)
        .set_align(FormatAlign::Left)
        .set_align(FormatAlign::VerticalCenter)
        .set_border(FormatBorder::Thin)
}
