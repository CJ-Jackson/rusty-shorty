#!/usr/bin/env python3
import os
import subprocess
from os.path import basename

top_path = os.path.dirname(os.path.abspath(__file__))

os.chdir(top_path + "/asset/js")

downloads = [
    "https://cdnjs.cloudflare.com/ajax/libs/htmx/2.0.7/htmx.esm.js"
]

for url in downloads:
    print(url)
    subprocess.run(["rm", basename(url)])
    subprocess.run(["wget", url])
