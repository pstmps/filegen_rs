import os
import json
import subprocess
from invoke import run
import threading

path = '/home/miki/remote_dev/watchywatchywatcher_rs/folders.json'

exec_path = '/home/miki/remote_dev/filegen_rs/target/release/filegen_rs'


number_of_files = 100#_000
file_size = 10#_000_000
subfolders = 2
purge = False

args = ['export RUST_LOG=info;', exec_path, '-n', str(number_of_files), '-f', str(file_size), '-s', str(subfolders)] + (['-p'] if purge else [])

with open(path, 'r') as f:
    data = json.load(f)

paths = [path for project in data['projects'] for path in project['paths']]
print(paths)

def run_cmd(cmd):
    result = run(cmd, hide=False)

for path in paths:

    cmd_args = args.copy()

    #if path does not exist, create it
    if not os.path.exists(path):
        os.makedirs(path)
    
    cmd_args.append('-r')
    cmd_args.append(path)

    cmd = ' '.join(cmd_args)
    thread = threading.Thread(target=run_cmd, args=(cmd,))
    thread.start()


