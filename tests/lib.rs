use std::fs::File;
use std::num::NonZeroU64;

use matroska_demux::{ContentEncodingType, MatroskaFile, TrackEntry, TrackType};

#[test]
pub fn parse_simple_mkv() {
    let file = File::open("tests/data/simple.mkv").unwrap();
    let _mkv = MatroskaFile::open(file).unwrap();
}

#[test]
pub fn parse_test1_mkv() {
    let file = File::open("tests/data/test1.mkv").unwrap();
    let mkv = MatroskaFile::open(file).unwrap();

    assert_eq!(mkv.ebml_header().version(), None);
    assert_eq!(mkv.ebml_header().read_version(), None);
    assert_eq!(mkv.ebml_header().doc_type(), "matroska");
    assert_eq!(mkv.ebml_header().doc_type_version(), 2);
    assert_eq!(mkv.ebml_header().doc_type_read_version(), 2);
    assert_eq!(mkv.ebml_header().max_id_length(), 4);
    assert_eq!(mkv.ebml_header().max_size_length(), 8);

    assert_eq!(
        mkv.info().timestamp_scale(),
        NonZeroU64::new(1000000).unwrap()
    );
    assert!((87336.0 - mkv.info().duration().unwrap()).abs() < f64::EPSILON);
    assert_eq!(
        mkv.info().muxing_app(),
        "libebml2 v0.10.0 + libmatroska2 v0.10.1"
    );
    assert_eq!(mkv.info().date_utc().unwrap(), 304068183000000000);
    assert_eq!(mkv.info().writing_app(), "mkclean 0.5.5 ru from libebml v1.0.0 + libmatroska v1.0.0 + mkvmerge v4.1.1 ('Bouncin' Back') built on Jul  3 2010 22:54:08");

    assert_eq!(mkv.tracks().len(), 2);
}

#[test]
pub fn parse_test2_mkv() {
    let file = File::open("tests/data/test2.mkv").unwrap();
    let mkv = MatroskaFile::open(file).unwrap();

    assert_eq!(mkv.ebml_header().version(), None);
    assert_eq!(mkv.ebml_header().read_version(), None);
    assert_eq!(mkv.ebml_header().doc_type(), "matroska");
    assert_eq!(mkv.ebml_header().doc_type_version(), 2);
    assert_eq!(mkv.ebml_header().doc_type_read_version(), 2);
    assert_eq!(mkv.ebml_header().max_id_length(), 4);
    assert_eq!(mkv.ebml_header().max_size_length(), 8);

    assert_eq!(
        mkv.info().timestamp_scale(),
        NonZeroU64::new(100000).unwrap()
    );
    assert!((475090.0 - mkv.info().duration().unwrap()).abs() < f64::EPSILON);
    assert_eq!(mkv.info().date_utc().unwrap(), 328711520000000000);

    assert_eq!(mkv.tracks().len(), 2);
}

#[test]
pub fn parse_test3_mkv() {
    let file = File::open("tests/data/test3.mkv").unwrap();
    let mkv = MatroskaFile::open(file).unwrap();

    assert_eq!(mkv.ebml_header().version(), None);
    assert_eq!(mkv.ebml_header().read_version(), None);
    assert_eq!(mkv.ebml_header().doc_type(), "matroska");
    assert_eq!(mkv.ebml_header().doc_type_version(), 2);
    assert_eq!(mkv.ebml_header().doc_type_read_version(), 2);
    assert_eq!(mkv.ebml_header().max_id_length(), 4);
    assert_eq!(mkv.ebml_header().max_size_length(), 8);

    assert_eq!(
        mkv.info().timestamp_scale(),
        NonZeroU64::new(1000000).unwrap()
    );
    assert!((49064.0 - mkv.info().duration().unwrap()).abs() < f64::EPSILON);
    assert_eq!(mkv.info().date_utc().unwrap(), 304119805000000000);

    assert_eq!(mkv.tracks().len(), 2);

    assert_eq!(mkv.tracks()[0].content_encodings().unwrap().len(), 1);
    assert_eq!(mkv.tracks()[1].content_encodings().unwrap().len(), 1);

    assert_eq!(
        mkv.tracks()[0].content_encodings().unwrap()[0].encoding_type(),
        ContentEncodingType::Compression
    );
    assert_eq!(
        mkv.tracks()[1].content_encodings().unwrap()[0].encoding_type(),
        ContentEncodingType::Compression
    );
}

#[test]
pub fn parse_test4_mkv() {
    let file = File::open("tests/data/test4.mkv").unwrap();
    let mkv = MatroskaFile::open(file).unwrap();

    assert_eq!(mkv.ebml_header().version(), None);
    assert_eq!(mkv.ebml_header().read_version(), None);
    assert_eq!(mkv.ebml_header().doc_type(), "matroska");
    assert_eq!(mkv.ebml_header().doc_type_version(), 1);
    assert_eq!(mkv.ebml_header().doc_type_read_version(), 1);
    assert_eq!(mkv.ebml_header().max_id_length(), 4);
    assert_eq!(mkv.ebml_header().max_size_length(), 8);

    assert_eq!(
        mkv.info().timestamp_scale(),
        NonZeroU64::new(1000000).unwrap()
    );
    assert_eq!(mkv.info().date_utc().unwrap(), 304072935000000000);

    assert_eq!(mkv.tracks().len(), 2);
}

#[test]
pub fn parse_test5_mkv() {
    let file = File::open("tests/data/test5.mkv").unwrap();
    let mkv = MatroskaFile::open(file).unwrap();

    assert_eq!(mkv.ebml_header().version(), Some(1));
    assert_eq!(mkv.ebml_header().read_version(), Some(1));
    assert_eq!(mkv.ebml_header().doc_type(), "matroska");
    assert_eq!(mkv.ebml_header().doc_type_version(), 2);
    assert_eq!(mkv.ebml_header().doc_type_read_version(), 2);
    assert_eq!(mkv.ebml_header().max_id_length(), 4);
    assert_eq!(mkv.ebml_header().max_size_length(), 8);

    assert_eq!(
        mkv.info().timestamp_scale(),
        NonZeroU64::new(1000000).unwrap()
    );
    assert!((46665.0 - mkv.info().duration().unwrap()).abs() < f64::EPSILON);
    assert_eq!(mkv.info().date_utc().unwrap(), 304106803000000000);

    assert_eq!(mkv.tracks().len(), 11);
    assert_eq!(
        mkv.tracks()
            .iter()
            .filter(|t| t.track_type() == TrackType::Audio)
            .count(),
        2
    );
    assert_eq!(
        mkv.tracks()
            .iter()
            .filter(|t| t.track_type() == TrackType::Subtitle)
            .count(),
        8
    );

    let audio_tracks: Vec<TrackEntry> = mkv
        .tracks()
        .iter()
        .filter(|t| t.track_type() == TrackType::Audio)
        .cloned()
        .collect();

    assert!((48000.0 - audio_tracks[0].audio().unwrap().sampling_frequency()).abs() < f64::EPSILON);
    assert_eq!(audio_tracks[0].audio().unwrap().channels().get(), 2);

    assert!((22050.0 - audio_tracks[1].audio().unwrap().sampling_frequency()).abs() < f64::EPSILON);
    assert!(
        (44100.0
            - audio_tracks[1]
                .audio()
                .unwrap()
                .output_sampling_frequency()
                .unwrap())
        .abs()
            < f64::EPSILON
    );
}

#[test]
pub fn parse_test6_mkv() {
    let file = File::open("tests/data/test6.mkv").unwrap();
    let mkv = MatroskaFile::open(file).unwrap();

    assert_eq!(mkv.ebml_header().max_id_length(), 4);
    assert_eq!(mkv.ebml_header().max_size_length(), 8);

    assert_eq!(
        mkv.info().timestamp_scale(),
        NonZeroU64::new(1000000).unwrap()
    );
    assert!((87336.0 - mkv.info().duration().unwrap()).abs() < f64::EPSILON);
    assert_eq!(mkv.info().date_utc().unwrap(), 304101115000000000);

    assert_eq!(mkv.tracks().len(), 2);
}

#[test]
pub fn parse_test7_mkv() {
    let file = File::open("tests/data/test7.mkv").unwrap();
    let mkv = MatroskaFile::open(file).unwrap();

    assert_eq!(mkv.ebml_header().version(), None);
    assert_eq!(mkv.ebml_header().read_version(), None);
    assert_eq!(mkv.ebml_header().doc_type(), "matroska");
    assert_eq!(mkv.ebml_header().doc_type_version(), 2);
    assert_eq!(mkv.ebml_header().doc_type_read_version(), 2);
    assert_eq!(mkv.ebml_header().max_id_length(), 4);
    assert_eq!(mkv.ebml_header().max_size_length(), 8);

    assert_eq!(
        mkv.info().timestamp_scale(),
        NonZeroU64::new(1000000).unwrap()
    );
    assert!((37043.0 - mkv.info().duration().unwrap()).abs() < f64::EPSILON);
    assert_eq!(mkv.info().date_utc().unwrap(), 304102823000000000);

    assert_eq!(mkv.tracks().len(), 2);
}

#[test]
pub fn parse_test8_mkv() {
    let file = File::open("tests/data/test8.mkv").unwrap();
    let mkv = MatroskaFile::open(file).unwrap();

    assert_eq!(mkv.ebml_header().version(), None);
    assert_eq!(mkv.ebml_header().read_version(), None);
    assert_eq!(mkv.ebml_header().doc_type(), "matroska");
    assert_eq!(mkv.ebml_header().doc_type_version(), 2);
    assert_eq!(mkv.ebml_header().doc_type_read_version(), 2);
    assert_eq!(mkv.ebml_header().max_id_length(), 4);
    assert_eq!(mkv.ebml_header().max_size_length(), 8);

    assert_eq!(
        mkv.info().timestamp_scale(),
        NonZeroU64::new(1000000).unwrap()
    );
    assert!((47341.0 - mkv.info().duration().unwrap()).abs() < f64::EPSILON);
    assert_eq!(mkv.info().date_utc().unwrap(), 304104134000000000);

    assert_eq!(mkv.tracks().len(), 2);
}
