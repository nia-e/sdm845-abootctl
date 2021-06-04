extern crate clap;
use gpt;
use clap::{Arg, App};
use std::path::Path;
use std::iter::FromIterator;

fn main() {

    // CLI stuff
    let matches = App::new("abootctl")
        .version("0.1.1")
        .author("Caleb C., Aissa Z. B. <aissa.zenaida@pm.me>")
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
    //This is probably partly redundant but it works
    unsafe {

        let mut boot_y: u64 = 0;
        let mut boot_n: u64 = 0;

        if *slot as i32 == 0 {

            boot_y = *&partitions[10].1.flags;
            boot_n = *&partitions[38].1.flags;
            //Flags are read first even though they should never differ from baseline, just in case
            boot_y = enable_aboot(boot_y);
            boot_n = disable_aboot(boot_n);
            //Prepare changes to rewrite
            let a_flags = gpt::partition::PartitionAttributes::from_bits_unchecked(boot_y);
            let b_flags = gpt::partition::PartitionAttributes::from_bits_unchecked(boot_n);
        }
        else if *slot as i32 == 1 {

            //Same as above
            boot_y = *&partitions[38].1.flags;
            boot_n = *&partitions[10].1.flags;
            boot_y = enable_aboot(boot_y);
            boot_n = disable_aboot(boot_n);
            let a_flags = gpt::partition::PartitionAttributes::from_bits_unchecked(boot_n);
            let b_flags = gpt::partition::PartitionAttributes::from_bits_unchecked(boot_y);
        }

        //Rewrite changes to GPT table
        println!("active slot: {}; inactive slot: {}", boot_y, boot_n);
    }
}

fn enable_aboot(bootflags: u64) -> u64 {

    //Sets 5th bit to 1, sets active boot partition
    let mut t_bootflags = bootflags;
    t_bootflags |= 0b0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    //KEEPING THIS OFF, MAY BRICK IF ENABLED
    //bootflags &= 0b1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    return t_bootflags;
}

fn disable_aboot(bootflags: u64) -> u64 {

    //Sets 5th bit to 0, unsets active boot partition
    let mut t_bootflags = bootflags;
    t_bootflags &= 0b0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    return t_bootflags;
}
