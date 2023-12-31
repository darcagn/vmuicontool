# vmuicontool
Convert an image file to a VMU icon byte array

## Customize output
See these lines in `main.rs`:
```
// These four constants control formatting settings:
// Indent all lines by this number of spaces...
const INDENT_ALL: usize = 0;
// Indent bytes inside blocks by this number of spaces...
const INDENT_BLOCK: usize = 4;
// This number of palette bytes will be printed per line...
const PAL_BYTES_PER_LINE: usize = 8;
// This number of data bytes will be printed per line...
const DATA_BYTES_PER_LINE: usize = 8;
```

## Build
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repo
3. `cargo build` inside root directory

## Usage
Create an icon 32x32 in size with no more than 16 colors (any image with more than 16 colors will be rejected). If your file is not 32x32 it will be resized using Lancosz3. Most common image file formats are accepted (bmp, png, jpg, etc.).

Example run using `./vmuicontool input.bmp` outputs:
```
static const unsigned short vmu_icon_pal[16] = {
    0xFFFF, 0xFDDD, 0xFFFC, 0xFBAB, 0xF998, 0xFCB7, 0xFFD6, 0xF000,
    0xF668, 0xF666, 0xF434, 0xF7D9, 0xF26D, 0xF3DC, 0xF29E, 0xFAFE,
};

static const unsigned char vmu_icon_data[512] = {
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
    0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x11, 0x20, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x21, 0x10, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x13, 0x11, 0x10, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x22, 0x21, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x03, 0x33, 0x11, 0x12, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x22, 0x22, 0x10, 0x00, 0x00,
    0x00, 0x00, 0x43, 0x33, 0x31, 0x11, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x22, 0x22, 0x21, 0x00, 0x00,
    0x00, 0x03, 0x55, 0x33, 0x33, 0x11, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x22, 0x22, 0x22, 0x10, 0x00,
    0x00, 0x15, 0x55, 0x53, 0x33, 0x11, 0x20, 0x00,
    0x00, 0x10, 0x22, 0x22, 0x22, 0x22, 0x11, 0x00,
    0x00, 0x56, 0x55, 0x55, 0x33, 0x31, 0x10, 0x00,
    0x00, 0x10, 0x22, 0x22, 0x22, 0x21, 0x13, 0x00,
    0x01, 0x66, 0x66, 0x55, 0x53, 0x31, 0x10, 0x00,
    0x00, 0x02, 0x22, 0x22, 0x21, 0x11, 0x13, 0x10,
    0x05, 0x66, 0x66, 0x66, 0x55, 0x33, 0x11, 0x10,
    0x01, 0x12, 0x22, 0x22, 0x11, 0x13, 0x33, 0x40,
    0x05, 0x66, 0x77, 0x76, 0x65, 0x77, 0x31, 0x00,
    0x07, 0x78, 0x22, 0x77, 0x77, 0x33, 0x34, 0x40,
    0x15, 0x55, 0x67, 0x66, 0x66, 0x67, 0x31, 0x00,
    0x07, 0x13, 0x91, 0x17, 0x33, 0x74, 0x44, 0x41,
    0x34, 0x45, 0x57, 0x55, 0x66, 0x47, 0x71, 0x21,
    0x77, 0x11, 0x84, 0x37, 0x44, 0x74, 0x44, 0x93,
    0x44, 0x44, 0x47, 0x44, 0x55, 0xA7, 0x73, 0x30,
    0x77, 0x13, 0x3A, 0x47, 0x48, 0x79, 0x99, 0x94,
    0x49, 0x99, 0x97, 0x44, 0x44, 0xA7, 0x37, 0x07,
    0x07, 0x43, 0x3A, 0x97, 0x77, 0x99, 0x99, 0x94,
    0x49, 0x99, 0x97, 0x88, 0x84, 0xA7, 0x37, 0x07,
    0x07, 0x33, 0x1A, 0x97, 0x97, 0x99, 0x99, 0xA9,
    0x49, 0x88, 0x87, 0x44, 0x44, 0xA7, 0x11, 0x70,
    0x07, 0x11, 0xB9, 0x87, 0x88, 0x7A, 0xAA, 0xA4,
    0x39, 0x44, 0x47, 0x44, 0x44, 0x47, 0x11, 0x73,
    0x17, 0x31, 0x58, 0xC7, 0xC8, 0x78, 0x88, 0xA3,
    0x18, 0x44, 0x77, 0x74, 0x44, 0x77, 0x72, 0x11,
    0x77, 0x74, 0x9D, 0x77, 0x7C, 0x77, 0x88, 0x81,
    0x09, 0x44, 0x44, 0x44, 0x43, 0x33, 0x55, 0x10,
    0x01, 0x39, 0xBB, 0xDE, 0xEC, 0xCC, 0xCC, 0x80,
    0x04, 0x44, 0x44, 0x44, 0x33, 0x3F, 0xF3, 0x33,
    0x13, 0x11, 0xBB, 0xBD, 0xDE, 0xEC, 0xCC, 0x80,
    0x01, 0x44, 0x44, 0x43, 0x33, 0x1F, 0xF0, 0x00,
    0x00, 0x01, 0x1B, 0xBB, 0xBD, 0xDE, 0xEC, 0x10,
    0x00, 0x44, 0x43, 0x33, 0x31, 0xFF, 0xFF, 0x00,
    0x00, 0x00, 0x11, 0xBB, 0xBB, 0xDD, 0xEE, 0x00,
    0x00, 0x14, 0x33, 0x33, 0x1F, 0xFF, 0xF0, 0x00,
    0x00, 0x00, 0x11, 0x3B, 0xBB, 0xBD, 0xD1, 0x00,
    0x00, 0x03, 0x33, 0x31, 0xFF, 0xFF, 0xF0, 0x00,
    0x00, 0x00, 0x01, 0x1B, 0xBB, 0xBB, 0xB0, 0x00,
    0x00, 0x00, 0x33, 0x1F, 0xFF, 0xFF, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x11, 0xBB, 0xB5, 0x00, 0x00,
    0x00, 0x00, 0x01, 0x1F, 0xFF, 0xFF, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x11, 0x3B, 0x30, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x1F, 0xFF, 0xF0, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x21, 0x33, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x02, 0x1F, 0xF0, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x12, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x10, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
};
```

The icon can then be added to a VMU file. See [this KOS example](https://github.com/KallistiOS/KallistiOS/blob/master/examples/dreamcast/vmu/vmu_pkg/vmu.c) and [VMS file header documentation](http://mc.pp.se/dc/vms/fileheader.html) for more info.
