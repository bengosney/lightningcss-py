from typing import Optional

class Browsers:
    android: Optional[str | None]
    chrome: Optional[str | None]
    edge: Optional[str | None]
    firefox: Optional[str | None]
    ie: Optional[str | None]
    ios_saf: Optional[str | None]
    opera: Optional[str | None]
    safari: Optional[str | None]
    samsung: Optional[str | None]

    def __init__(
        self,
        android: Optional[str | None] = None,
        chrome: Optional[str | None] = None,
        edge: Optional[str | None] = None,
        firefox: Optional[str | None] = None,
        ie: Optional[str | None] = None,
        ios_saf: Optional[str | None] = None,
        opera: Optional[str | None] = None,
        safari: Optional[str | None] = None,
        samsung: Optional[str | None] = None,
    ) -> None: ...

def browser_version(version: str) -> int: ...
def bundle(
    filename: str,
    minify: bool = False,
    source_map: bool = False,
    project_root: str = "/",
    targets: Optional[Browsers] = None,
    nesting: bool = True,
): ...
