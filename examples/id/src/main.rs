use druid::*;
use rand::{seq::SliceRandom, thread_rng};
use std::hint::black_box;
fn main() {
    let mut uids = Vec::new();
    for _ in 0..10 {
        uids.push(black_box(Druid::default().to_hex()));
    }
    let org = uids.clone();
    let mut rng = thread_rng();
    uids.shuffle(&mut rng);
    uids.sort_by(|a, b| {
        let timestamp_a = &a[0..16];
        let timestamp_b = &b[0..16];
        let ts_a = u64::from_str_radix(timestamp_a, 16).unwrap_or(0);
        let ts_b = u64::from_str_radix(timestamp_b, 16).unwrap_or(0);
        println!("{}", ts_a - ts_b);
        ts_a.cmp(&ts_b)
    });
    // println!("{:#?}", org);
    // println!("{:#?}", uids);
    assert!(uids == org);
}
