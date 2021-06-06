# sdm845-abootctl
Utility to control boot slots on OnePlus SDM845 devices from mainline Linux by flipping the relevant bit in the GPT partition table flags. Heavily WIP, but barebones functionality should be present.

WARNING: DO NOT USE YET. THIS MAY BRICK YOUR DEVICE, WIPE YOUR PARTITION TABLE, OR AWAKEN THE GREAT OLD ONES OF R'LYEH. NO MATTER WHAT THIS UTILITY CAUSES, MATERIAL OR ESOTERIC, YOU HAVE BEEN WARNED.
```
USAGE:
    sdm845-abootctl [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    hal-info                     Show info about boot_control HAL used
    get-number-slots             Prints number of slots
    get-current-slot             Prints currently running SLOT
    mark-boot-successful         Mark current slot as GOOD
    set-active-boot-slot         On next boot, load and execute SLOT
    set-slot-as-unbootable       Mark SLOT as invalid
    is-slot-bootable             Returns 0 only if SLOT is bootable
    is-slot-marked-successful    Returns 0 only if SLOT is marked GOOD
    get-suffix                   Prints suffix for SLOT
    help                         Prints this message or the help of the given subcommand(s)
```
Written by Aissa Z. B. <aissa.zenaida@pm.me> and Caleb C. <caleb@connolly.tech>
