use clap::{Arg, App, SubCommand};
use modular_bitfield::prelude::*;
use std::convert::TryInto;
use std::process;

mod partitions;

#[bitfield(bits = 8)]
#[derive(Debug)]
struct SlotInfo {
    #[skip] __: B2,
    is_active: B1,
    #[skip] __: B3,
    boot_successful: B1,
    is_unbootable: B1,
}

fn main() {
    // CLI stuff
    //Ideas: -r read-only (default), --set or -s set to SLOT, --debug dump all flags
    let matches = App::new("abootctl")
        .version("0.4.0")
        .author("Caleb C. <caleb@connolly.tech>, Aissa Z. B. <aissa.zenaida@pm.me>")
        .about("Switch active bootloader slot on SDM845 OnePlus devices. THIS MAY BRICK YOUR DEVICE - USE WITH CARE")
        .arg(Arg::with_name("mode")
            .short("m")
            .long("mode")
            .required(true)
            .takes_value(true)
            .value_name("MODE")
            .help("Mode of operation (r/w)"))
        .arg(Arg::with_name("slot")
            .short("s")
            .long("slot")
            .required(true)
            .takes_value(true)
            .value_name("SLOT")
            .help("Slot - sets as active boot slot if in write mode, reads slot data if in read mode"))
        .arg(Arg::with_name("debug")
            .long("debug")
            .help("Dumps entire header for boot partitions to standard output"))
        .get_matches();

    //TODO: read bootable flag option
    let mode: &str;
    let debug: bool;
    let slot: i32;

    if matches.is_present("debug") {
        debug = true;
    } else {debug = false; }

    let (flags_a, flags_b, slot_a, slot_b) = get_slot_info(debug);

    mode = matches.value_of("mode").unwrap_or("r");
    slot = matches.value_of("slot").unwrap().parse::<i32>().unwrap_or(-1);

    //Checking CLI args
    if !(mode.eq("r") || mode.eq("w")) { panic!("ERROR: Invalid mode specified"); }
    if !(slot == 0 || slot == 1) { panic!("ERROR: Invalid slot specified"); }

    if mode.eq("r") {
        println!("Slot A info: {:?}", slot_a);
        println!("Slot B info: {:?}", slot_b);
    }

    else { set_slot(&slot, flags_a, flags_b); }
}

fn get_slot_info(debug: bool) -> (u64, u64, SlotInfo, SlotInfo) {
    let (boot_a, boot_b) = partitions::get_boot_partitions();
    if debug { println!("boot_a: {:#018b} boot_b: {:#018b}", boot_a.flags >> 48, boot_b.flags >> 48); }
    let slot_a = SlotInfo::from_bytes([(((boot_a.flags >> 48) & 0xFF)).try_into().unwrap()]);
    let slot_b = SlotInfo::from_bytes([(((boot_b.flags >> 48) & 0xFF)).try_into().unwrap()]);
    return (boot_a.flags >> 48, boot_b.flags >> 48, slot_a, slot_b)
}

fn set_slot(slot: &i32, flags_a: u64, flags_b: u64) {

    if *slot as i32 == 0 {
        //Change _a and _b boot partition flags
        let new_flags_a = enable_aboot(flags_a);
        let new_flags_b = disable_aboot(flags_b);
    }
     else if *slot as i32 == 1 {
         //Same as above
         let new_flags_b = enable_aboot(flags_a);
         let new_flags_a = disable_aboot(flags_a);
    }
    else { panic!("Error: could not read partition table headers or invalid slot number specified"); }

    //Get actual boot partition objects
    let (boot_a, boot_b) = partitions::get_boot_partitions();


    // new_boot_a.flags = boot_a_flags;
    // new_boot_b.flags = boot_b_flags;
    // partitions.insert(BOOT_A_PARTNUM, new_boot_a);
    // partitions.insert(BOOT_B_PARTNUM, new_boot_b);
    // disk.update_partitions(partitions).unwrap();
    // disk.write().unwrap();
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
