use plex_media_organizer::{filename_parser::FilenameParser, tmdb_client::TmdbClient};

#[tokio::test]
async fn test_skipped_movies_full_analysis() {
    // Load TMDB API key from environment
    let api_key =
        std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY environment variable required");
    let tmdb_client = TmdbClient::new(api_key);
    let parser = FilenameParser::new();

    // Test cases from the skipped movies list
    let test_cases = vec![
        (
            "Hail.the.Judge.1994.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS.mkv",
            Some(1994),
        ),
        (
            "Saat.Po.Long.2005.1080p.HKG.BluRay.DTS.2Audio.x264-HDS.mkv",
            Some(2005),
        ),
        (
            "Marry.My.Dead.Body.2022.1080p.NF.WEB-DL.x264.DDP5.1-ADWeb.mkv",
            Some(2022),
        ),
        (
            "Fight.Back.to.School.3.1993.BDRip.X264.DualAudio.iNT-TLF.mkv",
            Some(1993),
        ),
        (
            "White.Snake.2019.2160p.HQ.WEB-DL.H265.60fps.DDP5.1.Atmos-CHDWEB.mkv",
            Some(2019),
        ),
        (
            "The.Monkey.King.2023.1080p.NF.WEB-DL.x264.DDP5.1.Atmos-PTerWEB.mkv",
            Some(2023),
        ),
        (
            "All For The Winner 1990 HDTVRip x264 MP3 2Audio.mkv",
            Some(1990),
        ),
        (
            "消失的她.Lost.in.the.Stars.2022.1080p.WEB-DL.mkv",
            Some(2022),
        ),
        (
            "If.You.Are.the.One.3.2023.2160p.WEB-DL.H265.HQ.60fps.DDP5.1-HHWEB.mkv",
            Some(2023),
        ),
        (
            "狙击手.Snipers.2022.60fps.2160p.WEB-DL.DDP5.1.AAC.H265-HDSWEB.mkv",
            Some(2022),
        ),
        (
            "家有喜事(修复加长版).All's.Well.End's.Well.1992.EXTENDED.Bluray.1080p.x265.AAC(5.1).2Audios.GREENOTEA.mkv",
            Some(1992),
        ),
        (
            "Royal.Tramp.1992.Bluray.1080p.x265.AAC(5.1).2Audios.GREENOTEA.mkv",
            Some(1992),
        ),
        (
            "2.Royal.Tramp.II.1992.Bluray.1080p.x265.AAC(5.1).2Audios.GREENOTEA.mkv",
            Some(1992),
        ),
        (
            "狄仁杰之幽兵借路.Ghost.Soldier.Borrowed.2023.WEB-DL.2160p.HEVC.AAC-ZmWeb.mp4",
            Some(2023),
        ),
        (
            "[戏梦巴黎].The.Dreamers.2003.ESP.BluRay.1080p.x264.DTS-CMCT.mkv",
            Some(2003),
        ),
        (
            "莫斯科行动.Moscow.Mission.2023.1080p.WEB-DL.AVC.AAC-HDVWEB.mp4",
            Some(2023),
        ),
        (
            "[怒火·重案].Raging.Fire.2021.GBR.UHD.BluRay.2160p.x265.10bit.HDR.DTS-HD.MA.7.1.2Audios-CMCT.mkv",
            Some(2021),
        ),
        (
            "[热搜].Trending.Topic.2023.2160p.WEB-DL.HEVC.10bit.DDP5.1.4Audios-NukeHD.mp4",
            Some(2023),
        ),
        (
            "[雏菊(导演剪辑版)].Daisy.2006.DVDRip.x264.DTS-CMCT.mkv",
            Some(2006),
        ),
        (
            "Beijing.Love.Story.2014.1080p.NF.WEB-DL.DDP5.1.x264-TJUPT.mkv",
            Some(2014),
        ),
        (
            "百变星君.Sixty.Million.Dollar.Man.1995.Bluray.1080p.x265.AAC(5.1).2Audios.GREENOTEA.mkv",
            Some(1995),
        ),
        (
            "CHC.Call.for.Love.2007.720p.HDTV.x264-CMCTV.mkv",
            Some(2007),
        ),
        ("A.Sun.2019.1080p.NF.WEB-DL.DDP5.1.x264-NTG.mkv", Some(2019)),
        (
            "Big.Shot's.Funeral.2001.1080p.BluRay.DDP.5.1.x264-PTer.mkv",
            Some(2001),
        ),
        (
            "CCTV6HD A World Without Thieves 2004 HDTV 720p x264-HDWTV.mkv",
            Some(2004),
        ),
        (
            "[青蛇].Green.Snake.1993.DVDRip.x264.2Audio.AC3.INT-NowOur.mkv",
            Some(1993),
        ),
        (
            "Dust.To.Dust.2023.1080p.WEB-DL.H264.AAC-TJUPT.mp4",
            Some(2023),
        ),
        (
            "City.of.Life.and.Death.2009.BluRay.720p.DTS.x264-HDxT.mkv",
            Some(2009),
        ),
        (
            "The.Dumpling.Queen.2025.1080p.WEB-DL.H264.AAC-PandaQT.mkv",
            Some(2025),
        ),
        (
            "Du.shen.2.AKA.God.of.Gamblers.Return.1994.DVDRip.x264-HANDJOB.mkv",
            Some(1994),
        ),
        (
            "Fight Back to School 1991 720p BluRay DD5.1 Hi10P x264-npuer.mkv",
            Some(1991),
        ),
        (
            "CCTV6HD.Crazy.Stone.2006.720p.HDTV.x264-HDWTV.mkv",
            Some(2006),
        ),
        (
            "Hung.Hei.Kwun.Siu.Lam.ng.zou.AKA.The.New.Legend.of.Shaolin.1994.720p.BluRay.x264-HANDJOB.mkv",
            Some(1994),
        ),
        (
            "The.Monk.and.the.Gun.2023.1080p.AMZN.WEB-DL.DDP5.1.H.264-BYNDR.mkv",
            Some(2023),
        ),
        (
            "The.Banquet.2006.1080p.BluRay.DTS.5.1.x264-PTer.mkv",
            Some(2006),
        ),
        (
            "Love.Will.Tear.Us.Apart.2021.1080p.WEBRip.x264.AAC-[YTS.MX].mp4",
            Some(2021),
        ),
        (
            "Lost.in.the.Stars.2022.1080p.WEB-DL.H264.AAC-TJUPT.mp4",
            Some(2022),
        ),
        (
            "Chungking.Express.1994.1080p.BluRay.DDP5.1.x264-c0kE.mkv",
            Some(1994),
        ),
        (
            "Let.the.Bullets.Fly.2010.BluRay.1080p.x264.2Audios-BYRHD.mkv",
            Some(2010),
        ),
        (
            "Star.Movie-HD.God.of.Gamblers.III.The.Early.Stage.1996.720p.HDTV.x264-CMCTV.mkv",
            Some(1996),
        ),
        (
            "Sien.Nui.Yau.Wan.AKA.A.Chinese.Ghost.Story.1987.1080p.BluRay.x264-PTer.mkv",
            Some(1987),
        ),
        (
            "From.Beijing.with.Love.1994.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS.mkv",
            Some(1994),
        ),
        (
            "White.Snake.Afloat.2024.1080p.BluRay.x264.Atmos.TrueHD7.1-WiKi.mkv",
            Some(2024),
        ),
        (
            "Ne Zha 2 2025 1080p WEB-DL Chinese DDP5.1 Atmos x264-TBMovies.mkv",
            Some(2025),
        ),
    ];

    println!("🔍 Analyzing skipped movies with TMDB search...\n");
    println!("Current confidence threshold: 0.7");
    println!("Current minimum TMDB score: 50.0\n");

    for (filename, expected_year) in test_cases {
        println!("📁 Testing: {}", filename);

        // Parse filename
        let components = parser.parse(filename);
        match components {
            Ok(parsed) => {
                println!("   📝 Parsed title: '{}'", parsed.title);
                println!("   📅 Parsed year: {:?}", parsed.year);

                // Search TMDB
                let search_result = tmdb_client
                    .enhanced_search(&parsed.title, parsed.year)
                    .await;
                match search_result {
                    Ok(Some(tmdb_result)) => {
                        println!("   ✅ TMDB Match Found:");
                        println!("      🎬 Title: '{}'", tmdb_result.movie.title);
                        if let Some(original) = &tmdb_result.movie.original_title {
                            println!("      🌍 Original: '{}'", original);
                        }
                        if let Some(date) = &tmdb_result.movie.release_date {
                            println!("      📅 Release: {}", date);
                        }
                        println!("      📊 Confidence: {:.3}", tmdb_result.confidence_score);

                        if tmdb_result.confidence_score < 0.7 {
                            println!("      ❌ BELOW THRESHOLD (0.7) - Would be skipped");
                        } else {
                            println!("      ✅ ABOVE THRESHOLD - Would be organized");
                        }
                    }
                    Ok(None) => {
                        println!("   ❌ No TMDB match found - Would be skipped");
                    }
                    Err(e) => {
                        println!("   💥 TMDB search error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("   💥 Parse error: {}", e);
            }
        }
        println!();
    }
}
