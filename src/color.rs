#[cfg(test)]
mod tests {
    use color::Color;
    use color;

    #[test]
    #[ignore]
    fn colorcode() {
        assert_eq!(color::colorcode(Color::Blue, Color::BrightMagenta), 0xD1);
        assert_eq!(color::colorcode(Color::Yellow, Color::Red), 0x4E);
        assert_eq!(color::colorcode(Color::DarkGray, Color::White), 0xF8);
    }

}
