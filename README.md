# ITP TL;DR;

The tool everyone needs to reason about Safari's ITP by providing
**real-time** statistics as you navigate through different domains.

## UI

![itp_tldr_ui](/images/ui.png?raw=true "usage")

## Usage

```
❯ ./itp_tldr --help
itp_tldr 0.1.0
ITP TL;DR; the tool you didn't know you needed to understand ITP

USAGE:
    itp_tldr [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --domains <domains>...    A list of comma separated domains
```

## FAQ

### This tool doesn't work!

This is probably due to your terminal not being able to access Safari's SQLite
path.

Steps to solve the access issue:

1. Go to `Security & Privacy`
2. Select `Full Disk Access`
3. Grant it to your favourite terminal
