use chrono::{Datelike, Timelike, Weekday};

fn main() {
    let next_meeting = get_meeting_type();
    println!("Next meeting is a {}.", next_meeting);
}

fn get_meeting_type() -> &'static str {
    let current_time = chrono::offset::Local::now();

    let _weekday = current_time.date().weekday();
    let _hour = current_time.hour();
    let _minute = current_time.minute();

    let _is_less_than_930 = _hour <= 9 && _minute <= 30;
    let _is_less_than_1430 = _hour <= 14 && _minute <= 30;

    match _weekday {
        _ if _weekday == Weekday::Tue || _weekday == Weekday::Thu => {
            if _is_less_than_930 {
                "Textual Standup"
            } else {
                "Verbal Standup"
            }
        }
        _ => {
            if _is_less_than_930 {
                "Verbal Standup"
            } else if _is_less_than_1430 {
                "Fika"
            } else {
                "Textual Standup"
            }
        }
    }
}
