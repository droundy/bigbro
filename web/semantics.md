# Bigbro Semantics

What does is mean to read from a file or directory, or to write to a
file or create a directory?

## File write

A file write is any operation that modifies a file's content, its
metadata, or its modification time.  On linux these system calls
include `open` (for writing), `truncate`, `utimes`, `rename`, etc.

## File read

A file read is any operation that could have its results be affected
by a file write.  Most obvious is `open` (for reading), but `stat`
also counts as a read.  It is common to stat a file in order to
determine whether it has been changed.

A file that is written to is never reported as also being read from.

## Directory read

A directory read is any operation that will query the contents of that
directory.  In POSIX this would be `readdir`, which on linux is
implemented using `getdents` or `getdents64`.

When a process reads a directory, it can determine the names of all
files in that directory.

On linux, it can also find the sizes and inode numbers of those files,
and even their file type (regular file, directory, symlink, etc).  For
purposes of bigbro semantics, we do *not* interpret this as a read of
each of those files, even though technically we ought to, as this
would lead to numerous false positive file reads.

## Mkdir

Finally, we track any directories that are created by the command.

Note that this list is *not* complementary to directory read (as file
read is complementary to file write) in the sense that the `mkdir`
list does not incorporate all actions that could affect the outcome of
a directory read.

## Handling renames and deletions

Bigbro will not report on any entity that is deleted by the process.
So if a file is created and then deleted, it will not be mentioned.
Similarly, if a file is renamed after being created, only the new name
is reported.

