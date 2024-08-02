use serde::Deserialize;
use serde_yaml::{from_yaml, parse_yaml};

#[derive(Deserialize, Debug)]
struct Test {
    i: i8,
    f: f32,
    foo: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let yaml = parse_yaml(
        "
    i: 9
    f: 8
    foo:
      - xxx
      - yyy
    ",
    )?;
    let t: Test = from_yaml(&yaml[0])?;
    println!("{t:#?}");
    Ok(())
}
