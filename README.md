# rename
Rename files either using basic strings or regular expressions.

# Usage:
         rename <from> <to> <path> [<path> ...]
     
Rename will rename all occurences of <from> to <to> in the file names of each file for which the path is provided. Rename will *only* modify the final part of the path itself. Any preceeding directories will not be changed. By default <from> is interpreted as a literal string. To use a regular expression, use the -r option. Rename will not replace an existing file.
     
Rename will return 0 (zero) if any files were successfully renamed, and 1 otherwise.
     
# Examples:
```
$rename less more /some/lesser/path/to/a/file/called/more-or-less
```
will result in the new name:
```
/some/lesser/path/to/a/file/called/more-or-more
```
and
```
$rename -r "or[a-z] uch /some/lesser/path/to/a/file/called/more-or-less
```
will result in the new name:
```
/some/lesser/path/to/a/file/called/much-or-less	
```
     
     Options:
          -h | --help  This usage information.
          -r           Indicates the <from> parmaeter will be a regular expression. The
                                  standard regular expression must be compatible with the
                                  rust implementation of regular expressions.
          -v           Verbose. Print more verbose messages as rename runs.
