# sdm845-abootctl
Utility to control boot slots on OnePlus SDM845 devices from mainline Linux by flipping the relevant bit in the GPT partition table flags. Heavily WIP, but barebones functionality should be present.

WARNING: THIS MAY BRICK YOUR DEVICE, WIPE YOUR PARTITION TABLE, OR AWAKEN THE GREAT OLD ONES OF R'LYEH. NO MATTER WHAT THIS UTILITY CAUSES, MATERIAL OR ESOTERIC, YOU HAVE BEEN WARNED.

USAGE:
    sdm845-abootctl <SLOT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <SLOT>    Slot to set as active (0 or 1)

Written by Aissa Z. B. <aissa.zenaida@pm.me> with great help from Caleb C. <caleb@connolly.tech>
