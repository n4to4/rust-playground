use spinach::Spinach;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    basic();
    creating();
    updating();
}

fn basic() {
    let s = Spinach::new("Running task 1");
    sleep(Duration::from_secs(1));
    s.text("Running task 2");
    sleep(Duration::from_secs(1));
    s.succeed(Some("Ran tasks successfully"));
}

fn creating() {
    use spinach::{Color, Spinner};

    let s = Spinach::new("custom text");
    sleep(Duration::from_secs(2));
    s.succeed(Some("done"));

    let spinner = Spinner::new(vec!["▮", "▯"], 80);
    let s = Spinach::new_with(Some(spinner), Some("custom text"), Some(Color::Red));
    sleep(Duration::from_secs(2));
    s.succeed(Some("done"));

    let s = Spinach::new_with(None, Some("custom text"), Some(Color::Green));
    sleep(Duration::from_secs(2));
    s.succeed(Some("done"));
}

fn updating() {
    use spinach::Color;
    let s = Spinach::new("updating");

    s.text("updating");
    sleep(Duration::from_secs(2));
    s.color(Color::White);
    sleep(Duration::from_secs(2));
    s.update_with(Some("new text"), Some(Color::Red));
    sleep(Duration::from_secs(2));
    s.update_with(None, Some(Color::Red));
    s.succeed(Some("done"));
}
