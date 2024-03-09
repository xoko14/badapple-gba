#pragma pattern_limit 99999999

struct Section{
u8 color;
u16 pixel_count;
};

struct Frame{
u16 section_count;
Section sections[section_count];
};

struct BaFile{
u8 frame_height;
u8 frame_width;
u8 frame_rate;
u32 frame_count;
Frame sections[frame_count];
};

BaFile bafile @ 0x00;