# include <stdio.h>
# define num_state_vars 2
struct packet {
  int x;
  int y;
};
int state [num_state_vars] = {0};

void write_results(struct packet pkt)
{
    FILE *fp;

    fp = fopen("results.txt", "w+");
    fprintf(fp, "%d, %d\n", pkt.x, pkt.y);
    for (int i = 0; i< num_state_vars; i++){
       if (i == num_state_vars - 1 )
         fprintf(fp, "%d\n", state[i]);
       else
         fprintf(fp, "%d,", state[i]);
    }
    fclose(fp);
}
void blue_increase (struct packet pkt)
{
  pkt.y = pkt.x - 10;
  if (pkt.y > state[1]) {
    state[0] = state[0] + 1;
    state[1] = pkt.x;
  }
  write_results (pkt);
}


