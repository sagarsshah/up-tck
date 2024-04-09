import os
import subprocess
import sys
import time
from threading import Thread
from typing import List

import git
from behave import formatter
from behave.runner import Context

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))


path = ["C:\\Users\\gzm6xl\\Documents\\Github\\sagarshah\\up-tck\\test_agent\\rust\\target\\debug\\rust_tck.exe"]


def create_subprocess(command: List[str]) -> subprocess.Popen:
    if sys.platform == "win32":
        process = subprocess.Popen(command, shell=True)
    elif sys.platform == "linux" or sys.platform == "linux2":
        process = subprocess.Popen(command)
    else:
        raise Exception("only handle Windows and Linux commands for now")
    return process


process: subprocess.Popen = create_subprocess(path)