"""Wrap index.html (artifact-style page content) in a full document so it can be served from any web host.

Breath sensing needs the page opened as its own top-level page over HTTPS (for example GitHub Pages);
pages framed inside the Claude viewer don't get motion-sensor access.
Usage: python3 build_standalone.py  -> writes standalone.html next to this script.
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
HEAD = (
    '<!doctype html><html lang="en"><head><meta charset="utf-8">'
    '<meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover">'
    "<style>:root{color-scheme:light;padding-top:env(safe-area-inset-top,0px);"
    "padding-bottom:env(safe-area-inset-bottom,0px)}body{margin:0;font:14px system-ui,sans-serif}"
    "img{max-width:100%}[hidden]{display:none!important}</style></head><body>"
)
body = open(os.path.join(HERE, "index.html"), encoding="utf-8").read()
open(os.path.join(HERE, "standalone.html"), "w", encoding="utf-8").write(HEAD + body + "</body></html>\n")
print("wrote standalone.html")
