use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Route {
    Simple,
    Animated,
    Built,
    Rounded,
    Bar,
    FillGradient,
}

impl From<String> for Route {
    fn from(path: String) -> Self {
        match path.as_str() {
            "/simple" => Self::Simple,
            "/animated" => Self::Animated,
            "/built" => Self::Built,
            "/rounded" => Self::Rounded,
            "/bar" => Self::Bar,
            "/fill_gradient" => Self::FillGradient,
            _ => Self::Animated,
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple => write!(f, "/simple"),
            Self::Animated => write!(f, "/animated"),
            Self::Built => write!(f, "/built"),
            Self::Rounded => write!(f, "/rounded"),
            Self::Bar => write!(f, "/bar"),
            Self::FillGradient => write!(f, "/fill_gradient"),
        }
    }
}
