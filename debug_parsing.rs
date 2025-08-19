use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // Enhanced movie pattern for complex filenames like "Ghost.in.the.Shell.1995.1080p.BluRay.x264-WiKi.mkv"
    static ref ENHANCED_MOVIE_PATTERN: Regex = Regex::new(
        r"^([A-Za-z][A-Za-z\s\.]+?)\.(\d{4})\.(.+?)\."
    ).unwrap();

    // Bracketed enhanced pattern for files like "[名侦探柯南：百万美元的五棱星].Detective.Conan.The.Million-dollar.Pentagram.2024"
    static ref BRACKETED_ENHANCED_PATTERN: Regex = Regex::new(
        r"^\[([^\]]+)\]\s*\.\s*([A-Za-z][A-Za-z\s\.]+?)\.(\d{4})"
    ).unwrap();

    // Basic movie pattern: Movie Name (Year) Quality Source.ext
    static ref BASIC_MOVIE_PATTERN: Regex = Regex::new(
        r"^(.+?)\s*\((\d{4})\)\s*(.+?)\s*\.([a-zA-Z0-9]+)$"
    ).unwrap();
}

fn main() {
    let test_files = vec![
        "Ghost.in.the.Shell.2.Innocence.2004.2160p.HDR.UHD.BluRay.DTS-HD.MA.7.1.x265-10bit-HDS.mkv",
        "Ghost.in.the.Shell.1995.1080p.BluRay.x264-WiKi.mkv",
        "Kokuhaku.2010.1080p.BluRay.DD5.1.x264-NTb.mkv",
        "Lupin.III.The.Castle.of.Cagliostro.1979.BluRay.720p.x264.AC3-HDWinG.mkv",
        "Overlord.The.Scared.Kingdom.2024.1080p.ATVP.WEB-DL.JPN.DD5.1.H.264.mkv",
        "Parasyte.Part.1.2014.BluRay.iPad.720p.x264.AAC-NYPAD.mp4",
        "Suzume.2022.1080p.iT.WEB-DL.DD5.1.H.264-ZigZag.mkv",
        "[名侦探柯南：百万美元的五棱星].Detective.Conan.The.Million-dollar.Pentagram.2024.JPN.BluRay.1080p.x265.10bit.DD5.1-CMCT.mkv",
    ];

    for filename in test_files {
        println!("Testing: {}", filename);
        
        if let Some(captures) = ENHANCED_MOVIE_PATTERN.captures(filename) {
            println!("  ✓ ENHANCED_MOVIE_PATTERN matches");
            println!("    Title: {}", captures.get(1).unwrap().as_str());
            println!("    Year: {}", captures.get(2).unwrap().as_str());
            println!("    Rest: {}", captures.get(3).unwrap().as_str());
        } else if let Some(captures) = BRACKETED_ENHANCED_PATTERN.captures(filename) {
            println!("  ✓ BRACKETED_ENHANCED_PATTERN matches");
            println!("    Chinese: {}", captures.get(1).unwrap().as_str());
            println!("    English: {}", captures.get(2).unwrap().as_str());
            println!("    Year: {}", captures.get(3).unwrap().as_str());
        } else if let Some(captures) = BASIC_MOVIE_PATTERN.captures(filename) {
            println!("  ✓ BASIC_MOVIE_PATTERN matches");
            println!("    Title: {}", captures.get(1).unwrap().as_str());
            println!("    Year: {}", captures.get(2).unwrap().as_str());
        } else {
            println!("  ❌ No pattern matches");
        }
        println!();
    }
}
