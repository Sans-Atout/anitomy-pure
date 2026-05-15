use anitomy_rust::Parser;

#[test]
fn default() {
    let tested = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv");
    assert!(tested.parse().is_ok());
}

#[test]
fn ep_number() {
    let tested = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv")
        .ep_number(false);
    assert!(tested.parse().is_ok());
}

#[test]
fn ep_title() {
    let tested = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv")
        .ep_title(false);
    assert!(tested.parse().is_ok());
}

#[test]
fn file_extension() {
    let tested = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv")
        .file_extension(false);
    assert!(tested.parse().is_ok());
}

#[test]
fn release_group() {
    let tested = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv")
        .release_group(false);
    assert!(tested.parse().is_ok());
}

#[test]
fn file_name() {
    let tested = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv")
        .file_name("[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF].mkv");
    assert!(tested.parse().is_ok());
}

#[test]
fn ignored() {
    let tested = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv")
        .ignored_string(vec!["TaigaSubs"]);
    assert!(tested.parse().is_ok());
}

#[test]
fn delimiter() {
    let tested = Parser::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv")
        .allowed_delimiters(vec!['_']);
    assert!(tested.parse().is_ok());
}
