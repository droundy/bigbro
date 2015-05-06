#include "realpath.h"

static inline char *interpret_path_at(pid_t pid, int fd, const char *path) {
  /* printf("path: %s\n", path); */
  const char *procself = "/proc/self/";
  const int procselflen = strlen(procself);
  if (strlen(path) > procselflen &&
      !memcmp(path, procself, procselflen)) {
    char *out = malloc(strlen(path) + 200);
    snprintf(out, 200, "/proc/%d/", pid);
    strcat(out, path+procselflen);
    /* printf("rawpath: %s\n", out); */
    return out;
  }
  /* printf("path '%s' does not match '%s'\n", path, procself); */

  const int pathmax = 4096;
  if (!path) return 0;
  if (path[0] == '/') {
    /* printf("rawpath: %s\n", path); */
    return strdup(path);
  }
  char *proc_fd = malloc(pathmax);
  if (fd >= 0) snprintf(proc_fd, 4000, "/proc/%d/fd/%d", pid, fd);
  else snprintf(proc_fd, pathmax, "/proc/%d/cwd", pid);
  char *cwd = malloc(pathmax);
  int linklen = readlink(proc_fd, cwd, pathmax);
  if (linklen < 0) {
    fprintf(stderr, "unable to determine cwd!!!\n");
    return 0;
  }
  cwd[linklen] = 0;
  /* printf("cwd(%d): %s (from %s)\n", fd, cwd, proc_fd); */
  if (strlen(cwd) + strlen(path) + 4 > pathmax) {
    fprintf(stderr, "too long a path: '%s/%s'\n", cwd, path);
    return 0;
  }
  strcat(cwd, "/");
  strcat(cwd, path);
  free(proc_fd);
  /* printf("rawpath: (%d) %s\n", fd, cwd); */
  return cwd;
}

static inline void read_dir_fd(pid_t pid, int dirfd, rw_status *h) {
  char *rawpath = interpret_path_at(pid, dirfd, ".");
  char *abspath = nice_realpath(rawpath, 0, h);
  if (!lookup_in_hash(&h->mkdir, abspath)) {
    insert_hashset(&h->readdir, abspath);
  }
  free(rawpath);
  free(abspath);
}

static inline void read_file_at(pid_t pid, int dirfd, const char *path,
                                rw_status *h) {
  char *rawpath = interpret_path_at(pid, dirfd, path);
  char *abspath = nice_realpath(rawpath, 0, h);
  /* printf("abspath: %s\n", abspath); */
  struct stat st;
  if (!lookup_in_hash(&h->written, abspath) && !stat(abspath, &st) && S_ISREG(st.st_mode)) {
    insert_hashset(&h->read, abspath);
  }
  free(rawpath);
  free(abspath);
}

static inline void write_file_at(pid_t pid, int dirfd, const char *path,
                                 rw_status *h) {
  char *rawpath = interpret_path_at(pid, dirfd, path);
  char *abspath = nice_realpath(rawpath, 0, h);
  insert_hashset(&h->written, abspath);
  delete_from_hashset(&h->read, abspath);
  free(rawpath);
  free(abspath);
}
