pub fn set_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

const COLORS: &[u32] = &[
    0xFFFFFF, 0xFF0000, 0x00FF00, 0x8888FF, 0xFFFF00, 0xFF00FF, 0x00FFFF, 0xFF8800, 0x0088FF,
    0x8800FF,
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
