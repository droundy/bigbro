/* bigbro filetracking library
   Copyright (C) 2015,2016 David Roundy

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
   02110-1301 USA */

#define _GNU_SOURCE

#define _XOPEN_SOURCE 700
#define __BSD_VISIBLE 1

#include "bigbro.h"

#ifndef _WIN32
#include "realpath.h"
#include <fcntl.h>
#else
#include <io.h>
#include <windows.h>
#endif

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>


int main(int argc, char **argv) {
  // the following are to test the corner and error cases of
  // flexible_realpath.
#ifndef _WIN32
  assert(! flexible_realpath(NULL, NULL, look_for_file_or_directory));
  assert(! flexible_realpath("", NULL, look_for_file_or_directory));
  // the following should give null because flexible_realpath wants an
  // absolute path.
  assert(! flexible_realpath("tmp", NULL, look_for_file_or_directory));
#endif

  if (argc < 2) {
    fprintf(stderr, "Usage: %s cmdline\n", argv[0]);
    exit(1);
  }
  bigbro_fd_t stdoutfd = invalid_bigbro_fd;
  if (strcmp(argv[1],"-o") == 0) {
    assert(argc > 2);
    printf("outputting to file %s\n", argv[2]);

#ifdef _WIN32
    SECURITY_ATTRIBUTES sa;
    sa.nLength = sizeof(sa);
    sa.lpSecurityDescriptor = NULL;
    sa.bInheritHandle = TRUE;
    stdoutfd = CreateFileA(argv[2],                // name of the write
                           GENERIC_WRITE,          // open for writing
                           0,                      // do not share
                           &sa,                   // default security
                           CREATE_ALWAYS,         // just be sure there is a file here
                           FILE_ATTRIBUTE_NORMAL,  // normal file
                           NULL);                  // no attr. template
#else
    stdoutfd = creat(argv[2], 0666);
#endif
    argc -= 2;
    argv += 2;
  }

  char **written_to_files = NULL;
  char **read_from_files = NULL;
  char **read_from_directories = NULL;
  char **mkdir_directories = NULL;

  int cmdlength = 10; // a little leeway
  for (int i=1; argv[i]; i++) {
    cmdlength += strlen(argv[i]) + 1;
  }
  char *cmdline = (char *)calloc(1,cmdlength);
  strcpy(cmdline, argv[1]);
  for (int i=2; argv[i]; i++) {
    strcat(cmdline, " ");
    strcat(cmdline, argv[i]);
  }
  pid_t child_pid;
  int retval = bigbro(".", &child_pid, stdoutfd, 0, NULL, cmdline,
                      &read_from_directories, &mkdir_directories,
                      &read_from_files, &written_to_files);
  free(cmdline);

  if (read_from_directories)
    for (int i=0; read_from_directories[i]; i++) {
      fprintf(stderr, "l: %s\n", read_from_directories[i]);
    }
  if (read_from_files)
    for (int i=0; read_from_files[i]; i++) {
      fprintf(stderr, "r: %s\n", read_from_files[i]);
    }
  if (written_to_files)
    for (int i=0; written_to_files[i]; i++) {
      fprintf(stderr, "w: %s\n", written_to_files[i]);
    }
  if (mkdir_directories)
    for (int i=0; mkdir_directories[i]; i++) {
      fprintf(stderr, "m: %s\n", mkdir_directories[i]);
    }
  free(read_from_directories);
  free(read_from_files);
  free(written_to_files);
  return retval;
}
