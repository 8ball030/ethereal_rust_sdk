#[inline(always)]
pub fn extract_id_tail(bytes: &[u8]) -> Option<u64> {
    let mut i = bytes.len();

    // Trim trailing whitespace
    while i > 0 {
        match bytes[i - 1] {
            b' ' | b'\t' | b'\n' | b'\r' => i -= 1,
            _ => break,
        }
    }

    // Expect final '}' (JSON object end) but tolerate if caller trimmed it earlier
    if i == 0 {
        return None;
    }
    if bytes[i - 1] == b'}' {
        i -= 1;
    }

    // Trim whitespace before digits
    while i > 0 {
        match bytes[i - 1] {
            b' ' | b'\t' | b'\n' | b'\r' => i -= 1,
            _ => break,
        }
    }

    // Parse digits backwards: ... "id":123
    let mut val: u64 = 0;
    let mut mul: u64 = 1;
    let mut ndigits = 0usize;

    while i > 0 {
        let b = bytes[i - 1];
        if b.is_ascii_digit() {
            val = val.wrapping_add(((b - b'0') as u64).wrapping_mul(mul));
            mul = mul.wrapping_mul(10);
            ndigits += 1;
            i -= 1;
        } else {
            break;
        }
    }

    if ndigits == 0 {
        return None;
    }

    // Trim whitespace before ':'
    while i > 0 {
        match bytes[i - 1] {
            b' ' | b'\t' | b'\n' | b'\r' => i -= 1,
            _ => break,
        }
    }

    // Expect ':' before the digits
    if i == 0 || bytes[i - 1] != b':' {
        return None;
    }
    i -= 1;

    // Trim whitespace before the key
    while i > 0 {
        match bytes[i - 1] {
            b' ' | b'\t' | b'\n' | b'\r' => i -= 1,
            _ => break,
        }
    }

    // Expect `"id"` immediately before ':'
    // bytes[..i] ends right before the key
    if i < 4 {
        return None;
    }
    // Check for ... " i d "
    if bytes[i - 4] != b'"' || bytes[i - 3] != b'i' || bytes[i - 2] != b'd' || bytes[i - 1] != b'"'
    {
        return None;
    }

    Some(val)
}

#[inline(always)]
pub fn extract_id(bytes: &[u8]) -> Option<u64> {
    // Quick check: does it start with "{"id":"
    if !bytes.starts_with(b"{\"id\":") {
        return None;
    }

    // Start parsing after `"id":`
    let mut i = 6; // skip {"id":
    let start = i;

    // parse digits
    while let Some(&b) = bytes.get(i) {
        if !b.is_ascii_digit() {
            break;
        }
        i += 1;
    }

    std::str::from_utf8(&bytes[start..i]).ok()?.parse().ok()
}

#[inline(always)]
pub fn extract_channel(bytes: &[u8]) -> Option<&str> {
    if !bytes.starts_with(b"{\"method\":\"subscription\"") {
        return None;
    }
    let mut i = 46;
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
