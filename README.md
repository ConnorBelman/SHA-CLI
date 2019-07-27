# SHA-CLI
SHA hash generator for the command line

## To build
`cargo build --release`
## How To Use
### Hashing a file
`./sha <variant> --file <path/to/file>`

### Hashing text
`./sha <variant> --text <message>`

### Help / Version
`./sha --help`
`./sha --version`

## Notes
- The text hash cannot read white space and will only accept an input of one word. To has a message with whitespace, use the file flag.
- I wasn't able to find another implementation of SHA-0 to test the accuracy of mine, so I cannot confirm that it is correct.
