use std::collections::HashSet;

// Unicode ranges data from the OpenType OS/2 table specification v1.8.4
// https://docs.microsoft.com/en-us/typography/opentype/spec/os2#ulunicoderange1-bits-031ulunicoderange2-bits-3263ulunicoderange3-bits-6495ulunicoderange4-bits-96127
const OS2_UNICODE_RANGES: [(i32, &str, i32, i32); 169] = [
    (0, "Basic Latin", 0x0000, 0x007F),
    (1, "Latin-1 Supplement", 0x0080, 0x00FF),
    (2, "Latin Extended-A", 0x0100, 0x017F),
    (3, "Latin Extended-B", 0x0180, 0x024F),
    (4, "IPA Extensions", 0x0250, 0x02AF),
    (4, "Phonetic Extensions", 0x1D00, 0x1D7F), // Added in OpenType 1.5 for OS/2 version 4.
    (4, "Phonetic Extensions Supplement", 0x1D80, 0x1DBF), // Added in OpenType 1.5 for OS/2 version 4.
    (5, "Spacing Modifier Letters", 0x02B0, 0x02FF),
    (5, "Modifier Tone Letters", 0xA700, 0xA71F), // Added in OpenType 1.5 for OS/2 version 4.
    (6, "Combining Diacritical Marks", 0x0300, 0x036F),
    (6, "Combining Diacritical Marks Supplement", 0x1DC0, 0x1DFF), // Added in OpenType 1.5 for OS/2 version 4.
    (7, "Greek and Coptic", 0x0370, 0x03FF),
    (8, "Coptic", 0x2C80, 0x2CFF), // Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    (9, "Cyrillic", 0x0400, 0x04FF),
    (9, "Cyrillic Supplement", 0x0500, 0x052F), // Added in OpenType 1.4 for OS/2 version 3.
    (9, "Cyrillic Extended-A", 0x2DE0, 0x2DFF), // Added in OpenType 1.5 for OS/2 version 4.
    (9, "Cyrillic Extended-B", 0xA640, 0xA69F), // Added in OpenType 1.5 for OS/2 version 4.
    (10, "Armenian", 0x0530, 0x058F),
    (11, "Hebrew", 0x0590, 0x05FF),
    (12, "Vai", 0xA500, 0xA63F), // Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    (13, "Arabic", 0x0600, 0x06FF),
    (13, "Arabic Supplement", 0x0750, 0x077F), // Added in OpenType 1.5 for OS/2 version 4.
    (14, "NKo", 0x07C0, 0x07FF), // Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    (15, "Devanagari", 0x0900, 0x097F),
    (16, "Bengali", 0x0980, 0x09FF),
    (17, "Gurmukhi", 0x0A00, 0x0A7F),
    (18, "Gujarati", 0x0A80, 0x0AFF),
    (19, "Oriya", 0x0B00, 0x0B7F),
    (20, "Tamil", 0x0B80, 0x0BFF),
    (21, "Telugu", 0x0C00, 0x0C7F),
    (22, "Kannada", 0x0C80, 0x0CFF),
    (23, "Malayalam", 0x0D00, 0x0D7F),
    (24, "Thai", 0x0E00, 0x0E7F),
    (25, "Lao", 0x0E80, 0x0EFF),
    (26, "Georgian", 0x10A0, 0x10FF),
    (26, "Georgian Supplement", 0x2D00, 0x2D2F), // Added in OpenType 1.5 for OS/2 version 4.
    (27, "Balinese", 0x1B00, 0x1B7F), // Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    (28, "Hangul Jamo", 0x1100, 0x11FF),
    (29, "Latin Extended Additional", 0x1E00, 0x1EFF),
    (29, "Latin Extended-C", 0x2C60, 0x2C7F), // Added in OpenType 1.5 for OS/2 version 4.
    (29, "Latin Extended-D", 0xA720, 0xA7FF), // Added in OpenType 1.5 for OS/2 version 4.
    (30, "Greek Extended", 0x1F00, 0x1FFF),
    (31, "General Punctuation", 0x2000, 0x206F),
    (31, "Supplemental Punctuation", 0x2E00, 0x2E7F), // Added in OpenType 1.5 for OS/2 version 4.
    (32, "Superscripts And Subscripts", 0x2070, 0x209F),
    (33, "Currency Symbols", 0x20A0, 0x20CF),
    (
        34,
        "Combining Diacritical Marks For Symbols",
        0x20D0,
        0x20FF,
    ),
    (35, "Letterlike Symbols", 0x2100, 0x214F),
    (36, "Number Forms", 0x2150, 0x218F),
    (37, "Arrows", 0x2190, 0x21FF),
    (37, "Supplemental Arrows-A", 0x27F0, 0x27FF), // Added in OpenType 1.4 for OS/2 version 3.
    (37, "Supplemental Arrows-B", 0x2900, 0x297F), // Added in OpenType 1.4 for OS/2 version 3.
    (37, "Miscellaneous Symbols and Arrows", 0x2B00, 0x2BFF), // Added in OpenType 1.5 for OS/2 version 4.
    (38, "Mathematical Operators", 0x2200, 0x22FF),
    (38, "Supplemental Mathematical Operators", 0x2A00, 0x2AFF), // Added in OpenType 1.4 for OS/2 version 3.
    (38, "Miscellaneous Mathematical Symbols-A", 0x27C0, 0x27EF), // Added in OpenType 1.4 for OS/2 version 3.
    (38, "Miscellaneous Mathematical Symbols-B", 0x2980, 0x29FF), // Added in OpenType 1.4 for OS/2 version 3.
    (39, "Miscellaneous Technical", 0x2300, 0x23FF),
    (40, "Control Pictures", 0x2400, 0x243F),
    (41, "Optical Character Recognition", 0x2440, 0x245F),
    (42, "Enclosed Alphanumerics", 0x2460, 0x24FF),
    (43, "Box Drawing", 0x2500, 0x257F),
    (44, "Block Elements", 0x2580, 0x259F),
    (45, "Geometric Shapes", 0x25A0, 0x25FF),
    (46, "Miscellaneous Symbols", 0x2600, 0x26FF),
    (47, "Dingbats", 0x2700, 0x27BF),
    (48, "CJK Symbols And Punctuation", 0x3000, 0x303F),
    (49, "Hiragana", 0x3040, 0x309F),
    (50, "Katakana", 0x30A0, 0x30FF),
    (50, "Katakana Phonetic Extensions", 0x31F0, 0x31FF), // Added in OpenType 1.4 for OS/2 version 3.
    (51, "Bopomofo", 0x3100, 0x312F),
    (51, "Bopomofo Extended", 0x31A0, 0x31BF), // Added in OpenType 1.3, extending OS/2 version 2.
    (52, "Hangul Compatibility Jamo", 0x3130, 0x318F),
    (53, "Phags-pa", 0xA840, 0xA87F), // Added in OpenType 1.5 for OS/2 version 4. See below for other version differences.
    (54, "Enclosed CJK Letters And Months", 0x3200, 0x32FF),
    (55, "CJK Compatibility", 0x3300, 0x33FF),
    (56, "Hangul Syllables", 0xAC00, 0xD7AF),
    (57, "Non-Plane 0", 0x10000, 0x10FFFF), // Implies at least one character beyond the Basic Multilingual Plane. First assigned in OpenType 1.3 for OS/2 version 2.
    (58, "Phoenician", 0x10900, 0x1091F),   // First assigned in OpenType 1.5 for OS/2 version 4.
    (59, "CJK Unified Ideographs", 0x4E00, 0x9FFF),
    (59, "CJK Radicals Supplement", 0x2E80, 0x2EFF), // Added in OpenType 1.3 for OS/2 version 2.
    (59, "Kangxi Radicals", 0x2F00, 0x2FDF),         // Added in OpenType 1.3 for OS/2 version 2.
    (59, "Ideographic Description Characters", 0x2FF0, 0x2FFF), // Added in OpenType 1.3 for OS/2 version 2.
    (59, "CJK Unified Ideographs Extension A", 0x3400, 0x4DBF), // Added in OpenType 1.3 for OS/2 version 2.
    (59, "CJK Unified Ideographs Extension B", 0x20000, 0x2A6DF), // Added in OpenType 1.4 for OS/2 version 3.
    (59, "Kanbun", 0x3190, 0x319F), // Added in OpenType 1.4 for OS/2 version 3.
    (60, "Private Use Area (plane 0)", 0xE000, 0xF8FF),
    (61, "CJK Strokes", 0x31C0, 0x31EF), // Range added in OpenType 1.5 for OS/2 version 4.
    (61, "CJK Compatibility Ideographs", 0xF900, 0xFAFF),
    (
        61,
        "CJK Compatibility Ideographs Supplement",
        0x2F800,
        0x2FA1F,
    ), // Added in OpenType 1.4 for OS/2 version 3.
    (62, "Alphabetic Presentation Forms", 0xFB00, 0xFB4F),
    (63, "Arabic Presentation Forms-A", 0xFB50, 0xFDFF),
    (64, "Combining Half Marks", 0xFE20, 0xFE2F),
    (65, "Vertical Forms", 0xFE10, 0xFE1F), // Range added in OpenType 1.5 for OS/2 version 4.
    (65, "CJK Compatibility Forms", 0xFE30, 0xFE4F),
    (66, "Small Form Variants", 0xFE50, 0xFE6F),
    (67, "Arabic Presentation Forms-B", 0xFE70, 0xFEFF),
    (68, "Halfwidth And Fullwidth Forms", 0xFF00, 0xFFEF),
    (69, "Specials", 0xFFF0, 0xFFFF),
    (70, "Tibetan", 0x0F00, 0x0FFF), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (71, "Syriac", 0x0700, 0x074F),  // First assigned in OpenType 1.3, extending OS/2 version 2.
    (72, "Thaana", 0x0780, 0x07BF),  // First assigned in OpenType 1.3, extending OS/2 version 2.
    (73, "Sinhala", 0x0D80, 0x0DFF), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (74, "Myanmar", 0x1000, 0x109F), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (75, "Ethiopic", 0x1200, 0x137F), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (75, "Ethiopic Supplement", 0x1380, 0x139F), // Added in OpenType 1.5 for OS/2 version 4.
    (75, "Ethiopic Extended", 0x2D80, 0x2DDF), // Added in OpenType 1.5 for OS/2 version 4.
    (76, "Cherokee", 0x13A0, 0x13FF), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (77, "Unified Canadian Aboriginal Syllabics", 0x1400, 0x167F), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (78, "Ogham", 0x1680, 0x169F), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (79, "Runic", 0x16A0, 0x16FF), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (80, "Khmer", 0x1780, 0x17FF), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (80, "Khmer Symbols", 0x19E0, 0x19FF), // Added in OpenType 1.5 for OS/2 version 4.
    (81, "Mongolian", 0x1800, 0x18AF), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (82, "Braille Patterns", 0x2800, 0x28FF), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (83, "Yi Syllables", 0xA000, 0xA48F), // First assigned in OpenType 1.3, extending OS/2 version 2.
    (83, "Yi Radicals", 0xA490, 0xA4CF),  // Added in OpenType 1.3, extending OS/2 version 2.
    (84, "Tagalog", 0x1700, 0x171F),      // First assigned in OpenType 1.4 for OS/2 version 3.
    (84, "Hanunoo", 0x1720, 0x173F),      // Added in OpenType 1.4 for OS/2 version 3.
    (84, "Buhid", 0x1740, 0x175F),        // Added in OpenType 1.4 for OS/2 version 3.
    (84, "Tagbanwa", 0x1760, 0x177F),     // Added in OpenType 1.4 for OS/2 version 3.
    (85, "Old Italic", 0x10300, 0x1032F), // First assigned in OpenType 1.4 for OS/2 version 3.
    (86, "Gothic", 0x10330, 0x1034F),     // First assigned in OpenType 1.4 for OS/2 version 3.
    (87, "Deseret", 0x10400, 0x1044F),    // First assigned in OpenType 1.4 for OS/2 version 3.
    (88, "Byzantine Musical Symbols", 0x1D000, 0x1D0FF), // First assigned in OpenType 1.4 for OS/2 version 3.
    (88, "Musical Symbols", 0x1D100, 0x1D1FF), // Added in OpenType 1.4 for OS/2 version 3.
    (88, "Ancient Greek Musical Notation", 0x1D200, 0x1D24F), // Added in OpenType 1.5 for OS/2 version 4.
    (89, "Mathematical Alphanumeric Symbols", 0x1D400, 0x1D7FF), // First assigned in OpenType 1.4 for OS/2 version 3.
    (90, "Private Use (plane 15)", 0xF0000, 0xFFFFD), // First assigned in OpenType 1.4 for OS/2 version 3.
    (90, "Private Use (plane 16)", 0x100000, 0x10FFFD), // Added in OpenType 1.4 for OS/2 version 3.
    (91, "Variation Selectors", 0xFE00, 0xFE0F), // First assigned in OpenType 1.4 for OS/2 version 3.
    (91, "Variation Selectors Supplement", 0xE0100, 0xE01EF), // Added in OpenType 1.4 for OS/2 version 3.
    (92, "Tags", 0xE0000, 0xE007F), // First assigned in OpenType 1.4 for OS/2 version 3.
    (93, "Limbu", 0x1900, 0x194F),  // First assigned in OpenType 1.5 for OS/2 version 4.
    (94, "Tai Le", 0x1950, 0x197F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (95, "New Tai Lue", 0x1980, 0x19DF), // First assigned in OpenType 1.5 for OS/2 version 4.
    (96, "Buginese", 0x1A00, 0x1A1F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (97, "Glagolitic", 0x2C00, 0x2C5F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (98, "Tifinagh", 0x2D30, 0x2D7F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (99, "Yijing Hexagram Symbols", 0x4DC0, 0x4DFF), // First assigned in OpenType 1.5 for OS/2 version 4.
    (100, "Syloti Nagri", 0xA800, 0xA82F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (101, "Linear B Syllabary", 0x10000, 0x1007F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (101, "Linear B Ideograms", 0x10080, 0x100FF), // Added in OpenType 1.5 for OS/2 version 4.
    (101, "Aegean Numbers", 0x10100, 0x1013F),     // Added in OpenType 1.5 for OS/2 version 4.
    (102, "Ancient Greek Numbers", 0x10140, 0x1018F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (103, "Ugaritic", 0x10380, 0x1039F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (104, "Old Persian", 0x103A0, 0x103DF), // First assigned in OpenType 1.5 for OS/2 version 4.
    (105, "Shavian", 0x10450, 0x1047F),  // First assigned in OpenType 1.5 for OS/2 version 4.
    (106, "Osmanya", 0x10480, 0x104AF),  // First assigned in OpenType 1.5 for OS/2 version 4.
    (107, "Cypriot Syllabary", 0x10800, 0x1083F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (108, "Kharoshthi", 0x10A00, 0x10A5F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (109, "Tai Xuan Jing Symbols", 0x1D300, 0x1D35F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (110, "Cuneiform", 0x12000, 0x123FF), // First assigned in OpenType 1.5 for OS/2 version 4.
    (110, "Cuneiform Numbers and Punctuation", 0x12400, 0x1247F), // Added in OpenType 1.5 for OS/2 version 4.
    (111, "Counting Rod Numerals", 0x1D360, 0x1D37F), // First assigned in OpenType 1.5 for OS/2 version 4.
    (112, "Sundanese", 0x1B80, 0x1BBF), // First assigned in OpenType 1.5 for OS/2 version 4.
    (113, "Lepcha", 0x1C00, 0x1C4F),    // First assigned in OpenType 1.5 for OS/2 version 4.
    (114, "Ol Chiki", 0x1C50, 0x1C7F),  // First assigned in OpenType 1.5 for OS/2 version 4.
    (115, "Saurashtra", 0xA880, 0xA8DF), // First assigned in OpenType 1.5 for OS/2 version 4.
    (116, "Kayah Li", 0xA900, 0xA92F),  // First assigned in OpenType 1.5 for OS/2 version 4.
    (117, "Rejang", 0xA930, 0xA95F),    // First assigned in OpenType 1.5 for OS/2 version 4.
    (118, "Cham", 0xAA00, 0xAA5F),      // First assigned in OpenType 1.5 for OS/2 version 4.
    (119, "Ancient Symbols", 0x10190, 0x101CF), // First assigned in OpenType 1.5 for OS/2 version 4.
    (120, "Phaistos Disc", 0x101D0, 0x101FF), // First assigned in OpenType 1.5 for OS/2 version 4.
    (121, "Carian", 0x102A0, 0x102DF),        // First assigned in OpenType 1.5 for OS/2 version 4.
    (121, "Lycian", 0x10280, 0x1029F),        // Added in OpenType 1.5 for OS/2 version 4.
    (121, "Lydian", 0x10920, 0x1093F),        // Added in OpenType 1.5 for OS/2 version 4.
    (122, "Domino Tiles", 0x1F030, 0x1F09F),  // First assigned in OpenType 1.5 for OS/2 version 4.
    (122, "Mahjong Tiles", 0x1F000, 0x1F02F), // First assigned in OpenType 1.5 for OS/2 version 4.
                                              // (123-127, Reserved for process-internal usage")
];

/// Determine a glyph's unicode range by using binary search so we get a O(n log m) run time
fn glyph_unicode_range(a: &[(i32, &str, i32, i32)], target: i32) -> i32 {
    let mut start = 0;
    let mut end = a.len();
    while start < end {
        let mid = (start + end) / 2;
        let (bit, _name, cp_start, cp_end) = a[mid];
        if target >= cp_start && target <= cp_end {
            return bit;
        }
        if target < cp_start {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    i32::MIN // This is horrendous
}

// Implementation loosely based on https://github.com/fonttools/fonttools/blob/main/Lib/fontTools/ttLib/tables/O_S_2f_2.py
fn unicode_ranges(unicodes: Vec<i32>) -> HashSet<i32> {
    let mut unicode_ranges = HashSet::new();
    // we need to sort OS2_UNICODE_RANGES by the 3rd column which is the starting code point
    // for each range because we use binary search.
    let mut sorted_os2_unicode_ranges = OS2_UNICODE_RANGES.clone();
    sorted_os2_unicode_ranges.sort_by_key(|&a| a.2);
    for uni in unicodes {
        unicode_ranges.insert(glyph_unicode_range(&sorted_os2_unicode_ranges, uni));
    }
    unicode_ranges
}

fn main() {
    let unicodes = vec![234, 110, 111, 112, 113, 512, 1000, 0x101D5];
    println!("{:?}", unicode_ranges(unicodes));
}
