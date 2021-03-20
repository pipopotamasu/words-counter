use std::io::BufRead;
use anyhow::Result;
use std::collections::HashMap;

pub fn run<R: BufRead>(reader: R) -> Result<()> {
  let mut counter: HashMap<String, u32> = HashMap::new();

  for line in reader.lines() {
      let l = line?;
      let word = l.split(",").collect::<Vec<&str>>()[0];

      let count = counter.entry(word.to_string()).or_insert(0);
      *count += 1;
  }

  println!("{:?}", counter);

  Ok(())
}