# include <stdio.h>
# define num_state_vars 2
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
void flowlet (struct packet pkt)
{
  if (pkt.y - state[1] > 5) {
   state[0] = pkt.x;
  }
  state[1] = pkt.y;
  pkt.z = state[0];
  write_results (pkt);
}


