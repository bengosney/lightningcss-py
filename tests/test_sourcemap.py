import json

import lightningcss_py as lcss


def test_sourcemap(css_files):
    css_file, all_files = css_files
    res = lcss.bundle(css_file, source_map=True)

    if res.source_map is None:
        raise Exception("Source map is None")

    sourcemap = json.loads(res.source_map)

    assert "sources" in sourcemap
    for css_path in all_files:
        assert css_path in sourcemap["sources"]


def test_no_sourcemap(css_files):
    css_file, _ = css_files
    res = lcss.bundle(css_file, source_map=False)
    assert res.source_map is None
