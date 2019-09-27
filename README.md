# rtail
[![Build Status](https://travis-ci.org/knarfytrebil/rtail.svg?branch=master)](https://travis-ci.org/knarfytrebil/rtail)
![Crates.io](https://img.shields.io/crates/v/rtail)

## Installation
```bash
cargo install rtail
```

## Usage
```bash
rtail 0.0.1
Relaed <knarfytrebil@gmail.com>
read remote files over http / https

USAGE:
    rtail [FLAGS] [OPTIONS] <URL>

FLAGS:
    -f, --follow     Continuesly watch the change of the url
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --milliseconds <interval>    Interval of the poll

ARGS:
    <URL>    URL of the file to read

```
