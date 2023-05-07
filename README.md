# namehash-rs

## CLI program for hashing domain names according to EIP-137

Compute the namehash of individual domains or multiple domains at once from a file.

## Usage

```shell
> .\namehash
Usage: namehash <COMMAND>

Commands:
  domain  Get the namehash of a single domain
  file    Get the namehashes of many domains at once
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
>
> .\namehash domain 8bit.x
8bit.x: 0xb7c50b1d5594286e2fff20a5aa108021ffb1cc70c1afb67934e3729aa3e44c39
>
> .\namehash file -h
Get the namehashes of many domains at once

Usage: namehash file [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to input file, domains to hash with 1 per line

Options:
  -o, --output <FILE>  File to save hashes to, stdout if not given
  -h, --help           Print help
```
