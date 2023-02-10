import lightningcss_py as lcss


def test_bundle(css_file):
    res = lcss.bundle(css_file)
    assert res.css == ".a {\n  color: #ff000080;\n}\n"


def test_bundle_targets_chrome_95(css_file):
    res = lcss.bundle(css_file, targets=lcss.Browsers(chrome="95.0"))
    assert res.css == ".a {\n  color: #ff000080;\n}\n"


def test_bundle_targets_chrome_1(css_file):
    res = lcss.bundle(css_file, targets=lcss.Browsers(chrome="1"))
    assert res.css == ".a {\n  color: rgba(255, 0, 0, .5);\n}\n"
