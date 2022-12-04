use advent_of_code_2022::core::get_lines;

#[derive(Clone, Debug)]
struct Segment {
    start: u16,
    end: u16,
}

impl Segment {
    pub fn contains(first: &Segment, second: &Segment) -> bool {
        (first.start <= second.start && first.end >= second.end)
            || (second.start <= first.start && second.end >= first.end)
    }

    pub fn overlaps(first: &Segment, second: &Segment) -> bool {
        (first.start <= second.start && first.end >= second.start)
            || (second.start <= first.start && second.end >= first.start)
    }
}

impl TryFrom<&str> for Segment {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lengths: Vec<String> = String::from(value).split("-").map(String::from).collect();

        match lengths.len() {
            2 => Ok(Segment {
                start: lengths[0].parse().map_err(|_| "Failed to convert")?,
                end: lengths[1].parse().map_err(|_| "Failed to convert")?,
            }),
            _ => Err("Invalid length for segment"),
        }
    }
}

#[derive(Clone, Debug)]
struct SegmentPair(Segment, Segment);

impl TryFrom<String> for SegmentPair {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split: Vec<Segment> = value
            .split(",")
            .map(Segment::try_from)
            .collect::<Result<Vec<Segment>, &'static str>>()?;

        match split.len() {
            2 => Ok(SegmentPair(split[0].clone(), split[1].clone())),
            _ => Err("Unexpected length for pair"),
        }
    }
}

impl SegmentPair {
    pub fn contains(&self) -> bool {
        Segment::contains(&self.0, &self.1)
    }

    pub fn overlaps(&self) -> bool {
        Segment::overlaps(&self.0, &self.1)
    }
}

fn part_one(input: &[SegmentPair]) -> usize {
    input
        .iter()
        .map(|segment_pair| segment_pair.contains() as usize)
        .sum()
}

fn part_two(input: &[SegmentPair]) -> usize {
    input
        .iter()
        .map(|segment_pair| segment_pair.overlaps() as usize)
        .sum()
}

fn get_input() -> Vec<SegmentPair> {
    get_lines("day-4")
        .expect("Couldn't get data")
        .into_iter()
        .map(SegmentPair::try_from)
        .collect::<Result<Vec<SegmentPair>, &'static str>>()
        .expect("")
}

fn main() -> std::io::Result<()> {
    let pairs = get_input();

    println!("Part One: {}", part_one(&pairs));
    println!("Part Two: {}", part_two(&pairs));

    Ok(())
}

#[cfg(test)]
mod day_4_tests {
    use crate::*;

    const TEST_INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn example_one() {
        let lines = TEST_INPUT
            .lines()
            .map(|x| SegmentPair::try_from(String::from(x)))
            .collect::<Result<Vec<SegmentPair>, &'static str>>()
            .expect("");

        assert_eq!(part_one(&lines), 2);
    }

    #[test]
    fn solution_one() {
        let lines = get_input();

        assert_eq!(part_one(&lines), 464)
    }

    #[test]
    fn example_two() {
        let lines = TEST_INPUT
            .lines()
            .map(|x| SegmentPair::try_from(String::from(x)))
            .collect::<Result<Vec<SegmentPair>, &'static str>>()
            .expect("");

        assert_eq!(part_two(&lines), 4);
    }
}
