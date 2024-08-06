use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time};

pub fn native_progress(limit: u64, delay: u64, task_name: String) {
    println!("Running: {}", task_name);
    let bar = ProgressBar::new(limit);
    bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}%").unwrap(),
    );
    let _delay = time::Duration::from_millis(delay);
    for _ in 0..limit {
        bar.inc(1);
        thread::sleep(_delay);
    }
    bar.finish();
    println!("{} Done !", task_name);
}

pub fn manual_progress(iterations: i32, delay: u64) {
    let mut i = 1;
    let mut percentage = 0;
    let mut dots: String = "".to_owned();
    let _delay = time::Duration::from_millis(delay);
    while i <= iterations {
        dots.push_str("▮");
        percentage = ((i as f64 / iterations as f64) * 100.0) as i32;
        print!("\x1B[2J\x1B[1;1H");
        if (percentage == 100) {
            println!("{} {}% COMPLETED!", dots, percentage);
        } else {
            println!("{} {}%", dots, percentage);
        }
        thread::sleep(_delay);
        i = i + 1;
    }
}
