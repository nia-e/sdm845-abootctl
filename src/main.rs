use gpt;
use clap::{Arg, App};
use std::path::Path;
use std::process;

//Global constants
const BOOT_A_PARTNUM: u32 = 11;
const BOOT_B_PARTNUM: u32 = 39;
const DEVICE_PATH: &str = "/dev/sde";

fn main() {

    // CLI stuff
    let matches = App::new("abootctl")
        .version("0.2.0")
        .author("Caleb C. <caleb@connolly.tech>, Aissa Z. B. <aissa.zenaida@pm.me>")
        .about("Switch active bootloader slot on SDM845 OnePlus devices. THIS MAY BRICK YOUR DEVICE - USE WITH CARE")
        .arg(Arg::with_name("SLOT")
            .help("Slot to set as active (0 or 1)")
            .required(true)
            .index(1))
        .arg(Arg::with_name("readonly")
            .short("r")
            .help("Reads value of boot partition headers without changing it"))
        .get_matches();

    //TODO: read bootable flag option
    let slot = matches.value_of("SLOT").unwrap().parse::<i32>().unwrap();
    let readonly: bool;
    match matches.occurrences_of("r") {
        0 => readonly = false,
        1 => readonly = true,
        _ => {eprintln!("This should never trigger. What have you done, you monster?"); process::exit(1)},
    }

    set_slot(&slot, readonly);
}

fn set_slot(slot: &i32, readonly: bool) {

    //Open relevant GPT stuff
    let disk_path = Path::new(DEVICE_PATH);
    let size = gpt::disk::LogicalBlockSize::Lb4096;

    let header = gpt::header::read_header(disk_path, size).unwrap();
    let mut partitions = gpt::partition::read_partitions(disk_path, &header, size).unwrap();
    let mut disk = gpt::disk::read_disk(&disk_path).unwrap();

    //Find relevant partitions
    //Flags are read first even though they should never differ from baseline, just in case
    let mut boot_a_flags = *&partitions[&BOOT_A_PARTNUM].flags;
    let mut boot_b_flags = *&partitions[&BOOT_B_PARTNUM].flags;

    if *slot as i32 == 0 {

        //Change _a and _b boot partition flags
        boot_a_flags = enable_aboot(boot_a_flags);
        boot_b_flags = disable_aboot(boot_b_flags);
    }
    else if *slot as i32 == 1 {

        //Same as above
        boot_b_flags = enable_aboot(boot_b_flags);
        boot_a_flags = disable_aboot(boot_a_flags);
    }
    else { eprintln!("Error: could not read partition table headers or invalid slot number specified"); process::exit(1); }

    //Break here if readonly
    if readonly {

        println!("boot_a: {} boot_b: {}", boot_a_flags, boot_b_flags);
        process::exit(0);
    }

    //Rewrite changes to GPT table
    let mut new_boot_a = partitions[&BOOT_A_PARTNUM].clone();
    let mut new_boot_b = partitions[&BOOT_B_PARTNUM].clone();

    new_boot_a.flags = boot_a_flags;
    new_boot_b.flags = boot_b_flags;
    partitions.insert(BOOT_A_PARTNUM, new_boot_a);
    partitions.insert(BOOT_B_PARTNUM, new_boot_b);
    disk.update_partitions(partitions).unwrap();
    disk.write().unwrap();
}

fn enable_aboot(bootflags: u64) -> u64 {

    //Sets 5th bit to 1, sets active boot partition
    let mut tmp_bootflags = bootflags;
    tmp_bootflags |= 0b0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    //KEEPING THIS OFF FOR NOW, MAY BRICK IF ENABLED
    //bootflags &= 0b1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    return tmp_bootflags;
}

fn disable_aboot(bootflags: u64) -> u64 {

    //Sets 5th bit to 0, unsets active boot partition
    let mut tmp_bootflags = bootflags;
    tmp_bootflags &= 0b1111_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    return tmp_bootflags;
}
