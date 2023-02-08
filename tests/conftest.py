import pytest


@pytest.fixture(scope="session")
def css_file(tmp_path_factory):
    fn = tmp_path_factory.mktemp("data") / "styles.css"
    with open(fn, "w") as f:
        f.write(
            """.a {
color: rgba(255, 0, 0, .5);
}"""
        )

    return str(fn)


@pytest.fixture(scope="session")
def css_file_nested(tmp_path_factory):
    fn = tmp_path_factory.mktemp("data") / "styles.css"
    with open(fn, "w") as f:
        f.write(
            """.a {
    &.b {
        colour: red;
    }
}"""
        )

    return str(fn)


@pytest.fixture(scope="session")
def css_files(tmp_path_factory) -> tuple[str, list[str]]:
    css_paths = []
    basepath = tmp_path_factory.mktemp("data")
    stylesheets = []
    for c in ["a", "b", "c"]:
        fn = basepath / f"{c}.css"
        css_paths.append(fn)
        with open(fn, "w") as f:
            f.write(f".{c} {{color: rgba(255, 0, 0, .5);}}")
        stylesheets.append(f'@import "{c}.css";')
        print(fn)

    fn = basepath / "styles.css"
    with open(fn, "w") as f:
        f.write("\n".join(stylesheets))
    css_paths.append(fn)

    return str(fn), [str(p).lstrip("/") for p in css_paths]
