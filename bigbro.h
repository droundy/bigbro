#include <sys/types.h>

/** @file

 \brief This is the documentation of the bigbro library. This library provides just
 two functions which you can use to execute a process and track (or
 not track) the files it accesses.

 For a simple example of the bigbro API in use, see
 [fileaccesses.c](coverage.fileaccesses.c.html)

 */

#include <stdlib.h>

#ifdef _MSC_VER
typedef int pid_t;
#endif

/**

  \brief The type of a file descriptor on POSIX systems, but a HANDLE on Windows.

  To write code that is portable to windows, use `invalid_bigbro_fd`
  to create an invalid file `bigbro_fd_t`.
*/
#ifdef _WIN32
typedef void *bigbro_fd_t; // actually a HANDLE
static const bigbro_fd_t invalid_bigbro_fd = NULL;
#else
typedef int bigbro_fd_t;
static const bigbro_fd_t invalid_bigbro_fd = -1;
#endif

/**

   \brief Run a command and track its file accesses.

   bigbro executes the requested program and tracks files (and
   directories) modified and read.  It is a blocking function, but is
   reentrant, so you can run several processes simultaneously in
   different threads.

   The char *** arguments read_from_directories, read_from_files, and
   written_to_files are pointers to pointers, where arrays of
   null-terminated C strings will be stored holding the set of
   absolute paths.  To free this data structure, one need a single
   call to free, since the entire array is actually stored as a
   contiguous block.

   \param workingdir This is the working directory in which the child
   process will be run.  If it is `NULL`, then the current working
   directory is inherited by the child.

   \param child_ptr This is a pointer to a `pid_t` (int on Windows),
   which will be set to the process ID of the child process.  This is
   needed if the parent process wants to kill the children before they
   complete, either due to user interrupt (e.g. SIGINT) or perhaps if
   they wanted to implement a timeout.  Note that making use of this
   will require using threads, since otherwise when bigbro exits, the
   child will no longer exist.  A null pointer may be passed, if the
   caller does not care what PID is spawned.

   \param stdoutfd This (if it is greater than zero) is a file descriptor to which
   stdout will be redirected for the process.  If not greater than
   zero, stdout is inherited.

   \param stderrfd This (if it is greater than zero) is a file descriptor to which
   stderr will be redirected for the process.  If not greater than
   zero, stderr is inherited.

   \param envp This is an array of strings, conventionally of the form key=value,
   which are passed as environment to the new program.  If it is null,
   the environment is inherited from the parent.

   \param commandline This is the command itself that is executed.  It is
   interpreted by the shell (`sh -c "commandline"` on linux).  Note
   that this differs from the `exec` family of functions, but is
   similar to the Windows API.

   \param read_from_directories This is pointer to a `char **` that will
   store the list of directories that the process read from.  This
   pointer should be freed using `free` after the information is no
   longer needed.  This is a NULL-terminated array of C strings, which
   can be read using code such as:

        char **read_from_directories = NULL;
        bigbro(..., &read_from_directories, ...);
        for (int i=0; read_from_directories[i]; i++) {
          printf("We read from directory %s\n", read_from_directories[i]);
        }
        free(read_from_directories);
   For details of what is meant by a process having "read from a
   directory", see [semantics](semantics.html).

   \param mkdir_directories A pointer to a `char **` that will store
   the list of directories that the process created.  This pointer
   should be freed using `free` after the information is no longer
   needed.  This is a NULL-terminated array of C strings, which can be
   read using code such as:

        char **mkdir_directories = NULL;
        bigbro(..., &mkdir_directories, ...);
        for (int i=0; mkdir_directories[i]; i++) {
          printf("We created directory %s\n", mkdir_directories[i]);
        }
        free(mkdir_directories);
   For details of what is meant by a process having "created a
   directory", see [semantics](semantics.html).

   \param read_from_files A pointer to a `char **` that will store the
   list of files that the process read from.  This pointer should be
   freed using `free` after the information is no longer needed.  This
   is a NULL-terminated array of C strings, which can be read using
   code such as:

        char **read_from_files = NULL;
        bigbro(..., &read_from_files, ...);
        for (int i=0; read_from_files[i]; i++) {
          printf("We read from directory %s\n", read_from_files[i]);
        }
        free(read_from_files);
   For details of what is meant by a process having "read from a
   file", see [semantics](semantics.html).

   \param written_to_files A pointer to a `char **` that will store
   the list of files that the process wrote to.  This pointer should
   be freed using `free` after the information is no longer needed.
   This is a NULL-terminated array of C strings, which can be read
   using code such as:

        char **written_to_files = NULL;
        bigbro(..., &written_to_files);
        for (int i=0; written_to_files[i]; i++) {
          printf("We read from directory %s\n", written_to_files[i]);
        }
        free(written_to_files);
   For details of what is meant by a process having "written to a
   file", see [semantics](semantics.html).

   \return The return value is the exit code of the command.  However, if
   there was a problem running the command, the return value is -1.

  */

int bigbro(const char *workingdir, pid_t *child_ptr,
           bigbro_fd_t stdoutfd, bigbro_fd_t stderrfd, char *envp[],
           const char *commandline,
           char ***read_from_directories, char ***mkdir_directories,
           char ***read_from_files, char ***written_to_files);


/**

   \brief Run a command without tracking its file accesses.

   `bigbro_blind` executes the requested program and tracks files (and
   directories) modified and read.  It is a blocking function, but is
   reentrant, so you can run several processes simultaneously in
   different threads.

   The parameters have the same meaning as those of `bigbro` defined
   above.

  */

int bigbro_blind(const char *workingdir, pid_t *child_ptr,
                 bigbro_fd_t stdoutfd, bigbro_fd_t stderrfd, char *envp[],
                 const char *commandline);

/**

   \brief Fuction to run after forking but before exec in order to
   enable a process to be traced by bigbro_process.

   In practice, I don't believe you need actually exec, if you're
   interested in tracing your own executable, for instance...

 */

void bigbro_before_exec(void);


/**

   \brief Track filesystem access from an existing process that has
   run `bigbro_before_exec`.

   \param child The process ID of the child to be traced.

   The rest of the parameters are the same as those of `bigbro`.

 */

int bigbro_process(pid_t child,
                   char ***read_from_directories,
                   char ***mkdir_directories,
                   char ***read_from_files,
                   char ***written_to_files);
