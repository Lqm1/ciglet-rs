#define _POSIX_C_SOURCE 200809L
#include <sys/wait.h>
#include <time.h>
#include "ciglet.h"

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  figure* fg = plotopenv(argv[1]);
  if (!fg || !fg->handle) return 3;
  FP_TYPE y[] = {1, 2};
  plot(fg, NULL, y, 2, 'g');
  plotclose(fg);
  struct timespec delay = {0, 10000000};
  for (int attempt = 0; attempt < 1000; ++attempt) {
    int status;
    pid_t child = waitpid(-1, &status, WNOHANG);
    if (child > 0) {
      if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) return 4;
      printf("Original plotclose left an unreaped exited child: %ld\n", (long)child);
      return 0;
    }
    if (child < 0) return 6;
    nanosleep(&delay, NULL);
  }
  return 7;
}
