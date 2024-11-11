# charts-cli

## Description

This is [charts-rs](https://github.com/vicanso/charts-rs) for CLI users.

⚠️ This is work in progress. breaking changes may occur.

```shell
# create chart from inline json
charts -c scatter -i '{"series_list": [{"data": [0.0, 1.0]}, {"data": [2.0, 3.0]}]}'

# create chart from json file
charts -n bar -p examples/bar.json
```

## Installation

```shell
git clone <this-repo>
cd charts-cli
cargo install --path .
```

## Usage

```shell
charts -h                                                                         ~/Hobby/charts 2 ↵
Charts Command Line Interface

Usage: charts [OPTIONS] --chart_name <chart_name>

Options:
  -n, --name <chart_name>  chart name
  -i, --inline <inline>          inline JSON string
  -p, --path <path>              json file path
  -h, --help                     Print help
  -V, --version                  Print version
```

## License

[MIT](LICENSE)
