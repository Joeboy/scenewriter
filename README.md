# Farce - a Fountain Parser

## This is an unfinished work in progress! Don't expect too much from it!

### What does it do?

It parses the text based [Fountain Screenplay Format](https://fountain.io/), and generates a PDF in something approximating the [standard screenplay format](https://www.nfi.edu/screenplay-format/). It doesn't implement the full spec, and definitely has some bugs and omissions. But it seems to kinda mostly work, to the small extent I've tested it so far. [Here](https://fountain.io/_downloads/Big%20Fish.pdf)'s the reference version of the Big Fish screenplay from fountain.io, and [here](./samples/Big-Fish.pdf)'s Farce's effort at formatting the same [fountain file](https://fountain.io/_downloads/Big-Fish.fountain). It's kind of in the ballpark, but not quite there yet.

If you'd especially like anything implemented or fixed please raise an issue. See the issues to get a vague idea of the current priorities.

This was partly a learning project, and it's the first proper thing I've done in Rust, so if you're a real rust dev feel free to educate me about all the things I'm doing wrong.

NB It looks for fonts under the directory the executable's in, so if you try to use `cargo run` you'll find it fails to generate the PDF. Instead do something like:

    cargo build
    cp target/debug/farce ./
    ./farce my-screenplay.fountain
    # PDF will be written to my-screenplay.pdf

Maybe at some point I'll release some binaries with the fonts in the right place.

### Usage

    ./farce --help
    
    Usage: farce [..options..] input_filename
    
    Options:
    
                  --pdf     Output PDF (default)
                 --html     Output HTML (Coming soon...)
    
               --letter     US Letter page size (default)
                     -l
    
                   --a4     A4 page size
                     -a
    
      --output filename     Choose output filename (default is the
            -o filename     input filename but with pdf extension)
    
                 --help     Show this help
    
    Eg. farce --a4 --pdf -o "My Screenplay final-v23.pdf" my_screenplay.fountain
