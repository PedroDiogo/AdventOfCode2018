extern crate chrono;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use chrono::prelude::*;
use regex::Regex;

struct GuardSleepInterval {
    id: u32,
    start: u32,
    end: u32
}

struct GuardMostTimesAsleep {
    id: u32,
    times: u32,
    minute: u32
}

#[derive(Debug)]
enum GuardEvent {
    BeginShift(u32),
    WakesUp,
    FallsAsleep,
}

struct TimedGuardEvent {
    date: chrono::DateTime<Utc>,
    event: GuardEvent,
}

fn main() {
    let file_name = get_input_file_name();
    let inputs = read_input_file(file_name);
    let mut timed_guard_events = parse_inputs_to_timed_guard_events(inputs.lines());
    timed_guard_events.sort_unstable_by_key(|event| event.date);

    let guards_sleep_intervals = transform_inputs_to_guard_sleep_intervals(timed_guard_events);
    let guards = guards_sleep_intervals.iter().fold(HashSet::new(), |mut acc, interval| {acc.insert(interval.id); return acc});

    let mut guards_most_times_asleep = Vec::new();
    for guard in guards {
        let sleep_intervals_for_guard_with_most_sleep = get_sleep_intervals_for_guard(guard, &guards_sleep_intervals);
        let most_likely_minute_asleep = get_most_likely_minute_asleep(sleep_intervals_for_guard_with_most_sleep);    
        guards_most_times_asleep.push(GuardMostTimesAsleep {id: guard, minute: most_likely_minute_asleep.0, times: most_likely_minute_asleep.1});
    }

    let guard_most_times_asleep_on_minute = guards_most_times_asleep.into_iter().max_by_key(|guard_most_times_asleep| guard_most_times_asleep.times).unwrap();
    println!("Result = {}", guard_most_times_asleep_on_minute.id * guard_most_times_asleep_on_minute.minute);
}

// Boilerplate code
fn get_input_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.len() {
        1 => filename = String::from("input"),
        _ => filename = args[1].clone(),
    }
    return filename;
}

fn read_input_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("file not found");
    let mut all_inputs = String::new();
    file.read_to_string(&mut all_inputs)
        .expect("something went wrong reading the file");

    return all_inputs;
}

// Problem Code
fn transform_inputs_to_guard_sleep_intervals(timed_guard_events: Vec<TimedGuardEvent>) -> Vec<GuardSleepInterval> {
    let mut guard_sleep_intervals = Vec::new();

    let mut guard_id = 0;
    let mut start_sleep = 0;
    for timed_guard_event in timed_guard_events {
        match timed_guard_event.event {
            GuardEvent::BeginShift(id) => guard_id = id,
            GuardEvent::FallsAsleep => start_sleep = timed_guard_event.date.minute(),
            GuardEvent::WakesUp => guard_sleep_intervals.push(GuardSleepInterval{id: guard_id, start: start_sleep, end: timed_guard_event.date.minute()})
        }
    }

    return guard_sleep_intervals;
}

fn parse_inputs_to_timed_guard_events(inputs: std::str::Lines) -> Vec<TimedGuardEvent> {
    let mut timed_guard_events = Vec::new();

    let re_wakes_up = Regex::new(r"\[([^\]]*)\] wakes up").unwrap();
    let re_falls_asleep = Regex::new(r"\[([^\]]*)\] falls asleep").unwrap();
    let re_new_shift = Regex::new(r"\[([^\]]*)\] Guard #(\d*)").unwrap();

    for line in inputs {
        if re_wakes_up.is_match(line) {
            let captures = re_wakes_up.captures(line).unwrap();
            let date = &captures[1];
            let date = Utc.datetime_from_str(date, "%Y-%m-%d %H:%M").unwrap();

            timed_guard_events.push(TimedGuardEvent {date: date, event: GuardEvent::WakesUp});
        } else if re_falls_asleep.is_match(line) {
            let captures = re_falls_asleep.captures(line).unwrap();
            let date = &captures[1];
            let date = Utc.datetime_from_str(date, "%Y-%m-%d %H:%M").unwrap();

            timed_guard_events.push(TimedGuardEvent {date: date, event: GuardEvent::FallsAsleep});
        } else if re_new_shift.is_match(line) {
            let captures = re_new_shift.captures(line).unwrap();
            let date = &captures[1];
            let date = Utc.datetime_from_str(date, "%Y-%m-%d %H:%M").unwrap();
            let guard_id: u32 = captures[2].parse().unwrap();

            timed_guard_events.push(TimedGuardEvent {date: date, event: GuardEvent::BeginShift(guard_id)});
        }
    }

    return timed_guard_events;
}

fn get_sleep_intervals_for_guard(guard_id: u32, sleep_intervals: &Vec<GuardSleepInterval>) -> Vec<&GuardSleepInterval> {
    return sleep_intervals.into_iter().filter(|interval| interval.id == guard_id).collect();
}

fn get_most_likely_minute_asleep(sleep_intervals: Vec<&GuardSleepInterval>) -> (u32, u32) {
    let mut minutes_in_hour = [0; 60];

    for interval in sleep_intervals {
        for minute in interval.start..interval.end {
            minutes_in_hour[minute as usize] += 1;
        }
    }

    let most_times_asleep = minutes_in_hour.iter().max().unwrap().clone();
    let most_likely_minute_asleep = minutes_in_hour.iter().position(|&r| r == most_times_asleep).unwrap();

    return (most_likely_minute_asleep as u32, most_times_asleep);
}