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
  int retval = utimensat(subd, "foo_symlink", ts, 0);
  if (retval < 0) {
    perror("trouble");
  }
  printf("retval was %d\n", retval);
  return 0;
}
