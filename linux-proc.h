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
    fprintf(stderr, "unable to determine cwd from %s i.e. fd %d!!!\n",
            proc_fd, fd);
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
  if (!rawpath) {
    fprintf(stderr, "read_dir_fd fails for pid %d and dirfd %d\n", pid, dirfd);
    exit(1);
  }
  char *abspath = flexible_realpath(rawpath, 0, h, look_for_file_or_directory);
  if (!lookup_in_hash(&h->mkdir, abspath)) {
    insert_hashset(&h->readdir, abspath);
  }
  free(rawpath);
  free(abspath);
}

static inline void read_something_at(pid_t pid, int dirfd, const char *path,
                                     rw_status *h, enum last_symlink_handling lh) {
  char *rawpath = interpret_path_at(pid, dirfd, path);
  if (!rawpath) {
    fprintf(stderr, "read_something_at fails for pid %d and dirfd %d and path %s\n",
            pid, dirfd, path);
    exit(1);
  }
  char *abspath = flexible_realpath(rawpath, 0, h, lh);
  /* printf("abspath: %s\n", abspath); */
  struct stat st;
  if (!lookup_in_hash(&h->written, abspath) && !stat(abspath, &st) && S_ISREG(st.st_mode)) {
    insert_hashset(&h->read, abspath);
  }
  free(rawpath);
  free(abspath);
}

static inline void write_something_at(pid_t pid, int dirfd, const char *path,
                                      rw_status *h, enum last_symlink_handling lh) {
  char *rawpath = interpret_path_at(pid, dirfd, path);
  if (!rawpath) {
    fprintf(stderr, "write_something_at fails for pid %d and dirfd %d and path %s\n",
            pid, dirfd, path);
    exit(1);
  }
  char *abspath = flexible_realpath(rawpath, 0, h, lh);
  insert_hashset(&h->written, abspath);
  delete_from_hashset(&h->read, abspath);
  free(rawpath);
  free(abspath);
}

static inline void read_file_at(pid_t pid, int dirfd, const char *path,
                                rw_status *h) {
  read_something_at(pid, dirfd, path, h, look_for_file_or_directory);
}

static inline void write_file_at(pid_t pid, int dirfd, const char *path,
                                 rw_status *h) {
  write_something_at(pid, dirfd, path, h, look_for_file_or_directory);
}

static inline void read_link_at(pid_t pid, int dirfd, const char *path,
                                rw_status *h) {
  read_something_at(pid, dirfd, path, h, look_for_symlink);
}

static inline void write_link_at(pid_t pid, int dirfd, const char *path,
                                 rw_status *h) {
  write_something_at(pid, dirfd, path, h, look_for_symlink);
}
