extern crate clap;
use gpt;
use clap::{Arg, App};
use std::path::Path;
use std::process;

fn main() {

    // CLI stuff
    let matches = App::new("abootctl")
        .version("0.2")
        .author("Caleb C., Aissa Z. B. <aissa.zenaida@pm.me>")
        .about("Switch active bootloader slot on SDM845 OnePlus devices. THIS MAY BRICK YOUR DEVICE - USE WITH CARE")
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

    //Open relevant GPT stuff
    let disk_path = Path::new("/dev/sde");
    let size = gpt::disk::LogicalBlockSize::Lb4096;

    let header = gpt::header::read_header(disk_path, size).unwrap();
    let mut partitions = gpt::partition::read_partitions(disk_path, &header, size).unwrap();
    let mut disk = gpt::disk::read_disk(&disk_path).unwrap();
    //println!("{}", partitions[&10]);

    //Find relevant partitions
    //Flags are read first even though they should never differ from baseline, just in case
    let mut boot_a_flags = *&partitions[&11].flags;
    let mut boot_b_flags = *&partitions[&39].flags;
    let mut success: bool = false;

    if *slot as i32 == 0 {

        boot_a_flags = enable_aboot(boot_a_flags);
        boot_b_flags = disable_aboot(boot_b_flags);
        success = true;
    }
    else if *slot as i32 == 1 {

        //Same as above
        boot_b_flags = enable_aboot(boot_b_flags);
        boot_a_flags = disable_aboot(boot_a_flags);
        success = true;
    }
    //else { a_flags = false; b_flags = false; }

    //Check flags have been updated; sanity
    //Rewrite changes to GPT table
    if success {

        let mut new_boot_a = partitions[&11].clone();
        let mut new_boot_b = partitions[&39].clone();

        new_boot_a.flags = boot_a_flags;
        new_boot_b.flags = boot_b_flags;
        partitions.insert(11, new_boot_a);
        partitions.insert(39, new_boot_b);
        disk.update_partitions(partitions).unwrap();
        disk.write().unwrap();
    }

    else { eprintln!("Error: could not read partition table headers or invalid slot number specified"); process::exit(1); }
    //println!("active slot: {}; inactive slot: {}", boot_y, boot_n);
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
