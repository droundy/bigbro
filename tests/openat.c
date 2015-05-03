#define _ATFILE_SOURCE

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

int main() {
  int tmpd = open("tmp", O_RDONLY);
  openat(tmpd, "openat", O_WRONLY | O_CREAT | O_EXCL);
  return 0;
}
