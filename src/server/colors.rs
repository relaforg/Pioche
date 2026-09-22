pub enum Color {
    Ink,
    Blue,
    Butter,
    Raspberry,
    Bg,
    Surface,
    Fg,
    Line,
}

impl Color {
    pub fn shadow3(self) -> &'static str {
        match self {
            Self::Ink => "shadow-ink-3",
            Self::Blue => "shadow-blue-3",
            Self::Butter => "shadow-butter-3",
            Self::Raspberry => "shadow-rapsberry-3",
            _ => "",
        }
    }
}
