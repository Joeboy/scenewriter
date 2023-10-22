# Scenewriter - a Fountain Parser

`Scenewriter` is a command line program that parses files in [Fountain markup language](https://fountain.io/),
and generates PDFs in the [standard screenplay format](https://www.nfi.edu/screenplay-format/).
It doesn't implement the full Fountain spec yet, and probably has bugs. But it
seems to kinda mostly work, to the limited extent I've tested it so far.

If you just want to download and use it, see the [releases](https://github.com/Joeboy/scenewriter/releases/tag/v0.0.7).
Note that it's still an unfinished / preview release. But for basic scenes,
it might already do everything you need. Issues that might be offputting are:

* Full Fountain spec is not yet implemented
* Only really tested with straightforward UTF8 / ASCII encoded documents,
  probably won't handle weirder encodings well
* Title page is very basic - eg. no contact details

Here's direct links to the
[Windows](https://github.com/Joeboy/scenewriter/releases/download/v0.0.7/scenewriter_v0.0.7_x86_64-pc-windows-gnu.zip),
[Mac](https://github.com/Joeboy/scenewriter/releases/download/v0.0.7/scenewriter_v0.0.7_x86_64-apple-darwin.zip)
and
[Linux](https://github.com/Joeboy/scenewriter/releases/download/v0.0.7/scenewriter_v0.0.7_x86_64-unknown-linux-musl.tar.gz)
versions.

[Here](https://fountain.io/_downloads/Big-Fish.pdf)'s the reference version
of the John August's Big Fish screenplay from fountain.io, and [here](./samples/Big-Fish.pdf)'s
Scenewriter's effort at formatting the same [fountain file](https://fountain.io/_downloads/Big-Fish.fountain).
It's in the ballpark, but not 100% there yet. For the even less polished HTML
output see [here](./samples/Big-Fish.html).

On my laptop, HTML generation is basically instant (~25ms for Big Fish).
Generating the 119 page PDF from the same file takes about 0.4s.

If you'd especially like anything implemented or fixed please raise an issue.
See the [issues](https://github.com/Joeboy/scenewriter/issues) to get a vague idea of
the current priorities.


### Usage

    ./scenewriter --help
    
    Usage: scenewriter [..options..] input_filename

    Options:

                 --pdf     Write PDF file (default)
                --html     Write HTML file
               --stats     Show screenplay stats

                  --a4     A4 page size (default)
                    -a

              --letter     US Letter page size
                    -l

     --output filename     Choose output filename (default is the input
           -o filename     filename but with .pdf or .html extension)

                --help     Show this help


         Eg: scenewriter --a4 --pdf -o "My Screenplay final-v23.pdf" my_screenplay.fountain

    or just: scenewriter my_screenplay.fountain

(if you're using the Windows version it'll be `scenewriter.exe`).


### Python bindings

If you want to call scenewriter code from python, there are very rudimentary bindings
[here](https://github.com/Joeboy/pyscenewriter).