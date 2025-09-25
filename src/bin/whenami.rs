use chrono::prelude::*;


fn main() {
    let date:String = Utc::now().to_string();
    let (year, month, day) = process_date(date);
    println!("{} {}, {}", month, day, year);
}

fn process_date(date: String) -> (String, String, String) { // Year, Month, Day
    // get year
    let mut year: String = String::new();
    for i in 0..4 {
        year = year + &date.chars().nth(i).expect("Error").to_string();
    }
    // get month
    let mut month: String = String::new();
    for i in 5..7 {
        month = month + &date.chars().nth(i).expect("Error").to_string();
    }
    month = match &month as &str {
        "01" => "January".to_string(),
        "02" => "February".to_string(),
        "03" => "March".to_string(),
        "04" => "April".to_string(),
        "05" => "May".to_string(),
        "06" => "June".to_string(),
        "07" => "July".to_string(),
        "08" => "August".to_string(),
        "09" => "September".to_string(),
        "10" => "October".to_string(),
        "11" => "November".to_string(),
        "12" => "December".to_string(),
        &_ => "Error".to_string(),
    };
    // get day
    let mut day: String = String::new();
    for i in 8..10 {
        day = day + &date.chars().nth(i).expect("Error").to_string();
    }
    let day = match &day.chars().nth(1).expect("Error").to_string() as &str {
        "1" => day + "st",
        "2" => day + "nd",
        "3" => day + "rd",
        _ => day + "th",
    };

    (year, month, day)
}
