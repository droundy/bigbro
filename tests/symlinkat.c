#define _BSD_SOURCE
#define _XOPEN_SOURCE 700

#include <fcntl.h>
#include <unistd.h>

int main(int argc, char **argv) {
  return symlinkat("tmp/new-symlink", AT_FDCWD, "foobar");
}
