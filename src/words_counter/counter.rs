use std::io::BufRead;
use anyhow::Result;
use std::collections::HashMap;

pub fn run<R: BufRead>(mut reader: R, skip_header: bool) -> Result<()> {
  let mut counter: HashMap<String, u32> = HashMap::new();

  if skip_header {
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap_err();
  }

  for line in reader.lines() {
      let l = line?;
      let word = l.split(",").collect::<Vec<&str>>()[0];

      let count = counter.entry(word.to_string()).or_insert(0);
      *count += 1;
  }

  println!("{:?}", counter);

  Ok(())
}
