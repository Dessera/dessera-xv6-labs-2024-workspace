// xv6 syscall wrapper

#[repr(C)]
pub struct Stat {
    pub dev: i32,
    pub ino: u32,
    pub r#type: i16,
    pub nlink: i16,
    pub size: u64,
}

extern "C" {
    pub fn fork() -> i32;
    pub fn exit(status: i32) -> !;
    pub fn wait(status: *mut i32) -> i32;
    pub fn pipe(fds: *mut i32) -> i32;
    pub fn write(fd: i32, buf: *const u8, count: i32) -> i32;
    pub fn read(fd: i32, buf: *mut u8, count: i32) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn kill(pid: i32) -> i32;
    pub fn exec(path: *const u8, argv: *const *const u8) -> i32;
    pub fn open(path: *const u8, flags: i32) -> i32;
    pub fn mknod(path: *const u8, major: i32, minor: i32) -> i32;
    pub fn unlink(path: *const u8) -> i32;
    pub fn fstat(fd: i32, st: *mut Stat) -> i32;
    pub fn link(src: *const u8, dst: *const u8) -> i32;
    pub fn mkdir(path: *const u8) -> i32;
    pub fn chdir(path: *const u8) -> i32;
    pub fn dup(fd: i32) -> i32;
    pub fn getpid() -> i32;
    pub fn sbrk(n: i32) -> *mut u8;
    pub fn sleep(n: i32) -> i32;
    pub fn uptime() -> u32;

    pub fn malloc(size: u32) -> *mut u8;
    pub fn free(ptr: *mut u8);
}
