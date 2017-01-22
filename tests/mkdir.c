#define _POSIX_C_SOURCE 200809L

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

int main() {
  mkdir("tmp/subdirnew", 0777);
  return 0;
}
