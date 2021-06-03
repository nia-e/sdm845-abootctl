extern crate clap;
use gpt;
use clap::{Arg, App};
use std::path::Path;
use std::iter::FromIterator;

fn main() {

    // CLI stuff
    let matches = App::new("abootctl")
        .version("0.1")
        .author("Aissa Z. B. <aissa.zenaida@pm.me>")
        .about("Switch active bootloader slot on SDM845 OnePlus devices")
        .arg(Arg::with_name("SLOT")
            .help("Slot to set as active (0 or 1)")
            .required(true)
            .index(1))
        .get_matches();
    
    let slot = matches.value_of("SLOT").unwrap().parse::<i32>().unwrap();
    
    set_slot(&slot);
    // println!("Invalid SLOT number; see bootctl --help");
}

fn set_slot(slot: &i32) {
    
    let disk_path = Path::new("/dev/sde");
    let size = gpt::disk::LogicalBlockSize::Lb4096;

    let header = gpt::header::read_header(disk_path, size).unwrap();
    let partitions_btm = gpt::partition::read_partitions(disk_path, &header, size).unwrap();
    //Change btreemap to vector
    let partitions = Vec::from_iter(partitions_btm);
    //Find relevant partitions
    let boot_a = &partitions[10].1;
    let boot_b = &partitions[38].1;

    println!("boot_a: {}; boot_b: {}", boot_a.flags, boot_b.flags);
}
