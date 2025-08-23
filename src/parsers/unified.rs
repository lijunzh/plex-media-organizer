//! Unified movie parser that integrates all modular parsing components

use crate::config::AppConfig;
use anyhow::Result;

use super::{
    extraction::TitleExtractor,
    patterns::{AnimeDetector, SeriesDetector, UnifiedPatternDetector},
    types::{FilenameComponents, ParserResult},
};

/// Unified movie parser that combines all parsing components
#[derive(Debug, Clone)]
pub struct UnifiedMovieParser {
    pattern_detector: UnifiedPatternDetector,
    series_detector: SeriesDetector,
    anime_detector: AnimeDetector,
    legacy_title_extractor: TitleExtractor,
    config: Option<AppConfig>,
}

impl Default for UnifiedMovieParser {
    fn default() -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: None,
        }
    }
}

impl UnifiedMovieParser {
    /// Create a new unified movie parser
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new unified movie parser with configuration
    pub fn with_config(config: AppConfig) -> Self {
        Self {
            pattern_detector: UnifiedPatternDetector::default(),
            series_detector: SeriesDetector::new(),
            anime_detector: AnimeDetector::new(),
            legacy_title_extractor: TitleExtractor::new(),
            config: Some(config),
        }
    }

    /// Parse a filename using the unified parser
    pub fn parse(&self, filename: &str) -> Result<ParserResult<FilenameComponents>> {
        // Get configuration parameters
        let language_codes = self.get_language_codes();
        let common_words = self.get_common_words();
        let known_titles = self.get_known_titles();
        let technical_japanese_terms = self.get_technical_japanese_terms();
        let release_groups = self.get_release_groups();
        let technical_terms = self.get_technical_terms();

        // Remove file extension
        let filename_without_ext = self.remove_extension(filename);

        // Tokenize the filename
        let tokens = self.tokenize(&filename_without_ext);

        // Extract components using pattern detection
        let year = self
            .pattern_detector
            .technical()
            .detect_year(&filename_without_ext);
        let quality = self
            .pattern_detector
            .technical()
            .detect_quality(&filename_without_ext);
        let source = self
            .pattern_detector
            .technical()
            .detect_source(&filename_without_ext);
        let audio = self
            .pattern_detector
            .technical()
            .detect_audio(&filename_without_ext);
        let codec = self
            .pattern_detector
            .technical()
            .detect_codec(&filename_without_ext);
        let group = None; // TODO: Implement group detection
        let language = None; // TODO: Implement language detection

        // Extract title and original title using legacy method for compatibility
        let (title, original_title) = self
            .legacy_title_extractor
            .extract_title_and_original_with_params(
                &tokens,
                &year,
                &quality,
                &source,
                &audio,
                &codec,
                &group,
                &language,
                filename,
                &common_words,
                &known_titles,
                &technical_japanese_terms,
                &language_codes,
                &technical_terms,
                &release_groups,
            );

        // Detect series information (for future use)
        let _series_info = self.series_detector.detect_series(filename);

        // Detect anime information (for future use)
        let _anime_info = self.anime_detector.detect_anime_pattern(&title, filename);

        // Detect language information (for future use)
        let _language_info = self
            .pattern_detector
            .language()
            .detect_language(&filename_without_ext);

        // Calculate confidence score
        let confidence = self.calculate_confidence(&tokens, &year, &quality, &source);

        // Create filename components
        let components = FilenameComponents {
            title,
            original_title,
            year,
            quality,
            source,
            language,
            audio,
            codec,
            group,
            confidence,
        };

        // Create parser result
        let result = ParserResult::new(components, confidence, "unified".to_string());

        Ok(result)
    }

    /// Remove file extension from filename
    fn remove_extension(&self, filename: &str) -> String {
        if let Some(dot_pos) = filename.rfind('.') {
            filename[..dot_pos].to_string()
        } else {
            filename.to_string()
        }
    }

    /// Tokenize filename into parts
    fn tokenize(&self, filename: &str) -> Vec<String> {
        filename
            .split(['.', '_', '-', ' '])
            .filter(|part| !part.is_empty())
            .map(|part| part.to_string())
            .collect()
    }

    /// Calculate confidence score for parsing result
    fn calculate_confidence(
        &self,
        tokens: &[String],
        year: &Option<u32>,
        quality: &Option<String>,
        source: &Option<String>,
    ) -> f32 {
        let mut confidence: f32 = 0.0;

        // Base confidence
        confidence += 0.2;

        // Year found
        if year.is_some() {
            confidence += 0.3;
        }

        // Quality found
        if quality.is_some() {
            confidence += 0.2;
        }

        // Source found
        if source.is_some() {
            confidence += 0.2;
        }

        // Title length (reasonable length)
        let title_length = tokens.len();
        if (2..=10).contains(&title_length) {
            confidence += 0.1;
        }

        confidence.clamp(0.0, 1.0)
    }

    /// Get language codes from config or default
    fn get_language_codes(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_language_codes())
            .unwrap_or_else(|| {
                vec![
                    "JPN".to_string(),
                    "ENG".to_string(),
                    "CHI".to_string(),
                    "KOR".to_string(),
                    "JAP".to_string(),
                    "EN".to_string(),
                    "CN".to_string(),
                ]
            })
    }

    /// Get common words from config or default
    fn get_common_words(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_common_words())
            .unwrap_or_else(|| vec!["The".to_string(), "A".to_string(), "An".to_string()])
    }

    /// Get known titles from config or default
    fn get_known_titles(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_known_titles())
            .unwrap_or_else(|| {
                vec![
                    "灌篮高手".to_string(),
                    "灌篮".to_string(),
                    "Slam".to_string(),
                    "Dunk".to_string(),
                ]
            })
    }

    /// Get technical Japanese terms from config or default
    fn get_technical_japanese_terms(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_technical_japanese_terms())
            .unwrap_or_else(|| {
                vec![
                    "国日双语".to_string(),
                    "双语".to_string(),
                    "国日".to_string(),
                    "日英".to_string(),
                    "英日".to_string(),
                    "中日".to_string(),
                    "日中".to_string(),
                ]
            })
    }

    /// Get release groups from config or default
    fn get_release_groups(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_release_groups())
            .unwrap_or_default()
    }

    /// Get technical terms from config or default
    fn get_technical_terms(&self) -> Vec<String> {
        self.config
            .as_ref()
            .map(|cfg| cfg.get_all_technical_terms())
            .unwrap_or_else(|| {
                vec![
                    // Video/audio codecs and quality
                    "10bit".to_string(),
                    "10bits".to_string(),
                    "bit".to_string(),
                    "bits".to_string(),
                    "DDP".to_string(),
                    "DTS".to_string(),
                    "AC3".to_string(),
                    "AAC".to_string(),
                    "FLAC".to_string(),
                    "THD".to_string(),
                    "MA".to_string(),
                    "HD".to_string(),
                    "x264".to_string(),
                    "x265".to_string(),
                    "H264".to_string(),
                    "H265".to_string(),
                    "AVC".to_string(),
                    "HEVC".to_string(),
                    "Atmos".to_string(),
                    "TrueHD".to_string(),
                    "DualAudio".to_string(),
                    "2Audio".to_string(),
                    "2Audios".to_string(),
                    "4Audios".to_string(),
                    "60fps".to_string(),
                    "HQ".to_string(),
                    "AAC(5".to_string(),
                    "1)".to_string(),
                    "Hi10P".to_string(),
                    "DD5".to_string(),
                    "TrueHD7".to_string(),
                    "H".to_string(),
                    "264".to_string(),
                    "265".to_string(),
                    "4Audio".to_string(),
                    "3Audio".to_string(),
                    "5Audio".to_string(),
                    "REPACK".to_string(),
                    "Remux".to_string(),
                    "VC-1".to_string(),
                    "DoVi".to_string(),
                    "HDR10".to_string(),
                    "EDR".to_string(),
                    "MULTi".to_string(),
                    "HDTS".to_string(),
                    "IMAX".to_string(),
                    "DSNP".to_string(),
                    "DTS-HD".to_string(),
                    "HDR".to_string(),
                    "120FPS".to_string(),
                    "4K".to_string(),
                    "WEB".to_string(),
                    "WEBRip".to_string(),
                    "UHD".to_string(),
                    "Blu-ray".to_string(),
                    "Bluray".to_string(),
                    "BluRay".to_string(),
                    "DD5".to_string(),
                    "DD+".to_string(),
                    "AC3".to_string(),
                    "AAC5".to_string(),
                    "AAC1".to_string(),
                    "10bit".to_string(),
                    "DV".to_string(),
                    "MP4".to_string(),
                    "MKV".to_string(),
                    // Source/platform names
                    "NF".to_string(),
                    "AMZN".to_string(),
                    "HKG".to_string(),
                    "ESP".to_string(),
                    "GBR".to_string(),
                    "INT".to_string(),
                    "JPN".to_string(),
                    "CHN".to_string(),
                    "CCTV6HD".to_string(),
                    "CHC".to_string(),
                    "Movie-HD".to_string(),
                    "AKA".to_string(),
                    "Chinese".to_string(),
                    "iTunes".to_string(),
                    "AMZN".to_string(),
                    "NF".to_string(),
                    "Netflix".to_string(),
                    "HMAX".to_string(),
                    "NOW".to_string(),
                    "ATVP".to_string(),
                    "HULU".to_string(),
                    "DSNP".to_string(),
                    // File formats and containers
                    "HDTVRip".to_string(),
                    "DVDRip".to_string(),
                    "BDRip".to_string(),
                    "HDRip".to_string(),
                    "WEBRip".to_string(),
                    "HDTV".to_string(),
                    "MP3".to_string(),
                    // Special editions and versions
                    "EXTENDED".to_string(),
                    "修复加长版".to_string(),
                    "导演剪辑版".to_string(),
                    "Extended".to_string(),
                    "RERIP".to_string(),
                    "Hybrid".to_string(),
                    "ES".to_string(),
                    // Release groups
                    "CMCT".to_string(),
                    "WiKi".to_string(),
                    "FRDS".to_string(),
                    "HDS".to_string(),
                    "ADWeb".to_string(),
                    "TLF".to_string(),
                    "CHDWEB".to_string(),
                    "PTerWEB".to_string(),
                    "GREENOTEA".to_string(),
                    "ZmWeb".to_string(),
                    "HDVWEB".to_string(),
                    "NukeHD".to_string(),
                    "TJUPT".to_string(),
                    "CMCTV".to_string(),
                    "NTG".to_string(),
                    "HDWTV".to_string(),
                    "NowOur".to_string(),
                    "PandaQT".to_string(),
                    "HANDJOB".to_string(),
                    "npuer".to_string(),
                    "BYRHD".to_string(),
                    "c0kE".to_string(),
                    "TBMovies".to_string(),
                    "MNHD".to_string(),
                    "YTS".to_string(),
                    "MX".to_string(),
                    "HDWinG".to_string(),
                    "NYPAD".to_string(),
                    "ZigZag".to_string(),
                    "NTb".to_string(),
                    "REMUX".to_string(),
                    "iT".to_string(),
                    "mUHD".to_string(),
                    "IAMABLE".to_string(),
                    "KRaLiMaRKo".to_string(),
                    "HDChina".to_string(),
                    "CtrlHD".to_string(),
                    "SWTYBLZ".to_string(),
                    "ADE".to_string(),
                    "PHOBOS".to_string(),
                    "PTHOME".to_string(),
                    "SyncUP".to_string(),
                    "YIFY".to_string(),
                    "SPARKS".to_string(),
                    "HiDt".to_string(),
                    "Geek".to_string(),
                    "TayTO".to_string(),
                    "nikt0".to_string(),
                    "beAst".to_string(),
                    "FoRM".to_string(),
                    "CRiME".to_string(),
                    "HVAC".to_string(),
                    "MaoZhan".to_string(),
                    "VietHD".to_string(),
                    "JYK".to_string(),
                    "GalaxyRG265".to_string(),
                    "PaODEQUEiJO".to_string(),
                    "SA89".to_string(),
                    "FANDANGO".to_string(),
                    "PTer".to_string(),
                    "ABM".to_string(),
                    "MZABI".to_string(),
                    "BYRPAD".to_string(),
                    "NCmt".to_string(),
                    "MTeam".to_string(),
                    "playWEB".to_string(),
                    "FLUX".to_string(),
                    "CMRG".to_string(),
                    "MZABARBiE".to_string(),
                    "SMURF".to_string(),
                    "AREY".to_string(),
                    "RABiDS".to_string(),
                    "ETHEL".to_string(),
                    "RightSiZE".to_string(),
                    "CiNEPHiLES".to_string(),
                    "Kitsune".to_string(),
                    "KBTV".to_string(),
                    "EbP".to_string(),
                ]
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_parser_basic() {
        let parser = UnifiedMovieParser::new();
        let result = parser
            .parse("The.Matrix.1999.1080p.BluRay.x264.mkv")
            .unwrap();

        assert_eq!(result.data.title, "The Matrix");
        assert_eq!(result.data.year, Some(1999));
        assert_eq!(result.data.quality, Some("1080p".to_string()));
        assert_eq!(result.data.source, Some("BluRay".to_string()));
        assert!(result.data.confidence > 0.5);
    }

    #[test]
    fn test_unified_parser_chinese_bilingual() {
        let parser = UnifiedMovieParser::new();
        let result = parser.parse("[BD-1080P] [名探偵コナン 緋色の弾丸] Detective Conan The Scarlet Bullet (2021) [BDRip][HEVC-10bit][1080p][CHS&CHT&ENG].mkv").unwrap();

        assert!(result.data.title.contains("Detective Conan"));
        assert_eq!(result.data.year, Some(2021));
        assert_eq!(result.data.quality, Some("1080p".to_string()));
    }

    #[test]
    fn test_unified_parser_series() {
        let parser = UnifiedMovieParser::new();
        let result = parser.parse("Iron.Man.Part.2.2010.1080p.mkv").unwrap();

        assert_eq!(result.data.title, "Iron Man Part");
        assert_eq!(result.data.year, Some(2010));
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_unified_parser_anime() {
        let parser = UnifiedMovieParser::new();
        let result = parser.parse("アニメ.Movie.2.1080p.mkv").unwrap();

        assert!(result.data.title.contains("アニメ"));
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_unified_parser_language_detection() {
        let parser = UnifiedMovieParser::new();
        let result = parser.parse("Movie.ENG.JPN.1080p.mkv").unwrap();

        assert!(result.data.title.contains("Movie"));
        assert!(result.confidence > 0.3);
    }
}
