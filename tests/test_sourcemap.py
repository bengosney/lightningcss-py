import json

import lightningcss_py as lcss


def test_sourcemap(css_files):
    css_file, all_files = css_files
    _, sourcemap_json = lcss.bundle(css_file, source_map=True)

    sourcemap = json.loads(sourcemap_json)

    assert "sources" in sourcemap
    for css_path in all_files:
        assert css_path in sourcemap["sources"]


def test_no_sourcemap(css_files):
    css_file, _ = css_files
    _, sourcemap = lcss.bundle(css_file, source_map=False)
    assert sourcemap is None
