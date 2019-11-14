#include <stdio.h>
# define NUM_STATE_VARS 3
struct Packet {
  int x;
  int y;
  int z;
};
int state [NUM_STATE_VARS] = {0};

void write_results(struct Packet pkt)
{
    FILE *fp;

    fp = fopen("results.txt", "w+");
    fprintf(fp, "%d, %d, %d\n", pkt.x, pkt.y, pkt.z);
    for (int i = 0; i< NUM_STATE_VARS; i++){
       if (i == NUM_STATE_VARS - 1 )
         fprintf(fp, "%d\n", state[i]);
       else
         fprintf(fp, "%d,", state[i]);
    }
    fclose(fp);
}
void rcp (struct Packet pkt)
{
  if (pkt.y < 2) {
   state[1] = state[1] + pkt.y;
   state[2] = state[2] + 1;
  }
  else {
    state[0] = pkt.x + state[0];
  }
  write_results (pkt);
}


