#include "kernel/types.h"
#include "user/user.h"

int
main(int argc, char** argv)
{
  if (argc < 2 || argc > 3) {
    fprintf(2, "Usage: sleep time\n");
    exit(1);
  }

  int time = atoi(argv[1]);
  int err = sleep(time);
  exit(err);
}