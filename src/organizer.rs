//! File organizer — builds Plex paths, executes moves, manages undo.
//!
//! Supports move, copy, and symlink strategies. Dry-run by default.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use tracing::{info, warn};

use crate::config::AppConfig;
use crate::models::{
    EnrichedMedia, OrganizeAction, UndoEntry, UndoManifest,
};
use crate::subtitles;
use crate::utils::sanitize_name;

// ── Path building ───────────────────────────────────────────────────────────

/// Build a Plex-compatible destination path for an enriched media file.
pub fn build_destination_path(
    enriched: &EnrichedMedia,
    source_file: &Path,
    dest_root: &Path,
    config: &AppConfig,
) -> PathBuf {
    let ext = source_file
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{e}"))
        .unwrap_or_default();

    if let Some(movie) = &enriched.movie {
        return build_movie_path(movie, &ext, dest_root, config);
    }
    if let Some(tv) = &enriched.tv_episode {
        return build_tv_path(tv, &ext, dest_root, config);
    }
    if let Some(music) = &enriched.music_track {
        return build_music_path(music, &ext, dest_root, config);
    }

    // Fallback
    let title = sanitize_name(enriched.best_title());
    dest_root.join("Unsorted").join(format!("{title}{ext}"))
}

fn build_movie_path(
    movie: &crate::models::Movie,
    ext: &str,
    root: &Path,
    config: &AppConfig,
) -> PathBuf {
    let title = sanitize_name(&movie.title);
    let folder = match movie.year {
        Some(y) => format!("{title} ({y})"),
        None => title.clone(),
    };
    let filename = format!("{folder}{ext}");
    root.join(&config.organize.movies_dir).join(&folder).join(filename)
}

fn build_tv_path(
    ep: &crate::models::TvEpisode,
    ext: &str,
    root: &Path,
    config: &AppConfig,
) -> PathBuf {
    let show = sanitize_name(&ep.show_title);
    let season_dir = format!("Season {:02}", ep.season);

    let mut ep_tag = format!("S{:02}E{:02}", ep.season, ep.episode);
    if let Some(end) = ep.episode_end {
        ep_tag.push_str(&format!("-E{end:02}"));
    }

    let filename = match &ep.episode_title {
        Some(t) if !t.is_empty() => {
            format!("{show} - {ep_tag} - {}{ext}", sanitize_name(t))
        }
        _ => format!("{show} - {ep_tag}{ext}"),
    };

    root.join(&config.organize.tv_dir)
        .join(&show)
        .join(&season_dir)
        .join(filename)
}

fn build_music_path(
    track: &crate::models::MusicTrack,
    ext: &str,
    root: &Path,
    config: &AppConfig,
) -> PathBuf {
    let artist = sanitize_name(if track.artist.is_empty() {
        "Unknown Artist"
    } else {
        &track.artist
    });
    let album_name = sanitize_name(
        track.album.as_deref().unwrap_or("Unknown Album"),
    );
    let album_dir = match track.year {
        Some(y) => format!("{album_name} ({y})"),
        None => album_name,
    };

    let track_name = sanitize_name(
        track.track_title.as_deref().unwrap_or("Track"),
    );
    let filename = match track.track_number {
        Some(n) => format!("{n:02} - {track_name}{ext}"),
        None => format!("{track_name}{ext}"),
    };

    root.join(&config.organize.music_dir)
        .join(&artist)
        .join(&album_dir)
        .join(filename)
}

// ── Plan ───────────────────────────────────────────────────────────────────

/// Generate planned file operations without executing them.
///
/// Discovers subtitle companions and creates co-located actions.
pub fn plan_actions(
    items: &[(PathBuf, EnrichedMedia)],
    dest_root: &Path,
    config: &AppConfig,
    strategy: &str,
) -> Vec<OrganizeAction> {
    let mut actions = Vec::new();
    let mut used_dests: HashSet<PathBuf> = HashSet::new();

    for (source, enriched) in items {
        let mut dest = build_destination_path(enriched, source, dest_root, config);

        // Handle duplicates with counter suffix
        let original_dest = dest.clone();
        let mut counter = 1u32;
        while used_dests.contains(&dest) || dest.exists() {
            let stem = original_dest.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
            let ext = original_dest
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| format!(".{e}"))
                .unwrap_or_default();
            dest = original_dest
                .parent()
                .unwrap_or(Path::new("."))
                .join(format!("{stem} ({counter}){ext}"));
            counter += 1;
        }
        used_dests.insert(dest.clone());

        actions.push(OrganizeAction {
            source: source.clone(),
            destination: dest.clone(),
            strategy: strategy.to_string(),
            media_type: enriched.media_type,
            title: enriched.best_title().to_string(),
            confidence: enriched.confidence,
        });

        // Discover and plan subtitle companions
        for companion in subtitles::find_companions(source) {
            let sub_ext = companion
                .path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| format!(".{e}"))
                .unwrap_or_default();
            let video_stem = dest.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
            let sub_dest = dest
                .parent()
                .unwrap_or(Path::new("."))
                .join(format!("{video_stem}{}{sub_ext}", companion.suffix));

            if !used_dests.contains(&sub_dest) {
                used_dests.insert(sub_dest.clone());
                actions.push(OrganizeAction {
                    source: companion.path,
                    destination: sub_dest,
                    strategy: strategy.to_string(),
                    media_type: enriched.media_type,
                    title: enriched.best_title().to_string(),
                    confidence: enriched.confidence,
                });
            }
        }
    }

    actions
}

// ── Execute ────────────────────────────────────────────────────────────────

/// Execute planned file operations and write an undo manifest.
pub fn execute_actions(actions: &[OrganizeAction], undo_dir: &Path) -> Result<UndoManifest> {
    let now = Utc::now();
    let mut manifest = UndoManifest {
        created_at: now.to_rfc3339(),
        description: format!("Organize run at {}", now.format("%Y-%m-%d %H:%M:%S UTC")),
        ..Default::default()
    };

    for action in actions {
        if !action.source.exists() {
            warn!("Source file missing, skipping: {}", action.source.display());
            continue;
        }
        if action.destination.exists() {
            warn!(
                "Destination exists, skipping: {}",
                action.destination.display()
            );
            continue;
        }

        // Create parent directories
        if let Some(parent) = action.destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create dir: {}", parent.display()))?;
        }

        match action.strategy.as_str() {
            "copy" => {
                fs::copy(&action.source, &action.destination).with_context(|| {
                    format!(
                        "Failed to copy {} → {}",
                        action.source.display(),
                        action.destination.display()
                    )
                })?;
            }
            "symlink" => {
                #[cfg(unix)]
                std::os::unix::fs::symlink(
                    fs::canonicalize(&action.source)?,
                    &action.destination,
                )
                .with_context(|| {
                    format!("Failed to symlink {}", action.source.display())
                })?;

                #[cfg(not(unix))]
                anyhow::bail!("Symlink strategy is only supported on Unix");
            }
            _ => {
                // Default: move
                fs::rename(&action.source, &action.destination).or_else(|_| {
                    // rename fails across filesystems; fall back to copy+delete
                    fs::copy(&action.source, &action.destination)?;
                    fs::remove_file(&action.source)?;
                    Ok::<(), std::io::Error>(())
                })
                .with_context(|| {
                    format!(
                        "Failed to move {} → {}",
                        action.source.display(),
                        action.destination.display()
                    )
                })?;
            }
        }

        info!(
            "Organized: {} → {}",
            action.source.display(),
            action.destination.display()
        );

        manifest.entries.push(UndoEntry {
            source: action.source.to_string_lossy().to_string(),
            destination: action.destination.to_string_lossy().to_string(),
            strategy: action.strategy.clone(),
            timestamp: now.to_rfc3339(),
            title: action.title.clone(),
            media_type: action.media_type.to_string(),
        });
    }

    // Write undo manifest
    if !manifest.entries.is_empty() {
        fs::create_dir_all(undo_dir)?;
        let manifest_path = undo_dir.join(format!(
            "undo_{}.json",
            now.format("%Y%m%d_%H%M%S")
        ));
        let json = serde_json::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, json)?;
        info!("Undo manifest written: {}", manifest_path.display());
    }

    Ok(manifest)
}

// ── Undo ───────────────────────────────────────────────────────────────────

/// Reverse the most recent organize operation.
pub fn undo_last(undo_dir: &Path) -> Result<u32> {
    if !undo_dir.exists() {
        anyhow::bail!("No undo directory found: {}", undo_dir.display());
    }

    let mut manifests: Vec<PathBuf> = fs::read_dir(undo_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("undo_") && n.ends_with(".json"))
                .unwrap_or(false)
        })
        .collect();

    manifests.sort();
    manifests.reverse();

    let manifest_path = match manifests.first() {
        Some(p) => p,
        None => anyhow::bail!("No undo manifests found"),
    };

    let content = fs::read_to_string(manifest_path)?;
    let manifest: UndoManifest = serde_json::from_str(&content)?;

    let mut reversed = 0u32;

    for entry in manifest.entries.iter().rev() {
        let dest = PathBuf::from(&entry.destination);
        let source = PathBuf::from(&entry.source);

        if !dest.exists() {
            warn!("Destination no longer exists: {}", dest.display());
            continue;
        }

        match entry.strategy.as_str() {
            "symlink" | "copy" => {
                fs::remove_file(&dest)?;
            }
            _ => {
                if let Some(parent) = source.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::rename(&dest, &source).or_else(|_| {
                    fs::copy(&dest, &source)?;
                    fs::remove_file(&dest)?;
                    Ok::<(), std::io::Error>(())
                })?;
            }
        }

        reversed += 1;
        info!("Reversed: {} → {}", dest.display(), source.display());

        // Clean up empty parent directories
        cleanup_empty_parents(&dest);
    }

    // Remove consumed manifest
    fs::remove_file(manifest_path)?;
    info!("Undo complete: {} files reversed", reversed);
    Ok(reversed)
}

fn cleanup_empty_parents(path: &Path) {
    let mut current = path.parent();
    for _ in 0..3 {
        match current {
            Some(p) if p.is_dir() => {
                if fs::read_dir(p).map(|mut d| d.next().is_none()).unwrap_or(false) {
                    let _ = fs::remove_dir(p);
                    current = p.parent();
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Movie, TvEpisode, MusicTrack, MediaType, ParsedMedia};

    fn make_movie_enriched(title: &str, year: Option<i32>) -> EnrichedMedia {
        let parsed = ParsedMedia {
            title: title.to_string(),
            year,
            media_type: MediaType::Movie,
            confidence: 80.0,
            ..Default::default()
        };
        let mut e = EnrichedMedia::from_parsed(parsed);
        e.movie = Some(Movie {
            title: title.to_string(),
            year,
            tmdb_id: None,
            original_title: None,
            confidence: 80.0,
        });
        e
    }

    #[test]
    fn test_movie_path() {
        let config = AppConfig::default();
        let enriched = make_movie_enriched("The Matrix", Some(1999));
        let source = Path::new("/downloads/The.Matrix.1999.mkv");
        let dest = build_destination_path(&enriched, source, Path::new("/plex"), &config);

        assert_eq!(
            dest,
            PathBuf::from("/plex/Movies/The Matrix (1999)/The Matrix (1999).mkv")
        );
    }

    #[test]
    fn test_tv_path() {
        let config = AppConfig::default();
        let parsed = ParsedMedia {
            title: "Breaking Bad".to_string(),
            media_type: MediaType::Tv,
            confidence: 80.0,
            ..Default::default()
        };
        let mut enriched = EnrichedMedia::from_parsed(parsed);
        enriched.tv_episode = Some(TvEpisode {
            show_title: "Breaking Bad".to_string(),
            season: 1,
            episode: 1,
            episode_end: None,
            episode_title: Some("Pilot".to_string()),
            year: Some(2008),
            tmdb_id: None,
            confidence: 80.0,
        });

        let source = Path::new("/downloads/Breaking.Bad.S01E01.mkv");
        let dest = build_destination_path(&enriched, source, Path::new("/plex"), &config);

        assert_eq!(
            dest,
            PathBuf::from("/plex/TV Shows/Breaking Bad/Season 01/Breaking Bad - S01E01 - Pilot.mkv")
        );
    }

    #[test]
    fn test_music_path() {
        let config = AppConfig::default();
        let parsed = ParsedMedia {
            title: "Song Title".to_string(),
            media_type: MediaType::Music,
            confidence: 60.0,
            ..Default::default()
        };
        let mut enriched = EnrichedMedia::from_parsed(parsed);
        enriched.music_track = Some(MusicTrack {
            artist: "Artist".to_string(),
            album: Some("Album".to_string()),
            track_title: Some("Song Title".to_string()),
            track_number: Some(1),
            year: Some(2020),
            confidence: 60.0,
        });

        let source = Path::new("/music/01 - Song Title.flac");
        let dest = build_destination_path(&enriched, source, Path::new("/plex"), &config);

        assert_eq!(
            dest,
            PathBuf::from("/plex/Music/Artist/Album (2020)/01 - Song Title.flac")
        );
    }

    #[test]
    fn test_execute_and_undo_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        let source_dir = tmp.path().join("source");
        let dest_dir = tmp.path().join("dest");
        let undo_dir = tmp.path().join("undo");
        fs::create_dir_all(&source_dir).unwrap();

        let source_file = source_dir.join("movie.mkv");
        fs::write(&source_file, b"video content").unwrap();

        let dest_file = dest_dir.join("Movies/Test (2024)/Test (2024).mkv");
        let actions = vec![OrganizeAction {
            source: source_file.clone(),
            destination: dest_file.clone(),
            strategy: "move".to_string(),
            media_type: MediaType::Movie,
            title: "Test".to_string(),
            confidence: 80.0,
        }];

        // Execute
        let manifest = execute_actions(&actions, &undo_dir).unwrap();
        assert_eq!(manifest.entries.len(), 1);
        assert!(dest_file.exists());
        assert!(!source_file.exists());

        // Undo
        let reversed = undo_last(&undo_dir).unwrap();
        assert_eq!(reversed, 1);
        assert!(source_file.exists());
        assert!(!dest_file.exists());
    }
}
