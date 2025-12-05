use std::fs::File;
use std::iter::once;
use std::io::{self, BufReader, BufRead};

fn fix_ranges(start_range: &mut Vec<u64>, end_range: &mut Vec<u64>, new_start: u64, new_end: u64) {
    let insert_pos = start_range.partition_point(|&helper| helper < new_start);

    let mut left_merge_index = None;
    if insert_pos > 0 {
        let left = insert_pos - 1;
        if end_range[left] >= new_start {
            left_merge_index = Some(left);
        }
    }

    let mut right_merge_index = insert_pos;
    while right_merge_index < start_range.len() && start_range[right_merge_index] <= new_end {
        right_merge_index += 1;
    }

    let merged_start = left_merge_index
        .map(|i| start_range[i])
        .unwrap_or(new_start);

    let merged_end = {
        let mut end = new_end;
        if let Some(left) = left_merge_index {
            end = end.max(end_range[left]);
        }
        if right_merge_index > insert_pos {
            end = end.max(end_range[right_merge_index - 1]);
        }
        end
    };

    let remove_from = left_merge_index.unwrap_or(insert_pos);
    let remove_to = right_merge_index;

    if remove_from == remove_to {
        start_range.insert(remove_from, merged_start);
        end_range.insert(remove_from, merged_end);
    } else {
        start_range.splice(remove_from..remove_to, once(merged_start));
        end_range.splice(remove_from..remove_to, once(merged_end));
    }
}


fn is_fresh(start_range: &[u64], end_range: &[u64], id: u64) -> bool {
    let idx = start_range.partition_point(|&s| s < id);
    if idx == 0 { return false; }
    
    let prev = idx - 1;

    id >= start_range[prev] && id <= end_range[prev]
}


fn fresh_count(start_range: &[u64], end_range: &[u64]) -> u64 {
    let mut fresh = 0;
    for i in 0..start_range.len() { fresh += end_range[i] - start_range[i] + 1; }

    fresh
}


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut start_range: Vec<u64> = Vec::new();
    let mut end_range: Vec<u64> = Vec::new();

    let mut bool_range = true;

    let mut ids: Vec<u64> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?.trim().to_string();

        if line.is_empty() {
            bool_range = false;
            continue;
        }

        if bool_range {
            if let Some((start, end)) = line.split_once('-') {
                if let (Ok(s), Ok(e)) = (start.parse::<u64>(), end.parse::<u64>()) {
                    fix_ranges(&mut start_range, &mut end_range, s, e);
                }
            }
        } else {
            if let Ok(id) = line.parse::<u64>() {
                ids.push(id);
            }
        }
    }

    // Part 1
    let mut answer1 = 0;
    for id in &ids {
        if is_fresh(&start_range, &end_range, *id) { answer1 += 1; }
    }

    println!("{}", answer1);

    // Part 2
    let answer2 = fresh_count(&start_range, &end_range);

    println!("{}", answer2);
    

    Ok(())
}
