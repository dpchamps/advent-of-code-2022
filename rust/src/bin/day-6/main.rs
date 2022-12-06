use std::collections::{HashMap, HashSet};
use advent_of_code_2022::core::get_data;

pub fn get_start_of_packet_marker(input: &str, marker_len: usize) -> Result<(usize, String), String> {
    let mut set = HashSet::new();
    let mut current_marker = String::default();

    for (idx, c) in input.chars().enumerate() {
        if current_marker.len() == marker_len {
            return Ok((idx, current_marker))
        }

        if set.contains(&c) {
            let idx = current_marker.find(c).expect("");
            let (removed, next) = current_marker.split_at(idx+1);

            for to_remove in removed.chars() {
                set.remove(&to_remove);
            }

            current_marker = next.into();
        }

        current_marker.push(c);
        set.insert(c);
    }

    Err("No valid SOP marker".to_string())
}

struct Device {
    signal: String
}

impl Device {}

fn main() -> std::io::Result<()> {
    let input = get_data("day-6")?;

    println!("Part one {}", get_start_of_packet_marker(&input, 4).expect("Couldn't solve part one").0);
    println!("Part two {}", get_start_of_packet_marker(&input, 14).expect("Couldn't solve part one").0);

    Ok(())
}

#[cfg(test)]
mod day_6_tests {
    use crate::get_start_of_packet_marker;

    #[test]
    fn examples_part_one(){
        assert_eq!(
            get_start_of_packet_marker(
                "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                4
            ),
            Ok((7, "jpqm".into()))
        );

        assert_eq!(
            get_start_of_packet_marker(
                "bvwbjplbgvbhsrlpgdmjqwftvncz",
                4
            ),
            Ok((5, "vwbj".into()))
        );

        assert_eq!(
            get_start_of_packet_marker(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                4
            ),
            Ok((10, "rfnt".into()))
        );

        assert_eq!(
            get_start_of_packet_marker(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                4
            ),
            Ok((11, "zqfr".into()))
        );


    }

    #[test]
    fn examples_part_two(){
        assert_eq!(
            get_start_of_packet_marker(
                "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                14
            ),
            Ok((19, "qmgbljsphdztnv".into()))
        );

        assert_eq!(
            get_start_of_packet_marker(
                "bvwbjplbgvbhsrlpgdmjqwftvncz",
                14
            ),
            Ok((23, "vbhsrlpgdmjqwf".into()))
        );

        assert_eq!(
            get_start_of_packet_marker(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                14
            ),
            Ok((29, "wmzdfjlvtqnbhc".into()))
        );

        assert_eq!(
            get_start_of_packet_marker(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                14
            ),
            Ok((26, "jwzlrfnpqdbhtm".into()))
        );


    }
}