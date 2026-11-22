// ANCHOR: listing
/// Recognize a complete frame and borrow its payload.
/// Format: 0xA5, kind in 1..=3, any payload bytes, 0x5A.
pub fn frame_body(frame: &[u8]) -> Option<(u8, &[u8])> {
    match frame {
        [0xA5, kind @ 1..=3, body @ .., 0x5A] => Some((*kind, body)),
        _ => None,
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_a_frame() {
        let packet = [0xA5, 2, 10, 20, 0x5A];
        assert_eq!(frame_body(&packet), Some((2, &[10, 20][..])));
    }

    #[test]
    fn accepts_all_supported_kinds_with_empty_payload() {
        for kind in 1..=3 {
            let packet = [0xA5, kind, 0x5A];
            assert_eq!(frame_body(&packet), Some((kind, &[][..])));
        }
    }

    #[test]
    fn rejects_short_inputs() {
        for packet in [&[][..], &[0xA5], &[0xA5, 2], &[0xA5, 0x5A]] {
            assert_eq!(frame_body(packet), None, "{packet:?}");
        }
    }

    #[test]
    fn rejects_wrong_markers() {
        for packet in [[0, 2, 0x5A], [0xA5, 2, 0], [0, 2, 0], [0x5A, 2, 0xA5]] {
            assert_eq!(frame_body(&packet), None, "{packet:?}");
        }
    }

    #[test]
    fn rejects_unsupported_kinds() {
        for kind in [0, 4, u8::MAX] {
            assert_eq!(frame_body(&[0xA5, kind, 0x5A]), None, "kind {kind}");
        }
    }

    #[test]
    fn preserves_arbitrary_payload_bytes() {
        let packet = [0xA5, 3, 0xff, 0x5A, 0, 0xA5, 0x5A];
        assert_eq!(frame_body(&packet), Some((3, &[0xff, 0x5A, 0, 0xA5][..])));
    }

    #[test]
    fn borrows_payload_from_input() {
        let packet = [0xA5, 1, 10, 20, 0x5A];
        let (_, body) = frame_body(&packet).unwrap();
        assert_eq!(body, &[10, 20]);
        assert!(std::ptr::eq(body.as_ptr(), packet[2..].as_ptr()));
    }

    #[test]
    fn treats_entire_input_as_one_frame() {
        let packet = [0xA5, 1, 10, 0x5A, 0xA5, 2, 20, 0x5A];
        assert_eq!(frame_body(&packet), Some((1, &[10, 0x5A, 0xA5, 2, 20][..])));
    }
}
// ANCHOR_END: tests
