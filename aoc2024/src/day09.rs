use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut files = 0;
    let mut file = true;
    let mut blocks = Vec::new();
    let mut intervals = Vec::new();
    for c in input.chars() {
        let count = c.to_digit(10).unwrap();
        let block = if file {
            let block = Some(files);
            intervals.push((block, count));
            files += 1;
            block
        } else {
            intervals.push((None, count));
            None
        };
        for _ in 0..count {
            blocks.push(block);
        }
        file = !file;
    }

    let mut i = 0;
    let mut j = blocks.len() - 1;
    loop {
        while blocks[i].is_some() {
            i += 1;
        }
        while blocks[j].is_none() {
            j -= 1;
        }

        if i >= j {
            break;
        }

        blocks[i] = blocks[j];
        blocks[j] = None;
    }

    let mut part1 = 0;
    for (i, &block) in blocks.iter().enumerate() {
        if let Some(id) = block {
            part1 += i as u64 * id as u64;
        } else {
            break;
        }
    }

    let mut j = intervals.len() - 1;
    while j != 0 {
        let (id, count) = intervals[j];
        if let Some(id) = id {
            for i in 0..j {
                let (test_id, test_count) = intervals[i];
                if test_id.is_some() || test_count < count {
                    continue;
                }

                intervals[j] = (None, count);
                intervals[i] = (Some(id), count);
                if test_count - count > 0 {
                    intervals.insert(i + 1, (None, test_count - count));
                }

                break;
            }
        }
        j -= 1;
    }

    let mut part2 = 0;
    let mut i = 0;
    for &(id, count) in intervals.iter() {
        if let Some(id) = id {
            for j in 0..count {
                part2 += (i + j) as u64 * id as u64;
            }
        }
        i += count;
    }

    Solution::from((part1, part2))
}
