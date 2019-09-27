#include <cstdio>
#include <assert.h>
#include <iostream>
using namespace std;
#include "vops.h"
#include "blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_verify_iter_14.h"
namespace ANONYMOUS{

StateGroup* StateGroup::create(int  state_0_){
  void* temp= malloc( sizeof(StateGroup)  ); 
  StateGroup* rv = new (temp)StateGroup();
  rv->state_0 =  state_0_;
  return rv;
}
StateAndPacket* StateAndPacket::create(int  pkt_0_, int  pkt_1_, int  state_group_0_state_0_, int  state_group_1_state_0_){
  void* temp= malloc( sizeof(StateAndPacket)  ); 
  StateAndPacket* rv = new (temp)StateAndPacket();
  rv->pkt_0 =  pkt_0_;
  rv->pkt_1 =  pkt_1_;
  rv->state_group_0_state_0 =  state_group_0_state_0_;
  rv->state_group_1_state_0 =  state_group_1_state_0_;
  return rv;
}
void main__Wrapper(int pkt_0, int pkt_1, int state_group_0_state_0, int state_group_1_state_0) {
  bool _tt0[17] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
  int*  constant_vector__ANONYMOUS_s422= new int [17]; CopyArr<int >(constant_vector__ANONYMOUS_s422,_tt0, 17, 17);
  glblInit_constant_vector__ANONYMOUS_s432(constant_vector__ANONYMOUS_s422);
  _main(pkt_0, pkt_1, state_group_0_state_0, state_group_1_state_0, constant_vector__ANONYMOUS_s422);
  delete[] constant_vector__ANONYMOUS_s422;
}
void main__WrapperNospec(int pkt_0, int pkt_1, int state_group_0_state_0, int state_group_1_state_0) {}
void glblInit_constant_vector__ANONYMOUS_s432(int* constant_vector__ANONYMOUS_s431/* len = 17 */) {
  int _tt1[17] = {2, 1023, 10, 11, 1, 13, 18, 3, 5, 12, 9, 20, 10, 7, 1014, 0, 1};
  CopyArr<int >(constant_vector__ANONYMOUS_s431,_tt1, 17, 17);
}
void _main(int pkt_0, int pkt_1, int state_group_0_state_0, int state_group_1_state_0, int* constant_vector__ANONYMOUS_s420/* len = 17 */) {
  int  pipeline_result_s1_state_group_0_state_0_s443=0;
  int  pipeline_result_s1_state_group_1_state_0_s444=0;
  int  pipeline_result_s1_pkt_0_s441=0;
  int  pipeline_result_s1_pkt_1_s442=0;
  pipeline(pkt_0, pkt_1, state_group_0_state_0, state_group_1_state_0, pipeline_result_s1_pkt_0_s441, pipeline_result_s1_pkt_1_s442, pipeline_result_s1_state_group_0_state_0_s443, pipeline_result_s1_state_group_1_state_0_s444, constant_vector__ANONYMOUS_s420);
  int  program_result_s3_state_group_0_state_0_s451=0;
  int  program_result_s3_state_group_1_state_0_s452=0;
  int  program_result_s3_pkt_0_s449=0;
  int  program_result_s3_pkt_1_s450=0;
  program(pkt_0, pkt_1, state_group_0_state_0, state_group_1_state_0, program_result_s3_pkt_0_s449, program_result_s3_pkt_1_s450, program_result_s3_state_group_0_state_0_s451, program_result_s3_state_group_1_state_0_s452);
  assert ((pipeline_result_s1_state_group_0_state_0_s443) == (program_result_s3_state_group_0_state_0_s451));;
  assert ((pipeline_result_s1_state_group_1_state_0_s444) == (program_result_s3_state_group_1_state_0_s452));;
  assert ((pipeline_result_s1_pkt_0_s441) == (program_result_s3_pkt_0_s449));;
  assert ((pipeline_result_s1_pkt_1_s442) == (program_result_s3_pkt_1_s450));;
}
void pipeline(int state_and_packet_pkt_0_s461, int state_and_packet_pkt_1_s462, int state_and_packet_state_group_0_state_0_s463, int state_and_packet_state_group_1_state_0_s464, int& _out_pkt_0_s465, int& _out_pkt_1_s466, int& _out_state_group_0_state_0_s467, int& _out_state_group_1_state_0_s468, int* constant_vector__ANONYMOUS_s421/* len = 17 */) {
  int  destination_0_0_s5=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0(state_and_packet_pkt_0_s461, state_and_packet_pkt_1_s462, 4, 2, 0, 1, destination_0_0_s5, constant_vector__ANONYMOUS_s421);
  int  destination_0_1_s7=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1(state_and_packet_pkt_0_s461, state_and_packet_pkt_1_s462, 7, 1, 1, 0, destination_0_1_s7, constant_vector__ANONYMOUS_s421);
  int  packet_operand_salu0_0_0_s9=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_0(state_and_packet_pkt_0_s461, state_and_packet_pkt_1_s462, 0, packet_operand_salu0_0_0_s9);
  int  packet_operand_salu0_0_1_s11=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_1(state_and_packet_pkt_0_s461, state_and_packet_pkt_1_s462, 0, packet_operand_salu0_0_1_s11);
  int  packet_operand_salu0_1_0_s13=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_0(state_and_packet_pkt_0_s461, state_and_packet_pkt_1_s462, 0, packet_operand_salu0_1_0_s13);
  int  packet_operand_salu0_1_1_s15=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_1(state_and_packet_pkt_0_s461, state_and_packet_pkt_1_s462, 0, packet_operand_salu0_1_1_s15);
  int  old_state_group_0_0_s17_state_0_s478=0;
  int  state_operand_salu_0_0_state_0_s469=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0(state_operand_salu_0_0_state_0_s469, 1, packet_operand_salu0_0_0_s9, packet_operand_salu0_0_1_s11, 0, 0, 0, 3, 2, 0, 0, 0, 0, 1, 14, 2, 0, 14, 5, 1, 2, old_state_group_0_0_s17_state_0_s478, constant_vector__ANONYMOUS_s421);
  int  old_state_group_0_1_s19_state_0_s480=0;
  int  state_operand_salu_0_1_state_0_s470=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1(state_operand_salu_0_1_state_0_s470, 0, packet_operand_salu0_1_0_s13, packet_operand_salu0_1_1_s15, 3, 1, 2, 2, 2, 0, 0, 0, 0, 1, 0, 1, 15, 9, 14, 0, 2, old_state_group_0_1_s19_state_0_s480, constant_vector__ANONYMOUS_s421);
  int  output_0_0_s21=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_0(old_state_group_0_0_s17_state_0_s478, old_state_group_0_1_s19_state_0_s480, destination_0_0_s5, 2, output_0_0_s21);
  int  output_0_1_s23=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_1(old_state_group_0_0_s17_state_0_s478, old_state_group_0_1_s19_state_0_s480, destination_0_1_s7, 2, output_0_1_s23);
  int  destination_1_0_s25=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0(output_0_0_s21, output_0_1_s23, 4, 16, 0, 1, destination_1_0_s25, constant_vector__ANONYMOUS_s421);
  int  destination_1_1_s27=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1(output_0_0_s21, output_0_1_s23, 6, 4, 0, 1, destination_1_1_s27, constant_vector__ANONYMOUS_s421);
  int  packet_operand_salu1_0_0_s29=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_0(output_0_0_s21, output_0_1_s23, 0, packet_operand_salu1_0_0_s29);
  int  packet_operand_salu1_0_1_s31=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_1(output_0_0_s21, output_0_1_s23, 1, packet_operand_salu1_0_1_s31);
  int  packet_operand_salu1_1_0_s33=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_0(output_0_0_s21, output_0_1_s23, 0, packet_operand_salu1_1_0_s33);
  int  packet_operand_salu1_1_1_s35=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_1(output_0_0_s21, output_0_1_s23, 0, packet_operand_salu1_1_1_s35);
  int  state_operand_salu_1_1_state_0_s472=state_and_packet_state_group_1_state_0_s464;
  int  old_state_group_1_0_s37_state_0_s482=0;
  int  state_operand_salu_1_0_state_0_s471=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0(state_operand_salu_1_0_state_0_s471, 0, packet_operand_salu1_0_0_s29, packet_operand_salu1_0_1_s31, 2, 1, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 1, 4, 0, 0, 3, old_state_group_1_0_s37_state_0_s482, constant_vector__ANONYMOUS_s421);
  int  old_state_group_1_1_s39_state_0_s484=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1(state_operand_salu_1_1_state_0_s472, 1, packet_operand_salu1_1_0_s33, packet_operand_salu1_1_1_s35, 1, 2, 1, 2, 3, 0, 1, 0, 0, 0, 2, 2, 4, 15, 15, 1, 1, old_state_group_1_1_s39_state_0_s484, constant_vector__ANONYMOUS_s421);
  int  output_1_0_s41=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_0(old_state_group_1_0_s37_state_0_s482, old_state_group_1_1_s39_state_0_s484, destination_1_0_s25, 3, output_1_0_s41);
  int  output_1_1_s43=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_1(old_state_group_1_0_s37_state_0_s482, old_state_group_1_1_s39_state_0_s484, destination_1_1_s27, 1, output_1_1_s43);
  int  destination_2_0_s45=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0(output_1_0_s41, output_1_1_s43, 3, 15, 0, 1, destination_2_0_s45, constant_vector__ANONYMOUS_s421);
  int  destination_2_1_s47=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1(output_1_0_s41, output_1_1_s43, 5, 4, 0, 0, destination_2_1_s47, constant_vector__ANONYMOUS_s421);
  int  packet_operand_salu2_0_0_s49=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_0(output_1_0_s41, output_1_1_s43, 0, packet_operand_salu2_0_0_s49);
  int  packet_operand_salu2_0_1_s51=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_1(output_1_0_s41, output_1_1_s43, 0, packet_operand_salu2_0_1_s51);
  int  packet_operand_salu2_1_0_s53=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_0(output_1_0_s41, output_1_1_s43, 0, packet_operand_salu2_1_0_s53);
  int  packet_operand_salu2_1_1_s55=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_1(output_1_0_s41, output_1_1_s43, 0, packet_operand_salu2_1_1_s55);
  int  old_state_group_2_0_s57_state_0_s486=0;
  int  state_operand_salu_2_0_state_0_s473=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0(state_operand_salu_2_0_state_0_s473, 0, packet_operand_salu2_0_0_s49, packet_operand_salu2_0_1_s51, 3, 0, 3, 0, 2, 0, 0, 0, 0, 1, 15, 4, 2, 2, 2, 0, 2, old_state_group_2_0_s57_state_0_s486, constant_vector__ANONYMOUS_s421);
  int  old_state_group_2_1_s59_state_0_s488=0;
  int  state_operand_salu_2_1_state_0_s474=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1(state_operand_salu_2_1_state_0_s474, 0, packet_operand_salu2_1_0_s53, packet_operand_salu2_1_1_s55, 3, 0, 1, 3, 0, 0, 0, 0, 0, 1, 15, 2, 0, 15, 0, 0, 0, old_state_group_2_1_s59_state_0_s488, constant_vector__ANONYMOUS_s421);
  int  output_2_0_s61=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_0(old_state_group_2_0_s57_state_0_s486, old_state_group_2_1_s59_state_0_s488, destination_2_0_s45, 2, output_2_0_s61);
  int  output_2_1_s63=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_1(old_state_group_2_0_s57_state_0_s486, old_state_group_2_1_s59_state_0_s488, destination_2_1_s47, 2, output_2_1_s63);
  int  destination_3_0_s65=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0(output_2_0_s61, output_2_1_s63, 5, 9, 1, 0, destination_3_0_s65, constant_vector__ANONYMOUS_s421);
  int  destination_3_1_s67=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1(output_2_0_s61, output_2_1_s63, 3, 4, 0, 0, destination_3_1_s67, constant_vector__ANONYMOUS_s421);
  int  packet_operand_salu3_0_0_s69=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_0(output_2_0_s61, output_2_1_s63, 0, packet_operand_salu3_0_0_s69);
  int  packet_operand_salu3_0_1_s71=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_1(output_2_0_s61, output_2_1_s63, 0, packet_operand_salu3_0_1_s71);
  int  packet_operand_salu3_1_0_s73=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_0(output_2_0_s61, output_2_1_s63, 1, packet_operand_salu3_1_0_s73);
  int  packet_operand_salu3_1_1_s75=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_1(output_2_0_s61, output_2_1_s63, 0, packet_operand_salu3_1_1_s75);
  int  state_operand_salu_3_0_state_0_s475=state_and_packet_state_group_0_state_0_s463;
  int  old_state_group_3_0_s77_state_0_s490=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0(state_operand_salu_3_0_state_0_s475, 0, packet_operand_salu3_0_0_s69, packet_operand_salu3_0_1_s71, 1, 2, 2, 2, 3, 1, 0, 0, 0, 1, 15, 15, 15, 15, 0, 0, 2, old_state_group_3_0_s77_state_0_s490, constant_vector__ANONYMOUS_s421);
  int  old_state_group_3_1_s79_state_0_s492=0;
  int  state_operand_salu_3_1_state_0_s476=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1(state_operand_salu_3_1_state_0_s476, 0, packet_operand_salu3_1_0_s73, packet_operand_salu3_1_1_s75, 2, 3, 3, 3, 0, 0, 0, 0, 0, 1, 2, 4, 12, 0, 2, 0, 3, old_state_group_3_1_s79_state_0_s492, constant_vector__ANONYMOUS_s421);
  int  output_3_0_s81=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_0(old_state_group_3_0_s77_state_0_s490, old_state_group_3_1_s79_state_0_s492, destination_3_0_s65, 2, output_3_0_s81);
  int  output_3_1_s83=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_1(old_state_group_3_0_s77_state_0_s490, old_state_group_3_1_s79_state_0_s492, destination_3_1_s67, 1, output_3_1_s83);
  _out_pkt_0_s465 = output_3_0_s81;
  _out_pkt_1_s466 = output_3_1_s83;
  _out_state_group_0_state_0_s467 = state_operand_salu_3_0_state_0_s475;
  _out_state_group_1_state_0_s468 = state_operand_salu_1_1_state_0_s472;
  return;
}
void program(int state_and_packet_pkt_0_s453, int state_and_packet_pkt_1_s454, int state_and_packet_state_group_0_state_0_s455, int state_and_packet_state_group_1_state_0_s456, int& _out_pkt_0_s457, int& _out_pkt_1_s458, int& _out_state_group_0_state_0_s459, int& _out_state_group_1_state_0_s460) {
  state_and_packet_pkt_1_s454 = state_and_packet_pkt_0_s453 - 10;
  if ((state_and_packet_pkt_1_s454) > (state_and_packet_state_group_1_state_0_s456)) {
    state_and_packet_state_group_0_state_0_s455 = state_and_packet_state_group_0_state_0_s455 - 2;
    state_and_packet_state_group_1_state_0_s456 = state_and_packet_pkt_0_s453;
  }
  _out_pkt_0_s457 = state_and_packet_pkt_0_s453;
  _out_pkt_1_s458 = state_and_packet_pkt_1_s454;
  _out_state_group_0_state_0_s459 = state_and_packet_state_group_0_state_0_s455;
  _out_state_group_1_state_0_s460 = state_and_packet_state_group_1_state_0_s456;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s415/* len = 17 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s415[immediate_operand_hole_local]);
  int  pkt_0_s369=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s369);
  int  pkt_1_s371=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s371);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s369 + pkt_1_s371;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s369 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s369 - pkt_1_s371;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s369 - immediate_operand;
            return;
          } else {
            _out = immediate_operand - pkt_0_s369;
            return;
          }
        }
      }
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s378/* len = 17 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s378[immediate_operand_hole_local]);
  int  pkt_0_s365=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s365);
  int  pkt_1_s367=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s367);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s365 + pkt_1_s367;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s365 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s365 - pkt_1_s367;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s365 - immediate_operand;
            return;
          } else {
            _out = immediate_operand - pkt_0_s365;
            return;
          }
        }
      }
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0(int& state_group_state_0_s514, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s515, int* constant_vector__ANONYMOUS_s428/* len = 17 */) {
  int  old_state_group_state_0_s516=state_group_state_0_s514;
  int  _out_s333=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_0(state_group_state_0_s514, Opt_0, _out_s333);
  int  _out_s335=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_0(const_0, _out_s335, constant_vector__ANONYMOUS_s428);
  int  _out_s337=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_0(pkt_0, pkt_1, _out_s335, Mux3_0, _out_s337);
  int  _out_s339=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_0(_out_s333, _out_s337, rel_op_0, _out_s339);
  int  state_0=0;
  if ((_out_s339) == (1)) {
    int  state_0_s341=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_1(state_group_state_0_s514, Opt_1, state_0_s341);
    int  state_0_s343=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_1(const_1, state_0_s343, constant_vector__ANONYMOUS_s428);
    int  state_0_s345=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_1(pkt_0, pkt_1, state_0_s343, Mux3_1, state_0_s345);
    int  state_0_s347=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_2(const_2, state_0_s347, constant_vector__ANONYMOUS_s428);
    int  state_0_s349=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_2(pkt_0, pkt_1, state_0_s347, Mux3_2, state_0_s349);
    int  state_0_s351=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_0(state_0_s345, state_0_s349, arith_op_0, state_0_s351);
    state_0 = state_0_s341 + state_0_s351;
  } else {
    int  state_0_s353=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_2(state_group_state_0_s514, Opt_2, state_0_s353);
    int  state_0_s355=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_3(const_3, state_0_s355, constant_vector__ANONYMOUS_s428);
    int  state_0_s357=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_3(pkt_0, pkt_1, state_0_s355, Mux3_3, state_0_s357);
    int  state_0_s359=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_4(const_4, state_0_s359, constant_vector__ANONYMOUS_s428);
    int  state_0_s361=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_4(pkt_0, pkt_1, state_0_s359, Mux3_4, state_0_s361);
    int  state_0_s363=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_1(state_0_s357, state_0_s361, arith_op_1, state_0_s363);
    state_0 = state_0_s353 + state_0_s363;
  }
  state_group_state_0_s514 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s515 = old_state_group_state_0_s516;
    return;
  } else {
    _out_state_0_s515 = state_0;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1(int& state_group_state_0_s511, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s512, int* constant_vector__ANONYMOUS_s425/* len = 17 */) {
  int  old_state_group_state_0_s513=state_group_state_0_s511;
  int  _out_s301=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_0(state_group_state_0_s511, Opt_0, _out_s301);
  int  _out_s303=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_0(const_0, _out_s303, constant_vector__ANONYMOUS_s425);
  int  _out_s305=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_0(pkt_0, pkt_1, _out_s303, Mux3_0, _out_s305);
  int  _out_s307=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_0(_out_s301, _out_s305, rel_op_0, _out_s307);
  int  state_0=0;
  if ((_out_s307) == (1)) {
    int  state_0_s309=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_1(state_group_state_0_s511, Opt_1, state_0_s309);
    int  state_0_s311=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_1(const_1, state_0_s311, constant_vector__ANONYMOUS_s425);
    int  state_0_s313=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_1(pkt_0, pkt_1, state_0_s311, Mux3_1, state_0_s313);
    int  state_0_s315=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_2(const_2, state_0_s315, constant_vector__ANONYMOUS_s425);
    int  state_0_s317=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_2(pkt_0, pkt_1, state_0_s315, Mux3_2, state_0_s317);
    int  state_0_s319=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_0(state_0_s313, state_0_s317, arith_op_0, state_0_s319);
    state_0 = state_0_s309 + state_0_s319;
  } else {
    int  state_0_s321=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_2(state_group_state_0_s511, Opt_2, state_0_s321);
    int  state_0_s323=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_3(const_3, state_0_s323, constant_vector__ANONYMOUS_s425);
    int  state_0_s325=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_3(pkt_0, pkt_1, state_0_s323, Mux3_3, state_0_s325);
    int  state_0_s327=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_4(const_4, state_0_s327, constant_vector__ANONYMOUS_s425);
    int  state_0_s329=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_4(pkt_0, pkt_1, state_0_s327, Mux3_4, state_0_s329);
    int  state_0_s331=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_1(state_0_s325, state_0_s329, arith_op_1, state_0_s331);
    state_0 = state_0_s321 + state_0_s331;
  }
  state_group_state_0_s511 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s512 = old_state_group_state_0_s513;
    return;
  } else {
    _out_state_0_s512 = state_0;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_0(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_1(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s376/* len = 17 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s376[immediate_operand_hole_local]);
  int  pkt_0_s297=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s297);
  int  pkt_1_s299=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s299);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s297 + pkt_1_s299;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s297 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s297 - pkt_1_s299;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s297 - immediate_operand;
            return;
          } else {
            _out = immediate_operand - pkt_0_s297;
            return;
          }
        }
      }
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s419/* len = 17 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s419[immediate_operand_hole_local]);
  int  pkt_0_s293=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s293);
  int  pkt_1_s295=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s295);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s293 + pkt_1_s295;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s293 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s293 - pkt_1_s295;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s293 - immediate_operand;
            return;
          } else {
            _out = immediate_operand - pkt_0_s293;
            return;
          }
        }
      }
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0(int& state_group_state_0_s508, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s509, int* constant_vector__ANONYMOUS_s426/* len = 17 */) {
  int  old_state_group_state_0_s510=state_group_state_0_s508;
  int  _out_s261=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_0(state_group_state_0_s508, Opt_0, _out_s261);
  int  _out_s263=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_0(const_0, _out_s263, constant_vector__ANONYMOUS_s426);
  int  _out_s265=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_0(pkt_0, pkt_1, _out_s263, Mux3_0, _out_s265);
  int  _out_s267=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_0(_out_s261, _out_s265, rel_op_0, _out_s267);
  int  state_0=0;
  if ((_out_s267) == (1)) {
    int  state_0_s269=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_1(state_group_state_0_s508, Opt_1, state_0_s269);
    int  state_0_s271=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_1(const_1, state_0_s271, constant_vector__ANONYMOUS_s426);
    int  state_0_s273=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_1(pkt_0, pkt_1, state_0_s271, Mux3_1, state_0_s273);
    int  state_0_s275=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_2(const_2, state_0_s275, constant_vector__ANONYMOUS_s426);
    int  state_0_s277=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_2(pkt_0, pkt_1, state_0_s275, Mux3_2, state_0_s277);
    int  state_0_s279=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_0(state_0_s273, state_0_s277, arith_op_0, state_0_s279);
    state_0 = state_0_s269 + state_0_s279;
  } else {
    int  state_0_s281=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_2(state_group_state_0_s508, Opt_2, state_0_s281);
    int  state_0_s283=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_3(const_3, state_0_s283, constant_vector__ANONYMOUS_s426);
    int  state_0_s285=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_3(pkt_0, pkt_1, state_0_s283, Mux3_3, state_0_s285);
    int  state_0_s287=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_4(const_4, state_0_s287, constant_vector__ANONYMOUS_s426);
    int  state_0_s289=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_4(pkt_0, pkt_1, state_0_s287, Mux3_4, state_0_s289);
    int  state_0_s291=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_1(state_0_s285, state_0_s289, arith_op_1, state_0_s291);
    state_0 = state_0_s281 + state_0_s291;
  }
  state_group_state_0_s508 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s509 = old_state_group_state_0_s510;
    return;
  } else {
    _out_state_0_s509 = state_0;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1(int& state_group_state_0_s505, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s506, int* constant_vector__ANONYMOUS_s427/* len = 17 */) {
  int  old_state_group_state_0_s507=state_group_state_0_s505;
  int  _out_s229=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_0(state_group_state_0_s505, Opt_0, _out_s229);
  int  _out_s231=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_0(const_0, _out_s231, constant_vector__ANONYMOUS_s427);
  int  _out_s233=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_0(pkt_0, pkt_1, _out_s231, Mux3_0, _out_s233);
  int  _out_s235=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_0(_out_s229, _out_s233, rel_op_0, _out_s235);
  int  state_0=0;
  if ((_out_s235) == (1)) {
    int  state_0_s237=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_1(state_group_state_0_s505, Opt_1, state_0_s237);
    int  state_0_s239=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_1(const_1, state_0_s239, constant_vector__ANONYMOUS_s427);
    int  state_0_s241=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_1(pkt_0, pkt_1, state_0_s239, Mux3_1, state_0_s241);
    int  state_0_s243=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_2(const_2, state_0_s243, constant_vector__ANONYMOUS_s427);
    int  state_0_s245=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_2(pkt_0, pkt_1, state_0_s243, Mux3_2, state_0_s245);
    int  state_0_s247=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_0(state_0_s241, state_0_s245, arith_op_0, state_0_s247);
    state_0 = state_0_s237 + state_0_s247;
  } else {
    int  state_0_s249=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_2(state_group_state_0_s505, Opt_2, state_0_s249);
    int  state_0_s251=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_3(const_3, state_0_s251, constant_vector__ANONYMOUS_s427);
    int  state_0_s253=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_3(pkt_0, pkt_1, state_0_s251, Mux3_3, state_0_s253);
    int  state_0_s255=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_4(const_4, state_0_s255, constant_vector__ANONYMOUS_s427);
    int  state_0_s257=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_4(pkt_0, pkt_1, state_0_s255, Mux3_4, state_0_s257);
    int  state_0_s259=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_1(state_0_s253, state_0_s257, arith_op_1, state_0_s259);
    state_0 = state_0_s249 + state_0_s259;
  }
  state_group_state_0_s505 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s506 = old_state_group_state_0_s507;
    return;
  } else {
    _out_state_0_s506 = state_0;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_0(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_1(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s390/* len = 17 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s390[immediate_operand_hole_local]);
  int  pkt_0_s225=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s225);
  int  pkt_1_s227=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s227);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s225 + pkt_1_s227;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s225 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s225 - pkt_1_s227;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s225 - immediate_operand;
            return;
          } else {
            _out = immediate_operand - pkt_0_s225;
            return;
          }
        }
      }
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s389/* len = 17 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s389[immediate_operand_hole_local]);
  int  pkt_0_s221=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s221);
  int  pkt_1_s223=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s223);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s221 + pkt_1_s223;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s221 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s221 - pkt_1_s223;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s221 - immediate_operand;
            return;
          } else {
            _out = immediate_operand - pkt_0_s221;
            return;
          }
        }
      }
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0(int& state_group_state_0_s502, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s503, int* constant_vector__ANONYMOUS_s429/* len = 17 */) {
  int  old_state_group_state_0_s504=state_group_state_0_s502;
  int  _out_s189=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_0(state_group_state_0_s502, Opt_0, _out_s189);
  int  _out_s191=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_0(const_0, _out_s191, constant_vector__ANONYMOUS_s429);
  int  _out_s193=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_0(pkt_0, pkt_1, _out_s191, Mux3_0, _out_s193);
  int  _out_s195=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_0(_out_s189, _out_s193, rel_op_0, _out_s195);
  int  state_0=0;
  if ((_out_s195) == (1)) {
    int  state_0_s197=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_1(state_group_state_0_s502, Opt_1, state_0_s197);
    int  state_0_s199=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_1(const_1, state_0_s199, constant_vector__ANONYMOUS_s429);
    int  state_0_s201=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_1(pkt_0, pkt_1, state_0_s199, Mux3_1, state_0_s201);
    int  state_0_s203=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_2(const_2, state_0_s203, constant_vector__ANONYMOUS_s429);
    int  state_0_s205=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_2(pkt_0, pkt_1, state_0_s203, Mux3_2, state_0_s205);
    int  state_0_s207=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_0(state_0_s201, state_0_s205, arith_op_0, state_0_s207);
    state_0 = state_0_s197 + state_0_s207;
  } else {
    int  state_0_s209=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_2(state_group_state_0_s502, Opt_2, state_0_s209);
    int  state_0_s211=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_3(const_3, state_0_s211, constant_vector__ANONYMOUS_s429);
    int  state_0_s213=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_3(pkt_0, pkt_1, state_0_s211, Mux3_3, state_0_s213);
    int  state_0_s215=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_4(const_4, state_0_s215, constant_vector__ANONYMOUS_s429);
    int  state_0_s217=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_4(pkt_0, pkt_1, state_0_s215, Mux3_4, state_0_s217);
    int  state_0_s219=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_1(state_0_s213, state_0_s217, arith_op_1, state_0_s219);
    state_0 = state_0_s209 + state_0_s219;
  }
  state_group_state_0_s502 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s503 = old_state_group_state_0_s504;
    return;
  } else {
    _out_state_0_s503 = state_0;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1(int& state_group_state_0_s499, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s500, int* constant_vector__ANONYMOUS_s424/* len = 17 */) {
  int  old_state_group_state_0_s501=state_group_state_0_s499;
  int  _out_s157=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_0(state_group_state_0_s499, Opt_0, _out_s157);
  int  _out_s159=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_0(const_0, _out_s159, constant_vector__ANONYMOUS_s424);
  int  _out_s161=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_0(pkt_0, pkt_1, _out_s159, Mux3_0, _out_s161);
  int  _out_s163=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_0(_out_s157, _out_s161, rel_op_0, _out_s163);
  int  state_0=0;
  if ((_out_s163) == (1)) {
    int  state_0_s165=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_1(state_group_state_0_s499, Opt_1, state_0_s165);
    int  state_0_s167=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_1(const_1, state_0_s167, constant_vector__ANONYMOUS_s424);
    int  state_0_s169=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_1(pkt_0, pkt_1, state_0_s167, Mux3_1, state_0_s169);
    int  state_0_s171=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_2(const_2, state_0_s171, constant_vector__ANONYMOUS_s424);
    int  state_0_s173=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_2(pkt_0, pkt_1, state_0_s171, Mux3_2, state_0_s173);
    int  state_0_s175=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_0(state_0_s169, state_0_s173, arith_op_0, state_0_s175);
    state_0 = state_0_s165 + state_0_s175;
  } else {
    int  state_0_s177=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_2(state_group_state_0_s499, Opt_2, state_0_s177);
    int  state_0_s179=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_3(const_3, state_0_s179, constant_vector__ANONYMOUS_s424);
    int  state_0_s181=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_3(pkt_0, pkt_1, state_0_s179, Mux3_3, state_0_s181);
    int  state_0_s183=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_4(const_4, state_0_s183, constant_vector__ANONYMOUS_s424);
    int  state_0_s185=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_4(pkt_0, pkt_1, state_0_s183, Mux3_4, state_0_s185);
    int  state_0_s187=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_1(state_0_s181, state_0_s185, arith_op_1, state_0_s187);
    state_0 = state_0_s177 + state_0_s187;
  }
  state_group_state_0_s499 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s500 = old_state_group_state_0_s501;
    return;
  } else {
    _out_state_0_s500 = state_0;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_0(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_1(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s388/* len = 17 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s388[immediate_operand_hole_local]);
  int  pkt_0_s153=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s153);
  int  pkt_1_s155=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s155);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s153 + pkt_1_s155;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s153 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s153 - pkt_1_s155;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s153 - immediate_operand;
            return;
          } else {
            _out = immediate_operand - pkt_0_s153;
            return;
          }
        }
      }
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s410/* len = 17 */) {
  int  immediate_operand=0;
  immediate_operand = (constant_vector__ANONYMOUS_s410[immediate_operand_hole_local]);
  int  pkt_0_s149=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux1(input0, input1, mux1_ctrl_hole_local, pkt_0_s149);
  int  pkt_1_s151=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux2(input0, input1, mux2_ctrl_hole_local, pkt_1_s151);
  if ((opcode_hole_local) == (0)) {
    _out = immediate_operand;
    return;
  } else {
    if ((opcode_hole_local) == (1)) {
      _out = pkt_0_s149 + pkt_1_s151;
      return;
    } else {
      if ((opcode_hole_local) == (2)) {
        _out = pkt_0_s149 + immediate_operand;
        return;
      } else {
        if ((opcode_hole_local) == (3)) {
          _out = pkt_0_s149 - pkt_1_s151;
          return;
        } else {
          if ((opcode_hole_local) == (4)) {
            _out = pkt_0_s149 - immediate_operand;
            return;
          } else {
            _out = immediate_operand - pkt_0_s149;
            return;
          }
        }
      }
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0(int& state_group_state_0_s496, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s497, int* constant_vector__ANONYMOUS_s423/* len = 17 */) {
  int  old_state_group_state_0_s498=state_group_state_0_s496;
  int  _out_s117=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_0(state_group_state_0_s496, Opt_0, _out_s117);
  int  _out_s119=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_0(const_0, _out_s119, constant_vector__ANONYMOUS_s423);
  int  _out_s121=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_0(pkt_0, pkt_1, _out_s119, Mux3_0, _out_s121);
  int  _out_s123=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_0(_out_s117, _out_s121, rel_op_0, _out_s123);
  int  state_0=0;
  if ((_out_s123) == (1)) {
    int  state_0_s125=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_1(state_group_state_0_s496, Opt_1, state_0_s125);
    int  state_0_s127=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_1(const_1, state_0_s127, constant_vector__ANONYMOUS_s423);
    int  state_0_s129=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_1(pkt_0, pkt_1, state_0_s127, Mux3_1, state_0_s129);
    int  state_0_s131=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_2(const_2, state_0_s131, constant_vector__ANONYMOUS_s423);
    int  state_0_s133=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_2(pkt_0, pkt_1, state_0_s131, Mux3_2, state_0_s133);
    int  state_0_s135=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_0(state_0_s129, state_0_s133, arith_op_0, state_0_s135);
    state_0 = state_0_s125 + state_0_s135;
  } else {
    int  state_0_s137=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_2(state_group_state_0_s496, Opt_2, state_0_s137);
    int  state_0_s139=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_3(const_3, state_0_s139, constant_vector__ANONYMOUS_s423);
    int  state_0_s141=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_3(pkt_0, pkt_1, state_0_s139, Mux3_3, state_0_s141);
    int  state_0_s143=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_4(const_4, state_0_s143, constant_vector__ANONYMOUS_s423);
    int  state_0_s145=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_4(pkt_0, pkt_1, state_0_s143, Mux3_4, state_0_s145);
    int  state_0_s147=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_1(state_0_s141, state_0_s145, arith_op_1, state_0_s147);
    state_0 = state_0_s137 + state_0_s147;
  }
  state_group_state_0_s496 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s497 = old_state_group_state_0_s498;
    return;
  } else {
    _out_state_0_s497 = state_0;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1(int& state_group_state_0_s493, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s494, int* constant_vector__ANONYMOUS_s430/* len = 17 */) {
  int  old_state_group_state_0_s495=state_group_state_0_s493;
  int  _out_s85=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_0(state_group_state_0_s493, Opt_0, _out_s85);
  int  _out_s87=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_0(const_0, _out_s87, constant_vector__ANONYMOUS_s430);
  int  _out_s89=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_0(pkt_0, pkt_1, _out_s87, Mux3_0, _out_s89);
  int  _out_s91=0;
  blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_0(_out_s85, _out_s89, rel_op_0, _out_s91);
  int  state_0=0;
  if ((_out_s91) == (1)) {
    int  state_0_s93=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_1(state_group_state_0_s493, Opt_1, state_0_s93);
    int  state_0_s95=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_1(const_1, state_0_s95, constant_vector__ANONYMOUS_s430);
    int  state_0_s97=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_1(pkt_0, pkt_1, state_0_s95, Mux3_1, state_0_s97);
    int  state_0_s99=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_2(const_2, state_0_s99, constant_vector__ANONYMOUS_s430);
    int  state_0_s101=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_2(pkt_0, pkt_1, state_0_s99, Mux3_2, state_0_s101);
    int  state_0_s103=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_0(state_0_s97, state_0_s101, arith_op_0, state_0_s103);
    state_0 = state_0_s93 + state_0_s103;
  } else {
    int  state_0_s105=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_2(state_group_state_0_s493, Opt_2, state_0_s105);
    int  state_0_s107=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_3(const_3, state_0_s107, constant_vector__ANONYMOUS_s430);
    int  state_0_s109=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_3(pkt_0, pkt_1, state_0_s107, Mux3_3, state_0_s109);
    int  state_0_s111=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_4(const_4, state_0_s111, constant_vector__ANONYMOUS_s430);
    int  state_0_s113=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_4(pkt_0, pkt_1, state_0_s111, Mux3_4, state_0_s113);
    int  state_0_s115=0;
    blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_1(state_0_s109, state_0_s113, arith_op_1, state_0_s115);
    state_0 = state_0_s105 + state_0_s115;
  }
  state_group_state_0_s493 = state_0;
  if ((output_mux_0) == (1)) {
    _out_state_0_s494 = old_state_group_state_0_s495;
    return;
  } else {
    _out_state_0_s494 = state_0;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_0(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_0_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_0_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_0_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_1(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_1_ctrl_local) == (1)) {
      _out = input1;
      return;
    } else {
      _out = input2;
      return;
    }
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux2_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux2_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s397/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s397[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s403/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s403[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s386/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s386[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_2(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s408/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s408[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_3(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s392/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s392[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_4(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s383/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s383[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s405/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s405[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s416/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s416[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_2(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s391/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s391[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_3(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s396/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s396[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_4(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux2_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux2_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s412/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s412[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s387/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s387[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s374/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s374[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_2(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s404/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s404[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_3(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s373/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s373[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_4(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s395/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s395[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s417/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s417[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s402/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s402[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_2(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s414/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s414[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_3(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s377/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s377[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_4(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux2_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux2_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s401/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s401[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s380/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s380[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s382/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s382[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_2(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s394/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s394[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_3(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s406/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s406[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_4(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s379/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s379[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s385/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s385[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s400/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s400[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_2(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s413/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s413[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_3(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s411/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s411[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_4(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux2_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux1_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux1_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux2_ctrl_local, int& _out) {
  if ((blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux2_ctrl_local) == (0)) {
    _out = input0;
    return;
  } else {
    _out = input1;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s381/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s381[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s399/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s399[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s384/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s384[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_2(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s407/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s407[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_3(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s398/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s398[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_4(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_0(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_0(int const_var, int& _out, int* constant_vector__ANONYMOUS_s418/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s418[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_0(int operand1, int operand2, int opcode, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_1(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_1(int const_var, int& _out, int* constant_vector__ANONYMOUS_s372/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s372[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_2(int const_var, int& _out, int* constant_vector__ANONYMOUS_s375/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s375[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_2(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_0(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_2(int op1, int enable, int& _out) {
  if ((enable) != (0)) {
    _out = 0;
    return;
  }
  _out = op1;
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_3(int const_var, int& _out, int* constant_vector__ANONYMOUS_s409/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s409[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_3(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_4(int const_var, int& _out, int* constant_vector__ANONYMOUS_s393/* len = 17 */) {
  _out = (constant_vector__ANONYMOUS_s393[const_var]);
  return;
}
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_4(int op1, int op2, int op3, int choice, int& _out) {
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
void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_1(int operand1, int operand2, int opcode, int& _out) {
  if ((opcode) == (0)) {
    _out = operand1 + operand2;
    return;
  } else {
    _out = operand1 - operand2;
    return;
  }
}

}
