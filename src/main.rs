use clap::{App, AppSettings, Arg, SubCommand};
use modular_bitfield::prelude::*;
use std::convert::TryInto;
use std::process;

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
        .subcommand(SubCommand::with_name("mark-boot-successful")
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
        .get_matches();

    match matches.subcommand_name() {
        Some("hal-info") => {
            println!("HAL Version: linux.hardware.boot@1.0::abootctl");
        }
        Some("get-number-slots") => {
            /* if let (_, _, _) = partitions::get_boot_partitions() { */
            println!("2"); /* } else { println!("1"); } */
        }
        Some("get-current-slot") => {
            println!("{}", get_current_slot());
        }
        Some("mark-boot-successful") => {
            mark_successful(get_current_slot());
        }
        Some("set-active-boot-slot") => {
            set_slot(matches.subcommand_matches("set-active-boot-slot").unwrap().value_of("SLOT").unwrap().parse::<i32>().unwrap());
        }
        Some("set-slot-as-unbootable") => {
            mark_unbootable(matches.subcommand_matches("set-slot-as-unbootable").unwrap().value_of("SLOT").unwrap().parse::<i32>().unwrap());
        }
        Some("is-slot-bootable") => {
            process::exit(
                is_bootable(matches.subcommand_matches("is-slot-bootable").unwrap().value_of("SLOT").unwrap().parse::<i32>().unwrap()) as i32,
            );
        }
        Some("is-slot-marked-successful") => {
            process::exit(
                is_successful(matches.subcommand_matches("is-slot-marked-successful").unwrap().value_of("SLOT").unwrap().parse::<i32>().unwrap()) as i32,
            );
        }
        Some("get-suffix") => {
            println!(
                "{}",
                get_suffix(matches.subcommand_matches("get_suffix").unwrap().value_of("SLOT").unwrap().parse::<i32>().unwrap())
            );
        }
        _ => {
            process::exit(64);
        } //Android does it this way idk
    }
}

fn get_slot_info() -> (SlotInfo, SlotInfo) {
    let (boot_a, boot_b, _) = partitions::get_boot_partitions();
    let slot_a = SlotInfo::from_bytes([((boot_a.flags >> 48) & 0xFF).try_into().unwrap()]);
    let slot_b = SlotInfo::from_bytes([((boot_b.flags >> 48) & 0xFF).try_into().unwrap()]);
    return (slot_a, slot_b);
}

fn get_current_slot() -> i32 {
    let (slot_a, slot_b) = get_slot_info();
    if (slot_a.is_active() == 1) && (slot_b.is_active() == 0) {
        return 0;
    } else if (slot_a.is_active() == 0) && (slot_b.is_active() == 1) {
        return 1;
    } else {
        panic!("Corrupted headers; none or both partitions marked active");
    }
}

fn mark_successful(slot: i32) {
    let (mut slot_a, mut slot_b) = get_slot_info();
    let (mut boot_a, mut boot_b, path) = partitions::get_boot_partitions();
    match slot {
        0 => {
            slot_a.set_boot_successful(1);
            boot_a.flags = (slot_a.into_bytes()[0] as u64) << 48;
        }
        1 => {
            slot_b.set_boot_successful(1);
            boot_b.flags = (slot_b.into_bytes()[0] as u64) << 48;
        }
        _ => panic!("This should never be reached"),
    }
    partitions::set_boot_partition(boot_a, &path);
    partitions::set_boot_partition(boot_b, &path);
}

fn mark_unbootable(slot: i32) {
    let (mut slot_a, mut slot_b) = get_slot_info();
    let (mut boot_a, mut boot_b, path) = partitions::get_boot_partitions();
    match slot {
        0 => {
            slot_a.set_is_unbootable(1);
            boot_a.flags = (slot_a.into_bytes()[0] as u64) << 48;
        }
        1 => {
            slot_b.set_is_unbootable(1);
            boot_b.flags = (slot_b.into_bytes()[0] as u64) << 48;
        }
        _ => {
            panic!("This should never be reached either");
        }
    }
    partitions::set_boot_partition(boot_a, &path);
    partitions::set_boot_partition(boot_b, &path)
}

fn is_bootable(slot: i32) -> bool {
    let (slot_a, slot_b) = get_slot_info();
    match slot {
        0 => {
            if slot_a.is_unbootable() != 0 {
                return true;
            } else {
                return false;
            };
        }
        1 => {
            if slot_b.is_unbootable() != 0 {
                return true;
            } else {
                return false;
            };
        }
        _ => {
            panic!("This should really never be reached");
        }
    }
}

fn is_successful(slot: i32) -> bool {
    let (slot_a, slot_b) = get_slot_info();
    match slot {
        0 => {
            if slot_a.boot_successful() == 0 {
                return true;
            } else {
                return false;
            }
        }
        1 => {
            if slot_b.boot_successful() == 0 {
                return true;
            } else {
                return false;
            };
        }
        _ => {
            panic!("This should really never be reached");
        }
    }
}

fn get_suffix(slot: i32) -> String {
    match slot {
        0 => {
            return "_a".to_string();
        }
        1 => {
            return "_b".to_string();
        }
        _ => {
            panic!("Seriously how did this happen");
        }
    }
}

fn set_slot(slot: i32) {
    let (mut boot_a, mut boot_b, path) = partitions::get_boot_partitions();
    let mut flags_a = SlotInfo::from_bytes([((boot_a.flags << 48) & 0xFF).try_into().unwrap()]);
    let mut flags_b = SlotInfo::from_bytes([((boot_b.flags << 48) & 0xFF).try_into().unwrap()]);

    if slot == 0 {
        flags_a.set_is_active(1);
        flags_b.set_is_active(0);
    } else if slot == 1 {
        flags_a.set_is_active(0);
        flags_b.set_is_active(1);
    } else {
        panic!("Error: could not read partition table headers");
    };

    boot_a.flags = (flags_a.into_bytes()[0] as u64) << 48;
    boot_b.flags = (flags_b.into_bytes()[0] as u64) << 48;
    partitions::set_boot_partition(boot_a, &path);
    partitions::set_boot_partition(boot_b, &path);
}
