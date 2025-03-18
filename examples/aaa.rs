use std::str::FromStr;
use zvariant::Signature;

fn main() {
  let a = Signature::from_str("sv");
  println!("{:?}", a);
  let a = Signature::from_str("as");
  println!("{:?}", a);
}
