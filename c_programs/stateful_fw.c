# include <stdio.h>
# define num_state_vars 1
struct packet {
  int pkt0;
  int pkt1;
  int pkt2;
  int pkt3;
};
int state [num_state_vars] = {0};

void write_results(struct packet pkt)
{
    FILE *fp;

    fp = fopen("results.txt", "w+");
    fprintf(fp, "%d, %d, %d, %d\n", pkt.pkt0, pkt.pkt1, pkt.pkt2, pkt.pkt3);
    for (int i = 0; i< num_state_vars; i++){
       if (i == num_state_vars - 1 )
         fprintf(fp, "%d\n", state[i]);
       else
         fprintf(fp, "%d,", state[i]);
    }
    fclose(fp);
}
void stateful_fw (struct packet pkt)
{
  pkt.pkt2 = pkt.pkt1 + pkt.pkt0;
  if (pkt.pkt1 != 102 && pkt.pkt0 == 102) {
    if (state[0] == 0) 
      pkt.pkt3 = 1;
    else 
      pkt.pkt3 = 0;
  }
  else {
    if (pkt.pkt1 == 102)
      state[0] = 1;
  }
  write_results (pkt);
}


