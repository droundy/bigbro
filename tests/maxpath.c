#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <assert.h>

// This tests what happens if we create a path longer than PATH_MAX.
// What should happen is that we fail to see all the file activity,
// but that nothing actually crashes.  This is not optimal, but since
// system calls such as readlink fail when encountering long paths
// like this, I figure it is okay for us to do so also.  I suppose we
// could report some sort of "fake" result, but for now this is what
// we are doing.

int main() {
  int subd = openat(AT_FDCWD, "tmp/subdir1", O_RDONLY | O_DIRECTORY);

  fprintf(stderr, "subd is %d\n", subd);
  const char *longstr = "abcdef_0123456789_ABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789_abcdef_0123456789_ABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";
  int fard = subd;
  for (int i=0; i<40; i++) {
    fprintf(stderr, "strlen is %lu\n", (unsigned long)strlen(longstr));
    fprintf(stderr, "making the %d %s in %d\n", i, longstr, fard);
    int retval = mkdirat(fard, longstr, 0777);
    if (retval) {
      perror("mkdirat");
    }
    fard = openat(fard, longstr, O_RDONLY | O_DIRECTORY);
    if (fard<0) {
      perror("openat");
      fprintf(stderr, "that was the error...\n");
    }
    fprintf(stderr, "fard = %d, errno = %d\n", fard, errno);
  }

  int retval = linkat(subd, "foo_symlink",
                      fard, longstr, AT_SYMLINK_FOLLOW);
  if (retval < 0) {
    perror("trouble");
  }
  printf("retval was %d\n", retval);
  return 0;
}
