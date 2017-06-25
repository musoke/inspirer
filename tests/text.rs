pub const AUX_BIBTEX: &'static [u8] = b"\
\\relax 
\\citation{Higgs:2014aqa}
\\citation{Higgs:2015mei}
\\bibstyle{unsrt}
\\bibdata{test_bibtex}
";

pub const AUX_BIBLATEX: &'static [u8] = b"\\
\\relax 
\\abx@aux@sortscheme{nty}
\\abx@aux@refcontext{nty/global/}
\\abx@aux@cite{Guth:1980zm}
\\abx@aux@segm{0}{0}{Guth:1980zm}
\\abx@aux@cite{Linde:1981mu}
\\abx@aux@segm{0}{0}{Linde:1981mu}
\\abx@aux@cite{Albrecht:1982wi}
\\abx@aux@segm{0}{0}{Albrecht:1982wi}
\\abx@aux@cite{1982PhRvL..48.1220A}
\\abx@aux@segm{0}{0}{1982PhRvL..48.1220A}
\\@writefile{toc}{\\boolfalse {citerequest}\\boolfalse {citetracker}\\boolfalse {pagetracker}\\boolfalse {backtracker}\\relax }
\\@writefile{lof}{\\boolfalse {citerequest}\\boolfalse {citetracker}\\boolfalse {pagetracker}\\boolfalse {backtracker}\\relax }
\\@writefile{lot}{\\boolfalse {citerequest}\\boolfalse {citetracker}\\boolfalse {pagetracker}\\boolfalse {backtracker}\\relax }";

pub const BLG_BIBTEX: &'static [u8] = b"\
This is BibTeX, Version 0.99d (TeX Live 2016/Arch Linux)
Capacity: max_strings=35307, hash_size=35307, hash_prime=30011
The top-level auxiliary file: test_bibtex.aux
The style file: unsrt.bst
Database file #1: test_bibtex.bib
Warning--I didn't find a database entry for \"Higgs:2014aqa\"
Warning--I didn't find a database entry for \"Higgs:2015mei\"
You've used 0 entries,
            1791 wiz_defined-function locations,
            448 strings with 3572 characters,
and the built_in function-call counts, 18 in all, are:
= -- 0
> -- 0
< -- 0
+ -- 0
- -- 0
* -- 2
:= -- 7
add.period$ -- 0
call.type$ -- 0
change.case$ -- 0
chr.to.int$ -- 0
cite$ -- 0
duplicate$ -- 0
empty$ -- 1
format.name$ -- 0
if$ -- 1
int.to.chr$ -- 0
int.to.str$ -- 0
missing$ -- 0
newline$ -- 3
num.names$ -- 0
pop$ -- 0
preamble$ -- 1
purify$ -- 0
quote$ -- 0
skip$ -- 1
stack$ -- 0
substring$ -- 0
swap$ -- 0
text.length$ -- 0
text.prefix$ -- 0
top$ -- 0
type$ -- 0
warning$ -- 0
while$ -- 0
width$ -- 0
write$ -- 2
(There were 2 warnings)
";

pub const BLG_BIBLATEX: &'static [u8] = b"\
[0] Config.pm:354> INFO - This is Biber 2.7
[0] Config.pm:357> INFO - Logfile is 'test_biber.blg'
[33] biber:303> INFO - === Mon Jun 12, 2017, 15:46:00
[48] Biber.pm:359> INFO - Reading 'test_biber.bcf'
[135] Biber.pm:835> INFO - Found 4 citekeys in bib section 0
[150] Biber.pm:3670> INFO - Processing section 0
[164] Biber.pm:3840> INFO - Looking for bibtex format file 'test_biber.bib' for section 0
[165] bibtex.pm:1435> INFO - Decoding LaTeX character macros into UTF-8
[169] bibtex.pm:1292> INFO - Found BibTeX data source 'test_biber.bib'
[170] Utils.pm:164> WARN - I didn't find a database entry for 'Guth:1980zm' (section 0)
[170] Utils.pm:164> WARN - I didn't find a database entry for 'Linde:1981mu' (section 0)
[170] Utils.pm:164> WARN - I didn't find a database entry for 'Albrecht:1982wi' (section 0)
[170] Utils.pm:164> WARN - I didn't find a database entry for '1982PhRvL..48.1220A' (section 0)
[178] UCollate.pm:68> INFO - Overriding locale 'en-US' defaults 'variable = shifted' with 'variable = non-ignorable'
[178] UCollate.pm:68> INFO - Overriding locale 'en-US' defaults 'normalization = NFD' with 'normalization = prenormalized'
[178] Biber.pm:3499> INFO - Sorting list 'nty/global/' of type 'entry' with scheme 'nty' and locale 'en-US'
[178] Biber.pm:3505> INFO - No sort tailoring available for locale 'en-US'
[179] bbl.pm:608> INFO - Writing 'test_biber.bbl' with encoding 'ascii'
[179] bbl.pm:712> INFO - Output to test_biber.bbl
[179] Biber.pm:109> INFO - WARNINGS: 4
";

pub const MONTH_STRINGS: &'static str = "\
@STRING{ jan = \"January\" }
@STRING{ feb = \"February\" }
@STRING{ mar = \"March\" }
@STRING{ apr = \"April\" }
@STRING{ may = \"May\" }
@STRING{ jun = \"June\" }
@STRING{ jul = \"July\" }
@STRING{ aug = \"August\" }
@STRING{ sep = \"September\" }
@STRING{ oct = \"October\" }
@STRING{ nov = \"November\" }
@STRING{ dec = \"December\" }
";
