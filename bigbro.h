#include <sys/types.h>

/* bigbro executes the requested program and tracks files (and
   directories) modified and read.  It is a blocking function, but is
   reentrant, so you can run several processes simultaneously in
   different threads.

   The char *** arguments read_from_directories, read_from_files, and
   written_to_files are pointers to pointers, where arrays of
   null-terminated C strings will be stored holding the set of
   absolute paths.  To free this data structure, one need a single
   call to free, since the entire array is actually stored as a
   contiguous block.

   workingdir is the directory in which the child process will be run.

   The pid of the child is stored in the address pointed to by
   child_ptr once the child process has been spawned.  This is needed
   if the parent process wants to kill the children before they
   complete, either due to user interrupt (e.g. SIGINT) or perhaps if
   they wanted to implement a timeout.  Note that this pointer is
   useless if the user is not using threads.

   stdouterrfd (if it is greater than zero) is a file descriptor to
   which stdout and stderr will be redirected for the process.

   *read_from_directories is where we store the pointer to the array
   holding the paths of directories that were read (e.g. with
   readdir).

   *read_from_files is where we store the pointer to the array
   holding the paths of files that were read (e.g. with
   open for reading, or with stat).

   *written_to_files is where we store the pointer to the array
   holding the paths of files that were written to (e.g. with
   open for writing, or with truncate).

  */

int bigbro(const char *workingdir, pid_t *child_ptr, int stdouterrfd,
           char **args, char ***read_from_directories,
           char ***read_from_files, char ***written_to_files);
