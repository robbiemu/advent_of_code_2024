#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

pub type ProblemDefinition = String;
pub type Consequent = usize;


#[cfg(not(feature = "part2"))]
fn get_digit(byte: u8, idx: usize) -> Result<usize, String> {
  if byte.is_ascii_digit() {
    Ok((byte - b'0') as usize)
  } else {
    Err(format!(
      "Invalid character '{}' at index {}.",
      byte as char, idx
    ))
  }
}

#[cfg(not(feature = "part2"))]
fn get_checksum(
  file_index: usize,
  space_index: usize,
  space_count: usize,
) -> usize {
  // Preserve the exact original implementation
  if space_count == 0 {
    return 0;
  }

  file_index * space_count * space_index
    + file_index * (space_count * (space_count - 1)) / 2
}

#[cfg(test)]
#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}
#[cfg(not(test))]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

pub mod prelude {
  #[cfg(not(feature = "part2"))]
  use std::collections::VecDeque;

  #[cfg(not(feature = "part2"))]
  use crate::{get_checksum, get_digit};
  use crate::{src_provider, Consequent, ProblemDefinition};

  pub fn extract() -> Result<ProblemDefinition, String> {
    Ok(src_provider()?.lines().map(String::from).collect())
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    #[cfg(not(feature = "part2"))]
    {
      let chs = data.as_bytes();
      let mut checksum: usize = 0;
      let mut spaces: VecDeque<usize> = VecDeque::new();
      let mut compacted_position: usize = 0;
      let mut left_index = 1;
      let mut right_index = data.len() - 1;

      if right_index % 2 == 1 {
        right_index -= 1;
      }

      while left_index < right_index {
        let mut used_space = 0;
        let mut available_space = 0;

        let right_value = right_index / 2;
        let mut right_count = get_digit(chs[right_index], right_index)?;

        while right_count > 0 && right_index > left_index {
          if spaces.is_empty() {
            let file_from_left_index = left_index - 1;
            let file_from_left_value = file_from_left_index / 2;
            let file_from_left_count =
              get_digit(chs[file_from_left_index], file_from_left_index)?;
            checksum += get_checksum(
              file_from_left_value,
              compacted_position,
              file_from_left_count,
            );
            compacted_position += file_from_left_count;

            let count = get_digit(chs[left_index], left_index)?;
            spaces.push_back(count);
            left_index += 2;
          }
          let Some(space_count) = spaces.pop_front() else {
            unreachable!();
          };
          available_space = space_count;
          used_space = right_count.min(space_count);
          checksum += get_checksum(right_value, compacted_position, used_space);

          compacted_position += used_space;
          right_count -= used_space;
        }

        if right_count > 0 {
          // Take care of remaining unmoved right-file values in the inner loop
          checksum +=
            get_checksum(right_value, compacted_position, right_count);
          compacted_position += right_count;
        } else if used_space < available_space {
          spaces.push_front(available_space - used_space);
        }

        right_index -= 2;
      }

      return Ok(checksum);
    }
    // part 2 is much simpler, 3 simple passes
    #[cfg(feature = "part2")]
    {
      let mut files = Vec::new();
      let mut spaces = Vec::new();
      let mut current_index = 0;

      let bytes = data.as_bytes();
      for (i, &ch) in bytes.iter().enumerate() {
        let count = (ch - b'0') as usize;

        if i % 2 == 0 {
          // Files
          if count > 0 {
            files.push((files.len(), count, current_index));
          }
        } else {
          // Free spaces
          if count > 0 {
            spaces.push((count, current_index));
          }
        }

        current_index += count;
      }

      files.sort_by(|a, b| b.0.cmp(&a.0));

      for file in &mut files {
        let (file_id, file_size, mut file_start) = *file;

        if let Some((space_index, (space_size, space_start))) = spaces
          .iter_mut()
          .enumerate()
          .find(|(_, (space_size, space_start))| {
            *space_size >= file_size && *space_start < file_start
          })
        {
          // Move the file
          file_start = *space_start;

          // Update the space (reduce or remove it)
          if *space_size > file_size {
            spaces[space_index] =
              (*space_size - file_size, *space_start + file_size);
          } else {
            spaces.remove(space_index);
          }
        }

        // Update file start position in-place
        *file = (file_id, file_size, file_start);
      }

      let mut checksum = 0;
      for (file_id, file_size, file_start) in files {
        for offset in 0..file_size {
          checksum += file_id * (file_start + offset);
        }
      }

      Ok(checksum)
    }
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(checksum) => println!("Checksum: {}", checksum),
      Err(e) => println!("Error: {}", e),
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::prelude::*;

  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  fn test_transform() {
    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");
    assert_eq!(result, 1928);
  }
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[test]
  fn test_transform() {
    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");
    assert_eq!(result, 2858);
  }
}
