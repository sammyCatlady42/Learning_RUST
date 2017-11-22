fn main() {
    let years = [1900, 1997, 1833, 2016, 1996, 2000, 1004];

    for i in 0..years.len() {
        match is_leap_year(years[i]) {
            true => println!("{}*", years[i]),
            false => println!("{}", years[i]),
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}
