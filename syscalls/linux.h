
enum syscall {
  sc_invalid_syscall = 0,
  sc_restart_syscall = 1,
  sc_exit = 2,
  sc_fork = 3,
  sc_read = 4,
  sc_write = 5,
  sc_open = 6,
  sc_close = 7,
  sc_waitpid = 8,
  sc_creat = 9,
  sc_link = 10,
  sc_unlink = 11,
  sc_execve = 12,
  sc_chdir = 13,
  sc_time = 14,
  sc_mknod = 15,
  sc_chmod = 16,
  sc_lchown = 17,
  sc_break = 18,
  sc_oldstat = 19,
  sc_lseek = 20,
  sc_getpid = 21,
  sc_mount = 22,
  sc_umount = 23,
  sc_setuid = 24,
  sc_getuid = 25,
  sc_stime = 26,
  sc_ptrace = 27,
  sc_alarm = 28,
  sc_oldfstat = 29,
  sc_pause = 30,
  sc_utime = 31,
  sc_stty = 32,
  sc_gtty = 33,
  sc_access = 34,
  sc_nice = 35,
  sc_ftime = 36,
  sc_sync = 37,
  sc_kill = 38,
  sc_rename = 39,
  sc_mkdir = 40,
  sc_rmdir = 41,
  sc_dup = 42,
  sc_pipe = 43,
  sc_times = 44,
  sc_prof = 45,
  sc_brk = 46,
  sc_setgid = 47,
  sc_getgid = 48,
  sc_signal = 49,
  sc_geteuid = 50,
  sc_getegid = 51,
  sc_acct = 52,
  sc_umount2 = 53,
  sc_lock = 54,
  sc_ioctl = 55,
  sc_fcntl = 56,
  sc_mpx = 57,
  sc_setpgid = 58,
  sc_ulimit = 59,
  sc_oldolduname = 60,
  sc_umask = 61,
  sc_chroot = 62,
  sc_ustat = 63,
  sc_dup2 = 64,
  sc_getppid = 65,
  sc_getpgrp = 66,
  sc_setsid = 67,
  sc_sigaction = 68,
  sc_sgetmask = 69,
  sc_ssetmask = 70,
  sc_setreuid = 71,
  sc_setregid = 72,
  sc_sigsuspend = 73,
  sc_sigpending = 74,
  sc_sethostname = 75,
  sc_setrlimit = 76,
  sc_getrlimit = 77,
  sc_getrusage = 78,
  sc_gettimeofday = 79,
  sc_settimeofday = 80,
  sc_getgroups = 81,
  sc_setgroups = 82,
  sc_select = 83,
  sc_symlink = 84,
  sc_oldlstat = 85,
  sc_readlink = 86,
  sc_uselib = 87,
  sc_swapon = 88,
  sc_reboot = 89,
  sc_readdir = 90,
  sc_mmap = 91,
  sc_munmap = 92,
  sc_truncate = 93,
  sc_ftruncate = 94,
  sc_fchmod = 95,
  sc_fchown = 96,
  sc_getpriority = 97,
  sc_setpriority = 98,
  sc_profil = 99,
  sc_statfs = 100,
  sc_fstatfs = 101,
  sc_ioperm = 102,
  sc_socketcall = 103,
  sc_syslog = 104,
  sc_setitimer = 105,
  sc_getitimer = 106,
  sc_stat = 107,
  sc_lstat = 108,
  sc_fstat = 109,
  sc_olduname = 110,
  sc_iopl = 111,
  sc_vhangup = 112,
  sc_idle = 113,
  sc_vm86old = 114,
  sc_wait4 = 115,
  sc_swapoff = 116,
  sc_sysinfo = 117,
  sc_ipc = 118,
  sc_fsync = 119,
  sc_sigreturn = 120,
  sc_clone = 121,
  sc_setdomainname = 122,
  sc_uname = 123,
  sc_modify_ldt = 124,
  sc_adjtimex = 125,
  sc_mprotect = 126,
  sc_sigprocmask = 127,
  sc_create_module = 128,
  sc_init_module = 129,
  sc_delete_module = 130,
  sc_get_kernel_syms = 131,
  sc_quotactl = 132,
  sc_getpgid = 133,
  sc_fchdir = 134,
  sc_bdflush = 135,
  sc_sysfs = 136,
  sc_personality = 137,
  sc_afs_syscall = 138,
  sc_setfsuid = 139,
  sc_setfsgid = 140,
  sc__llseek = 141,
  sc_getdents = 142,
  sc__newselect = 143,
  sc_flock = 144,
  sc_msync = 145,
  sc_readv = 146,
  sc_writev = 147,
  sc_getsid = 148,
  sc_fdatasync = 149,
  sc__sysctl = 150,
  sc_mlock = 151,
  sc_munlock = 152,
  sc_mlockall = 153,
  sc_munlockall = 154,
  sc_sched_setparam = 155,
  sc_sched_getparam = 156,
  sc_sched_setscheduler = 157,
  sc_sched_getscheduler = 158,
  sc_sched_yield = 159,
  sc_sched_get_priority_max = 160,
  sc_sched_get_priority_min = 161,
  sc_sched_rr_get_interval = 162,
  sc_nanosleep = 163,
  sc_mremap = 164,
  sc_setresuid = 165,
  sc_getresuid = 166,
  sc_vm86 = 167,
  sc_query_module = 168,
  sc_poll = 169,
  sc_nfsservctl = 170,
  sc_setresgid = 171,
  sc_getresgid = 172,
  sc_prctl = 173,
  sc_rt_sigreturn = 174,
  sc_rt_sigaction = 175,
  sc_rt_sigprocmask = 176,
  sc_rt_sigpending = 177,
  sc_rt_sigtimedwait = 178,
  sc_rt_sigqueueinfo = 179,
  sc_rt_sigsuspend = 180,
  sc_pread64 = 181,
  sc_pwrite64 = 182,
  sc_chown = 183,
  sc_getcwd = 184,
  sc_capget = 185,
  sc_capset = 186,
  sc_sigaltstack = 187,
  sc_sendfile = 188,
  sc_getpmsg = 189,
  sc_putpmsg = 190,
  sc_vfork = 191,
  sc_ugetrlimit = 192,
  sc_mmap2 = 193,
  sc_truncate64 = 194,
  sc_ftruncate64 = 195,
  sc_stat64 = 196,
  sc_lstat64 = 197,
  sc_fstat64 = 198,
  sc_lchown32 = 199,
  sc_getuid32 = 200,
  sc_getgid32 = 201,
  sc_geteuid32 = 202,
  sc_getegid32 = 203,
  sc_setreuid32 = 204,
  sc_setregid32 = 205,
  sc_getgroups32 = 206,
  sc_setgroups32 = 207,
  sc_fchown32 = 208,
  sc_setresuid32 = 209,
  sc_getresuid32 = 210,
  sc_setresgid32 = 211,
  sc_getresgid32 = 212,
  sc_chown32 = 213,
  sc_setuid32 = 214,
  sc_setgid32 = 215,
  sc_setfsuid32 = 216,
  sc_setfsgid32 = 217,
  sc_pivot_root = 218,
  sc_mincore = 219,
  sc_madvise = 220,
  sc_getdents64 = 221,
  sc_fcntl64 = 222,
  sc_gettid = 223,
  sc_readahead = 224,
  sc_setxattr = 225,
  sc_lsetxattr = 226,
  sc_fsetxattr = 227,
  sc_getxattr = 228,
  sc_lgetxattr = 229,
  sc_fgetxattr = 230,
  sc_listxattr = 231,
  sc_llistxattr = 232,
  sc_flistxattr = 233,
  sc_removexattr = 234,
  sc_lremovexattr = 235,
  sc_fremovexattr = 236,
  sc_tkill = 237,
  sc_sendfile64 = 238,
  sc_futex = 239,
  sc_sched_setaffinity = 240,
  sc_sched_getaffinity = 241,
  sc_set_thread_area = 242,
  sc_get_thread_area = 243,
  sc_io_setup = 244,
  sc_io_destroy = 245,
  sc_io_getevents = 246,
  sc_io_submit = 247,
  sc_io_cancel = 248,
  sc_fadvise64 = 249,
  sc_exit_group = 250,
  sc_lookup_dcookie = 251,
  sc_epoll_create = 252,
  sc_epoll_ctl = 253,
  sc_epoll_wait = 254,
  sc_remap_file_pages = 255,
  sc_set_tid_address = 256,
  sc_timer_create = 257,
  sc_timer_settime = 258,
  sc_timer_gettime = 259,
  sc_timer_getoverrun = 260,
  sc_timer_delete = 261,
  sc_clock_settime = 262,
  sc_clock_gettime = 263,
  sc_clock_getres = 264,
  sc_clock_nanosleep = 265,
  sc_statfs64 = 266,
  sc_fstatfs64 = 267,
  sc_tgkill = 268,
  sc_utimes = 269,
  sc_fadvise64_64 = 270,
  sc_vserver = 271,
  sc_mbind = 272,
  sc_get_mempolicy = 273,
  sc_set_mempolicy = 274,
  sc_mq_open = 275,
  sc_mq_unlink = 276,
  sc_mq_timedsend = 277,
  sc_mq_timedreceive = 278,
  sc_mq_notify = 279,
  sc_mq_getsetattr = 280,
  sc_kexec_load = 281,
  sc_waitid = 282,
  sc_add_key = 283,
  sc_request_key = 284,
  sc_keyctl = 285,
  sc_ioprio_set = 286,
  sc_ioprio_get = 287,
  sc_inotify_init = 288,
  sc_inotify_add_watch = 289,
  sc_inotify_rm_watch = 290,
  sc_migrate_pages = 291,
  sc_openat = 292,
  sc_mkdirat = 293,
  sc_mknodat = 294,
  sc_fchownat = 295,
  sc_futimesat = 296,
  sc_fstatat64 = 297,
  sc_unlinkat = 298,
  sc_renameat = 299,
  sc_linkat = 300,
  sc_symlinkat = 301,
  sc_readlinkat = 302,
  sc_fchmodat = 303,
  sc_faccessat = 304,
  sc_pselect6 = 305,
  sc_ppoll = 306,
  sc_unshare = 307,
  sc_set_robust_list = 308,
  sc_get_robust_list = 309,
  sc_splice = 310,
  sc_sync_file_range = 311,
  sc_tee = 312,
  sc_vmsplice = 313,
  sc_move_pages = 314,
  sc_getcpu = 315,
  sc_epoll_pwait = 316,
  sc_utimensat = 317,
  sc_signalfd = 318,
  sc_timerfd_create = 319,
  sc_eventfd = 320,
  sc_fallocate = 321,
  sc_timerfd_settime = 322,
  sc_timerfd_gettime = 323,
  sc_signalfd4 = 324,
  sc_eventfd2 = 325,
  sc_epoll_create1 = 326,
  sc_dup3 = 327,
  sc_pipe2 = 328,
  sc_inotify_init1 = 329,
  sc_preadv = 330,
  sc_pwritev = 331,
  sc_rt_tgsigqueueinfo = 332,
  sc_perf_event_open = 333,
  sc_recvmmsg = 334,
  sc_fanotify_init = 335,
  sc_fanotify_mark = 336,
  sc_prlimit64 = 337,
  sc_name_to_handle_at = 338,
  sc_open_by_handle_at = 339,
  sc_clock_adjtime = 340,
  sc_syncfs = 341,
  sc_sendmmsg = 342,
  sc_setns = 343,
  sc_process_vm_readv = 344,
  sc_process_vm_writev = 345,
  sc_kcmp = 346,
  sc_finit_module = 347,
  sc_sched_setattr = 348,
  sc_sched_getattr = 349,
  sc_renameat2 = 350,
  sc_seccomp = 351,
  sc_getrandom = 352,
  sc_memfd_create = 353,
  sc_bpf = 354,
  sc_execveat = 355,
  sc_socket = 356,
  sc_socketpair = 357,
  sc_bind = 358,
  sc_connect = 359,
  sc_listen = 360,
  sc_accept4 = 361,
  sc_getsockopt = 362,
  sc_setsockopt = 363,
  sc_getsockname = 364,
  sc_getpeername = 365,
  sc_sendto = 366,
  sc_sendmsg = 367,
  sc_recvfrom = 368,
  sc_recvmsg = 369,
  sc_shutdown = 370,
  sc_userfaultfd = 371,
  sc_membarrier = 372,
  sc_mlock2 = 373,
  sc_copy_file_range = 374,
  sc_preadv2 = 375,
  sc_pwritev2 = 376,
  sc_shmget = 377,
  sc_shmat = 378,
  sc_shmctl = 379,
  sc_accept = 380,
  sc_semget = 381,
  sc_semop = 382,
  sc_semctl = 383,
  sc_shmdt = 384,
  sc_msgget = 385,
  sc_msgsnd = 386,
  sc_msgrcv = 387,
  sc_msgctl = 388,
  sc_arch_prctl = 389,
  sc_tuxcall = 390,
  sc_security = 391,
  sc_epoll_ctl_old = 392,
  sc_epoll_wait_old = 393,
  sc_semtimedop = 394,
  sc_newfstatat = 395,
  sc_kexec_file_load
};


static const char *syscall_names[] = {
  "invalid_syscall",
  "restart_syscall",
  "exit",
  "fork",
  "read",
  "write",
  "open",
  "close",
  "waitpid",
  "creat",
  "link",
  "unlink",
  "execve",
  "chdir",
  "time",
  "mknod",
  "chmod",
  "lchown",
  "break",
  "oldstat",
  "lseek",
  "getpid",
  "mount",
  "umount",
  "setuid",
  "getuid",
  "stime",
  "ptrace",
  "alarm",
  "oldfstat",
  "pause",
  "utime",
  "stty",
  "gtty",
  "access",
  "nice",
  "ftime",
  "sync",
  "kill",
  "rename",
  "mkdir",
  "rmdir",
  "dup",
  "pipe",
  "times",
  "prof",
  "brk",
  "setgid",
  "getgid",
  "signal",
  "geteuid",
  "getegid",
  "acct",
  "umount2",
  "lock",
  "ioctl",
  "fcntl",
  "mpx",
  "setpgid",
  "ulimit",
  "oldolduname",
  "umask",
  "chroot",
  "ustat",
  "dup2",
  "getppid",
  "getpgrp",
  "setsid",
  "sigaction",
  "sgetmask",
  "ssetmask",
  "setreuid",
  "setregid",
  "sigsuspend",
  "sigpending",
  "sethostname",
  "setrlimit",
  "getrlimit",
  "getrusage",
  "gettimeofday",
  "settimeofday",
  "getgroups",
  "setgroups",
  "select",
  "symlink",
  "oldlstat",
  "readlink",
  "uselib",
  "swapon",
  "reboot",
  "readdir",
  "mmap",
  "munmap",
  "truncate",
  "ftruncate",
  "fchmod",
  "fchown",
  "getpriority",
  "setpriority",
  "profil",
  "statfs",
  "fstatfs",
  "ioperm",
  "socketcall",
  "syslog",
  "setitimer",
  "getitimer",
  "stat",
  "lstat",
  "fstat",
  "olduname",
  "iopl",
  "vhangup",
  "idle",
  "vm86old",
  "wait4",
  "swapoff",
  "sysinfo",
  "ipc",
  "fsync",
  "sigreturn",
  "clone",
  "setdomainname",
  "uname",
  "modify_ldt",
  "adjtimex",
  "mprotect",
  "sigprocmask",
  "create_module",
  "init_module",
  "delete_module",
  "get_kernel_syms",
  "quotactl",
  "getpgid",
  "fchdir",
  "bdflush",
  "sysfs",
  "personality",
  "afs_syscall",
  "setfsuid",
  "setfsgid",
  "_llseek",
  "getdents",
  "_newselect",
  "flock",
  "msync",
  "readv",
  "writev",
  "getsid",
  "fdatasync",
  "_sysctl",
  "mlock",
  "munlock",
  "mlockall",
  "munlockall",
  "sched_setparam",
  "sched_getparam",
  "sched_setscheduler",
  "sched_getscheduler",
  "sched_yield",
  "sched_get_priority_max",
  "sched_get_priority_min",
  "sched_rr_get_interval",
  "nanosleep",
  "mremap",
  "setresuid",
  "getresuid",
  "vm86",
  "query_module",
  "poll",
  "nfsservctl",
  "setresgid",
  "getresgid",
  "prctl",
  "rt_sigreturn",
  "rt_sigaction",
  "rt_sigprocmask",
  "rt_sigpending",
  "rt_sigtimedwait",
  "rt_sigqueueinfo",
  "rt_sigsuspend",
  "pread64",
  "pwrite64",
  "chown",
  "getcwd",
  "capget",
  "capset",
  "sigaltstack",
  "sendfile",
  "getpmsg",
  "putpmsg",
  "vfork",
  "ugetrlimit",
  "mmap2",
  "truncate64",
  "ftruncate64",
  "stat64",
  "lstat64",
  "fstat64",
  "lchown32",
  "getuid32",
  "getgid32",
  "geteuid32",
  "getegid32",
  "setreuid32",
  "setregid32",
  "getgroups32",
  "setgroups32",
  "fchown32",
  "setresuid32",
  "getresuid32",
  "setresgid32",
  "getresgid32",
  "chown32",
  "setuid32",
  "setgid32",
  "setfsuid32",
  "setfsgid32",
  "pivot_root",
  "mincore",
  "madvise",
  "getdents64",
  "fcntl64",
  "gettid",
  "readahead",
  "setxattr",
  "lsetxattr",
  "fsetxattr",
  "getxattr",
  "lgetxattr",
  "fgetxattr",
  "listxattr",
  "llistxattr",
  "flistxattr",
  "removexattr",
  "lremovexattr",
  "fremovexattr",
  "tkill",
  "sendfile64",
  "futex",
  "sched_setaffinity",
  "sched_getaffinity",
  "set_thread_area",
  "get_thread_area",
  "io_setup",
  "io_destroy",
  "io_getevents",
  "io_submit",
  "io_cancel",
  "fadvise64",
  "exit_group",
  "lookup_dcookie",
  "epoll_create",
  "epoll_ctl",
  "epoll_wait",
  "remap_file_pages",
  "set_tid_address",
  "timer_create",
  "timer_settime",
  "timer_gettime",
  "timer_getoverrun",
  "timer_delete",
  "clock_settime",
  "clock_gettime",
  "clock_getres",
  "clock_nanosleep",
  "statfs64",
  "fstatfs64",
  "tgkill",
  "utimes",
  "fadvise64_64",
  "vserver",
  "mbind",
  "get_mempolicy",
  "set_mempolicy",
  "mq_open",
  "mq_unlink",
  "mq_timedsend",
  "mq_timedreceive",
  "mq_notify",
  "mq_getsetattr",
  "kexec_load",
  "waitid",
  "add_key",
  "request_key",
  "keyctl",
  "ioprio_set",
  "ioprio_get",
  "inotify_init",
  "inotify_add_watch",
  "inotify_rm_watch",
  "migrate_pages",
  "openat",
  "mkdirat",
  "mknodat",
  "fchownat",
  "futimesat",
  "fstatat64",
  "unlinkat",
  "renameat",
  "linkat",
  "symlinkat",
  "readlinkat",
  "fchmodat",
  "faccessat",
  "pselect6",
  "ppoll",
  "unshare",
  "set_robust_list",
  "get_robust_list",
  "splice",
  "sync_file_range",
  "tee",
  "vmsplice",
  "move_pages",
  "getcpu",
  "epoll_pwait",
  "utimensat",
  "signalfd",
  "timerfd_create",
  "eventfd",
  "fallocate",
  "timerfd_settime",
  "timerfd_gettime",
  "signalfd4",
  "eventfd2",
  "epoll_create1",
  "dup3",
  "pipe2",
  "inotify_init1",
  "preadv",
  "pwritev",
  "rt_tgsigqueueinfo",
  "perf_event_open",
  "recvmmsg",
  "fanotify_init",
  "fanotify_mark",
  "prlimit64",
  "name_to_handle_at",
  "open_by_handle_at",
  "clock_adjtime",
  "syncfs",
  "sendmmsg",
  "setns",
  "process_vm_readv",
  "process_vm_writev",
  "kcmp",
  "finit_module",
  "sched_setattr",
  "sched_getattr",
  "renameat2",
  "seccomp",
  "getrandom",
  "memfd_create",
  "bpf",
  "execveat",
  "socket",
  "socketpair",
  "bind",
  "connect",
  "listen",
  "accept4",
  "getsockopt",
  "setsockopt",
  "getsockname",
  "getpeername",
  "sendto",
  "sendmsg",
  "recvfrom",
  "recvmsg",
  "shutdown",
  "userfaultfd",
  "membarrier",
  "mlock2",
  "copy_file_range",
  "preadv2",
  "pwritev2",
  "shmget",
  "shmat",
  "shmctl",
  "accept",
  "semget",
  "semop",
  "semctl",
  "shmdt",
  "msgget",
  "msgsnd",
  "msgrcv",
  "msgctl",
  "arch_prctl",
  "tuxcall",
  "security",
  "epoll_ctl_old",
  "epoll_wait_old",
  "semtimedop",
  "newfstatat",
  "kexec_file_load"
};


static inline enum syscall syscalls_32(int num) {
   switch (num) {
    case 0: return sc_restart_syscall;
    case 1: return sc_exit;
    case 2: return sc_fork;
    case 3: return sc_read;
    case 4: return sc_write;
    case 5: return sc_open;
    case 6: return sc_close;
    case 7: return sc_waitpid;
    case 8: return sc_creat;
    case 9: return sc_link;
    case 10: return sc_unlink;
    case 11: return sc_execve;
    case 12: return sc_chdir;
    case 13: return sc_time;
    case 14: return sc_mknod;
    case 15: return sc_chmod;
    case 16: return sc_lchown;
    case 17: return sc_break;
    case 18: return sc_oldstat;
    case 19: return sc_lseek;
    case 20: return sc_getpid;
    case 21: return sc_mount;
    case 22: return sc_umount;
    case 23: return sc_setuid;
    case 24: return sc_getuid;
    case 25: return sc_stime;
    case 26: return sc_ptrace;
    case 27: return sc_alarm;
    case 28: return sc_oldfstat;
    case 29: return sc_pause;
    case 30: return sc_utime;
    case 31: return sc_stty;
    case 32: return sc_gtty;
    case 33: return sc_access;
    case 34: return sc_nice;
    case 35: return sc_ftime;
    case 36: return sc_sync;
    case 37: return sc_kill;
    case 38: return sc_rename;
    case 39: return sc_mkdir;
    case 40: return sc_rmdir;
    case 41: return sc_dup;
    case 42: return sc_pipe;
    case 43: return sc_times;
    case 44: return sc_prof;
    case 45: return sc_brk;
    case 46: return sc_setgid;
    case 47: return sc_getgid;
    case 48: return sc_signal;
    case 49: return sc_geteuid;
    case 50: return sc_getegid;
    case 51: return sc_acct;
    case 52: return sc_umount2;
    case 53: return sc_lock;
    case 54: return sc_ioctl;
    case 55: return sc_fcntl;
    case 56: return sc_mpx;
    case 57: return sc_setpgid;
    case 58: return sc_ulimit;
    case 59: return sc_oldolduname;
    case 60: return sc_umask;
    case 61: return sc_chroot;
    case 62: return sc_ustat;
    case 63: return sc_dup2;
    case 64: return sc_getppid;
    case 65: return sc_getpgrp;
    case 66: return sc_setsid;
    case 67: return sc_sigaction;
    case 68: return sc_sgetmask;
    case 69: return sc_ssetmask;
    case 70: return sc_setreuid;
    case 71: return sc_setregid;
    case 72: return sc_sigsuspend;
    case 73: return sc_sigpending;
    case 74: return sc_sethostname;
    case 75: return sc_setrlimit;
    case 76: return sc_getrlimit;
    case 77: return sc_getrusage;
    case 78: return sc_gettimeofday;
    case 79: return sc_settimeofday;
    case 80: return sc_getgroups;
    case 81: return sc_setgroups;
    case 82: return sc_select;
    case 83: return sc_symlink;
    case 84: return sc_oldlstat;
    case 85: return sc_readlink;
    case 86: return sc_uselib;
    case 87: return sc_swapon;
    case 88: return sc_reboot;
    case 89: return sc_readdir;
    case 90: return sc_mmap;
    case 91: return sc_munmap;
    case 92: return sc_truncate;
    case 93: return sc_ftruncate;
    case 94: return sc_fchmod;
    case 95: return sc_fchown;
    case 96: return sc_getpriority;
    case 97: return sc_setpriority;
    case 98: return sc_profil;
    case 99: return sc_statfs;
    case 100: return sc_fstatfs;
    case 101: return sc_ioperm;
    case 102: return sc_socketcall;
    case 103: return sc_syslog;
    case 104: return sc_setitimer;
    case 105: return sc_getitimer;
    case 106: return sc_stat;
    case 107: return sc_lstat;
    case 108: return sc_fstat;
    case 109: return sc_olduname;
    case 110: return sc_iopl;
    case 111: return sc_vhangup;
    case 112: return sc_idle;
    case 113: return sc_vm86old;
    case 114: return sc_wait4;
    case 115: return sc_swapoff;
    case 116: return sc_sysinfo;
    case 117: return sc_ipc;
    case 118: return sc_fsync;
    case 119: return sc_sigreturn;
    case 120: return sc_clone;
    case 121: return sc_setdomainname;
    case 122: return sc_uname;
    case 123: return sc_modify_ldt;
    case 124: return sc_adjtimex;
    case 125: return sc_mprotect;
    case 126: return sc_sigprocmask;
    case 127: return sc_create_module;
    case 128: return sc_init_module;
    case 129: return sc_delete_module;
    case 130: return sc_get_kernel_syms;
    case 131: return sc_quotactl;
    case 132: return sc_getpgid;
    case 133: return sc_fchdir;
    case 134: return sc_bdflush;
    case 135: return sc_sysfs;
    case 136: return sc_personality;
    case 137: return sc_afs_syscall;
    case 138: return sc_setfsuid;
    case 139: return sc_setfsgid;
    case 140: return sc__llseek;
    case 141: return sc_getdents;
    case 142: return sc__newselect;
    case 143: return sc_flock;
    case 144: return sc_msync;
    case 145: return sc_readv;
    case 146: return sc_writev;
    case 147: return sc_getsid;
    case 148: return sc_fdatasync;
    case 149: return sc__sysctl;
    case 150: return sc_mlock;
    case 151: return sc_munlock;
    case 152: return sc_mlockall;
    case 153: return sc_munlockall;
    case 154: return sc_sched_setparam;
    case 155: return sc_sched_getparam;
    case 156: return sc_sched_setscheduler;
    case 157: return sc_sched_getscheduler;
    case 158: return sc_sched_yield;
    case 159: return sc_sched_get_priority_max;
    case 160: return sc_sched_get_priority_min;
    case 161: return sc_sched_rr_get_interval;
    case 162: return sc_nanosleep;
    case 163: return sc_mremap;
    case 164: return sc_setresuid;
    case 165: return sc_getresuid;
    case 166: return sc_vm86;
    case 167: return sc_query_module;
    case 168: return sc_poll;
    case 169: return sc_nfsservctl;
    case 170: return sc_setresgid;
    case 171: return sc_getresgid;
    case 172: return sc_prctl;
    case 173: return sc_rt_sigreturn;
    case 174: return sc_rt_sigaction;
    case 175: return sc_rt_sigprocmask;
    case 176: return sc_rt_sigpending;
    case 177: return sc_rt_sigtimedwait;
    case 178: return sc_rt_sigqueueinfo;
    case 179: return sc_rt_sigsuspend;
    case 180: return sc_pread64;
    case 181: return sc_pwrite64;
    case 182: return sc_chown;
    case 183: return sc_getcwd;
    case 184: return sc_capget;
    case 185: return sc_capset;
    case 186: return sc_sigaltstack;
    case 187: return sc_sendfile;
    case 188: return sc_getpmsg;
    case 189: return sc_putpmsg;
    case 190: return sc_vfork;
    case 191: return sc_ugetrlimit;
    case 192: return sc_mmap2;
    case 193: return sc_truncate64;
    case 194: return sc_ftruncate64;
    case 195: return sc_stat64;
    case 196: return sc_lstat64;
    case 197: return sc_fstat64;
    case 198: return sc_lchown32;
    case 199: return sc_getuid32;
    case 200: return sc_getgid32;
    case 201: return sc_geteuid32;
    case 202: return sc_getegid32;
    case 203: return sc_setreuid32;
    case 204: return sc_setregid32;
    case 205: return sc_getgroups32;
    case 206: return sc_setgroups32;
    case 207: return sc_fchown32;
    case 208: return sc_setresuid32;
    case 209: return sc_getresuid32;
    case 210: return sc_setresgid32;
    case 211: return sc_getresgid32;
    case 212: return sc_chown32;
    case 213: return sc_setuid32;
    case 214: return sc_setgid32;
    case 215: return sc_setfsuid32;
    case 216: return sc_setfsgid32;
    case 217: return sc_pivot_root;
    case 218: return sc_mincore;
    case 219: return sc_madvise;
    case 220: return sc_getdents64;
    case 221: return sc_fcntl64;
    case 224: return sc_gettid;
    case 225: return sc_readahead;
    case 226: return sc_setxattr;
    case 227: return sc_lsetxattr;
    case 228: return sc_fsetxattr;
    case 229: return sc_getxattr;
    case 230: return sc_lgetxattr;
    case 231: return sc_fgetxattr;
    case 232: return sc_listxattr;
    case 233: return sc_llistxattr;
    case 234: return sc_flistxattr;
    case 235: return sc_removexattr;
    case 236: return sc_lremovexattr;
    case 237: return sc_fremovexattr;
    case 238: return sc_tkill;
    case 239: return sc_sendfile64;
    case 240: return sc_futex;
    case 241: return sc_sched_setaffinity;
    case 242: return sc_sched_getaffinity;
    case 243: return sc_set_thread_area;
    case 244: return sc_get_thread_area;
    case 245: return sc_io_setup;
    case 246: return sc_io_destroy;
    case 247: return sc_io_getevents;
    case 248: return sc_io_submit;
    case 249: return sc_io_cancel;
    case 250: return sc_fadvise64;
    case 252: return sc_exit_group;
    case 253: return sc_lookup_dcookie;
    case 254: return sc_epoll_create;
    case 255: return sc_epoll_ctl;
    case 256: return sc_epoll_wait;
    case 257: return sc_remap_file_pages;
    case 258: return sc_set_tid_address;
    case 259: return sc_timer_create;
    case 260: return sc_timer_settime;
    case 261: return sc_timer_gettime;
    case 262: return sc_timer_getoverrun;
    case 263: return sc_timer_delete;
    case 264: return sc_clock_settime;
    case 265: return sc_clock_gettime;
    case 266: return sc_clock_getres;
    case 267: return sc_clock_nanosleep;
    case 268: return sc_statfs64;
    case 269: return sc_fstatfs64;
    case 270: return sc_tgkill;
    case 271: return sc_utimes;
    case 272: return sc_fadvise64_64;
    case 273: return sc_vserver;
    case 274: return sc_mbind;
    case 275: return sc_get_mempolicy;
    case 276: return sc_set_mempolicy;
    case 277: return sc_mq_open;
    case 278: return sc_mq_unlink;
    case 279: return sc_mq_timedsend;
    case 280: return sc_mq_timedreceive;
    case 281: return sc_mq_notify;
    case 282: return sc_mq_getsetattr;
    case 283: return sc_kexec_load;
    case 284: return sc_waitid;
    case 286: return sc_add_key;
    case 287: return sc_request_key;
    case 288: return sc_keyctl;
    case 289: return sc_ioprio_set;
    case 290: return sc_ioprio_get;
    case 291: return sc_inotify_init;
    case 292: return sc_inotify_add_watch;
    case 293: return sc_inotify_rm_watch;
    case 294: return sc_migrate_pages;
    case 295: return sc_openat;
    case 296: return sc_mkdirat;
    case 297: return sc_mknodat;
    case 298: return sc_fchownat;
    case 299: return sc_futimesat;
    case 300: return sc_fstatat64;
    case 301: return sc_unlinkat;
    case 302: return sc_renameat;
    case 303: return sc_linkat;
    case 304: return sc_symlinkat;
    case 305: return sc_readlinkat;
    case 306: return sc_fchmodat;
    case 307: return sc_faccessat;
    case 308: return sc_pselect6;
    case 309: return sc_ppoll;
    case 310: return sc_unshare;
    case 311: return sc_set_robust_list;
    case 312: return sc_get_robust_list;
    case 313: return sc_splice;
    case 314: return sc_sync_file_range;
    case 315: return sc_tee;
    case 316: return sc_vmsplice;
    case 317: return sc_move_pages;
    case 318: return sc_getcpu;
    case 319: return sc_epoll_pwait;
    case 320: return sc_utimensat;
    case 321: return sc_signalfd;
    case 322: return sc_timerfd_create;
    case 323: return sc_eventfd;
    case 324: return sc_fallocate;
    case 325: return sc_timerfd_settime;
    case 326: return sc_timerfd_gettime;
    case 327: return sc_signalfd4;
    case 328: return sc_eventfd2;
    case 329: return sc_epoll_create1;
    case 330: return sc_dup3;
    case 331: return sc_pipe2;
    case 332: return sc_inotify_init1;
    case 333: return sc_preadv;
    case 334: return sc_pwritev;
    case 335: return sc_rt_tgsigqueueinfo;
    case 336: return sc_perf_event_open;
    case 337: return sc_recvmmsg;
    case 338: return sc_fanotify_init;
    case 339: return sc_fanotify_mark;
    case 340: return sc_prlimit64;
    case 341: return sc_name_to_handle_at;
    case 342: return sc_open_by_handle_at;
    case 343: return sc_clock_adjtime;
    case 344: return sc_syncfs;
    case 345: return sc_sendmmsg;
    case 346: return sc_setns;
    case 347: return sc_process_vm_readv;
    case 348: return sc_process_vm_writev;
    case 349: return sc_kcmp;
    case 350: return sc_finit_module;
    case 351: return sc_sched_setattr;
    case 352: return sc_sched_getattr;
    case 353: return sc_renameat2;
    case 354: return sc_seccomp;
    case 355: return sc_getrandom;
    case 356: return sc_memfd_create;
    case 357: return sc_bpf;
    case 358: return sc_execveat;
    case 359: return sc_socket;
    case 360: return sc_socketpair;
    case 361: return sc_bind;
    case 362: return sc_connect;
    case 363: return sc_listen;
    case 364: return sc_accept4;
    case 365: return sc_getsockopt;
    case 366: return sc_setsockopt;
    case 367: return sc_getsockname;
    case 368: return sc_getpeername;
    case 369: return sc_sendto;
    case 370: return sc_sendmsg;
    case 371: return sc_recvfrom;
    case 372: return sc_recvmsg;
    case 373: return sc_shutdown;
    case 374: return sc_userfaultfd;
    case 375: return sc_membarrier;
    case 376: return sc_mlock2;
    case 377: return sc_copy_file_range;
    case 378: return sc_preadv2;
    case 379: return sc_pwritev2;
default: return sc_invalid_syscall;
    }
};


static inline enum syscall syscalls_64(int num) {
   switch (num) {
    case 0: return sc_read;
    case 1: return sc_write;
    case 2: return sc_open;
    case 3: return sc_close;
    case 4: return sc_stat;
    case 5: return sc_fstat;
    case 6: return sc_lstat;
    case 7: return sc_poll;
    case 8: return sc_lseek;
    case 9: return sc_mmap;
    case 10: return sc_mprotect;
    case 11: return sc_munmap;
    case 12: return sc_brk;
    case 13: return sc_rt_sigaction;
    case 14: return sc_rt_sigprocmask;
    case 15: return sc_rt_sigreturn;
    case 16: return sc_ioctl;
    case 17: return sc_pread64;
    case 18: return sc_pwrite64;
    case 19: return sc_readv;
    case 20: return sc_writev;
    case 21: return sc_access;
    case 22: return sc_pipe;
    case 23: return sc_select;
    case 24: return sc_sched_yield;
    case 25: return sc_mremap;
    case 26: return sc_msync;
    case 27: return sc_mincore;
    case 28: return sc_madvise;
    case 29: return sc_shmget;
    case 30: return sc_shmat;
    case 31: return sc_shmctl;
    case 32: return sc_dup;
    case 33: return sc_dup2;
    case 34: return sc_pause;
    case 35: return sc_nanosleep;
    case 36: return sc_getitimer;
    case 37: return sc_alarm;
    case 38: return sc_setitimer;
    case 39: return sc_getpid;
    case 40: return sc_sendfile;
    case 41: return sc_socket;
    case 42: return sc_connect;
    case 43: return sc_accept;
    case 44: return sc_sendto;
    case 45: return sc_recvfrom;
    case 46: return sc_sendmsg;
    case 47: return sc_recvmsg;
    case 48: return sc_shutdown;
    case 49: return sc_bind;
    case 50: return sc_listen;
    case 51: return sc_getsockname;
    case 52: return sc_getpeername;
    case 53: return sc_socketpair;
    case 54: return sc_setsockopt;
    case 55: return sc_getsockopt;
    case 56: return sc_clone;
    case 57: return sc_fork;
    case 58: return sc_vfork;
    case 59: return sc_execve;
    case 60: return sc_exit;
    case 61: return sc_wait4;
    case 62: return sc_kill;
    case 63: return sc_uname;
    case 64: return sc_semget;
    case 65: return sc_semop;
    case 66: return sc_semctl;
    case 67: return sc_shmdt;
    case 68: return sc_msgget;
    case 69: return sc_msgsnd;
    case 70: return sc_msgrcv;
    case 71: return sc_msgctl;
    case 72: return sc_fcntl;
    case 73: return sc_flock;
    case 74: return sc_fsync;
    case 75: return sc_fdatasync;
    case 76: return sc_truncate;
    case 77: return sc_ftruncate;
    case 78: return sc_getdents;
    case 79: return sc_getcwd;
    case 80: return sc_chdir;
    case 81: return sc_fchdir;
    case 82: return sc_rename;
    case 83: return sc_mkdir;
    case 84: return sc_rmdir;
    case 85: return sc_creat;
    case 86: return sc_link;
    case 87: return sc_unlink;
    case 88: return sc_symlink;
    case 89: return sc_readlink;
    case 90: return sc_chmod;
    case 91: return sc_fchmod;
    case 92: return sc_chown;
    case 93: return sc_fchown;
    case 94: return sc_lchown;
    case 95: return sc_umask;
    case 96: return sc_gettimeofday;
    case 97: return sc_getrlimit;
    case 98: return sc_getrusage;
    case 99: return sc_sysinfo;
    case 100: return sc_times;
    case 101: return sc_ptrace;
    case 102: return sc_getuid;
    case 103: return sc_syslog;
    case 104: return sc_getgid;
    case 105: return sc_setuid;
    case 106: return sc_setgid;
    case 107: return sc_geteuid;
    case 108: return sc_getegid;
    case 109: return sc_setpgid;
    case 110: return sc_getppid;
    case 111: return sc_getpgrp;
    case 112: return sc_setsid;
    case 113: return sc_setreuid;
    case 114: return sc_setregid;
    case 115: return sc_getgroups;
    case 116: return sc_setgroups;
    case 117: return sc_setresuid;
    case 118: return sc_getresuid;
    case 119: return sc_setresgid;
    case 120: return sc_getresgid;
    case 121: return sc_getpgid;
    case 122: return sc_setfsuid;
    case 123: return sc_setfsgid;
    case 124: return sc_getsid;
    case 125: return sc_capget;
    case 126: return sc_capset;
    case 127: return sc_rt_sigpending;
    case 128: return sc_rt_sigtimedwait;
    case 129: return sc_rt_sigqueueinfo;
    case 130: return sc_rt_sigsuspend;
    case 131: return sc_sigaltstack;
    case 132: return sc_utime;
    case 133: return sc_mknod;
    case 134: return sc_uselib;
    case 135: return sc_personality;
    case 136: return sc_ustat;
    case 137: return sc_statfs;
    case 138: return sc_fstatfs;
    case 139: return sc_sysfs;
    case 140: return sc_getpriority;
    case 141: return sc_setpriority;
    case 142: return sc_sched_setparam;
    case 143: return sc_sched_getparam;
    case 144: return sc_sched_setscheduler;
    case 145: return sc_sched_getscheduler;
    case 146: return sc_sched_get_priority_max;
    case 147: return sc_sched_get_priority_min;
    case 148: return sc_sched_rr_get_interval;
    case 149: return sc_mlock;
    case 150: return sc_munlock;
    case 151: return sc_mlockall;
    case 152: return sc_munlockall;
    case 153: return sc_vhangup;
    case 154: return sc_modify_ldt;
    case 155: return sc_pivot_root;
    case 156: return sc__sysctl;
    case 157: return sc_prctl;
    case 158: return sc_arch_prctl;
    case 159: return sc_adjtimex;
    case 160: return sc_setrlimit;
    case 161: return sc_chroot;
    case 162: return sc_sync;
    case 163: return sc_acct;
    case 164: return sc_settimeofday;
    case 165: return sc_mount;
    case 166: return sc_umount2;
    case 167: return sc_swapon;
    case 168: return sc_swapoff;
    case 169: return sc_reboot;
    case 170: return sc_sethostname;
    case 171: return sc_setdomainname;
    case 172: return sc_iopl;
    case 173: return sc_ioperm;
    case 174: return sc_create_module;
    case 175: return sc_init_module;
    case 176: return sc_delete_module;
    case 177: return sc_get_kernel_syms;
    case 178: return sc_query_module;
    case 179: return sc_quotactl;
    case 180: return sc_nfsservctl;
    case 181: return sc_getpmsg;
    case 182: return sc_putpmsg;
    case 183: return sc_afs_syscall;
    case 184: return sc_tuxcall;
    case 185: return sc_security;
    case 186: return sc_gettid;
    case 187: return sc_readahead;
    case 188: return sc_setxattr;
    case 189: return sc_lsetxattr;
    case 190: return sc_fsetxattr;
    case 191: return sc_getxattr;
    case 192: return sc_lgetxattr;
    case 193: return sc_fgetxattr;
    case 194: return sc_listxattr;
    case 195: return sc_llistxattr;
    case 196: return sc_flistxattr;
    case 197: return sc_removexattr;
    case 198: return sc_lremovexattr;
    case 199: return sc_fremovexattr;
    case 200: return sc_tkill;
    case 201: return sc_time;
    case 202: return sc_futex;
    case 203: return sc_sched_setaffinity;
    case 204: return sc_sched_getaffinity;
    case 205: return sc_set_thread_area;
    case 206: return sc_io_setup;
    case 207: return sc_io_destroy;
    case 208: return sc_io_getevents;
    case 209: return sc_io_submit;
    case 210: return sc_io_cancel;
    case 211: return sc_get_thread_area;
    case 212: return sc_lookup_dcookie;
    case 213: return sc_epoll_create;
    case 214: return sc_epoll_ctl_old;
    case 215: return sc_epoll_wait_old;
    case 216: return sc_remap_file_pages;
    case 217: return sc_getdents64;
    case 218: return sc_set_tid_address;
    case 219: return sc_restart_syscall;
    case 220: return sc_semtimedop;
    case 221: return sc_fadvise64;
    case 222: return sc_timer_create;
    case 223: return sc_timer_settime;
    case 224: return sc_timer_gettime;
    case 225: return sc_timer_getoverrun;
    case 226: return sc_timer_delete;
    case 227: return sc_clock_settime;
    case 228: return sc_clock_gettime;
    case 229: return sc_clock_getres;
    case 230: return sc_clock_nanosleep;
    case 231: return sc_exit_group;
    case 232: return sc_epoll_wait;
    case 233: return sc_epoll_ctl;
    case 234: return sc_tgkill;
    case 235: return sc_utimes;
    case 236: return sc_vserver;
    case 237: return sc_mbind;
    case 238: return sc_set_mempolicy;
    case 239: return sc_get_mempolicy;
    case 240: return sc_mq_open;
    case 241: return sc_mq_unlink;
    case 242: return sc_mq_timedsend;
    case 243: return sc_mq_timedreceive;
    case 244: return sc_mq_notify;
    case 245: return sc_mq_getsetattr;
    case 246: return sc_kexec_load;
    case 247: return sc_waitid;
    case 248: return sc_add_key;
    case 249: return sc_request_key;
    case 250: return sc_keyctl;
    case 251: return sc_ioprio_set;
    case 252: return sc_ioprio_get;
    case 253: return sc_inotify_init;
    case 254: return sc_inotify_add_watch;
    case 255: return sc_inotify_rm_watch;
    case 256: return sc_migrate_pages;
    case 257: return sc_openat;
    case 258: return sc_mkdirat;
    case 259: return sc_mknodat;
    case 260: return sc_fchownat;
    case 261: return sc_futimesat;
    case 262: return sc_newfstatat;
    case 263: return sc_unlinkat;
    case 264: return sc_renameat;
    case 265: return sc_linkat;
    case 266: return sc_symlinkat;
    case 267: return sc_readlinkat;
    case 268: return sc_fchmodat;
    case 269: return sc_faccessat;
    case 270: return sc_pselect6;
    case 271: return sc_ppoll;
    case 272: return sc_unshare;
    case 273: return sc_set_robust_list;
    case 274: return sc_get_robust_list;
    case 275: return sc_splice;
    case 276: return sc_tee;
    case 277: return sc_sync_file_range;
    case 278: return sc_vmsplice;
    case 279: return sc_move_pages;
    case 280: return sc_utimensat;
    case 281: return sc_epoll_pwait;
    case 282: return sc_signalfd;
    case 283: return sc_timerfd_create;
    case 284: return sc_eventfd;
    case 285: return sc_fallocate;
    case 286: return sc_timerfd_settime;
    case 287: return sc_timerfd_gettime;
    case 288: return sc_accept4;
    case 289: return sc_signalfd4;
    case 290: return sc_eventfd2;
    case 291: return sc_epoll_create1;
    case 292: return sc_dup3;
    case 293: return sc_pipe2;
    case 294: return sc_inotify_init1;
    case 295: return sc_preadv;
    case 296: return sc_pwritev;
    case 297: return sc_rt_tgsigqueueinfo;
    case 298: return sc_perf_event_open;
    case 299: return sc_recvmmsg;
    case 300: return sc_fanotify_init;
    case 301: return sc_fanotify_mark;
    case 302: return sc_prlimit64;
    case 303: return sc_name_to_handle_at;
    case 304: return sc_open_by_handle_at;
    case 305: return sc_clock_adjtime;
    case 306: return sc_syncfs;
    case 307: return sc_sendmmsg;
    case 308: return sc_setns;
    case 309: return sc_getcpu;
    case 310: return sc_process_vm_readv;
    case 311: return sc_process_vm_writev;
    case 312: return sc_kcmp;
    case 313: return sc_finit_module;
    case 314: return sc_sched_setattr;
    case 315: return sc_sched_getattr;
    case 316: return sc_renameat2;
    case 317: return sc_seccomp;
    case 318: return sc_getrandom;
    case 319: return sc_memfd_create;
    case 320: return sc_kexec_file_load;
    case 321: return sc_bpf;
    case 322: return sc_execveat;
    case 323: return sc_userfaultfd;
    case 324: return sc_membarrier;
    case 325: return sc_mlock2;
    case 326: return sc_copy_file_range;
    case 327: return sc_preadv2;
    case 328: return sc_pwritev2;
    case 512: return sc_rt_sigaction;
    case 513: return sc_rt_sigreturn;
    case 514: return sc_ioctl;
    case 515: return sc_readv;
    case 516: return sc_writev;
    case 517: return sc_recvfrom;
    case 518: return sc_sendmsg;
    case 519: return sc_recvmsg;
    case 520: return sc_execve;
    case 521: return sc_ptrace;
    case 522: return sc_rt_sigpending;
    case 523: return sc_rt_sigtimedwait;
    case 524: return sc_rt_sigqueueinfo;
    case 525: return sc_sigaltstack;
    case 526: return sc_timer_create;
    case 527: return sc_mq_notify;
    case 528: return sc_kexec_load;
    case 529: return sc_waitid;
    case 530: return sc_set_robust_list;
    case 531: return sc_get_robust_list;
    case 532: return sc_vmsplice;
    case 533: return sc_move_pages;
    case 534: return sc_preadv;
    case 535: return sc_pwritev;
    case 536: return sc_rt_tgsigqueueinfo;
    case 537: return sc_recvmmsg;
    case 538: return sc_sendmmsg;
    case 539: return sc_process_vm_readv;
    case 540: return sc_process_vm_writev;
    case 541: return sc_setsockopt;
    case 542: return sc_getsockopt;
    case 543: return sc_io_setup;
    case 544: return sc_io_submit;
    case 545: return sc_execveat;
default: return sc_invalid_syscall;
    }
};

