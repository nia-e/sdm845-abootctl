# sdm845-abootctl
Utility to control boot slots on OnePlus SDM845 devices from mainline Linux by flipping the relevant bit in the GPT partition table flags. Heavily WIP, but barebones functionality should be present.

WARNING: DO NOT USE YET. THIS MAY BRICK YOUR DEVICE, WIPE YOUR PARTITION TABLE, OR AWAKEN THE GREAT OLD ONES OF R'LYEH. NO MATTER WHAT THIS UTILITY CAUSES, MATERIAL OR ESOTERIC, YOU HAVE BEEN WARNED.
```
USAGE:
    sdm845-abootctl [FLAGS] --mode <MODE> --slot <SLOT>

FLAGS:
        --debug      Dumps entire header for boot partitions to standard output
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --mode <MODE>    Mode of operation (r/w)
    -s, --slot <SLOT>    Slot - sets as active boot slot if in write mode, reads slot data if in read mode
```
Written by Aissa Z. B. <aissa.zenaida@pm.me> and Caleb C. <caleb@connolly.tech>
