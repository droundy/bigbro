#define _POSIX_C_SOURCE 200809L

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>

int main() {
  open("tmp/useless", O_WRONLY | O_CREAT, 0666);
  mkdir("tmp/subdirnew.temporary", 0777);
  mkdir("tmp/subdirnew.temporary/subsub", 0777);
  open("tmp/subdirnew.temporary/subfile", O_WRONLY | O_CREAT, 0666);
  rename("tmp/subdirnew.temporary", "tmp/subdirnew");
  return 0;
}
