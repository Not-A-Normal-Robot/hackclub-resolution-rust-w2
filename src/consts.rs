use core::{ops::RangeInclusive, time::Duration};

pub static APP_NAME: &str = "20³";

pub(crate) const WINDOW_MIN_WIDTH: f32 = 256.0;
pub(crate) const WINDOW_MIN_HEIGHT: f32 = 144.0;

pub(crate) static TEXT_SETTINGS_MENU: &str = "Settings";
pub(crate) static TEXT_SETTINGS_DELAY: &str = "Break delay";
pub(crate) static TEXT_SETTINGS_DELAY_TOOLTIP: &str =
    "How long to wait between breaks.\nDefault: 00:20:00";
pub(crate) static TEXT_SETTINGS_LENGTH: &str = "Break length";
pub(crate) static TEXT_SETTINGS_LENGTH_TOOLTIP: &str =
    "How long each break should be.\nDefault: 00:00:20";
pub(crate) static TEXT_DURING_DELAY: &str = "until next break";
pub(crate) static TEXT_PRE_BREAK: &str = "It's time to rest your eyes";
pub(crate) static TEXT_PRE_BREAK_BUTTON: &str = "Begin break";
pub(crate) static TEXT_DURING_BREAK: &str = "Look at something 20 ft (6 m) away";

pub(crate) const DEFAULT_DELAY: Duration = Duration::from_secs(6);
pub(crate) const DEFAULT_LENGTH: Duration = Duration::from_secs(3);

pub(crate) const DELAY_RANGE: RangeInclusive<Duration> =
    Duration::from_mins(5)..=Duration::from_hours(5);
pub(crate) const DELAY_RANGE_SECS: RangeInclusive<u64> =
    DELAY_RANGE.start().as_secs()..=DELAY_RANGE.end().as_secs();
pub(crate) const LENGTH_RANGE: RangeInclusive<Duration> =
    Duration::from_secs(5)..=Duration::from_mins(10);
pub(crate) const LENGTH_RANGE_SECS: RangeInclusive<u64> =
    LENGTH_RANGE.start().as_secs()..=LENGTH_RANGE.end().as_secs();
