# libbigbro documentation

$docnav

This is the documentation bigbro library.  This libray provides a
single function which you can use to execute a process and track the
files it accesses.

    int bigbro_with_mkdir(const char *workingdir, pid_t *child_ptr,
                          bigbro_fd_t stdoutfd, bigbro_fd_t stderrfd, char *envp[],
                          const char *commandline,
                          char ***read_from_directories, char ***mkdir_directories,
                          char ***read_from_files, char ***written_to_files);

This function has several arguments.

`workingdir`
: This is the working directory when the executable is run.  If
  `NULL`, then using the current working directory.

`child_ptr`
: This is a pointer to a `pid_t`, which will be set to the process ID
  of the child process.  You should use this if you want to send a
  signal to the child process (e.g. to kill it).  Note that making use
  of this will require using threads, since otherwise when bigbro
  exits, the child will no longer exist.

`stdoutfd`
: This is a file descriptor for `stdout` of the child.  If it is zero,
  then `stdout` is inherited.

`stderrfd`
: This is a file descriptor for `stderr` of the child.  If it is zero,
  then `stderr` is inherited.

`envp`
: This is the environment for the child.  If set to `NULL`, the
  environment is inherited.

`commandline`
: This is the command itself that is executed.  It is interpreted by
  the shell (`sh -c "commandline"` on linux).  Note that this differs
  from the `exec` family of functions, but is similar to the Windows
  API.

`read_from_directories`
: A pointer to a `char **` that will store the list of directories
  that the process read from.  This pointer should be freed using
  `free` after the information is no longer needed.  This is a
  NULL-terminated array of C strings, which can be read using code
  such as:

        char **read_from_directories = NULL;
        bigbro_with_mkdir(..., &read_from_directories, ...);
        for (int i=0; read_from_directories[i]; i++) {
          printf("We read from directory %s\n", read_from_directories[i]);
        }
        free(read_from_directories);
  For details of what is meant by a process having "read from a
  directory", see [semantics](semantics.html).

`mkdir_directories`
: A pointer to a `char **` that will store the list of directories
  that the process created.  This pointer should be freed using
  `free` after the information is no longer needed.  This is a
  NULL-terminated array of C strings, which can be read using code
  such as:

        char **mkdir_directories = NULL;
        bigbro_with_mkdir(..., &mkdir_directories, ...);
        for (int i=0; mkdir_directories[i]; i++) {
          printf("We created directory %s\n", mkdir_directories[i]);
        }
        free(mkdir_directories);
  For details of what is meant by a process having "created a
  directory", see [semantics](semantics.html).

`read_from_files`
: A pointer to a `char **` that will store the list of files
  that the process read from.  This pointer should be freed using
  `free` after the information is no longer needed.  This is a
  NULL-terminated array of C strings, which can be read using code
  such as:

        char **read_from_files = NULL;
        bigbro_with_mkdir(..., &read_from_files, ...);
        for (int i=0; read_from_files[i]; i++) {
          printf("We read from directory %s\n", read_from_files[i]);
        }
        free(read_from_files);
  For details of what is meant by a process having "read from a
  file", see [semantics](semantics.html).

`written_to_files`
: A pointer to a `char **` that will store the list of files
  that the process wrote to.  This pointer should be freed using
  `free` after the information is no longer needed.  This is a
  NULL-terminated array of C strings, which can be read using code
  such as:

        char **written_to_files = NULL;
        bigbro_with_mkdir(..., &written_to_files, ...);
        for (int i=0; written_to_files[i]; i++) {
          printf("We read from directory %s\n", written_to_files[i]);
        }
        free(written_to_files);
  For details of what is meant by a process having "written to a
  file", see [semantics](semantics.html).

For a simple example of the bigbro API in use, see [fileaccesses.c](coverage.fileaccesses.c.html)
