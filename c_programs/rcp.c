# include <stdio.h>
# define num_state_vars 3
struct packet {
  int x;
  int y;
  int z;
};
int state [num_state_vars] = {0};

void write_results(struct packet pkt)
{
    FILE *fp;

    fp = fopen("results.txt", "w+");
    fprintf(fp, "%d, %d, %d\n", pkt.x, pkt.y, pkt.z);
    for (int i = 0; i< num_state_vars; i++){
       if (i == num_state_vars - 1 )
         fprintf(fp, "%d\n", state[i]);
       else
         fprintf(fp, "%d,", state[i]);
    }
    fclose(fp);
}
void rcp (struct packet pkt)
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


