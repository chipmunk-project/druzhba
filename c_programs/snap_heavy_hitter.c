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
void rcp (struct packet pkt)
{
  if (state[1] == 0){
    state[0]+=1;
    if (state[0] == 1000) {
      state[1] = 1;
    }
  }
  write_results (pkt);
}

