# sdm845-abootctl
Utility to control boot slots on OnePlus SDM845 devices from mainline Linux by flipping the relevant bit in the GPT partition table flags. Heavily WIP, but barebones functionality should be present.

WARNING: THIS MAY BRICK YOUR DEVICE, WIPE YOUR PARTITION TABLE, OR AWAKEN THE GREAT OLD ONES OF R'LYEH. NO MATTER WHAT THIS UTILITY CAUSES, MATERIAL OR ESOTERIC, YOU HAVE BEEN WARNED.
```
USAGE:
    sdm845-abootctl [FLAGS] <SLOT>

FLAGS:
    -h, --help       Prints help information
<<<<<<< HEAD
    -r               Read-only mode: reads value of boot partition headers without changing them
=======
    -r               Reads value of boot partition headers without changing them
>>>>>>> 8d8d275e1e448bafa4d4708cf0b169caa15aca6c
    -V, --version    Prints version information

ARGS:
    <SLOT>    Slot to set as active (0 or 1)
```
Written by Aissa Z. B. <aissa.zenaida@pm.me> with great help from Caleb C. <caleb@connolly.tech>
