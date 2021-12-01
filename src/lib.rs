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
