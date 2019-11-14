# include <stdio.h>
# define num_state_vars 1
struct packet {
  int x;
};
int state [num_state_vars] = {0};

void write_results(struct packet pkt)
{
    FILE *fp;

    fp = fopen("results.txt", "w+");
    fprintf(fp, "%d\n", pkt.x);
    for (int i = 0; i< num_state_vars; i++){
       if (i == num_state_vars - 1 )
         fprintf(fp, "%d\n", state[i]);
       else
         fprintf(fp, "%d,", state[i]);
    }
    fclose(fp);
}
void marple_new_flow (struct packet pkt)
{
  if (state[0] == 0) {
    pkt.x = 1;
    state[0] = 1;
  }
  write_results (pkt);
}


