# Farce - a Fountain Parser

## This is an unfinished work in progress! Don't expect too much from it!

### What does it do?

It parses the [Fountain Screenplay Format](https://fountain.io/), and generates a PDF in something approximating the [standard screenplay format](https://www.nfi.edu/screenplay-format/). It doesn't implement the full spec, and definitely has some bugs and omissions. But it seems to kinda mostly work, to the small extent I've tested it so far. If you'd like anything implemented or fixed please raise an issue.

This was partly a learning project, and it's the first proper thing I've done in Rust, so if you're a real rust dev feel free to educate me about all the things I'm doing wrong.

NB It looks for fonts under the directory the executable's in, so if you try to use `cargo run` you'll find it fails to generate the PDF. Instead do soemthing like:

    cargo build
    cp target/debug/farce ./
    ./farce my-screenplay.fountain
    # PDF will be written to my-screenplay.pdf

Maybe at some point I'll release some binaries with the fonts in the right place.