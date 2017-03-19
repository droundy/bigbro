#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <sys/types.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <fcntl.h>
#include <stdio.h>
#include <errno.h>

// NOTE: futimesat is obsolete (in favor of utimesat), but we should
// still support any programs that might use it.

int main() {
  int subd = openat(AT_FDCWD, "tmp/subdir1", O_RDONLY | O_DIRECTORY);
  struct timeval *ts = NULL;

  // the following should all fail, but should also not cause a crash
  // that would prevent the following utimensat from being traced.
  futimesat(subd, "foo_symlink_typo", ts);
  // The following is a way to silence a warning on using a null
  // pointer here.  We are intentionally giving a bad value to ensure
  // it doesn't screw up bigbro.
  char *mynull = ((char *)5) - 5;
  futimesat(subd, mynull, ts);
  futimesat(subd, "", ts);

  int retval = futimesat(subd, "foo_symlink", ts);
  if (retval < 0) {
    perror("trouble");
  }
  printf("retval was %d\n", retval);
  return 0;
}
