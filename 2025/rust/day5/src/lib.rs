pub mod parser;

use std::iter::once;

pub struct Ranges {
    pub start_range: Vec<u64>,
    pub end_range: Vec<u64>,
    pub ids: Vec<u64>,
}

pub fn fix_ranges(
    start_range: &mut Vec<u64>,
    end_range: &mut Vec<u64>,
    new_start: u64,
    new_end: u64,
) {
    let insert_pos = start_range.partition_point(|&helper| helper < new_start);

    let mut left_merge_index = None;
    if insert_pos > 0 {
        let left = insert_pos - 1;
        if end_range[left] >= new_start.saturating_sub(1) {
            left_merge_index = Some(left);
        }
    }

    let mut right_merge_index = insert_pos;
    while right_merge_index < start_range.len()
        && start_range[right_merge_index] <= new_end.saturating_add(1)
    {
        right_merge_index += 1;
    }

    let merged_start = left_merge_index
        .map(|i| start_range[i])
        .unwrap_or(new_start)
        .min(new_start);

    let merged_end = {
        let mut end = new_end;
        if let Some(left) = left_merge_index {
            end = end.max(end_range[left]);
        }
        if right_merge_index > insert_pos {
            end = end.max(end_range[right_merge_index - 1]);
        }
        end.max(new_end)
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

pub fn is_fresh(start_range: &[u64], end_range: &[u64], id: u64) -> bool {
    let idx = start_range.partition_point(|&s| s <= id);
    if idx == 0 {
        return false;
    }

    let prev = idx - 1;

    id >= start_range[prev] && id <= end_range[prev]
}

fn fresh_count(start_range: &[u64], end_range: &[u64]) -> u64 {
    let mut total_fresh = 0;
    for i in 0..start_range.len() {
        total_fresh += end_range[i] - start_range[i] + 1;
    }
    total_fresh
}

pub fn how_many_fresh(data: &Ranges) -> u64 {
    data.ids
        .iter()
        .filter(|&&id| is_fresh(&data.start_range, &data.end_range, id))
        .count() as u64
}

pub fn how_many_fresh_max(data: &Ranges) -> u64 {
    fresh_count(&data.start_range, &data.end_range)
}
