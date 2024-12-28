#include "kernel/types.h"
#include "user/user.h"
#include <stddef.h>

struct numpipe
{
  int rfd, wfd;
};

#define NUMPIPE_NEW(np) pipe(&(np)->rfd)
#define NUMPIPE_CLOSE(np) close((np)->rfd), close((np)->wfd)
#define NUMPIPE_CLOSE_READER(np) close((np)->rfd)
#define NUMPIPE_CLOSE_WRITER(np) close((np)->wfd)
#define NUMPIPE_AS_READER(np) close((np)->wfd)
#define NUMPIPE_AS_WRITER(np) close((np)->rfd)
#define NUMPIPE_READ(np, buf) read((np)->rfd, &(buf), sizeof(buf))
#define NUMPIPE_WRITE(np, buf) write((np)->wfd, &(buf), sizeof(buf))

enum
{
  GENERATOR_LB = 2,
  GENERATOR_UB = 280
};

int
seq_generator(struct numpipe* out)
{
  NUMPIPE_AS_WRITER(out);

  for (int i = GENERATOR_LB; i <= GENERATOR_UB; i++) {
    if (NUMPIPE_WRITE(out, i) < 0) {
      fprintf(2, "generator: write failed\n");
      return -1;
    }
  }

  NUMPIPE_CLOSE_WRITER(out);
  wait(NULL);
  return 0;
}

int
seq_filter(struct numpipe* in)
{
  NUMPIPE_AS_READER(in);
  int base = 0;
  int num = 0;
  int err = 0;

  err = NUMPIPE_READ(in, base);
  if (err < 0) {
    NUMPIPE_CLOSE_READER(in);
    fprintf(2, "filter: read failed\n");
    return -1;
  }
  if (err == 0) {
    NUMPIPE_CLOSE_READER(in);
    return 0;
  }

  fprintf(1, "prime %d\n", base);

  struct numpipe np;
  NUMPIPE_NEW(&np);

  int pid = fork();

  if (pid == -1) {
    NUMPIPE_CLOSE(&np);
    fprintf(2, "fork failed\n");
    return -1;
  }

  if (pid == 0) {
    NUMPIPE_CLOSE_READER(in);
    return seq_filter(&np);
  }

  NUMPIPE_AS_WRITER(&np);
  while ((err = NUMPIPE_READ(in, num)) > 0) {
    if (num % base != 0) {
      if (NUMPIPE_WRITE(&np, num) < 0) {
        fprintf(2, "filter: write failed\n");
        return -1;
      }
    }
  }
  NUMPIPE_CLOSE_READER(in);
  NUMPIPE_CLOSE_WRITER(&np);
  wait(NULL);
  return err;
}

int
main()
{
  struct numpipe np;
  NUMPIPE_NEW(&np);

  int pid = fork();

  if (pid == -1) {
    fprintf(2, "fork failed\n");
    exit(1);
  }

  if (pid == 0) {
    exit(seq_filter(&np));
  }

  int err = seq_generator(&np);
  exit(err);
}