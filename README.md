# Farce - a Fountain Parser

## This is an unfinished work in progress! Don't expect too much from it!

### What does it do?

It parses the text based [Fountain Screenplay Format](https://fountain.io/), and generates a PDF in something approximating the [standard screenplay format](https://www.nfi.edu/screenplay-format/). It doesn't implement the full spec, and definitely has some bugs and omissions. But it seems to kinda mostly work, to the small extent I've tested it so far.

[Here](https://fountain.io/_downloads/Big%20Fish.pdf)'s the reference version of the Big Fish screenplay from fountain.io, and [here](./samples/Big-Fish.pdf)'s Farce's effort at formatting the same [fountain file](https://fountain.io/_downloads/Big-Fish.fountain). It's kind of in the ballpark, but not quite there yet. For the even less polished HTML output see [here](./samples/Big-Fish.html).

On my laptop, HTML generation is basically instant (~50ms for Big Fish). Generating the 119 page PDF from the same file takes about 0.4s.

If you'd especially like anything implemented or fixed please raise an issue. See the issues to get a vague idea of the current priorities.

This was partly a learning project, and it's the first proper thing I've done in Rust, so if you're a real rust dev feel free to educate me about all the things I'm doing wrong.

### Usage

    ./farce --help
    
    Usage: farce [..options..] input_filename

    Options:

                --pdf     Output PDF (default)
                --html     Output HTML

                  --a4     A4 page size (default)
                    -a

              --letter     US Letter page size
                    -l

    --output filename     Choose output filename (default is the
          -o filename     input filename but with pdf extension)

                --help     Show this help


          Eg: farce --a4 --pdf -o "My Screenplay final-v23.pdf" my_screenplay.fountain

    or just: farce my_screenplay.fountain
