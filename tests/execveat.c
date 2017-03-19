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
#include <sys/syscall.h>
#include <linux/fs.h>

// the following syscalls do not have wrappers in glibc, so we need to
// implement them using syscall.  :(
static int renameat2(int olddirfd, const char *oldpath,
                     int newdirfd, const char *newpath, unsigned int flags) {
  return syscall(SYS_renameat2, olddirfd, oldpath, newdirfd, newpath, flags);
}

static int execveat(int dirfd, const char *pathname,
                    char *const argv[], char *const envp[],
                    int flags) {
  return syscall(SYS_execveat, dirfd, pathname, argv, envp, flags);
}


int main() {
  int tmpd = openat(AT_FDCWD, "tmp", O_RDONLY | O_DIRECTORY);
  int subd = openat(tmpd, "subdir1", O_RDONLY | O_DIRECTORY);
  int sub2d = openat(tmpd, "subdir2", O_RDONLY | O_DIRECTORY);
  int testsd = openat(AT_FDCWD, "tests", O_RDONLY | O_DIRECTORY);

  int ret = linkat(testsd, "linkat.test", tmpd, "awesome.exe", 0);
  fprintf(stderr, "linkat -> %d\n", ret);
  ret = renameat(tmpd, "awesome.exe", subd, "awesome");
  fprintf(stderr, "renameat -> %d\n", ret);

  ret = renameat2(tmpd, "subdir1", sub2d, "hidden", 0);
  fprintf(stderr, "renameat2 -> %d\n", ret);

  char *const argv[] = { "awesome", NULL };
  execveat(testsd, "null.test", argv, NULL, 0);

  return 0;
}
