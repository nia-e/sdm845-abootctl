use clap::{Arg, App};
use modular_bitfield::prelude::*;
use std::convert::TryInto;

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
    let matches = App::new("abootctl")
        .version("0.3.0")
        .author("Aissa Z. B. <aissa.zenaida@pm.me>")
        .author("Caleb C. <caleb@connolly.tech>")
        .about("Switch active bootloader slot on SDM845 OnePlus devices. THIS MAY BRICK YOUR DEVICE - USE WITH CARE")
        .arg(Arg::with_name("SLOT")
            .help("Slot to set as active (0 or 1)")
            .index(1))
        .get_matches();

    //TODO: read bootable flag option
    let slot = matches.value_of("SLOT").unwrap().parse::<i32>().unwrap();

    let (slot_a, slot_b) = get_slot_info();
    println!("Slot A info: {:?}", slot_a);
    println!("Slot B info: {:?}", slot_b);

    //set_slot(&slot, readonly);
}

fn get_slot_info() -> (SlotInfo, SlotInfo) {
    let (boot_a, boot_b) = partitions::get_boot_partitions();
    println!("boot_a: {:#018b} boot_b: {:#018b}", boot_a.flags >> 48, boot_b.flags >> 48);
    let slot_a = SlotInfo::from_bytes([(((boot_a.flags >> 48) & 0xFF)).try_into().unwrap()]);
    let slot_b = SlotInfo::from_bytes([(((boot_b.flags >> 48) & 0xFF)).try_into().unwrap()]);
    return (slot_a, slot_b)
}

fn set_slot(slot: &i32, readonly: bool) {
    //Find relevant partitions
    //Flags are read first even though they should never differ from baseline, just in case

    
    
    // let mut boot_a_flags = .unwrap().flags;//*&partitions[&BOOT_A_PARTNUM].flags;
    // let mut boot_b_flags = *&partitions[&BOOT_B_PARTNUM].flags;

    // if *slot as i32 == 0 {

    //     //Change _a and _b boot partition flags
    //     boot_a_flags = enable_aboot(boot_a_flags);
    //     boot_b_flags = disable_aboot(boot_b_flags);
    // }
    // else if *slot as i32 == 1 {

    //     //Same as above
    //     boot_b_flags = enable_aboot(boot_b_flags);
    //     boot_a_flags = disable_aboot(boot_a_flags);
    // }
    // else { eprintln!("Error: could not read partition table headers or invalid slot number specified"); process::exit(1); }

    // //Break here if readonly
    // if readonly {

    //     println!("boot_a: {} boot_b: {}", boot_a_flags, boot_b_flags);
    //     process::exit(0);
    // }

    // //Rewrite changes to GPT table
    // let mut new_boot_a = partitions[&BOOT_A_PARTNUM].clone();
    // let mut new_boot_b = partitions[&BOOT_B_PARTNUM].clone();

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
