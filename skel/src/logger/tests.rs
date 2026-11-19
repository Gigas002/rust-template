use super::*;

#[test]
fn levels_map_in_order() {
    let levels = [
        LogLevel::Off,
        LogLevel::Error,
        LogLevel::Warn,
        LogLevel::Info,
        LogLevel::Debug,
        LogLevel::Trace,
    ];
    let filters: Vec<LevelFilter> = levels.into_iter().map(level_filter).collect();
    assert!(
        filters.windows(2).all(|pair| pair[0] < pair[1]),
        "{filters:?}"
    );
}
