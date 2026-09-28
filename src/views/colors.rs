pub enum Color {
    Ink,
    Blue,
    Butter,
    Raspberry,
    Bg,
    Surface,
    Fg,
    Line,
    Transparent,
}

impl Default for Color {
    fn default() -> Self {
        Self::Bg
    }
}

impl Color {
    pub fn shadow3(self) -> &'static str {
        match self {
            Self::Ink => "shadow-ink-3",
            Self::Blue => "shadow-blue-3",
            Self::Butter => "shadow-butter-3",
            Self::Raspberry => "shadow-raspberry-3",
            _ => "",
        }
    }

    pub fn shadow5(self) -> &'static str {
        match self {
            Self::Ink => "shadow-ink-5",
            Self::Blue => "shadow-blue-5",
            Self::Butter => "shadow-butter-5",
            Self::Raspberry => "shadow-raspberry-5",
            _ => "",
        }
    }

    pub fn bg(self) -> &'static str {
        match self {
            Self::Ink => "bg-ink",
            Self::Blue => "bg-blue-500",
            Self::Butter => "bg-butter-500",
            Self::Raspberry => "bg-raspberry-500",
            Self::Bg => "bg-bg",
            Self::Surface => "bg-surface",
            Self::Fg => "bg-fg",
            Self::Line => "bg-line",
            Self::Transparent => "bg-transparent",
        }
    }

    pub fn btn_press(self) -> &'static str {
        match self {
            Self::Ink => "btn-press-ink",
            Self::Blue => "btn-press-blue-500",
            Self::Butter => "btn-press-butter-500",
            Self::Raspberry => "btn-press-raspberry-500",
            Self::Bg => "btn-press-bg",
            Self::Surface => "btn-press-surface",
            Self::Fg => "btn-press-fg",
            Self::Line => "btn-press-line",
            Self::Transparent => "btn-press-transparent",
        }
    }
}
