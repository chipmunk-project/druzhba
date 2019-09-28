#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <iostream>
#include "vops.h"
#include "stateful_fw_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_verify_iter_9.h"

using namespace std;

void main__Wrapper_ANONYMOUSTest(Parameters& _p_) {
  for(int _test_=0;_test_<3000 ;_test_++) {
    int  pkt_0;
    pkt_0=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"pkt_0="<<pkt_0<<endl;
    }
    int  pkt_1;
    pkt_1=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"pkt_1="<<pkt_1<<endl;
    }
    int  pkt_2;
    pkt_2=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"pkt_2="<<pkt_2<<endl;
    }
    int  pkt_3;
    pkt_3=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"pkt_3="<<pkt_3<<endl;
    }
    int  state_group_0_state_0;
    state_group_0_state_0=abs(rand()) % 100;
    if(_p_.verbosity > 2){
      cout<<"state_group_0_state_0="<<state_group_0_state_0<<endl;
    }
    try{
      ANONYMOUS::main__WrapperNospec(pkt_0,pkt_1,pkt_2,pkt_3,state_group_0_state_0);
      ANONYMOUS::main__Wrapper(pkt_0,pkt_1,pkt_2,pkt_3,state_group_0_state_0);
    }catch(AssumptionFailedException& afe){  }
  }
}

int main(int argc, char** argv) {
  Parameters p(argc, argv);
  srand(time(0));
  main__Wrapper_ANONYMOUSTest(p);
  printf("Automated testing passed for stateful_fw_equivalent_5_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_verify_iter_9\n");
  return 0;
}
