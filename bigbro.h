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
   If it is null, then the current working directory is inherited.

   The pid of the child is stored in the address pointed to by
   child_ptr once the child process has been spawned.  This is needed
   if the parent process wants to kill the children before they
   complete, either due to user interrupt (e.g. SIGINT) or perhaps if
   they wanted to implement a timeout.  Note that this pointer is
   useless if the user is not using threads.  A null pointer may be
   passed, if the caller does not care what PID is spawned.

   stdoutfd (if it is greater than zero) is a file descriptor to which
   stdout will be redirected for the process.  If not greater than
   zero, stdout is inherited.

   stderrfd (if it is greater than zero) is a file descriptor to which
   stderr will be redirected for the process.  If not greater than
   zero, stderr is inherited.

   envp is an array of strings, conventionally of the form key=value,
   which are passed as environment to the new program.  If it is null,
   the environment is inherited from the parent.

   *read_from_directories is where we store the pointer to the array
   holding the paths of directories that were read (e.g. with
   readdir).

   *read_from_files is where we store the pointer to the array holding
   the paths of files that were read (e.g. with open for reading, or
   with stat). Note that if the file is renamed or written to after
   being read, it is not listed as having been read.

   *written_to_files is where we store the pointer to the array
   holding the paths of files that were written to (e.g. with
   open for writing, or with truncate).

   The return value is the exit code of the command.  However, if
   there was a problem running the command, the return value is -1.

  */

#ifdef _MSC_VER
typedef int pid_t;
#endif

#ifdef _WIN32
typedef void *bigbro_fd_t; // actually a HANDLE
#else
typedef int bigbro_fd_t;
#endif


int bigbro(const char *workingdir, pid_t *child_ptr,
           bigbro_fd_t stdoutfd, bigbro_fd_t stderrfd, char *envp[],
           const char *commandline, char ***read_from_directories,
           char ***read_from_files, char ***written_to_files);

int bigbro_with_mkdir(const char *workingdir, pid_t *child_ptr,
                      bigbro_fd_t stdoutfd, bigbro_fd_t stderrfd, char *envp[],
                      const char *commandline,
                      char ***read_from_directories, char ***mkdir_directories,
                      char ***read_from_files, char ***written_to_files);
