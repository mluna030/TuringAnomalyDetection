use crate::domain::packet::{Packet, PacketHeader};
use crate::domain::rule::{Action, Filter};
use chrono::{Datelike, Local, NaiveTime, Weekday};

pub struct TimeWindowRule {
    name: String,
    windows: Vec<TimeWindow>,
    action: Action,

    priority: i32,
}
#[derive(Debug, Clone)]
pub struct TimeWindow {
    start_time: NaiveTime,
    end_time: NaiveTime,
    days: Vec<Weekday>,
    timezone: Option<String>,
}

impl TimeWindow {
    pub fn new(start_hour: u32, start_min: u32, end_hour: u32, end_min: u32) -> Self {
        Self {
            start_time: NaiveTime::from_hms_opt(start_hour, start_min, 0).unwrap(),
            end_time: NaiveTime::from_hms_opt(end_hour, end_min, 0).unwrap(),
            days: Vec::new(),  // Empty = all days
            timezone: None,    // Use local timezone
        }
    }
    pub fn on_days(mut self, days: Vec<Weekday>) -> Self {
        self.days = days;
        self
    }
    pub fn weekdays(self) -> Self {
        self.on_days(vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
        ])
    }
    pub fn weekends(self) -> Self {
        self.on_days(vec![Weekday::Sat, Weekday::Sun])
    }
    pub fn is_now_in_window(&self) -> bool {
        let now = Local::now();
        let current_time = now.time();
        let current_day = now.weekday();

        // Check day of week (if specified)
        if !self.days.is_empty() && !self.days.contains(&current_day) {
            return false;
        }
        if self.start_time <= self.end_time {
            // Normal case: 09:00 - 17:00
            current_time >= self.start_time && current_time <= self.end_time
        } else {
            // Spans midnight: 22:00 - 02:00
            current_time >= self.start_time || current_time <= self.end_time
        }
    }
}

impl TimeWindowRule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            windows: Vec::new(),
            action: Action::Block,
            priority: 60,
        }
    }
    pub fn add_window(mut self, window: TimeWindow) -> Self {
        self.windows.push(window);
        self
    }
    pub fn add_windows(mut self, windows: Vec<TimeWindow>) -> Self {
        self.windows.extend(windows);
        self
    }
    pub fn during_business_hours(self) -> Self {
        self.add_window(
            TimeWindow::new(9, 0, 17, 0).weekdays()
        )
    }
    pub fn during_night_hours(self) -> Self {
        self.add_window(
            TimeWindow::new(22, 0, 6, 0)
        )
    }
    pub fn with_action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    fn is_within_time_window(&self) -> bool {
        for window in &self.windows {
            if window.is_now_in_window() {
                return true;
            }
        }
        false
    }
}

impl Filter for TimeWindowRule {
    fn quick_match(&self, _header: &PacketHeader) -> bool {
        self.is_within_time_window()
    }
    fn check_packet(&self, _packet: &Packet) -> Option<Action> {
        Some(self.action)
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn priority(&self) -> i32 {
        self.priority
    }
}
pub struct RateLimitByTimeRule {
    name: String,
    limits: Vec<(TimeWindow, u32)>,
    default_limit: u32,
    priority: i32,
}

impl RateLimitByTimeRule {
    pub fn new(name: impl Into<String>, default_limit: u32) -> Self {
        Self {
            name: name.into(),
            limits: Vec::new(),
            default_limit,
            priority: 50,
        }
    }

    pub fn add_limit(mut self, window: TimeWindow, limit: u32) -> Self {
        self.limits.push((window, limit));
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn current_limit(&self) -> u32 {
        for (window, limit) in &self.limits {
            if window.is_now_in_window() {
                return *limit;
            }
        }
        self.default_limit
    }
}

impl Filter for RateLimitByTimeRule {
    fn quick_match(&self, _header: &PacketHeader) -> bool {
        true
    }

    fn check_packet(&self, _packet: &Packet) -> Option<Action> {
        let _current_limit = self.current_limit();
        None
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}