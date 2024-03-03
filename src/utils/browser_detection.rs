fn user_agent() -> String {
    web_sys::window()
        .unwrap()
        .navigator()
        .user_agent()
        .unwrap()
        .to_lowercase()
}
// Safari (WebKit) example:
// Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1.2 Safari/605.1.15

// Safari Technical Preview (WebKit) example:
// "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15" = $1

// Firefox (Gecko) example:
// Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:123.0) Gecko/20100101 Firefox/123.0

// Chrome (Blink) example:
// Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36

// Chromium (Blink) example:
// Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36

#[allow(dead_code)]
pub(crate) fn is_gecko() -> bool {
    let ua = user_agent();

    ua.contains("gecko")
        && !ua.contains("like gecko")
        && !ua.contains("webkit")
        && !ua.contains("chrome")
        && !ua.contains("safari")
}

pub(crate) fn is_blink() -> bool {
    let ua = user_agent();
    ua.contains("chrome")
}
