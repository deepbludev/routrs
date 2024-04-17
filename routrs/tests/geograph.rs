use routrs::geograph::PathType;
use routrs::prelude::*;
use routrs::MARITIME;

fn it_reads_geograph(geograph: &Geograph) {
    assert!(geograph.nodes().count() > 1);
}

#[test]
fn it_reads_marnet() {
    it_reads_geograph(&MARITIME);
}

#[test]
fn it_calculates_distance() {
    let from: Geoloc = (40.6759, -74.0504); // USNYC
    let to: Geoloc = (41.0067858, 28.9732219); // TRIST
    let (distance, path, path_type) = MARITIME.distance(&from, &to).unwrap();

    assert_eq!(distance, 9224.95741604269);
    assert_eq!(path.len(), 117);
    assert_eq!(path_type, PathType::ViaNodes);
}
