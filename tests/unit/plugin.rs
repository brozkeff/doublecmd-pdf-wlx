use super::{page_count, write_detect_string};

#[test]
fn parses_page_count() {
    assert_eq!(page_count(b"PDF-1.7\nPages: 3\n"), Some(3));
    assert_eq!(page_count(b"invalid"), None);
}

#[test]
fn writes_terminated_detection_string() {
    let mut output = [0xff; 4];
    write_detect_string(&mut output);
    assert_eq!(output, [b'E', b'X', b'T', 0]);
}
