use plex_media_organizer::filename_parser::FilenameParser;

#[test]
fn test_english_release_groups_parsing() {
    let parser = FilenameParser::new();

    // Test cases from the skipped English movies list
    let test_cases = vec![
        (
            "The.Avengers.2012.Bluray.2160p.x265.10bit.HDR.3Audio.mUHD-FRDS.mkv",
            "The Avengers",
        ),
        (
            "The.Dark.Knight.2008.2160p.UHD.BluRay.X265-IAMABLE.mkv",
            "The Dark Knight",
        ),
        (
            "Constantine 2005 1080p Blu-ray Remux VC-1 TrueHD 5.1 - KRaLiMaRKo.mkv",
            "Constantine",
        ),
        (
            "Blue.Beetle.2023.2160p.iTunes.WEB-DL.DDP5.1.Atmos.HDR.H.265-HHWEB.mkv",
            "Blue Beetle",
        ),
        (
            "American.Beauty.1999.REPACK.1080p.Blu-ray.DTS.x264-CtrlHD.mkv",
            "American Beauty",
        ),
        (
            "Avengers.Age.of.Ultron.2015.Bluray.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv",
            "Avengers Age of Ultron",
        ),
        (
            "Avengers.Endgame.2019.BluRay.2160p.x265.10bit.HDR.2Audios.mUHD-FRDS.mkv",
            "Avengers Endgame",
        ),
        (
            "Babylon.5.the.Road.Home.2023.2160p.UHD.BluRay.x265.10bit.HDR.DTS-HD.MA.5.1-ADE.mkv",
            "Babylon 5 the Road Home",
        ),
        (
            "The.Batman.2022.2160p.Remux.HEVC.DoVi.TrueHD.7.1-3L.mkv",
            "The Batman",
        ),
        (
            "Avengers.Infinity.War.2018.IMAX.2160p.DSNP.WEB-DL.x265.10bit.HDR.DTS-HD.MA.TrueHD.7.1.Atmos-SWTYBLZ.mkv",
            "Avengers Infinity War",
        ),
        (
            "Godzilla.Mothra.And.King.Ghidorah.Giant.Monsters.All-Out.Attack.2001.1080p.BluRay.x264-PHOBOS.mkv",
            "Godzilla Mothra And King Ghidorah Giant Monsters All-Out Attack",
        ),
        (
            "钢铁侠.Iron.Man.2008.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv",
            "Iron Man",
        ),
        (
            "Love.and.Other.Drugs.2010.BluRay.720p.DTS.x264-CHD.mkv",
            "Love and Other Drugs",
        ),
        (
            "Joker.2019.2160p.UHD.BluRay.REMUX.HEVC.TrueHD.7.1.Atmos-PTHOME.mkv",
            "Joker",
        ),
        (
            "Jurassic.Park.1993.1080P.BluRay.x264.DTS-HDChina.mkv",
            "Jurassic Park",
        ),
        (
            "Jurassic.World.Rebirth.2025.MULTi.1080p.HDTS-SyncUP.mkv",
            "Jurassic World Rebirth",
        ),
        (
            "Jurassic.Park.III.2001.1080p.BluRay.x264.DTS-HDChina.mkv",
            "Jurassic Park III",
        ),
        (
            "钢铁侠2.Iron.Man.2.2010.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv",
            "Iron Man 2",
        ),
        (
            "The.Lord.of.the.Rings.The.Two.Towers.2002.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
            "The Lord of the Rings The Two Towers",
        ),
        (
            "The.Lord.of.the.Rings.The.Fellowship.of.the.Ring.2001.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
            "The Lord of the Rings The Fellowship of the Ring",
        ),
        (
            "The.Lord.of.the.Rings.The.Return.of.the.King.2003.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
            "The Lord of the Rings The Return of the King",
        ),
        (
            "Star.Wars.Episode.I.The.Phantom.Menace.1999.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode I The Phantom Menace",
        ),
        (
            "Star.Wars.Episode.II.Attack.of.the.Clones.2002.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode II Attack of the Clones",
        ),
        (
            "Star.Wars.Episode.III.Revenge.of.the.Sith.2005.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode III Revenge of the Sith",
        ),
        (
            "Star.Wars.Episode.IV.A.New.Hope.1977.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode IV A New Hope",
        ),
        (
            "Star.Wars.Episode.V.The.Empire.Strikes.Back.1980.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode V The Empire Strikes Back",
        ),
        (
            "Star.Wars.Episode.VI.Return.of.the.Jedi.1983.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode VI Return of the Jedi",
        ),
        (
            "Star.Wars.Episode.VII.The.Force.Awakens.2015.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode VII The Force Awakens",
        ),
        (
            "Star.Wars.Episode.VIII.The.Last.Jedi.2017.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode VIII The Last Jedi",
        ),
        (
            "Star.Wars.Episode.IX.The.Rise.of.Skywalker.2019.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Star Wars Episode IX The Rise of Skywalker",
        ),
        (
            "Rogue.One.A.Star.Wars.Story.2016.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Rogue One A Star Wars Story",
        ),
        (
            "Solo.A.Star.Wars.Story.2018.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD7.1-HDChina.mkv",
            "Solo A Star Wars Story",
        ),
        (
            "Transformers 2007 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
            "Transformers",
        ),
        (
            "Transformers Age of Extinction 2014 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
            "Transformers Age of Extinction",
        ),
        (
            "Transformers Dark of the Moon 2011 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
            "Transformers Dark of the Moon",
        ),
        (
            "Transformers Revenge of the Fallen 2009 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
            "Transformers Revenge of the Fallen",
        ),
        (
            "Transformers The Last Knight 2017 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
            "Transformers The Last Knight",
        ),
        (
            "Transformers.One.2024.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "Transformers One",
        ),
        (
            "The.Fast.and.the.Furious.2001.1080p.UHD.BluRay.DDP7.1.HDR.x265-NCmt.mkv",
            "The Fast and the Furious",
        ),
        (
            "The.Fast.and.the.Furious.Tokyo.Drift.2006.1080p.UHD.BluRay.DDP7.1.HDR.x265-NCmt.mkv",
            "The Fast and the Furious Tokyo Drift",
        ),
        (
            "The.Shawshank.Redemption.1994.1080p.BluRay.DD+.5.1.x264-c0kE.mkv",
            "The Shawshank Redemption",
        ),
        (
            "Forrest Gump 1994 720p BluRay DD5.1 x264-LoRD.mkv",
            "Forrest Gump",
        ),
        (
            "A.I.Artificial.Intelligence.2001.1080p.BluRay.x264-EbP.mkv",
            "A I Artificial Intelligence",
        ),
        (
            "Children.of.Men.2006.1080p.BluRay.DD5.1.x264-SA89.mkv",
            "Children of Men",
        ),
        (
            "Frequency.2000.1080p.BluRay.DTS.x264-FANDANGO.mkv",
            "Frequency",
        ),
        (
            "The.Wolf.of.Wall.Street.2013.1080p.BluRay.DTS.x264-DON.mkv",
            "The Wolf of Wall Street",
        ),
        (
            "Pearl.Harbor.2001.1080p.Bluray.DTS.x264-D-Z0N3.mkv",
            "Pearl Harbor",
        ),
        (
            "Stranger.Than.Paradise.1984.BluRay.720p.x264.AAC1.0-PTer.mkv",
            "Stranger Than Paradise",
        ),
        (
            "The.Beasts.(2022).1080p.BluRay.DD+5.1.x264-DON.mkv",
            "The Beasts",
        ),
        (
            "Schindler's.List.1993.BluRay.2160p.x265.10bit.HDR.5Audio.mUHD-FRDS.mkv",
            "Schindler's List",
        ),
        (
            "The.Banshees.of.Inisherin.2022.2160p.WEB-DL.DDP5.1.DV.HDR.H.265-FLUX.mkv",
            "The Banshees of Inisherin",
        ),
        (
            "The.Hunger.Games.The.Ballad.of.Songbirds.and.Snakes.2023.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "The Hunger Games The Ballad of Songbirds and Snakes",
        ),
        (
            "Dune.Part.Two.2024.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "Dune Part Two",
        ),
        (
            "Killers.of.the.Flower.Moon.2023.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "Killers of the Flower Moon",
        ),
        (
            "Guardians.of.the.Galaxy.Vol.3.2023.2160p.WEB-DL.DDP5.1.Atmos.HDR.DV.HEVC-CMRG.mkv",
            "Guardians of the Galaxy Vol 3",
        ),
        (
            "Fast.X.2023.2160p.MA.WEB-DL.DDP5.1.Atmos.HDR.DV.HEVC-CMRG.mkv",
            "Fast X",
        ),
        (
            "Dungeons.and.Dragons.Honor.Among.Thieves.2023.1080p.AMZN.WEBRip.DDP5.1.x264-Kitsune.mkv",
            "Dungeons and Dragons Honor Among Thieves",
        ),
        (
            "Glass.Onion.A.Knives.Out.Mystery.2022.2160p.NF.WEB-DL.DDP5.1.Atmos.HDR.DV-MZABI.mkv",
            "Glass Onion A Knives Out Mystery",
        ),
        (
            "Barbie.2023.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-MZABARBiE.mkv",
            "Barbie",
        ),
        (
            "The.Killer.2023.1080p.NF.WEB-DL.DDP5.1.H.264-playWEB.mkv",
            "The Killer",
        ),
        (
            "The.Marvels.2023.1080p.WEB-DL.DDP5.1.Atmos.H.264-FLUX.mkv",
            "The Marvels",
        ),
        (
            "Free.Guy.2021.2160p.4K.WEB.x265.10bit.AAC5.1-[YTS.MX].mkv",
            "Free Guy",
        ),
        (
            "A.Walk.to.Remember.2002.1080p.BluRay.x264.YIFY.mp4",
            "A Walk to Remember",
        ),
        ("Klaus.2019.Netflix.WEB-DL.1080p.HEVC.DDP-AREY.mkv", "Klaus"),
        ("Land.Of.Bad.2024.1080p.WEB.H264-RABiDS.mkv", "Land Of Bad"),
        (
            "Kingdom.Of.The.Planet.Of.The.Apes.2024.HDR.2160p.WEB.h265-ETHEL.mkv",
            "Kingdom Of The Planet Of The Apes",
        ),
        (
            "Leon.The.Professional.1994.BluRay.2160p.x265.10bit.HDR.3Audio.mUHD-FRDS.mkv",
            "Leon The Professional",
        ),
        ("Moon 2009 1080p BluRay DD5.1 x264-RightSiZE.mkv", "Moon"),
        (
            "Now.You.See.Me.2013.2160p.UHD.BluRay.X265-IAMABLE.mkv",
            "Now You See Me",
        ),
        (
            "Oppenheimer.2023.1080p.Blu-ray.Remux.AVC.DTS-HD.MA.5.1-CiNEPHiLES.mkv",
            "Oppenheimer",
        ),
        (
            "Poirot.Murder.on.the.Orient.Express.2010.720p.BluRay.x264-DON.mkv",
            "Poirot Murder on the Orient Express",
        ),
        (
            "Roald.Dahls.Matilda.The.Musical.2022.1080p.NF.WEB-DL.DDP5.1.Atmos.H.264-SMURF.mkv",
            "Roald Dahls Matilda The Musical",
        ),
        (
            "The.Foreigner.2017.Hybrid.BluRay.1080p.DTS.x264-MTeam.mkv",
            "The Foreigner",
        ),
        (
            "The.Fog.of.War.2003.1080p.AMZN.WEB-DL.DDP5.1.x264-ABM.mkv",
            "The Fog of War",
        ),
        (
            "Greyhound.2020.1080p.ATVP.WEB-DL.DDP5.1.Atmos.H.264-MZABI.mkv",
            "Greyhound",
        ),
        (
            "The.Man.From.Earth.2007.BluRay.iPad.720p.x264.AAC-BYRPAD.mp4",
            "The Man From Earth",
        ),
        (
            "Air.Force.One.1997.REPACK.1080p.UHD.BluRay.DDP.7.1.DoVi.HDR10.x265-c0kE.mkv",
            "Air Force One",
        ),
        ("Les Misérables.mkv", "Les Misérables"),
        ("The Story of the Story.mkv", "The Story of the Story"),
        (
            "Star Trek - Jerome Bixby's Sci Fi Legacy.mkv",
            "Star Trek - Jerome Bixby's Sci Fi Legacy",
        ),
        (
            "The Man from Earth - Legacy.mkv",
            "The Man from Earth - Legacy",
        ),
        ("Wish.2023.1080p.WEBRip.x264.AAC5.1-[YTS.MX].mp4", "Wish"),
        ("Home.2015.720p.BluRay.x264.YIFY.mp4", "Home"),
        (
            "How.To.Train.Your.Dragon.The.Hidden.World.2019.1080p.BluRay.x264-[YTS.AM].mp4",
            "How To Train Your Dragon The Hidden World",
        ),
        (
            "How.to.Train.Your.Dragon.2.2014.1080p.BluRay.x264.YIFY.mp4",
            "How to Train Your Dragon 2",
        ),
        ("Inside.Out.2015.720p.BluRay.x264-SPARKS.mkv", "Inside Out"),
        (
            "Penguins.of.Madagascar.2014.1080p.BluRay.x264.YIFY.mp4",
            "Penguins of Madagascar",
        ),
        ("Cars 2 2011 Hybrid 1080p BluRay x264-EbP.mkv", "Cars 2"),
        (
            "Inside.Out.2.2024.1080p.WEBRip.DDP5.1.x265.10bit-GalaxyRG265.mkv",
            "Inside Out 2",
        ),
        ("Cars.2006.1080p.BluRay.DTS.x264-Geek.mkv", "Cars"),
        (
            "Despicable Me 4 2024 2160p WEB-DL DDP5 1 Atmos DV HDR H 265-FLUX.mkv",
            "Despicable Me 4",
        ),
        (
            "Cars.3.2017.1080p.UHD.BluRay.DD5.1.HDR.x265-CtrlHD.mkv",
            "Cars 3",
        ),
        (
            "Elemental.2023.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "Elemental",
        ),
        (
            "Fantastic.Beasts.The.Crimes.of.Grindelwald.2018.Extended.Cut.1080p.BluRay.DTS.x264-TayTO.mkv",
            "Fantastic Beasts The Crimes of Grindelwald",
        ),
        (
            "Fantastic.Beasts.The.Secrets.of.Dumbledore.2022.2160p.HMAX.WEB-DL.DDP5.1.Atmos.HDR.H.265-SMURF.mkv",
            "Fantastic Beasts The Secrets of Dumbledore",
        ),
        ("Aladdin.1992.1080p.BluRay.DTS.x264-DON.mkv", "Aladdin"),
        ("Mulan.1998.1080p.BluRay.x264-nikt0.mkv", "Mulan"),
        ("Flow.2024.1080p.AMZN.WEB-DL.DDP5.1.H.264-FLUX.mkv", "Flow"),
        (
            "Finding Nemo 2003 1080p BluRay DD5.1 x264-EbP.mkv",
            "Finding Nemo",
        ),
        (
            "Guillermo.del.Toros.Pinocchio.2022.1080p.NF.WEB-DL.DDP5.1.Atmos.H.264-SMURF.mkv",
            "Guillermo del Toros Pinocchio",
        ),
        (
            "Frozen.II.2019.RERIP.UHD.BluRay.2160p.10bit.HDR.4Audio.TrueHD(Atmos).7.1.x265-beAst.mkv",
            "Frozen II",
        ),
        (
            "Frozen.2013.UHD.BluRay.2160p.10bit.HDR.4Audio.TrueHD(Atmos).7.1.DTS-HD.MA.7.1.x265-beAst (1).mkv",
            "Frozen",
        ),
        (
            "Kung.Fu.Panda.4.2024.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "Kung Fu Panda 4",
        ),
        (
            "Kung.Fu.Panda.2008.720p.BluRay.DTS.x264-FoRM.mkv",
            "Kung Fu Panda",
        ),
        (
            "Secrects of the furious five.mkv",
            "Secrects of the furious five",
        ),
        (
            "Kung Fu Panda 2 2011 720p BluRay x264-EbP.mkv",
            "Kung Fu Panda 2",
        ),
        (
            "Lightyear.2022.2160p.MA.WEB-DL.DDP5.1.HDR.HEVC-PaODEQUEiJO.mkv",
            "Lightyear",
        ),
        (
            "Kung.Fu.Panda.3.2016.720p.BluRay.DTS-ES.x264-CRiME.mkv",
            "Kung Fu Panda 3",
        ),
        (
            "New.Gods.Nezha.Reborn.2021.2160p.WEB-DL.HEVC.DDP5.1-HVAC.mkv",
            "New Gods Nezha Reborn",
        ),
        (
            "Monsters.Inc.2001.UHD.BluRay.2160p.x265.HDR.TrueHD.7.1.RERIP.mUHD-FRDS.mkv",
            "Monsters Inc",
        ),
        (
            "Pinocchio.2022.2160p.DSNP.WEB-DL.DDP5.1.Atmos.HDR.H.265-SMURF.mkv",
            "Pinocchio",
        ),
        (
            "New.Gods.Yang.Jian.2022.2160p.WEB-DL.H265.120FPS.10bit.AAC-MaoZhan.mkv",
            "New Gods Yang Jian",
        ),
        (
            "Incredibles.2.2018.1080p.UHD.BluRay.DDP7.1.HDR.x265-NCmt.mkv",
            "Incredibles 2",
        ),
        (
            "Ne Zha zhi mo tong jiang shi 2019 1080p UHD BluRay DD 5.1 HDR x265-DON.mkv",
            "Ne Zha zhi mo tong jiang shi",
        ),
        ("Sing.2016.1080p.BluRay.DD5.1.x264-VietHD.mkv", "Sing"),
        ("Storks 2016 1080p BluRay x264 AC3-JYK.mkv", "Storks"),
        ("Smallfoot (2018).mkv", "Smallfoot"),
        (
            "The.Amazing.Maurice.2022.1080p.NOW.WEB-DL.DDP5.1.H.264-SMURF.mkv",
            "The Amazing Maurice",
        ),
        (
            "Sonic.the.Hedgehog.2020.1080p.UHD.BluRay.DDP7.1.HDR.x265-NCmt.mkv",
            "Sonic the Hedgehog",
        ),
        (
            "Troll.2022.1080p.NF.WEB-DL.DDP5.1.Atmos.H.264-playWEB.mkv",
            "Troll",
        ),
        ("Up.2009.1080p.BluRay.DTS-ES.Hi10P.x264-DON.mkv", "Up"),
        (
            "Smurfs The Lost Village 2017 1080p HULU WEB-DL DDP 5 1 H 264-PiRaTeS.mkv",
            "Smurfs The Lost Village",
        ),
        (
            "The.Incredibles.2004.720p.BluRay.x264-EbP.mkv",
            "The Incredibles",
        ),
        (
            "Puss.in.Boots.The.Last.Wish.2022.REPACK.R2160p.MA.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "Puss in Boots The Last Wish",
        ),
        (
            "The.Wild.Robot.2024.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "The Wild Robot",
        ),
        ("Tangled.2010.1080p.BluRay.x264.YIFY.mp4", "Tangled"),
    ];

    println!("🔍 Testing English movies with release group parsing...\n");

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
                    && !parsed.title.contains("FRDS")
                    && !parsed.title.contains("CHD")
                    && !parsed.title.contains("HDChina")
                    && !parsed.title.contains("CtrlHD")
                    && !parsed.title.contains("IAMABLE")
                    && !parsed.title.contains("KRaLiMaRKo")
                    && !parsed.title.contains("HHWEB")
                    && !parsed.title.contains("SWTYBLZ")
                    && !parsed.title.contains("ADE")
                    && !parsed.title.contains("PHOBOS")
                    && !parsed.title.contains("PTHOME")
                    && !parsed.title.contains("SyncUP")
                    && !parsed.title.contains("mUHD")
                    && !parsed.title.contains("4Audio")
                    && !parsed.title.contains("3Audio")
                    && !parsed.title.contains("2Audio")
                    && !parsed.title.contains("5Audio")
                    && !parsed.title.contains("REPACK")
                    && !parsed.title.contains("Remux")
                    && !parsed.title.contains("VC-1")
                    && !parsed.title.contains("DoVi")
                    && !parsed.title.contains("HDR10")
                    && !parsed.title.contains("EDR")
                    && !parsed.title.contains("MULTi")
                    && !parsed.title.contains("HDTS")
                    && !parsed.title.contains("Extended")
                    && !parsed.title.contains("IMAX")
                    && !parsed.title.contains("DSNP")
                    && !parsed.title.contains("TrueHD")
                    && !parsed.title.contains("Atmos")
                    && !parsed.title.contains("DTS-HD")
                    && !parsed.title.contains("MA")
                    && !parsed.title.contains("HEVC")
                    && !parsed.title.contains("HDR")
                    && !parsed.title.contains("YIFY")
                    && !parsed.title.contains("YTS")
                    && !parsed.title.contains("MX")
                    && !parsed.title.contains("AM")
                    && !parsed.title.contains("SPARKS")
                    && !parsed.title.contains("HiDt")
                    && !parsed.title.contains("Geek")
                    && !parsed.title.contains("TayTO")
                    && !parsed.title.contains("nikt0")
                    && !parsed.title.contains("beAst")
                    && !parsed.title.contains("FoRM")
                    && !parsed.title.contains("CRiME")
                    && !parsed.title.contains("HVAC")
                    && !parsed.title.contains("MaoZhan")
                    && !parsed.title.contains("VietHD")
                    && !parsed.title.contains("JYK")
                    && !parsed.title.contains("PiRaTeS")
                    && !parsed.title.contains("GalaxyRG265")
                    && !parsed.title.contains("PaODEQUEiJO")
                    && !parsed.title.contains("Silence")
                    && !parsed.title.contains("LoRD")
                    && !parsed.title.contains("SA89")
                    && !parsed.title.contains("FANDANGO")
                    && !parsed.title.contains("DON")
                    && !parsed.title.contains("D-Z0N3")
                    && !parsed.title.contains("PTer")
                    && !parsed.title.contains("ABM")
                    && !parsed.title.contains("MZABI")
                    && !parsed.title.contains("BYRPAD")
                    && !parsed.title.contains("c0kE")
                    && !parsed.title.contains("NCmt")
                    && !parsed.title.contains("MTeam")
                    && !parsed.title.contains("playWEB")
                    && !parsed.title.contains("FLUX")
                    && !parsed.title.contains("CMRG")
                    && !parsed.title.contains("MZABARBiE")
                    && !parsed.title.contains("SMURF")
                    && !parsed.title.contains("AREY")
                    && !parsed.title.contains("RABiDS")
                    && !parsed.title.contains("ETHEL")
                    && !parsed.title.contains("RightSiZE")
                    && !parsed.title.contains("CiNEPHiLES")
                    && !parsed.title.contains("Kitsune")
                    && !parsed.title.contains("RERIP")
                    && !parsed.title.contains("Hybrid")
                    && !parsed.title.contains("ES")
                    && !parsed.title.contains("HMAX")
                    && !parsed.title.contains("DSNP")
                    && !parsed.title.contains("NOW")
                    && !parsed.title.contains("ATVP")
                    && !parsed.title.contains("HULU")
                    && !parsed.title.contains("120FPS")
                    && !parsed.title.contains("4K")
                    && !parsed.title.contains("WEB")
                    && !parsed.title.contains("WEBRip")
                    && !parsed.title.contains("AMZN")
                    && !parsed.title.contains("NF")
                    && !parsed.title.contains("Netflix")
                    && !parsed.title.contains("iTunes")
                    && !parsed.title.contains("UHD")
                    && !parsed.title.contains("Blu-ray")
                    && !parsed.title.contains("Bluray")
                    && !parsed.title.contains("BluRay")
                    && !parsed.title.contains("DD5")
                    && !parsed.title.contains("DD+")
                    && !parsed.title.contains("AC3")
                    && !parsed.title.contains("AAC5")
                    && !parsed.title.contains("AAC1")
                    && !parsed.title.contains("AAC")
                    && !parsed.title.contains("10bit")
                    && !parsed.title.contains("DV")
                    && !parsed.title.contains("MP4")
                    && !parsed.title.contains("MKV");

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
