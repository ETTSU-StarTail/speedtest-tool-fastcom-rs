use chrono::Timelike;

/// value prefix(k, M, G, T)
pub enum ValuePrefix {
    k,
    M,
    G,
    T,
}
impl ValuePrefix {
    fn to_exponent(&self) -> i32 {
        match self {
            ValuePrefix::k => 3,
            ValuePrefix::M => 6,
            ValuePrefix::G => 9,
            ValuePrefix::T => 12,
        }
    }
}

pub fn bits_to_byte(value_bits: f64) -> f64 {
    value_bits / f64::from(8)
}

pub fn change_order(value: f64, value_prefix: ValuePrefix) -> f64 {
    value / f64::from(10).powi(value_prefix.to_exponent())
}

pub fn clear_order(value: f64, units: &str) -> f64 {
    match units.to_uppercase().as_str() {
        "K" => value * f64::from(10).powi(ValuePrefix::k.to_exponent()),
        "M" => value * f64::from(10).powi(ValuePrefix::M.to_exponent()),
        "G" => value * f64::from(10).powi(ValuePrefix::G.to_exponent()),
        "T" => value * f64::from(10).powi(ValuePrefix::T.to_exponent()),
        _ => value,
    }
}

/// - 20:23:34 -> 20:20:00<br>
/// - 20:28:23 -> 20:25:00
pub fn round_datetime(dt: chrono::DateTime<chrono::Local>) -> chrono::DateTime<chrono::Local> {
    let minute: i64 = i64::from(dt.minute());
    let second: i64 = i64::from(dt.second());
    let minute_ones_place: i64 = minute % 10;

    if 5 < minute_ones_place {
        dt - chrono::Duration::minutes(minute_ones_place - 5) - chrono::Duration::seconds(second)
    } else if minute_ones_place != 5 {
        dt - chrono::Duration::minutes(minute_ones_place) - chrono::Duration::seconds(second)
    } else {
        dt
    }
}
