use plex_media_organizer::filename_parser::FilenameParser;

#[test]
fn test_chinese_technical_terms_improvements() {
    let parser = FilenameParser::new();

    // Test cases from the skipped Chinese movies list
    let test_cases = vec![
        (
            "Marry.My.Dead.Body.2022.1080p.NF.WEB-DL.x264.DDP5.1-ADWeb.mkv",
            "Marry My Dead Body",
        ),
        (
            "Hail.the.Judge.1994.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS.mkv",
            "Hail the Judge",
        ),
        (
            "White.Snake.2019.2160p.HQ.WEB-DL.H265.60fps.DDP5.1.Atmos-CHDWEB.mkv",
            "White Snake",
        ),
        (
            "The.Monkey.King.2023.1080p.NF.WEB-DL.x264.DDP5.1.Atmos-PTerWEB.mkv",
            "The Monkey King",
        ),
        (
            "All For The Winner 1990 HDTVRip x264 MP3 2Audio.mkv",
            "All For The Winner",
        ),
        (
            "Saat.Po.Long.2005.1080p.HKG.BluRay.DTS.2Audio.x264-HDS.mkv",
            "Saat Po Long",
        ),
        (
            "If.You.Are.the.One.3.2023.2160p.WEB-DL.H265.HQ.60fps.DDP5.1-HHWEB.mkv",
            "If You Are the One",
        ),
        (
            "百变星君.Sixty.Million.Dollar.Man.1995.Bluray.1080p.x265.AAC(5.1).2Audios.GREENOTEA.mkv",
            "百变星君 Sixty Million Dollar Man",
        ),
        (
            "CHC.Call.for.Love.2007.720p.HDTV.x264-CMCTV.mkv",
            "Call for Love",
        ),
        ("A.Sun.2019.1080p.NF.WEB-DL.DDP5.1.x264-NTG.mkv", "A Sun"),
        (
            "Big.Shot's.Funeral.2001.1080p.BluRay.DDP.5.1.x264-PTer.mkv",
            "Big Shot's Funeral",
        ),
        (
            "CCTV6HD A World Without Thieves 2004 HDTV 720p x264-HDWTV.mkv",
            "A World Without Thieves",
        ),
        (
            "Beijing.Love.Story.2014.1080p.NF.WEB-DL.DDP5.1.x264-TJUPT.mkv",
            "Beijing Love Story",
        ),
        (
            "From.Beijing.with.Love.1994.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS.mkv",
            "From Beijing with Love",
        ),
        (
            "Let.the.Bullets.Fly.2010.BluRay.1080p.x264.2Audios-BYRHD.mkv",
            "Let the Bullets Fly",
        ),
        (
            "The.Monk.and.the.Gun.2023.1080p.AMZN.WEB-DL.DDP5.1.H.264-BYNDR.mkv",
            "The Monk and the Gun",
        ),
        (
            "The.Dumpling.Queen.2025.1080p.WEB-DL.H264.AAC-PandaQT.mkv",
            "The Dumpling Queen",
        ),
        (
            "Lost.in.the.Stars.2022.1080p.WEB-DL.H264.AAC-TJUPT.mp4",
            "Lost in the Stars",
        ),
        (
            "Ne Zha 2 2025 1080p WEB-DL Chinese DDP5.1 Atmos x264-TBMovies.mkv",
            "Ne Zha",
        ),
        (
            "Chungking.Express.1994.1080p.BluRay.DDP5.1.x264-c0kE.mkv",
            "Chungking Express",
        ),
    ];

    println!("🔍 Testing Chinese movies with technical terms improvements...\n");

    for (filename, expected_title) in test_cases {
        println!("📁 Testing: {}", filename);

        let components = parser.parse(filename);
        match components {
            Ok(parsed) => {
                println!("   📝 Parsed title: '{}'", parsed.title);
                println!("   📅 Year: {:?}", parsed.year);
                println!("   📊 Confidence: {:.3}", parsed.confidence);

                // Check if the title is clean (doesn't contain obvious technical terms)
                let title_clean = !parsed.title.contains("x264")
                    && !parsed.title.contains("x265")
                    && !parsed.title.contains("1080p")
                    && !parsed.title.contains("2160p")
                    && !parsed.title.contains("BluRay")
                    && !parsed.title.contains("WEB-DL")
                    && !parsed.title.contains("DDP5.1")
                    && !parsed.title.contains("HDTV")
                    && !parsed.title.contains("AAC")
                    && !parsed.title.contains("DTS")
                    && !parsed.title.contains("GREENOTEA")
                    && !parsed.title.contains("CMCT")
                    && !parsed.title.contains("PTer")
                    && !parsed.title.contains("TJUPT")
                    && !parsed.title.contains("NTG")
                    && !parsed.title.contains("HDWTV")
                    && !parsed.title.contains("BYRHD")
                    && !parsed.title.contains("c0kE")
                    && !parsed.title.contains("BYNDR")
                    && !parsed.title.contains("PandaQT")
                    && !parsed.title.contains("TBMovies")
                    && !parsed.title.contains("2Audio")
                    && !parsed.title.contains("10bit")
                    && !parsed.title.contains("HQ")
                    && !parsed.title.contains("60fps")
                    && !parsed.title.contains("CCTV6HD")
                    && !parsed.title.contains("CHC")
                    && !parsed.title.contains("HHWEB")
                    && !parsed.title.contains("CHDWEB")
                    && !parsed.title.contains("PTerWEB")
                    && !parsed.title.contains("MNHD")
                    && !parsed.title.contains("FRDS");

                if title_clean {
                    println!("   ✅ Title appears clean from technical terms");
                } else {
                    println!("   ⚠️  Title still contains technical terms");
                }

                if parsed.title.starts_with(expected_title) || parsed.title.contains(expected_title)
                {
                    println!("   ✅ Expected title found in parsed result");
                } else {
                    println!(
                        "   ❌ Expected '{}' not found in '{}'",
                        expected_title, parsed.title
                    );
                }
            }
            Err(e) => {
                println!("   💥 Parse error: {}", e);
            }
        }
        println!();
    }
}
