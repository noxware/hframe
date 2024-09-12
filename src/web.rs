use std::sync::LazyLock;

static SCRIPT_TAG: LazyLock<String> = LazyLock::new(|| {
    let html = include_str!("../companion-js-app/index.html");
    let script_begin = html.find("<script type=\"module\">").unwrap();
    let script_end = html.find("</script>").unwrap();
    let script = &html[script_begin..script_end];
});

pub(crate) fn install() {
    let html = include_str!("../companion-js-app/index.html");
}

pub(crate) fn send_areas(areas: Vec<Area>) {}
