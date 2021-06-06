use clap::{App, Arg, SubCommand, AppSettings};
use modular_bitfield::prelude::*;
use std::convert::TryInto;

mod partitions;

#[bitfield(bits = 8)]
#[derive(Debug)]
struct SlotInfo {
    #[skip]
    __: B2,
    is_active: B1,
    #[skip]
    __: B3,
    boot_successful: B1,
    is_unbootable: B1,
}
/*
arg_enum!{
    enum Mode {
        r,
        w
    }
}*/
fn main() {
    // CLI stuff
    let matches = App::new("abootctl")
        .version("0.6.0")
        .author("Aissa Z. B. <aissa.zenaida@pm.me>, Caleb C. <caleb@connolly.tech>")
        .about("Bootloader control for SDM845 OnePlus devices. CLI arguments compatible with Android bootctl. THIS MAY BRICK YOUR DEVICE - USE WITH CARE")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("hal-info")
            .about("Show info about boot_control HAL used")
            .display_order(1))
        .subcommand(SubCommand::with_name("get-number-slots")
            .about("Prints number of slots")
            .display_order(2))
        .subcommand(SubCommand::with_name("get-current-slot")
            .about("Prints currently running SLOT")
            .display_order(3))
        .subcommand(SubCommand::with_name("mark-boot-succesful")
            .about("Mark current slot as GOOD")
            .display_order(4))
        .subcommand(SubCommand::with_name("set-active-boot-slot")
            .about("On next boot, load and execute SLOT")
            .arg(Arg::with_name("SLOT")
                .required(true)
                .index(1)
                .possible_values(&["0", "1"]))
            .display_order(5))
        .subcommand(SubCommand::with_name("set-slot-as-unbootable")
            .about("Mark SLOT as invalid")
            .arg(Arg::with_name("SLOT")
                .required(true)
                .index(1)
                .possible_values(&["0", "1"]))
            .display_order(6))
        .subcommand(SubCommand::with_name("is-slot-bootable")
            .about("Returns 0 only if SLOT is bootable")
            .arg(Arg::with_name("SLOT")
                .required(true)
                .index(1)
                .possible_values(&["0", "1"]))
            .display_order(7))
        .subcommand(SubCommand::with_name("is-slot-marked-successful")
            .about("Returns 0 only if SLOT is marked GOOD")
            .arg(Arg::with_name("SLOT")
                .required(true)
                .index(1)
                .possible_values(&["0", "1"]))
            .display_order(8))
        .subcommand(SubCommand::with_name("get-suffix")
            .about("Prints suffix for SLOT")
            .arg(Arg::with_name("SLOT")
                .required(true)
                .index(1)
                .possible_values(&["0", "1"]))
            .display_order(9))
        /*
        .arg(Arg::with_name("mode")
            .short("m")
            .long("mode")
            .default_value("r")
            .possible_values(&Mode::variants())
            .value_name("MODE")
            .help("Mode of operation"))
        .arg(Arg::with_name("slot")
            .short("s")
            .long("slot")
            .required(true)
            .possible_values(&["0", "1"])
            .value_name("SLOT")
            .help("Slot - sets as active boot slot if in write mode, reads slot data if in read mode"))
        .arg(Arg::with_name("debug")
            .long("debug")
            .help("Dumps entire header for boot partitions to standard output"))
        */
        .get_matches();

    //TODO: read bootable flag option
    /*
    let mode = value_t!(matches, "MODE", Mode).unwrap_or_else(|x| x.exit());
    let slot = value_t!(matches, "SLOT", i32).unwrap_or_else(|x| x.exit());
    let debug = matches.is_present("debug");

    let (flags_a, flags_b, slot_a, slot_b) = get_slot_info(debug);

    if mode == Mode::r {
        println!("Slot A info: {:?}", slot_a);
        println!("Slot B info: {:?}", slot_b);
    } else {
        set_slot(&slot, flags_a, flags_b);
    }*/
}

fn get_slot_info(debug: bool) -> (u64, u64, SlotInfo, SlotInfo) {
    let (boot_a, boot_b, _) = partitions::get_boot_partitions();
    if debug {
        println!(
            "boot_a: {:#018b} boot_b: {:#018b}",
            boot_a.flags >> 48,
            boot_b.flags >> 48
        );
    }
    let slot_a = SlotInfo::from_bytes([((boot_a.flags >> 48) & 0xFF).try_into().unwrap()]);
    let slot_b = SlotInfo::from_bytes([((boot_b.flags >> 48) & 0xFF).try_into().unwrap()]);
    return (boot_a.flags >> 48, boot_b.flags >> 48, slot_a, slot_b);
}

fn set_slot(slot: &i32, flags_a: u64, flags_b: u64) {
    let (new_flags_a, new_flags_b) = if *slot as i32 == 0 {
        //Change _a and _b boot partition flags
        (enable_aboot(flags_a), disable_aboot(flags_b))
    } else if *slot as i32 == 1 {
        //Same as above
        (enable_aboot(flags_b), disable_aboot(flags_a))
    } else {
        panic!("Error: could not read partition table headers or invalid slot number specified");
    };

    //Get actual boot partition objects
    let (mut boot_a, mut boot_b, path) = partitions::get_boot_partitions();
    boot_a.flags = new_flags_a;
    boot_b.flags = new_flags_b;
    partitions::set_boot_partitions(boot_a, boot_b, path);
}

fn enable_aboot(bootflags: u64) -> u64 {
    //Sets 5th bit to 1, sets active boot partition
    let mut tmp_bootflags = bootflags;
    tmp_bootflags |=
        0b0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    //KEEPING THIS OFF FOR NOW, MAY BRICK IF ENABLED
    //bootflags &= 0b1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    return tmp_bootflags;
}

fn disable_aboot(bootflags: u64) -> u64 {
    //Sets 5th bit to 0, unsets active boot partition
    let mut tmp_bootflags = bootflags;
    tmp_bootflags &=
        0b1111_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    return tmp_bootflags;
}
