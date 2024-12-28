#include "kernel/types.h"
#include "user/user.h"
#include <stddef.h>

#define FINAL_ANSWER 42

int
main()
{
  int fds[2] = { 0 };
  pipe(fds);

  int pid = fork();
  char buffer = FINAL_ANSWER;

  switch (pid) {
    case -1:
      fprintf(2, "pingpong fork failed\n");
      exit(1);
    case 0:
      read(fds[0], &buffer, 1);
      fprintf(1, "%d: received ping\n", getpid());
      write(fds[1], &buffer, 1);
      break;
    default:
      write(fds[1], &buffer, 1);
      wait(NULL);
      read(fds[0], &buffer, 1);
      fprintf(1, "%d: received pong\n", getpid());
  }

  close(fds[0]);
  close(fds[1]);

  exit(0);
}