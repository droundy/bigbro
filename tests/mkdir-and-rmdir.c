#define _POSIX_C_SOURCE 200809L

#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>

int main() {
  mkdir("tmp/subdirnew.temporary", 0777);

  mkdir("tmp/subdirnew.temporary/subsub", 0777);
  int subd = open("tmp/subdirnew.temporary", O_RDONLY | O_DIRECTORY);
  unlinkat(subd, "subsub", AT_REMOVEDIR);

  rmdir("tmp/subdirnew.temporary");
  return 0;
}
