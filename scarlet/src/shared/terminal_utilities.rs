pub fn set_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

// Tweaked resistor color code.
const COLORS: &[u32] = &[
    0x884400, 0xFF0000, 0xFF8800, 0xFFFF00, 0x00FF00, 0x00DDDD, 0x0088FF, 0xDD00DD, 0x888888,
    0xFFFFFF,
];

pub fn set_color_index(index: usize) -> String {
    let color = COLORS[index % COLORS.len()];
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;
    set_color(r as _, g as _, b as _)
}

pub fn reset_color() -> &'static str {
    "\x1b[0m"
}
