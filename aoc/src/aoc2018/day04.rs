use crate::{error, Solution};
use chrono::{NaiveDateTime, Timelike};
use pest::Parser;
use std::collections::HashMap;
use std::str::FromStr;

pub fn run(input: &str) -> error::Result<Solution> {
    let guard_events = parse(input);

    if guard_events.is_empty() {
        Err(error::Error::msg(&"No guard events parsed from input"))?
    }

    let grouped_guard_events = group_event_by_guard(&guard_events);
    let sleepy = find_sleepy_guard_minute_hash(&grouped_guard_events);
    let consistent_sleeper = find_consistent_sleepy_guard(&grouped_guard_events);

    Ok(Solution {
        part_one: (sleepy.id * sleepy.ideal_minute).to_string(),
        part_two: (consistent_sleeper.0 * consistent_sleeper.1).to_string(),
    })
}

fn parse(input: &str) -> Vec<GuardEvent> {
    let mut v = input
        .lines()
        .filter_map(|line| GuardEvent::from_str(line.trim()).ok())
        .collect::<Vec<GuardEvent>>();

    v.sort_by(|a, b| a.date.cmp(&b.date));

    v
}

type GroupedGuardEvents<'a> = HashMap<u32, Vec<&'a GuardEvent>>;

fn find_sleepy_guard_minute_hash(grouped_guard_events: &GroupedGuardEvents) -> SleepyGuard {
    sleeper_stats(&grouped_guard_events)
        .into_iter()
        .max_by(|a, b| a.minutes_alseep.cmp(&b.minutes_alseep))
        .unwrap()
}

fn sleeper_stats(grouped_guard_events: &GroupedGuardEvents) -> Vec<SleepyGuard> {
    grouped_guard_events
        .iter()
        .map(|(&id, events)| {
            let mut minute_blocks: HashMap<u32, u32> = HashMap::new();
            let mut last_event: Option<&GuardEvent> = None;
            let mut minutes_alseep = 0;

            for event in events.iter().filter(|event| event.date.hour() == 0) {
                if let Some(last_event) = last_event {
                    if let GuardEventKind::FallingAsleep = last_event.kind {
                        if last_event.date.date() == event.date.date() {
                            let time_elasped = event.date.minute() - last_event.date.minute();
                            minutes_alseep += time_elasped;

                            for minute in last_event.date.minute()..event.date.minute() {
                                let count = minute_blocks.entry(minute).or_insert(0);
                                *count += 1;
                            }
                        } else {
                            let time_elasped = 59 - last_event.date.minute();
                            minutes_alseep += time_elasped;

                            for minute in last_event.date.minute()..60 {
                                let count = minute_blocks.entry(minute).or_insert(0);
                                *count += 1;
                            }
                        }
                    }
                }

                last_event = Some(event);
            }

            let ideal_minute = minute_blocks
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap_or((&0, &0));

            SleepyGuard {
                id: id,
                minutes_alseep: minutes_alseep,
                ideal_minute: *ideal_minute.0,
                minute_blocks: minute_blocks,
            }
        })
        .collect()
}

fn find_consistent_sleepy_guard(grouped_guard_events: &GroupedGuardEvents) -> (u32, u32) {
    let sleppy_guards = sleeper_stats(&grouped_guard_events);

    let best_guard = sleppy_guards
        .iter()
        .max_by(|a, b| {
            let a = a
                .minute_blocks
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap_or((&0, &0));

            let b = b
                .minute_blocks
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap_or((&0, &0));

            a.1.cmp(b.1)
        })
        .unwrap();

    let minute = best_guard
        .minute_blocks
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    (best_guard.id, *minute.0)
}

fn group_event_by_guard(events: &[GuardEvent]) -> GroupedGuardEvents<'_> {
    let mut map = HashMap::new();
    let mut tracker = 0;

    for event in events {
        if let GuardEventKind::BeginShift { guard_id } = event.kind {
            tracker = guard_id;
        }

        let v = map.entry(tracker).or_insert_with(Vec::new);

        v.push(event);
    }

    map
}

#[derive(Debug)]
struct GuardEvent {
    date: NaiveDateTime,
    kind: GuardEventKind,
}

#[derive(Debug)]
enum GuardEventKind {
    Waking,
    FallingAsleep,
    BeginShift { guard_id: u32 },
}

struct SleepyGuard {
    id: u32,
    minutes_alseep: u32,
    ideal_minute: u32,
    minute_blocks: HashMap<u32, u32>,
}

impl FromStr for GuardEvent {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<GuardEvent, error::Error> {
        let mut pair = parser::Day04Parser::parse(parser::Rule::event, s)?
            .next()
            .ok_or_else(|| error::Error::msg(&"No guard events in input"))?
            .into_inner();

        let date_string = pair.next().expect("should be impossible").as_str();

        let date = NaiveDateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M")?;

        let event_pair = pair.next().expect("should be impossible");

        let event = match event_pair.as_rule() {
            parser::Rule::begin_shift => GuardEventKind::BeginShift {
                guard_id: event_pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| error::Error::msg(&"No guard events in input"))?
                    .as_str()
                    .parse()?,
            },
            parser::Rule::wakes_up => GuardEventKind::Waking,
            parser::Rule::falls_asleep => GuardEventKind::FallingAsleep,
            _ => Err(error::Error::msg(&"Invalid guard event"))?,
        };

        Ok(GuardEvent {
            date: date,
            kind: event,
        })
    }
}

mod parser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "aoc2018/day04.pest"]
    pub struct Day04Parser;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn original_example() {
        let input = r#"
        [1518-11-01 00:00] Guard #10 begins shift
        [1518-11-01 00:05] falls asleep
        [1518-11-01 00:25] wakes up
        [1518-11-01 00:30] falls asleep
        [1518-11-01 00:55] wakes up
        [1518-11-01 23:58] Guard #99 begins shift
        [1518-11-02 00:40] falls asleep
        [1518-11-02 00:50] wakes up
        [1518-11-03 00:05] Guard #10 begins shift
        [1518-11-03 00:24] falls asleep
        [1518-11-03 00:29] wakes up
        [1518-11-04 00:02] Guard #99 begins shift
        [1518-11-04 00:36] falls asleep
        [1518-11-04 00:46] wakes up
        [1518-11-05 00:03] Guard #99 begins shift
        [1518-11-05 00:45] falls asleep
        [1518-11-05 00:55] wakes up
        "#;

        let guard_events = parse(&input);
        assert_eq!(guard_events.len(), 17);

        let grouped_guard_events = group_event_by_guard(&guard_events);
        assert_eq!(grouped_guard_events.keys().len(), 2);

        let sleepy = find_sleepy_guard_minute_hash(&grouped_guard_events);
        assert_eq!(sleepy.id, 10);
        assert_eq!(sleepy.ideal_minute, 24);

        let consistent_sleepy = find_consistent_sleepy_guard(&grouped_guard_events);
        assert_eq!(consistent_sleepy.0, 99);
        assert_eq!(consistent_sleepy.1, 45);
    }
}
