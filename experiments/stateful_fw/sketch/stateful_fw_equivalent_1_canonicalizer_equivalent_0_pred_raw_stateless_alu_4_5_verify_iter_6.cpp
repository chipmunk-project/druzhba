#include <cstdio>
#include <assert.h>
#include <iostream>
using namespace std;
#include "vops.h"
#include "stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_verify_iter_6.h"
namespace ANONYMOUS{

StateGroup* StateGroup::create(int  state_0_){
  void* temp= malloc( sizeof(StateGroup)  ); 
  StateGroup* rv = new (temp)StateGroup();
  rv->state_0 =  state_0_;
  return rv;
}
StateAndPacket* StateAndPacket::create(int  pkt_0_, int  pkt_1_, int  pkt_2_, int  pkt_3_, int  state_group_0_state_0_){
  void* temp= malloc( sizeof(StateAndPacket)  ); 
  StateAndPacket* rv = new (temp)StateAndPacket();
  rv->pkt_0 =  pkt_0_;
  rv->pkt_1 =  pkt_1_;
  rv->pkt_2 =  pkt_2_;
  rv->pkt_3 =  pkt_3_;
  rv->state_group_0_state_0 =  state_group_0_state_0_;
  return rv;
}
void main__Wrapper(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int state_group_0_state_0) {
  bool _tt0[9] = {0, 0, 0, 0, 0, 0, 0, 0, 0};
  int*  constant_vector__ANONYMOUS_s316= new int [9]; CopyArr<int >(constant_vector__ANONYMOUS_s316,_tt0, 9, 9);
  glblInit_constant_vector__ANONYMOUS_s320(constant_vector__ANONYMOUS_s316);
  _main(pkt_0, pkt_1, pkt_2, pkt_3, state_group_0_state_0, constant_vector__ANONYMOUS_s316);
  delete[] constant_vector__ANONYMOUS_s316;
}
void main__WrapperNospec(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int state_group_0_state_0) {}
void glblInit_constant_vector__ANONYMOUS_s320(int* constant_vector__ANONYMOUS_s319/* len = 9 */) {
  int _tt1[9] = {0, 1, 2, 3, 102, 102, 2, 101, 1};
  CopyArr<int >(constant_vector__ANONYMOUS_s319,_tt1, 9, 9);
}
void _main(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int state_group_0_state_0, int* constant_vector__ANONYMOUS_s312/* len = 9 */) {
  
  pkt_0 = 96;
  pkt_1 = 43;
  pkt_2 = 99;
  pkt_3 = 46;
  state_group_0_state_0 = 0;
  std::cout<<"pkt0 = " <<pkt_0 <<" pkt1 = "<<pkt_1<<" pkt2 = "<<pkt_2<<" pkt_3 = "<<pkt_3<<" state00 = "<<state_group_0_state_0<<"\n";
  int  pipeline_result_s1_pkt_3_s334=0;
  int  pipeline_result_s1_state_group_0_state_0_s335=0;
  int  pipeline_result_s1_pkt_0_s331=0;
  int  pipeline_result_s1_pkt_1_s332=0;
  int  pipeline_result_s1_pkt_2_s333=0;
  pipeline(pkt_0, pkt_1, pkt_2, pkt_3, state_group_0_state_0, pipeline_result_s1_pkt_0_s331, pipeline_result_s1_pkt_1_s332, pipeline_result_s1_pkt_2_s333, pipeline_result_s1_pkt_3_s334, pipeline_result_s1_state_group_0_state_0_s335, constant_vector__ANONYMOUS_s312);
  std::cout<<"Result pkkt0 = "<<pipeline_result_s1_pkt_0_s331<<" pkt1 = "<<pipeline_result_s1_pkt_1_s332<< " pkt2 = " <<pipeline_result_s1_pkt_2_s333<<" pkt3 = "<<pipeline_result_s1_pkt_3_s334<<"state00 = "<< pipeline_result_s1_state_group_0_state_0_s335<<"\n\n\n";
  int  program_result_s3_pkt_3_s344=0;
  int  program_result_s3_state_group_0_state_0_s345=0;
  int  program_result_s3_pkt_0_s341=0;
  int  program_result_s3_pkt_1_s342=0;
  int  program_result_s3_pkt_2_s343=0;
  program(pkt_0, pkt_1, pkt_2, pkt_3, state_group_0_state_0, program_result_s3_pkt_0_s341, program_result_s3_pkt_1_s342, program_result_s3_pkt_2_s343, program_result_s3_pkt_3_s344, program_result_s3_state_group_0_state_0_s345);
  assert ((pipeline_result_s1_state_group_0_state_0_s335) == (program_result_s3_state_group_0_state_0_s345));;
  assert ((pipeline_result_s1_pkt_0_s331) == (program_result_s3_pkt_0_s341));;
  assert ((pipeline_result_s1_pkt_1_s332) == (program_result_s3_pkt_1_s342));;
  assert ((pipeline_result_s1_pkt_2_s333) == (program_result_s3_pkt_2_s343));;
  assert ((pipeline_result_s1_pkt_3_s334) == (program_result_s3_pkt_3_s344));;
}
void pipeline(int state_and_packet_pkt_0_s356, int state_and_packet_pkt_1_s357, int state_and_packet_pkt_2_s358, int state_and_packet_pkt_3_s359, int state_and_packet_state_group_0_state_0_s360, int& _out_pkt_0_s361, int& _out_pkt_1_s362, int& _out_pkt_2_s363, int& _out_pkt_3_s364, int& _out_state_group_0_state_0_s365, int* constant_vector__ANONYMOUS_s313/* len = 9 */) {
  int  destination_0_0_s5=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0(state_and_packet_pkt_0_s356, state_and_packet_pkt_1_s357, state_and_packet_pkt_2_s358, state_and_packet_pkt_3_s359, 0, 17, 0, 3, 4, 4, destination_0_0_s5, constant_vector__ANONYMOUS_s313);
  int  destination_0_1_s7=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1(state_and_packet_pkt_0_s356, state_and_packet_pkt_1_s357, state_and_packet_pkt_2_s358, state_and_packet_pkt_3_s359, 0, 14, 4, 3, 2, 1, destination_0_1_s7, constant_vector__ANONYMOUS_s313);
  int  destination_0_2_s9=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2(state_and_packet_pkt_0_s356, state_and_packet_pkt_1_s357, state_and_packet_pkt_2_s358, state_and_packet_pkt_3_s359, 0, 4, 7, 1, 2, 1, destination_0_2_s9, constant_vector__ANONYMOUS_s313);
  int  destination_0_3_s11=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3(state_and_packet_pkt_0_s356, state_and_packet_pkt_1_s357, state_and_packet_pkt_2_s358, state_and_packet_pkt_3_s359, 0, 1, 2, 3, 0, 0, destination_0_3_s11, constant_vector__ANONYMOUS_s313);
  int  destination_0_4_s13=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4(state_and_packet_pkt_0_s356, state_and_packet_pkt_1_s357, state_and_packet_pkt_2_s358, state_and_packet_pkt_3_s359, 0, 5, 0, 0, 0, 6, destination_0_4_s13, constant_vector__ANONYMOUS_s313);
  int  packet_operand_salu0_0_0_s15=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_0(state_and_packet_pkt_0_s356, state_and_packet_pkt_1_s357, state_and_packet_pkt_2_s358, state_and_packet_pkt_3_s359, 0, 3, packet_operand_salu0_0_0_s15);
  int  packet_operand_salu0_0_1_s17=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_1(state_and_packet_pkt_0_s356, state_and_packet_pkt_1_s357, state_and_packet_pkt_2_s358, state_and_packet_pkt_3_s359, 0, 0, packet_operand_salu0_0_1_s17);
  int  old_state_group_0_0_s19_state_0_s371=0;
  int  state_operand_salu_0_0_state_0_s366=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0(state_operand_salu_0_0_state_0_s366, 0, packet_operand_salu0_0_0_s15, packet_operand_salu0_0_1_s17, 0, 1, 0, 0, 0, 3, 0, 1, old_state_group_0_0_s19_state_0_s371, constant_vector__ANONYMOUS_s313);
  int  output_0_0_s21=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_0(old_state_group_0_0_s19_state_0_s371, destination_0_0_s5, 0, output_0_0_s21);
  int  output_0_1_s23=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_1(old_state_group_0_0_s19_state_0_s371, destination_0_1_s7, 1, output_0_1_s23);
  int  output_0_2_s25=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_2(old_state_group_0_0_s19_state_0_s371, destination_0_2_s9, 1, output_0_2_s25);
  int  output_0_3_s27=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_3(old_state_group_0_0_s19_state_0_s371, destination_0_3_s11, 1, output_0_3_s27);
  int  output_0_4_s29=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_4(old_state_group_0_0_s19_state_0_s371, destination_0_4_s13, 1, output_0_4_s29);
  int  destination_1_0_s31=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0(output_0_0_s21, output_0_1_s23, output_0_2_s25, output_0_3_s27, output_0_4_s29, 3, 4, 2, 6, 2, destination_1_0_s31, constant_vector__ANONYMOUS_s313);
  int  destination_1_1_s33=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1(output_0_0_s21, output_0_1_s23, output_0_2_s25, output_0_3_s27, output_0_4_s29, 10, 1, 7, 2, 0, destination_1_1_s33, constant_vector__ANONYMOUS_s313);
  int  destination_1_2_s35=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2(output_0_0_s21, output_0_1_s23, output_0_2_s25, output_0_3_s27, output_0_4_s29, 2, 2, 0, 2, 6, destination_1_2_s35, constant_vector__ANONYMOUS_s313);
  int  destination_1_3_s37=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3(output_0_0_s21, output_0_1_s23, output_0_2_s25, output_0_3_s27, output_0_4_s29, 5, 0, 2, 0, 2, destination_1_3_s37, constant_vector__ANONYMOUS_s313);
  int  destination_1_4_s39=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4(output_0_0_s21, output_0_1_s23, output_0_2_s25, output_0_3_s27, output_0_4_s29, 1, 0, 3, 4, 2, destination_1_4_s39, constant_vector__ANONYMOUS_s313);
  int  packet_operand_salu1_0_0_s41=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_0(output_0_0_s21, output_0_1_s23, output_0_2_s25, output_0_3_s27, output_0_4_s29, 2, packet_operand_salu1_0_0_s41);
  int  packet_operand_salu1_0_1_s43=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_1(output_0_0_s21, output_0_1_s23, output_0_2_s25, output_0_3_s27, output_0_4_s29, 3, packet_operand_salu1_0_1_s43);
  int  old_state_group_1_0_s45_state_0_s373=0;
  int  state_operand_salu_1_0_state_0_s367=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0(state_operand_salu_1_0_state_0_s367, 0, packet_operand_salu1_0_0_s41, packet_operand_salu1_0_1_s43, 0, 3, 0, 0, 6, 3, 0, 2, old_state_group_1_0_s45_state_0_s373, constant_vector__ANONYMOUS_s313);
  int  output_1_0_s47=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_0(old_state_group_1_0_s45_state_0_s373, destination_1_0_s31, 1, output_1_0_s47);
  int  output_1_1_s49=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_1(old_state_group_1_0_s45_state_0_s373, destination_1_1_s33, 1, output_1_1_s49);
  int  output_1_2_s51=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_2(old_state_group_1_0_s45_state_0_s373, destination_1_2_s35, 1, output_1_2_s51);
  int  output_1_3_s53=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_3(old_state_group_1_0_s45_state_0_s373, destination_1_3_s37, 1, output_1_3_s53);
  int  output_1_4_s55=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_4(old_state_group_1_0_s45_state_0_s373, destination_1_4_s39, 1, output_1_4_s55);
  int  destination_2_0_s57=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0(output_1_0_s47, output_1_1_s49, output_1_2_s51, output_1_3_s53, output_1_4_s55, 15, 8, 1, 5, 2, destination_2_0_s57, constant_vector__ANONYMOUS_s313);
  int  destination_2_1_s59=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1(output_1_0_s47, output_1_1_s49, output_1_2_s51, output_1_3_s53, output_1_4_s55, 1, 1, 3, 0, 1, destination_2_1_s59, constant_vector__ANONYMOUS_s313);
  int  destination_2_2_s61=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2(output_1_0_s47, output_1_1_s49, output_1_2_s51, output_1_3_s53, output_1_4_s55, 2, 7, 0, 3, 3, destination_2_2_s61, constant_vector__ANONYMOUS_s313);
  int  destination_2_3_s63=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3(output_1_0_s47, output_1_1_s49, output_1_2_s51, output_1_3_s53, output_1_4_s55, 15, 0, 5, 4, 1, destination_2_3_s63, constant_vector__ANONYMOUS_s313);
  int  destination_2_4_s65=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4(output_1_0_s47, output_1_1_s49, output_1_2_s51, output_1_3_s53, output_1_4_s55, 9, 7, 3, 2, 0, destination_2_4_s65, constant_vector__ANONYMOUS_s313);
  int  packet_operand_salu2_0_0_s67=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_0(output_1_0_s47, output_1_1_s49, output_1_2_s51, output_1_3_s53, output_1_4_s55, 3, packet_operand_salu2_0_0_s67);
  int  packet_operand_salu2_0_1_s69=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_1(output_1_0_s47, output_1_1_s49, output_1_2_s51, output_1_3_s53, output_1_4_s55, 2, packet_operand_salu2_0_1_s69);
  int  state_operand_salu_2_0_state_0_s368=state_and_packet_state_group_0_state_0_s360;
  int  old_state_group_2_0_s71_state_0_s375=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0(state_operand_salu_2_0_state_0_s368, 0, packet_operand_salu2_0_0_s67, packet_operand_salu2_0_1_s69, 0, 2, 1, 0, 0, 8, 0, 2, old_state_group_2_0_s71_state_0_s375, constant_vector__ANONYMOUS_s313);
  int  output_2_0_s73=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_0(old_state_group_2_0_s71_state_0_s375, destination_2_0_s57, 1, output_2_0_s73);
  int  output_2_1_s75=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_1(old_state_group_2_0_s71_state_0_s375, destination_2_1_s59, 1, output_2_1_s75);
  int  output_2_2_s77=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_2(old_state_group_2_0_s71_state_0_s375, destination_2_2_s61, 1, output_2_2_s77);
  int  output_2_3_s79=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_3(old_state_group_2_0_s71_state_0_s375, destination_2_3_s63, 1, output_2_3_s79);
  int  output_2_4_s81=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_4(old_state_group_2_0_s71_state_0_s375, destination_2_4_s65, 0, output_2_4_s81);
  int  destination_3_0_s83=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0(output_2_0_s73, output_2_1_s75, output_2_2_s77, output_2_3_s79, output_2_4_s81, 16, 5, 2, 3, 1, destination_3_0_s83, constant_vector__ANONYMOUS_s313);
  int  destination_3_1_s85=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1(output_2_0_s73, output_2_1_s75, output_2_2_s77, output_2_3_s79, output_2_4_s81, 3, 5, 2, 1, 1, destination_3_1_s85, constant_vector__ANONYMOUS_s313);
  int  destination_3_2_s87=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2(output_2_0_s73, output_2_1_s75, output_2_2_s77, output_2_3_s79, output_2_4_s81, 14, 7, 3, 2, 2, destination_3_2_s87, constant_vector__ANONYMOUS_s313);
  int  destination_3_3_s89=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3(output_2_0_s73, output_2_1_s75, output_2_2_s77, output_2_3_s79, output_2_4_s81, 14, 0, 4, 3, 0, destination_3_3_s89, constant_vector__ANONYMOUS_s313);
  int  destination_3_4_s91=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4(output_2_0_s73, output_2_1_s75, output_2_2_s77, output_2_3_s79, output_2_4_s81, 14, 0, 6, 0, 0, destination_3_4_s91, constant_vector__ANONYMOUS_s313);
  int  packet_operand_salu3_0_0_s93=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_0(output_2_0_s73, output_2_1_s75, output_2_2_s77, output_2_3_s79, output_2_4_s81, 1, packet_operand_salu3_0_0_s93);
  int  packet_operand_salu3_0_1_s95=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_1(output_2_0_s73, output_2_1_s75, output_2_2_s77, output_2_3_s79, output_2_4_s81, 1, packet_operand_salu3_0_1_s95);
  int  old_state_group_3_0_s97_state_0_s377=0;
  int  state_operand_salu_3_0_state_0_s369=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0(state_operand_salu_3_0_state_0_s369, 0, packet_operand_salu3_0_0_s93, packet_operand_salu3_0_1_s95, 1, 0, 0, 0, 8, 8, 0, 1, old_state_group_3_0_s97_state_0_s377, constant_vector__ANONYMOUS_s313);
  int  output_3_0_s99=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_0(old_state_group_3_0_s97_state_0_s377, destination_3_0_s83, 0, output_3_0_s99);
  int  output_3_1_s101=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_1(old_state_group_3_0_s97_state_0_s377, destination_3_1_s85, 1, output_3_1_s101);
  int  output_3_2_s103=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_2(old_state_group_3_0_s97_state_0_s377, destination_3_2_s87, 1, output_3_2_s103);
  int  output_3_3_s105=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_3(old_state_group_3_0_s97_state_0_s377, destination_3_3_s89, 1, output_3_3_s105);
  int  output_3_4_s107=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_4(old_state_group_3_0_s97_state_0_s377, destination_3_4_s91, 0, output_3_4_s107);
  _out_pkt_0_s361 = output_3_0_s99;
  _out_pkt_1_s362 = output_3_1_s101;
  _out_pkt_2_s363 = output_3_2_s103;
  _out_pkt_3_s364 = output_3_3_s105;
  _out_state_group_0_state_0_s365 = state_operand_salu_2_0_state_0_s368;
  return;
}
void program(int state_and_packet_pkt_0_s346, int state_and_packet_pkt_1_s347, int state_and_packet_pkt_2_s348, int state_and_packet_pkt_3_s349, int state_and_packet_state_group_0_state_0_s350, int& _out_pkt_0_s351, int& _out_pkt_1_s352, int& _out_pkt_2_s353, int& _out_pkt_3_s354, int& _out_state_group_0_state_0_s355) {
  state_and_packet_pkt_2_s348 = state_and_packet_pkt_1_s347 + state_and_packet_pkt_0_s346;
  if (!(!(!((state_and_packet_pkt_1_s347) == (102))))) {
    if (!((state_and_packet_pkt_1_s347) == (102))) {
      if ((state_and_packet_pkt_0_s346) == (102)) {
        state_and_packet_pkt_3_s349 = (state_and_packet_state_group_0_state_0_s350) == (0);
      }
    }
  } else {
    if (!(!((state_and_packet_pkt_1_s347) == (102)))) {
      if ((state_and_packet_pkt_1_s347) == (102)) {
        state_and_packet_state_group_0_state_0_s350 = 1;
      }
    }
  }
  _out_pkt_0_s351 = state_and_packet_pkt_0_s346;
  _out_pkt_1_s352 = state_and_packet_pkt_1_s347;
  _out_pkt_2_s353 = state_and_packet_pkt_2_s348;
  _out_pkt_3_s354 = state_and_packet_pkt_3_s349;
  _out_state_group_0_state_0_s355 = state_and_packet_state_group_0_state_0_s350;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s285/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s285[immediate_operand_hole_local]);
  int  pkt_0_s279=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s279);
  int  pkt_1_s281=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s281);
  int  pkt_2_s283=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s283);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s279 + pkt_1_s281;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s279 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s279 - pkt_1_s281;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s279 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s279;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s279) != (pkt_1_s281);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s279) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s279) == (pkt_1_s281);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s279) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s279) >= (pkt_1_s281);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s279) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s279) < (pkt_1_s281);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s279) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s279) != (0)) {
                                  _out = pkt_1_s281;
                                  return;
                                } else {
                                  _out = pkt_2_s283;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s279) != (0)) {
                                    _out = pkt_1_s281;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s279) != (0)) || ((pkt_1_s281) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s279) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s279) != (0)) && ((pkt_1_s281) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s279) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s279) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s304/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s304[immediate_operand_hole_local]);
  int  pkt_0_s273=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s273);
  int  pkt_1_s275=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s275);
  int  pkt_2_s277=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s277);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s273 + pkt_1_s275;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s273 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s273 - pkt_1_s275;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s273 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s273;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s273) != (pkt_1_s275);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s273) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s273) == (pkt_1_s275);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s273) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s273) >= (pkt_1_s275);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s273) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s273) < (pkt_1_s275);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s273) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s273) != (0)) {
                                  _out = pkt_1_s275;
                                  return;
                                } else {
                                  _out = pkt_2_s277;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s273) != (0)) {
                                    _out = pkt_1_s275;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s273) != (0)) || ((pkt_1_s275) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s273) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s273) != (0)) && ((pkt_1_s275) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s273) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s273) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s310/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s310[immediate_operand_hole_local]);
  int  pkt_0_s267=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s267);
  int  pkt_1_s269=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s269);
  int  pkt_2_s271=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s271);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s267 + pkt_1_s269;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s267 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s267 - pkt_1_s269;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s267 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s267;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s267) != (pkt_1_s269);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s267) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s267) == (pkt_1_s269);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s267) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s267) >= (pkt_1_s269);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s267) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s267) < (pkt_1_s269);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s267) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s267) != (0)) {
                                  _out = pkt_1_s269;
                                  return;
                                } else {
                                  _out = pkt_2_s271;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s267) != (0)) {
                                    _out = pkt_1_s269;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s267) != (0)) || ((pkt_1_s269) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s267) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s267) != (0)) && ((pkt_1_s269) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s267) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s267) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s288/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s288[immediate_operand_hole_local]);
  int  pkt_0_s261=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s261);
  int  pkt_1_s263=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s263);
  int  pkt_2_s265=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s265);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s261 + pkt_1_s263;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s261 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s261 - pkt_1_s263;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s261 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s261;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s261) != (pkt_1_s263);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s261) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s261) == (pkt_1_s263);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s261) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s261) >= (pkt_1_s263);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s261) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s261) < (pkt_1_s263);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s261) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s261) != (0)) {
                                  _out = pkt_1_s263;
                                  return;
                                } else {
                                  _out = pkt_2_s265;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s261) != (0)) {
                                    _out = pkt_1_s263;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s261) != (0)) || ((pkt_1_s263) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s261) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s261) != (0)) && ((pkt_1_s263) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s261) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s261) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s308/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s308[immediate_operand_hole_local]);
  int  pkt_0_s255=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s255);
  int  pkt_1_s257=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s257);
  int  pkt_2_s259=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s259);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s255 + pkt_1_s257;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s255 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s255 - pkt_1_s257;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s255 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s255;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s255) != (pkt_1_s257);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s255) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s255) == (pkt_1_s257);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s255) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s255) >= (pkt_1_s257);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s255) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s255) < (pkt_1_s257);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s255) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s255) != (0)) {
                                  _out = pkt_1_s257;
                                  return;
                                } else {
                                  _out = pkt_2_s259;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s255) != (0)) {
                                    _out = pkt_1_s257;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s255) != (0)) || ((pkt_1_s257) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s255) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s255) != (0)) && ((pkt_1_s257) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s255) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s255) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_0(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_0_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_0_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_0_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0(int& state_group_state_0_s387, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s388, int* constant_vector__ANONYMOUS_s318/* len = 9 */) {
  int  old_state_group_state_0_s389=state_group_state_0_s387;
  int  state_0=0;
  state_0 = state_group_state_0_s387;
  int  _out_s241=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_0(state_group_state_0_s387, Opt_0, _out_s241);
  int  _out_s243=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_C_0(const_0, _out_s243, constant_vector__ANONYMOUS_s318);
  int  _out_s245=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_0(pkt_0, pkt_1, _out_s243, Mux3_0, _out_s245);
  int  _out_s247=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_rel_op_0(_out_s241, _out_s245, rel_op_0, _out_s247);
  if ((_out_s247) == (1)) {
    int  state_0_s249=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_1(state_group_state_0_s387, Opt_1, state_0_s249);
    int  state_0_s251=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_C_1(const_1, state_0_s251, constant_vector__ANONYMOUS_s318);
    int  state_0_s253=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_1(pkt_0, pkt_1, state_0_s251, Mux3_1, state_0_s253);
    state_0 = state_0_s249 + state_0_s253;
  }
  state_group_state_0_s387 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s388 = old_state_group_state_0_s389;
    return;
  } else {
    _out_state_0_s388 = state_0;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_0(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_0_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_1(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_2(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_3(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_4(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_4_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_4_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s295/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s295[immediate_operand_hole_local]);
  int  pkt_0_s235=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s235);
  int  pkt_1_s237=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s237);
  int  pkt_2_s239=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s239);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s235 + pkt_1_s237;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s235 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s235 - pkt_1_s237;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s235 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s235;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s235) != (pkt_1_s237);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s235) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s235) == (pkt_1_s237);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s235) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s235) >= (pkt_1_s237);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s235) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s235) < (pkt_1_s237);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s235) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s235) != (0)) {
                                  _out = pkt_1_s237;
                                  return;
                                } else {
                                  _out = pkt_2_s239;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s235) != (0)) {
                                    _out = pkt_1_s237;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s235) != (0)) || ((pkt_1_s237) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s235) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s235) != (0)) && ((pkt_1_s237) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s235) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s235) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s300/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s300[immediate_operand_hole_local]);
  int  pkt_0_s229=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s229);
  int  pkt_1_s231=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s231);
  int  pkt_2_s233=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s233);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s229 + pkt_1_s231;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s229 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s229 - pkt_1_s231;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s229 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s229;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s229) != (pkt_1_s231);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s229) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s229) == (pkt_1_s231);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s229) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s229) >= (pkt_1_s231);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s229) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s229) < (pkt_1_s231);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s229) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s229) != (0)) {
                                  _out = pkt_1_s231;
                                  return;
                                } else {
                                  _out = pkt_2_s233;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s229) != (0)) {
                                    _out = pkt_1_s231;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s229) != (0)) || ((pkt_1_s231) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s229) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s229) != (0)) && ((pkt_1_s231) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s229) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s229) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s307/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s307[immediate_operand_hole_local]);
  int  pkt_0_s223=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s223);
  int  pkt_1_s225=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s225);
  int  pkt_2_s227=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s227);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s223 + pkt_1_s225;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s223 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s223 - pkt_1_s225;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s223 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s223;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s223) != (pkt_1_s225);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s223) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s223) == (pkt_1_s225);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s223) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s223) >= (pkt_1_s225);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s223) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s223) < (pkt_1_s225);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s223) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s223) != (0)) {
                                  _out = pkt_1_s225;
                                  return;
                                } else {
                                  _out = pkt_2_s227;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s223) != (0)) {
                                    _out = pkt_1_s225;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s223) != (0)) || ((pkt_1_s225) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s223) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s223) != (0)) && ((pkt_1_s225) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s223) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s223) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s287/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s287[immediate_operand_hole_local]);
  int  pkt_0_s217=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s217);
  int  pkt_1_s219=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s219);
  int  pkt_2_s221=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s221);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s217 + pkt_1_s219;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s217 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s217 - pkt_1_s219;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s217 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s217;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s217) != (pkt_1_s219);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s217) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s217) == (pkt_1_s219);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s217) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s217) >= (pkt_1_s219);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s217) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s217) < (pkt_1_s219);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s217) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s217) != (0)) {
                                  _out = pkt_1_s219;
                                  return;
                                } else {
                                  _out = pkt_2_s221;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s217) != (0)) {
                                    _out = pkt_1_s219;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s217) != (0)) || ((pkt_1_s219) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s217) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s217) != (0)) && ((pkt_1_s219) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s217) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s217) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s297/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s297[immediate_operand_hole_local]);
  int  pkt_0_s211=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s211);
  int  pkt_1_s213=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s213);
  int  pkt_2_s215=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s215);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s211 + pkt_1_s213;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s211 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s211 - pkt_1_s213;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s211 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s211;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s211) != (pkt_1_s213);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s211) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s211) == (pkt_1_s213);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s211) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s211) >= (pkt_1_s213);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s211) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s211) < (pkt_1_s213);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s211) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s211) != (0)) {
                                  _out = pkt_1_s213;
                                  return;
                                } else {
                                  _out = pkt_2_s215;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s211) != (0)) {
                                    _out = pkt_1_s213;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s211) != (0)) || ((pkt_1_s213) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s211) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s211) != (0)) && ((pkt_1_s213) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s211) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s211) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_0(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_0_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_0_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_0_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0(int& state_group_state_0_s384, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s385, int* constant_vector__ANONYMOUS_s315/* len = 9 */) {
  int  old_state_group_state_0_s386=state_group_state_0_s384;
  int  state_0=0;
  state_0 = state_group_state_0_s384;
  int  _out_s197=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_0(state_group_state_0_s384, Opt_0, _out_s197);
  int  _out_s199=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_C_0(const_0, _out_s199, constant_vector__ANONYMOUS_s315);
  int  _out_s201=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_0(pkt_0, pkt_1, _out_s199, Mux3_0, _out_s201);
  int  _out_s203=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_rel_op_0(_out_s197, _out_s201, rel_op_0, _out_s203);
  if ((_out_s203) == (1)) {
    int  state_0_s205=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_1(state_group_state_0_s384, Opt_1, state_0_s205);
    int  state_0_s207=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_C_1(const_1, state_0_s207, constant_vector__ANONYMOUS_s315);
    int  state_0_s209=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_1(pkt_0, pkt_1, state_0_s207, Mux3_1, state_0_s209);
    state_0 = state_0_s205 + state_0_s209;
  }
  state_group_state_0_s384 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s385 = old_state_group_state_0_s386;
    return;
  } else {
    _out_state_0_s385 = state_0;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_0(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_0_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_1(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_2(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_3(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_4(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_4_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_4_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s289/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s289[immediate_operand_hole_local]);
  int  pkt_0_s191=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s191);
  int  pkt_1_s193=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s193);
  int  pkt_2_s195=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s195);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s191 + pkt_1_s193;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s191 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s191 - pkt_1_s193;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s191 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s191;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s191) != (pkt_1_s193);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s191) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s191) == (pkt_1_s193);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s191) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s191) >= (pkt_1_s193);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s191) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s191) < (pkt_1_s193);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s191) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s191) != (0)) {
                                  _out = pkt_1_s193;
                                  return;
                                } else {
                                  _out = pkt_2_s195;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s191) != (0)) {
                                    _out = pkt_1_s193;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s191) != (0)) || ((pkt_1_s193) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s191) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s191) != (0)) && ((pkt_1_s193) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s191) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s191) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s294/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s294[immediate_operand_hole_local]);
  int  pkt_0_s185=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s185);
  int  pkt_1_s187=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s187);
  int  pkt_2_s189=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s189);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s185 + pkt_1_s187;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s185 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s185 - pkt_1_s187;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s185 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s185;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s185) != (pkt_1_s187);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s185) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s185) == (pkt_1_s187);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s185) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s185) >= (pkt_1_s187);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s185) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s185) < (pkt_1_s187);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s185) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s185) != (0)) {
                                  _out = pkt_1_s187;
                                  return;
                                } else {
                                  _out = pkt_2_s189;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s185) != (0)) {
                                    _out = pkt_1_s187;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s185) != (0)) || ((pkt_1_s187) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s185) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s185) != (0)) && ((pkt_1_s187) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s185) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s185) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s284/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s284[immediate_operand_hole_local]);
  int  pkt_0_s179=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s179);
  int  pkt_1_s181=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s181);
  int  pkt_2_s183=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s183);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s179 + pkt_1_s181;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s179 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s179 - pkt_1_s181;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s179 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s179;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s179) != (pkt_1_s181);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s179) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s179) == (pkt_1_s181);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s179) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s179) >= (pkt_1_s181);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s179) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s179) < (pkt_1_s181);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s179) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s179) != (0)) {
                                  _out = pkt_1_s181;
                                  return;
                                } else {
                                  _out = pkt_2_s183;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s179) != (0)) {
                                    _out = pkt_1_s181;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s179) != (0)) || ((pkt_1_s181) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s179) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s179) != (0)) && ((pkt_1_s181) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s179) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s179) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s309/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s309[immediate_operand_hole_local]);
  int  pkt_0_s173=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s173);
  int  pkt_1_s175=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s175);
  int  pkt_2_s177=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s177);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s173 + pkt_1_s175;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s173 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s173 - pkt_1_s175;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s173 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s173;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s173) != (pkt_1_s175);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s173) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s173) == (pkt_1_s175);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s173) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s173) >= (pkt_1_s175);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s173) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s173) < (pkt_1_s175);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s173) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s173) != (0)) {
                                  _out = pkt_1_s175;
                                  return;
                                } else {
                                  _out = pkt_2_s177;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s173) != (0)) {
                                    _out = pkt_1_s175;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s173) != (0)) || ((pkt_1_s175) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s173) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s173) != (0)) && ((pkt_1_s175) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s173) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s173) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s296/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s296[immediate_operand_hole_local]);
  int  pkt_0_s167=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s167);
  int  pkt_1_s169=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s169);
  int  pkt_2_s171=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s171);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s167 + pkt_1_s169;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s167 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s167 - pkt_1_s169;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s167 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s167;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s167) != (pkt_1_s169);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s167) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s167) == (pkt_1_s169);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s167) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s167) >= (pkt_1_s169);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s167) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s167) < (pkt_1_s169);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s167) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s167) != (0)) {
                                  _out = pkt_1_s169;
                                  return;
                                } else {
                                  _out = pkt_2_s171;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s167) != (0)) {
                                    _out = pkt_1_s169;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s167) != (0)) || ((pkt_1_s169) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s167) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s167) != (0)) && ((pkt_1_s169) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s167) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s167) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_0(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_0_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_0_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_0_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0(int& state_group_state_0_s381, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s382, int* constant_vector__ANONYMOUS_s314/* len = 9 */) {
  int  old_state_group_state_0_s383=state_group_state_0_s381;
  int  state_0=0;
  state_0 = state_group_state_0_s381;
  int  _out_s153=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_0(state_group_state_0_s381, Opt_0, _out_s153);
  int  _out_s155=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_C_0(const_0, _out_s155, constant_vector__ANONYMOUS_s314);
  int  _out_s157=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_0(pkt_0, pkt_1, _out_s155, Mux3_0, _out_s157);
  int  _out_s159=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_rel_op_0(_out_s153, _out_s157, rel_op_0, _out_s159);
  if ((_out_s159) == (1)) {
    int  state_0_s161=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_1(state_group_state_0_s381, Opt_1, state_0_s161);
    int  state_0_s163=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_C_1(const_1, state_0_s163, constant_vector__ANONYMOUS_s314);
    int  state_0_s165=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_1(pkt_0, pkt_1, state_0_s163, Mux3_1, state_0_s165);
    state_0 = state_0_s161 + state_0_s165;
  }
  state_group_state_0_s381 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s382 = old_state_group_state_0_s383;
    return;
  } else {
    _out_state_0_s382 = state_0;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_0(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_0_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_1(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_2(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_3(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_4(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_4_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_4_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s286/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s286[immediate_operand_hole_local]);
  int  pkt_0_s147=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s147);
  int  pkt_1_s149=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s149);
  int  pkt_2_s151=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s151);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s147 + pkt_1_s149;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s147 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s147 - pkt_1_s149;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s147 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s147;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s147) != (pkt_1_s149);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s147) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s147) == (pkt_1_s149);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s147) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s147) >= (pkt_1_s149);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s147) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s147) < (pkt_1_s149);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s147) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s147) != (0)) {
                                  _out = pkt_1_s149;
                                  return;
                                } else {
                                  _out = pkt_2_s151;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s147) != (0)) {
                                    _out = pkt_1_s149;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s147) != (0)) || ((pkt_1_s149) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s147) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s147) != (0)) && ((pkt_1_s149) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s147) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s147) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s298/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s298[immediate_operand_hole_local]);
  int  pkt_0_s141=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s141);
  int  pkt_1_s143=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s143);
  int  pkt_2_s145=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s145);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s141 + pkt_1_s143;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s141 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s141 - pkt_1_s143;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s141 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s141;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s141) != (pkt_1_s143);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s141) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s141) == (pkt_1_s143);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s141) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s141) >= (pkt_1_s143);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s141) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s141) < (pkt_1_s143);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s141) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s141) != (0)) {
                                  _out = pkt_1_s143;
                                  return;
                                } else {
                                  _out = pkt_2_s145;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s141) != (0)) {
                                    _out = pkt_1_s143;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s141) != (0)) || ((pkt_1_s143) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s141) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s141) != (0)) && ((pkt_1_s143) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s141) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s141) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s290/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s290[immediate_operand_hole_local]);
  int  pkt_0_s135=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s135);
  int  pkt_1_s137=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s137);
  int  pkt_2_s139=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s139);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s135 + pkt_1_s137;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s135 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s135 - pkt_1_s137;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s135 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s135;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s135) != (pkt_1_s137);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s135) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s135) == (pkt_1_s137);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s135) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s135) >= (pkt_1_s137);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s135) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s135) < (pkt_1_s137);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s135) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s135) != (0)) {
                                  _out = pkt_1_s137;
                                  return;
                                } else {
                                  _out = pkt_2_s139;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s135) != (0)) {
                                    _out = pkt_1_s137;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s135) != (0)) || ((pkt_1_s137) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s135) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s135) != (0)) && ((pkt_1_s137) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s135) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s135) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s291/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s291[immediate_operand_hole_local]);
  int  pkt_0_s129=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s129);
  int  pkt_1_s131=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s131);
  int  pkt_2_s133=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s133);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s129 + pkt_1_s131;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s129 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s129 - pkt_1_s131;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s129 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s129;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s129) != (pkt_1_s131);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s129) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s129) == (pkt_1_s131);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s129) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s129) >= (pkt_1_s131);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s129) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s129) < (pkt_1_s131);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s129) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s129) != (0)) {
                                  _out = pkt_1_s131;
                                  return;
                                } else {
                                  _out = pkt_2_s133;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s129) != (0)) {
                                    _out = pkt_1_s131;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s129) != (0)) || ((pkt_1_s131) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s129) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s129) != (0)) && ((pkt_1_s131) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s129) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s129) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s301/* len = 9 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s301[immediate_operand_hole_local]);
  int  pkt_0_s123=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux1(input0, input1, input2, input3, input4, mux1_ctrl_hole_local, pkt_0_s123);
  int  pkt_1_s125=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux2(input0, input1, input2, input3, input4, mux2_ctrl_hole_local, pkt_1_s125);
  int  pkt_2_s127=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux3(input0, input1, input2, input3, input4, mux3_ctrl_hole_local, pkt_2_s127);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s123 + pkt_1_s125;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s123 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s123 - pkt_1_s125;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s123 - immediate_operand;
            return;
          } else {
            if ((opcode_hole_local) == (5)) {
              _out = immediate_operand - pkt_0_s123;
              return;
            } else {
              if ((opcode_hole_local) == (6)) {
                _out = (pkt_0_s123) != (pkt_1_s125);
                return;
              } else {
                if ((opcode_hole_local) == (7)) {
                  _out = (pkt_0_s123) != (immediate_operand);
                  return;
                } else {
                  if ((opcode_hole_local) == (8)) {
                    _out = (pkt_0_s123) == (pkt_1_s125);
                    return;
                  } else {
                    if ((opcode_hole_local) == (9)) {
                      _out = (pkt_0_s123) == (immediate_operand);
                      return;
                    } else {
                      if ((opcode_hole_local) == (10)) {
                        _out = (pkt_0_s123) >= (pkt_1_s125);
                        return;
                      } else {
                        if ((opcode_hole_local) == (11)) {
                          _out = (pkt_0_s123) >= (immediate_operand);
                          return;
                        } else {
                          if ((opcode_hole_local) == (12)) {
                            _out = (pkt_0_s123) < (pkt_1_s125);
                            return;
                          } else {
                            if ((opcode_hole_local) == (13)) {
                              _out = (pkt_0_s123) < (immediate_operand);
                              return;
                            } else {
                              if ((opcode_hole_local) == (14)) {
                                if ((pkt_0_s123) != (0)) {
                                  _out = pkt_1_s125;
                                  return;
                                } else {
                                  _out = pkt_2_s127;
                                  return;
                                }
                              } else {
                                if ((opcode_hole_local) == (15)) {
                                  if ((pkt_0_s123) != (0)) {
                                    _out = pkt_1_s125;
                                    return;
                                  } else {
                                    _out = immediate_operand;
                                    return;
                                  }
                                } else {
                                  if ((opcode_hole_local) == (16)) {
                                    _out = ((pkt_0_s123) != (0)) || ((pkt_1_s125) != (0));
                                    return;
                                  } else {
                                    if ((opcode_hole_local) == (17)) {
                                      _out = ((pkt_0_s123) != (0)) || ((immediate_operand) != (0));
                                      return;
                                    } else {
                                      if ((opcode_hole_local) == (18)) {
                                        _out = ((pkt_0_s123) != (0)) && ((pkt_1_s125) != (0));
                                        return;
                                      } else {
                                        if ((opcode_hole_local) == (19)) {
                                          _out = ((pkt_0_s123) != (0)) && ((immediate_operand) != (0));
                                          return;
                                        } else {
                                          _out = (pkt_0_s123) == (0);
                                          return;
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_0(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_0_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_0_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_0_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0(int& state_group_state_0_s378, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s379, int* constant_vector__ANONYMOUS_s317/* len = 9 */) {
  int  old_state_group_state_0_s380=state_group_state_0_s378;
  int  state_0=0;
  state_0 = state_group_state_0_s378;
  int  _out_s109=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_0(state_group_state_0_s378, Opt_0, _out_s109);
  int  _out_s111=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_C_0(const_0, _out_s111, constant_vector__ANONYMOUS_s317);
  int  _out_s113=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_0(pkt_0, pkt_1, _out_s111, Mux3_0, _out_s113);
  int  _out_s115=0;
  stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_rel_op_0(_out_s109, _out_s113, rel_op_0, _out_s115);
  if ((_out_s115) == (1)) {
    int  state_0_s117=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_1(state_group_state_0_s378, Opt_1, state_0_s117);
    int  state_0_s119=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_C_1(const_1, state_0_s119, constant_vector__ANONYMOUS_s317);
    int  state_0_s121=0;
    stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_1(pkt_0, pkt_1, state_0_s119, Mux3_1, state_0_s121);
    state_0 = state_0_s117 + state_0_s121;
  }
  state_group_state_0_s378 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s379 = old_state_group_state_0_s380;
    return;
  } else {
    _out_state_0_s379 = state_0;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_0(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_0_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_1(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_2(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_3(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_4(int input0, int input1, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_4_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_4_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s305/* len = 9 */) {
  _out = (constant_vector__ANONYMOUS_s305[const_var]);
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = ((operand1) != (operand2) ? 1 : 0);
    return;
  } else {
    if ((opcode) == (1)) {
      _out = ((operand1) < (operand2) ? 1 : 0);
      return;
    } else {
      if ((opcode) == (2)) {
        _out = ((operand1) > (operand2) ? 1 : 0);
        return;
      } else {
        _out = ((operand1) == (operand2) ? 1 : 0);
        return;
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s299/* len = 9 */) {
  _out = (constant_vector__ANONYMOUS_s299[const_var]);
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s293/* len = 9 */) {
  _out = (constant_vector__ANONYMOUS_s293[const_var]);
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = ((operand1) != (operand2) ? 1 : 0);
    return;
  } else {
    if ((opcode) == (1)) {
      _out = ((operand1) < (operand2) ? 1 : 0);
      return;
    } else {
      if ((opcode) == (2)) {
        _out = ((operand1) > (operand2) ? 1 : 0);
        return;
      } else {
        _out = ((operand1) == (operand2) ? 1 : 0);
        return;
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s302/* len = 9 */) {
  _out = (constant_vector__ANONYMOUS_s302[const_var]);
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s292/* len = 9 */) {
  _out = (constant_vector__ANONYMOUS_s292[const_var]);
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = ((operand1) != (operand2) ? 1 : 0);
    return;
  } else {
    if ((opcode) == (1)) {
      _out = ((operand1) < (operand2) ? 1 : 0);
      return;
    } else {
      if ((opcode) == (2)) {
        _out = ((operand1) > (operand2) ? 1 : 0);
        return;
      } else {
        _out = ((operand1) == (operand2) ? 1 : 0);
        return;
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s306/* len = 9 */) {
  _out = (constant_vector__ANONYMOUS_s306[const_var]);
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux1(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux1_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux1_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux1_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux2(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux2_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux2_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux2_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux2_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux3(int input0, int input1, int input2, int input3, int input4, int stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux3_ctrl_local, int& _out) {
  if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux3_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux3_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux3_ctrl_local) == (2)) {
        _out = input2;
        return;
      } else {
        if ((stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux3_ctrl_local) == (3)) {
          _out = input3;
          return;
        } else {
          _out = input4;
          return;
        }
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s303/* len = 9 */) {
  _out = (constant_vector__ANONYMOUS_s303[const_var]);
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = ((operand1) != (operand2) ? 1 : 0);
    return;
  } else {
    if ((opcode) == (1)) {
      _out = ((operand1) < (operand2) ? 1 : 0);
      return;
    } else {
      if ((opcode) == (2)) {
        _out = ((operand1) > (operand2) ? 1 : 0);
        return;
      } else {
        _out = ((operand1) == (operand2) ? 1 : 0);
        return;
      }
    }
  }
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s311/* len = 9 */) {
  _out = (constant_vector__ANONYMOUS_s311[const_var]);
  return;
}
void stateful_fw_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
  if ((choice) == (0)) {
    _out = op1;
    return;
  } else {
    if ((choice) == (1)) {
      _out = op2;
      return;
    } else {
      _out = op3;
      return;
    }
  }
}

}
