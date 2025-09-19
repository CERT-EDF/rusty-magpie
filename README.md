<p align="center"><img width="120" src="./.github/logo.png"></p>
<h2 align="center">Rusty Magpie</h2>

<div align="center">

![Status](https://img.shields.io/badge/status-active-success?style=for-the-badge)
![Powered By: EDF](https://img.shields.io/badge/Powered_By-CERT_EDF-FFFF33.svg?style=for-the-badge)
[![License: MIT](https://img.shields.io/badge/License-MIT-2596be.svg?style=for-the-badge)](LICENSE)

</div>

<br>

# Introduction

Right now 'Rusty Magpie' collects the following artifacts directly on the Android phone, with many details:

- processes
- file list (name, digest, etc)

It can also run the Yara-X scanner (w00t) directly on the phone by providing your list of rules.

Feel free to send any PR in order to collect other artifacts!

<br>

## Getting Started

### Dependencies

You need to install the following dependencies and specify some env paths in
order to compile the project correctly.

```bash
export ANDROID_NDK_HOME="/path"
rustup target add aarch64-linux-android
cargo install cargo-ndk
```

### Build

Compile for aarch64, or any other platform for your mobile phone:

```bash
export RUSTFLAGS="-Clink-arg=-z -Clink-arg=nostart-stop-gc"
cargo ndk -t aarch64-linux-android build --release
```

### Usage

When the binary is compiled, it is located in the following directory

```
target/aarch64-linux-android/release
```

You can push it on your phone after enabling the Developer option and the USB debug:

```bash
adb push target/aarch64-linux-android/release/rusty_magpie /data/local/tmp/
adb shell chmod 0755 /data/local/tmp/rusty_magpie
```

#### Collecting process

In `adb shell`, run `rusty_magpie ps` command

```bash
cd /data/local/tmp/
./rusty_magpie ps > results.json
```

Then pull `rusty_magpie` output file from the mobile phone

```bash
adb pull /data/local/tmp/results.json
```
```python
import json, pprint
results = json.load(open("results.json"))
len(results)
# 866
pprint(results[0])
# {'command_line': ['/system/bin/init', 'second_stage'],
#  'context': '',
#  'cwd': '',
#  'env': [],
#  'filename': 'init',
#  'kernel_time': 4578,
#  'path': '',
#  'pgroup': 0,
#  'pid': 1,
#  'ppid': 0,
#  'previous_context': '',
#  'priority': 20,
#  'psid': 0,
#  'state': 'S',
#  'uid': 0,
#  'user_time': 5310}
```

#### Collecting files

In `adb shell`, run `rusty_magpie find` command

```bash
cd /data/local/tmp
./rusty_magpie find --path /sdcard/Music/ --max-depth 5 > results.json
```

Then pull `rusty_magpie` output file from the mobile phone

```bash
adb pull /data/local/tmp/results.json
```
```python
import json, pprint
results = json.load(open("results.json"))
pprint(results[0])
# {'access_time': 1726042508,
#  'changed_time': 0,
#  'context': '',
#  'error': '',
#  'group_id': 0,
#  'group_name': '',
#  'mode': '',
#  'modified_time': 1726042508,
#  'path': '/storage/emulated/0/Music/Samsung/Over_the_Horizon.m4a',
#  'sha256': '643a2ed480a60abca0e25e56669a6077a3b6cdd00d81798d47babee8021ac86e',
#  'size': 19948513,
#  'user_id': 0,
#  'user_name': ''}
```

#### Scanning files with Yara

Right now, it is possible to scan files directly on the Android phone by providing your list of Yara rules. First, you need to compile your rules with [Yara-x](https://github.com/VirusTotal/yara-x):

```bash
target/release/yr compile myrules.yar
```

Compiled rules will be stored in `output.yarc`, and now you can push it on the phone and run the binary:

```bash
adb push output.yarc /data/local/tmp/
```

In `adb shell`, run `rusty_magpie yara` command

```bash
./rusty_magpie yara --path ./ --rule-path output.yarc
```

<br>

## License

Distributed under the [Apache License, Version 2.0](LICENSE).

<br>

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

<br>

## Security

To report a (suspected) security issue, see [SECURITY.md](SECURITY.md).
