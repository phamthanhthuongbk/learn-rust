use chrono::{DateTime, Utc};


pub fn main() {
    // Chuỗi datetime cần chuyển đổi
    let datetime_str = "1905-01-01T00:00:00Z";
    
    // Chuyển đổi chuỗi sang đối tượng DateTime<Utc>
    let datetime_utc = datetime_str.parse::<DateTime<Utc>>().unwrap();
    
    // In đối tượng DateTime<Utc> ra màn hình
    println!("{:?}", datetime_utc);
}
