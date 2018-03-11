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
  int retval = openat(subd, "foo_symlink", O_WRONLY);
  if (retval < 0) {
    perror("trouble");
  }
  printf("retval was %d\n", retval);
  return 0;
}
