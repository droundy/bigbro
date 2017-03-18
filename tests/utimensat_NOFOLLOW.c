#define _POSIX_C_SOURCE 200809L

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>
#include <errno.h>

int main() {
  int tmpd = open("tmp", O_RDONLY | O_DIRECTORY);
  int subd = openat(tmpd, "subdir1", O_RDONLY | O_DIRECTORY);
  fprintf(stderr, "subd is %d\n", subd);
  openat(subd, "openat", O_WRONLY | O_CREAT | O_EXCL, 0666);
  struct timespec ts[2] = {{ UTIME_NOW, UTIME_NOW },
                           { UTIME_NOW, UTIME_NOW }};

  // the following should all fail, but should also not cause a crash
  // that would prevent the following utimensat from being traced.
  utimensat(subd, "foo_symlink_typo", ts, AT_SYMLINK_NOFOLLOW);
  utimensat(subd, NULL, ts, AT_SYMLINK_NOFOLLOW);
  utimensat(subd, "", ts, AT_SYMLINK_NOFOLLOW);

  int retval = utimensat(subd, "foo_symlink", ts, AT_SYMLINK_NOFOLLOW);
  if (retval < 0) {
    perror("trouble");
  }
  printf("retval was %d\n", retval);
  return 0;
}
