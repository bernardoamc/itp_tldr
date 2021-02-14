# ITP TL;DR;

The tool everyone needs to reason about Safari's ITP by providing
**real-time** statistics as you navigate through different domains.

Safari's ITP Debug Mode is far from helpful, this tool is an attempt to make your life as a developer a bit easier.

## How does it work?

Safari aggregates information about any domain you visit in a SQLite database,
this program reads from this database in real-time and updates the UI accordingly.

## UI

![itp_tldr_ui](/images/ui.png?raw=true "usage")

## Usage

ITP TL;DR; supports arguments from the command line or from an `.itprc` located in your home folder.

If not specified the default SQLite path is:

- `/Users/bernardo/<user>/Containers/com.apple.Safari/Data/Library/WebKit/WebsiteData/ResourceLoadStatistics/observations.db`

### Command line

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
    -p, --path <path>             Safari's SQLite path
```

### Configuration file

The configuration file follows the `TOML` format and accepts the same arguments as the command line.

- domains
- path

Any of these can be omitted.

Example:

```toml
path = "full/path/to/the/sqlite/database"
domains = ["itp.com", "mydomain.com"]
```

## FAQ

### This tool doesn't work!

This is probably due to your terminal not being able to access Safari's SQLite
path.

Steps to solve the access issue:

1. Go to `Security & Privacy`
2. Select `Full Disk Access`
3. Grant it to your favourite terminal

### How to I erase these statistics?

Clear Safari's history and you can start from scratch.
