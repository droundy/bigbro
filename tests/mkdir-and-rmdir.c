#define _POSIX_C_SOURCE 200809L

#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>

int main() {
  mkdir("tmp/subdirnew.temporary", 0777);
  rmdir("tmp/subdirnew.temporary");
  return 0;
}
