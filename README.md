# Inspirer [![crates.io](https://img.shields.io/crates/v/inspirer.svg)](https://crates.io/crates/inspirer) [![crates.io](https://img.shields.io/crates/l/inspirer.svg)](https://crates.io/crates/inspirer)

[![Travis build status](https://img.shields.io/travis/musoke/inspirer.svg)](https://travis-ci.org/musoke/inspirer/)
[![AppVeyor build status](https://img.shields.io/appveyor/ci/musoke/inspirer.svg)](https://ci.appveyor.com/project/musoke/inspirer)
[![Gitlab build status](https://gitlab.com/musoke/inspirer/badges/master/build.svg)](https://gitlab.com/musoke/inspirer/pipelines)


For fetching BibTeX entries from [INSPIRE](https://inspirehep.net/) and
[ADS](http://adsabs.harvard.edu/abstract_service.html).

This currently a rust clone of some features from
[inspiretools](https://github.com/DavidMStraub/inspiretools), with the added
ability to fetch BibTeX entries from
[ADS](http://adsabs.harvard.edu/abstract_service.html).


## Installation

### From pre-compiled binaries

TBD

### From source

First, [install rust](https://www.rust-lang.org/en-US/install.html).

To install from [crates.io](https://crates.io):
```
cargo install inspirer
```

To install the latest version from git:
```
cargo install --git https://github.com/musoke/inspirer
cargo install --git https://gitlab.com/musoke/inspirer
```

I haven't really tested with Windows, but do have tests on
[Appveyor](https://ci.appveyor.com/project/musoke/inspirer) which seem to pass
sometimes.

[![AppVeyor build status](https://img.shields.io/appveyor/ci/musoke/inspirer.svg)](https://ci.appveyor.com/project/musoke/inspirer)


## Usage

To read from file `test_bibtex.aux` (usually corresponding to a LaTeX file
`test_bibtex.tex`) and write to `stdout`:
```
aux2bib test_bibtex.aux
```
Either of these will append the fetched BibTeX entries to a file:
```
aux2bib test_bibtex.aux bibliography.bib
aux2bib test_bibtex.aux >> bibliography.bib
```
This will check what references appear in `test_bibtex.tex`, check if they look
like the style used by INSPIRE or ADS, and if so, fetch the corresponding
BibTeX records.

`blg2bib` works analogously but takes a BibTeX or BibLaTeX log as input.
This allows retrieval of only entries which are not currently in the database,
which may be significantly faster if you have a lot of references.
```
blg2bib test_bibtex.blg bibliography.bib
```

There are some sample input files in `example_files`. If you have cargo & rustc
installed you can test on them like so:
```
cargo run --bin aux2bib example_files/test_bitex.aux
cargo run --bin blg2bib example_files/test_bitex.blg
cargo run --bin blg2bib example_files/test_biber.blg
```


## Things that are stupid

  - not yet parallelized
  - overly verbose logging
  - 7 MB binaries
  - binaries are distributed yet
  - too many `.unwrap()`s
  - haven't yet tried fuzzing


## Licence

Apache Licence 2.0
