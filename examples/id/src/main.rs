use druid::{Druid, DruidV7};

fn main() {
    let id = Druid::new();
    println!("Druid:     {id}");
    println!("timestamp: {} ns", id.timestamp_nanos());

    let uuid_v7 = DruidV7::new();
    println!("UUIDv7:    {uuid_v7}");
    println!("timestamp: {} ms", uuid_v7.timestamp_millis());
}
