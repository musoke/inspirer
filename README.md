# Inspirer [![crates.io](https://img.shields.io/crates/v/inspirer.svg)](https://crates.io/crates/inspirer) [![crates.io](https://img.shields.io/crates/l/inspirer.svg)](https://crates.io/crates/inspirer)

[![Travis build status](https://img.shields.io/travis/musoke/inspirer.svg)](https://travis-ci.org/musoke/inspirer/)
[![AppVeyor build status](https://img.shields.io/appveyor/ci/musoke/inspirer.svg)](https://ci.appveyor.com/project/musoke/inspirer)
[![Gitlab build status](https://gitlab.com/musoke/inspirer/badges/master/build.svg)](https://gitlab.com/musoke/inspirer/pipelines)


For fetching BibTeX entries from [INSPIRE](https://inspirehep.net/).

This currently a rust clone of some features from
[inspiretools](https://github.com/DavidMStraub/inspiretools).
I will add more in the future.


## Installation

First, [install rust](https://www.rust-lang.org/en-US/install.html).

To install from git:
```
cargo install --git https://github.com/musoke/inspirer
cargo install --git https://gitlab.com/musoke/inspirer
```

To install from [crates.io](https://crates.io): to be released.

I haven't really tested with Windows, but do have tests on [Appveyor](https://ci.appveyor.com/project/musoke/inspirer)
which seem to pass sometimes [![AppVeyor build status](https://img.shields.io/appveyor/ci/musoke/inspirer.svg)](https://ci.appveyor.com/project/musoke/inspirer).


## Usage

To read from file `test_bibtex.aux` (usually corresponding to a LaTeX file `test_bibtex.tex`) and write to `stdout`:
```
aux2bib test_bibtex.aux
```
To write to a file:
```
aux2bib test_bibtex.aux bibliography.bib
```

`blg2bib` works analogously but takes a BibTeX or BibLaTeX log as input.
This allows retrieval of only entries which are not currently in the database.

## Things that are stupid

  - not yet parallelized
  - overly verbose logging
  - 7 MB binaries


## Licence

Apache Licence 2.0
