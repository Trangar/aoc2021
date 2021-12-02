#[macro_export]
macro_rules! input_numbered_lines {
    ($file:expr) => {{
        let lines = include_str!(concat!("../../../input/", $file));
        lines
            .split('\n')
            .filter_map(|l| l.parse().ok())
            .collect::<Vec<_>>()
    }};
}

#[macro_export]
macro_rules! input_command_lines {
    ($file:expr) => {{
        let lines = include_str!(concat!("../../../input/", $file));
        lines
            .split('\n')
            .filter_map(Command::from_str)
            .collect::<Vec<_>>()
    }};
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    pub fn from_str(s: &str) -> Option<Self> {
        let mut split = s.split(' ');
        match (
            split.next(),
            split.next().and_then(|v| v.parse::<u32>().ok()),
        ) {
            (Some("forward"), Some(num)) => return Some(Self::Forward(num)),
            (Some("down"), Some(num)) => return Some(Self::Down(num)),
            (Some("up"), Some(num)) => return Some(Self::Up(num)),
            _ => {}
        }

        None
    }
}
