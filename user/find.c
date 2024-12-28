#include "kernel/types.h"

#include "kernel/fcntl.h"
#include "kernel/fs.h"
#include "kernel/stat.h"
#include "user/user.h"

#define BUF_SIZE 512

// int
// find_impl2(char* base, char* name)
// {
// }

int
find_impl(char* base, char* name)
{
  int fd = open(base, O_RDONLY);
  if (fd < 0) {
    fprintf(2, "find: cannot open %s\n", base);
    return -1;
  }

  struct stat st;
  if (fstat(fd, &st) < 0) {
    fprintf(2, "find: cannot stat %s\n", base);
    close(fd);
    return -1;
  }

  // just a file or device
  if (st.type != T_DIR) {
    if (strcmp(base, name) == 0) {
      fprintf(1, "%s\n", base);
    }
    close(fd);
    return 0;
  }

  char* cur = base + strlen(base);
  *cur++ = '/';

  // check if path is too long
  if (cur - base + DIRSIZ + 1 > BUF_SIZE) {
    fprintf(2, "find: sub path too long\n");
    close(fd);
    return -1;
  }

  // directory
  struct dirent de;
  int err = 0;
  while ((err = read(fd, &de, sizeof(de))) == sizeof(de)) {
    if (de.inum == 0) {
      continue;
    }

    if (strcmp(de.name, ".") == 0 || strcmp(de.name, "..") == 0) {
      continue;
    }

    // mk new path
    memmove(cur, de.name, DIRSIZ);
    cur[DIRSIZ] = 0;

    // check if it's the target
    if (strcmp(de.name, name) == 0) {
      fprintf(1, "%s\n", base);
    }

    if (stat(base, &st) < 0) {
      fprintf(2, "find: cannot stat %s\n", base);
      continue;
    }

    // if it's a directory, recurse
    if (st.type == T_DIR) {
      if (find_impl(base, name) < 0) {
        fprintf(2, "find: error in %s\n", base);
      }
    }
  }

  close(fd);
  return err;
}

int
find(char* path, char* name)
{
  char path_buf[BUF_SIZE + 1] = { 0 };
  if (strlen(path) >= BUF_SIZE) {
    fprintf(2, "find: base path too long\n");
    return -1;
  }

  strcpy(path_buf, path);

  return find_impl(path_buf, name);
}

int
main(int argc, char** argv)
{
  if (argc != 3) {
    fprintf(2, "Usage: find <path> <name>\n");
    exit(1);
  }

  exit(find(argv[1], argv[2]));
}