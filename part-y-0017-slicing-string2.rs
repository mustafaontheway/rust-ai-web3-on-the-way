fn main() {

    let proverb = String::from("Out of sight, out of mind!");

    let s1 = &proverb[..3];

    let s2 = &proverb[0..=2];

    assert!(s1 == s2);

    let s3 = proverb.as_str();

    assert!(proverb == s3);
 
}

