use std::env;
use chrono::{NaiveDate, Datelike, Duration};
use clap::{App, Arg};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("ERR:引数がひとつ必要です");
    }
    // コマンドライン引数の定義
    let matches = App::new("Create weekly link table")
        .version("1.0")
        .author("Kazumichi INOUE")
        .about("Create a table of weekly links.")
        .arg(
            Arg::with_name("start date of sunday")
                .index(1)
                .required(true) // ここで引数を必須に設定
                .help("Sets the start date directly after --"),
        )
        .get_matches();

    // 引数を日付としてパース
    let arg1 = matches.value_of("start date of sunday").unwrap();
    match NaiveDate::parse_from_str(arg1, "%Y-%m-%d") {
        Ok(parsed_date) => {
            // パース成功時の処理
            println!("パースされた日付: {:?}", parsed_date);

            let sun = parsed_date;
            let mon = sun + Duration::days(1);
            let tue = sun + Duration::days(2);
            let wed = sun + Duration::days(3);
            let thu = sun + Duration::days(4);
            let fri = sun + Duration::days(5);
            let sat = sun + Duration::days(6);

            let sun_date_link = format!("{}{:02}{:02}", sun.year() % 100, sun.month0() + 1, sun.day());
            let mon_date_link = format!("{}{:02}{:02}", mon.year() % 100, mon.month0() + 1, mon.day());
            let tue_date_link = format!("{}{:02}{:02}", tue.year() % 100, tue.month0() + 1, tue.day());
            let wed_date_link = format!("{}{:02}{:02}", wed.year() % 100, wed.month0() + 1, wed.day());
            let thu_date_link = format!("{}{:02}{:02}", thu.year() % 100, thu.month0() + 1, thu.day());
            let fri_date_link = format!("{}{:02}{:02}", fri.year() % 100, fri.month0() + 1, fri.day());
            let sat_date_link = format!("{}{:02}{:02}", sat.year() % 100, sat.month0() + 1, sat.day());

            println!("|Sun|Mon|Tue|Wed|Thu|Fri|Sat|\n|---|---|---|---|---|---|---|");
            println!("|[{}]({})|[{}]({})|[{}]({})|[{}]({})|[{}]({})|[{}]({})|[{}]({})|",
            sun.day(), sun_date_link,
            mon.day(), mon_date_link,
            tue.day(), tue_date_link,
            wed.day(), wed_date_link,
            thu.day(), thu_date_link,
            fri.day(), fri_date_link,
            sat.day(), sat_date_link);
        }
        Err(err) => {
            // パース失敗時のエラーハンドリング
            eprintln!("エラー: {}", err);
        }
    }
}
