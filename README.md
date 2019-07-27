# SHA-CLI
SHA hash generator for the command line

## To build
`cargo build --release`

## Hashing a file
`./sha <variant> --file <path/to/file>`

## Hashing text
`./sha <variant> --text <message>`

NOTE: The text hash cannot read white space and will only accept an input of one word. To has a message with whitespace, use the file flag.
