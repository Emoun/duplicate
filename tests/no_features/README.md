Expansion Tests
=====================

This folder contains expansion tests. 
When testing, the files in `from` are copied to a new testing folder called `testing` (which is ignored by git).
The files in `expected` are the expected results of the expansion in `files` (they match by name when ignoring `.expanded`).
These files are also copied into the testing folder before testing starts.
The `expected_both` folder contains the expected expansion for both syntaxes. They are copied to the testing folder with a prefix of either "short_" or "verbose_".
The two copied files should match two files from `from` (again ignoring `.expanded`).

Don't remove `testing` from the git ignoring, or use it as a place for permanent files. It should be deletable at any time.


