use plex_media_organizer::organizer::Organizer;
use plex_media_organizer::types::{MediaFile, MediaMetadata};
use std::path::PathBuf;

#[test]
fn test_plex_directory_structure_fix() {
    let organizer = Organizer::new(true, None);

    // Create a test media file with the problematic structure
    let media_file = MediaFile {
        id: "test".to_string(),
        file_path: PathBuf::from(
            "/Volumes/media/movie/Japanese/5.Centimeters.Per.Second.2007.BluRay.1080p-ted423@FRDS/5.Centimeters.Per.Second.2007.BluRay.1080p.x265.10bit.DDP.5.1-ted423@FRDS.mkv",
        ),
        file_name: "5.Centimeters.Per.Second.2007.BluRay.1080p.x265.10bit.DDP.5.1-ted423@FRDS.mkv"
            .to_string(),
        file_size: 1000,
        media_type: plex_media_organizer::types::MediaType::Movie,
        content_hash: "hash".to_string(),
        last_modified: chrono::Utc::now(),
        metadata: MediaMetadata::default(),
    };

    let mut metadata = MediaMetadata::default();
    metadata.title = Some("秒速5センチメートル [5 Centimeters per Second]".to_string());
    metadata.year = Some(2007);
    metadata.quality = Some("1080p BluRay".to_string());

    // Generate the new path
    let new_path = organizer
        .generate_plex_path(&media_file, &metadata)
        .unwrap();

    // The new path should be under the Japanese collection directory, not the release folder
    let expected_path = PathBuf::from(
        "/Volumes/media/movie/Japanese/秒速5センチメートル [5 Centimeters per Second] (2007)/秒速5センチメートル [5 Centimeters per Second] (2007) 1080p BluRay.mkv",
    );

    assert_eq!(new_path, expected_path);

    // Verify that the release folder name is not preserved in the path
    assert!(
        !new_path
            .to_string_lossy()
            .contains("5.Centimeters.Per.Second.2007.BluRay.1080p-ted423@FRDS")
    );

    // Verify that the path is under the Japanese collection directory
    assert!(new_path.to_string_lossy().contains("/Japanese/"));
}

#[test]
fn test_plex_directory_structure_with_extras() {
    let organizer = Organizer::new(true, None);

    // Test with an extras file
    let media_file = MediaFile {
        id: "test".to_string(),
        file_path: PathBuf::from(
            "/Volumes/media/movie/Japanese/5.Centimeters.Per.Second.2007.BluRay.1080p-ted423@FRDS/Extras/BDMenu(JPGLBL).mkv",
        ),
        file_name: "BDMenu(JPGLBL).mkv".to_string(),
        file_size: 1000,
        media_type: plex_media_organizer::types::MediaType::Movie,
        content_hash: "hash".to_string(),
        last_modified: chrono::Utc::now(),
        metadata: MediaMetadata::default(),
    };

    let mut metadata = MediaMetadata::default();
    metadata.title = Some("BDMenu(JPGLBL)".to_string());
    metadata.year = None; // No year for extras

    // Generate the new path
    let new_path = organizer
        .generate_plex_path(&media_file, &metadata)
        .unwrap();

    // The new path should be under the Japanese collection directory, not the release folder
    let expected_path = PathBuf::from(
        "/Volumes/media/movie/Japanese/BDMenu(JPGLBL) (Unknown Year)/BDMenu(JPGLBL).mkv",
    );

    assert_eq!(new_path, expected_path);

    // Verify that the release folder name is not preserved
    assert!(
        !new_path
            .to_string_lossy()
            .contains("5.Centimeters.Per.Second.2007.BluRay.1080p-ted423@FRDS")
    );

    // Verify that the Extras folder is not preserved
    assert!(!new_path.to_string_lossy().contains("/Extras/"));
}
