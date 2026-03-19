#[inline(always)]
pub fn extract_event(bytes: &[u8]) -> Option<&str> {
    if !bytes.starts_with(b"{\"e\":\"") {
        return None;
    }
    let mut i = 6; // Skip past the initial {"e":"
    let start = i;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b'"' {
            break;
        }
        i += 1;
    }

    unsafe { Some(std::str::from_utf8_unchecked(&bytes[start..i])) }
}
