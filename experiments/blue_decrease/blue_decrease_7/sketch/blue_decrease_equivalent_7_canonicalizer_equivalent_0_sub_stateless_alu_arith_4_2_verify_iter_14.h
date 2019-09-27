#ifndef BLUE_DECREASE_EQUIVALENT_7_CANONICALIZER_EQUIVALENT_0_SUB_STATELESS_ALU_ARITH_4_2_VERIFY_ITER_14_H
#define BLUE_DECREASE_EQUIVALENT_7_CANONICALIZER_EQUIVALENT_0_SUB_STATELESS_ALU_ARITH_4_2_VERIFY_ITER_14_H

#include <cstring>

#include "vops.h"

namespace ANONYMOUS{
class StateGroup; 
class StateAndPacket; 
}
namespace ANONYMOUS{
class StateGroup; 
class StateAndPacket; 
class StateGroup{
  public:
  int  state_0;
  StateGroup(){}
  static StateGroup* create(  int  state_0_);
  ~StateGroup(){
  }
  void operator delete(void* p){ free(p); }
};
class StateAndPacket{
  public:
  int  pkt_0;
  int  pkt_1;
  int  state_group_0_state_0;
  int  state_group_1_state_0;
  StateAndPacket(){}
  static StateAndPacket* create(  int  pkt_0_,   int  pkt_1_,   int  state_group_0_state_0_,   int  state_group_1_state_0_);
  ~StateAndPacket(){
  }
  void operator delete(void* p){ free(p); }
};
extern void main__Wrapper(int pkt_0, int pkt_1, int state_group_0_state_0, int state_group_1_state_0);
extern void main__WrapperNospec(int pkt_0, int pkt_1, int state_group_0_state_0, int state_group_1_state_0);
extern void glblInit_constant_vector__ANONYMOUS_s432(int* constant_vector__ANONYMOUS_s431/* len = 17 */);
extern void _main(int pkt_0, int pkt_1, int state_group_0_state_0, int state_group_1_state_0, int* constant_vector__ANONYMOUS_s420/* len = 17 */);
extern void pipeline(int state_and_packet_pkt_0_s461, int state_and_packet_pkt_1_s462, int state_and_packet_state_group_0_state_0_s463, int state_and_packet_state_group_1_state_0_s464, int& _out_pkt_0_s465, int& _out_pkt_1_s466, int& _out_state_group_0_state_0_s467, int& _out_state_group_1_state_0_s468, int* constant_vector__ANONYMOUS_s421/* len = 17 */);
extern void program(int state_and_packet_pkt_0_s453, int state_and_packet_pkt_1_s454, int state_and_packet_state_group_0_state_0_s455, int state_and_packet_state_group_1_state_0_s456, int& _out_pkt_0_s457, int& _out_pkt_1_s458, int& _out_state_group_0_state_0_s459, int& _out_state_group_1_state_0_s460);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s415/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s378/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_0_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_0_1_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0(int& state_group_state_0_s514, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s515, int* constant_vector__ANONYMOUS_s428/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1(int& state_group_state_0_s511, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s512, int* constant_vector__ANONYMOUS_s425/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_0(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_1(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_0_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s376/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s419/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_0_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_1_1_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0(int& state_group_state_0_s508, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s509, int* constant_vector__ANONYMOUS_s426/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1(int& state_group_state_0_s505, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s506, int* constant_vector__ANONYMOUS_s427/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_0(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_1(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_1_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s390/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s389/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_0_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_2_1_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0(int& state_group_state_0_s502, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s503, int* constant_vector__ANONYMOUS_s429/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1(int& state_group_state_0_s499, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s500, int* constant_vector__ANONYMOUS_s424/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_0(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_1(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_2_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s388/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s410/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_0_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_0(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_operand_mux_3_1_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0(int& state_group_state_0_s496, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s497, int* constant_vector__ANONYMOUS_s423/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1(int& state_group_state_0_s493, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Mux3_2, int Mux3_3, int Mux3_4, int Opt_0, int Opt_1, int Opt_2, int arith_op_0, int arith_op_1, int const_0, int const_1, int const_2, int const_3, int const_4, int output_mux, int rel_op_0, int& _out_state_0_s494, int* constant_vector__ANONYMOUS_s430/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_0(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_0_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_1(int input0, int input1, int input2, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_output_mux_phv_3_1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_0_mux2_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_0_1_mux2_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_0(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s397/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_1(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s403/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s386/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_2(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Opt_2(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s408/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_3(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s392/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_Mux3_4(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_0_arith_op_1(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_0(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s383/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_1(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s405/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s416/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_2(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Opt_2(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s391/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_3(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s396/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_Mux3_4(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_0_1_arith_op_1(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_0_mux2_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_1_1_mux2_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_0(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s412/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_1(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s387/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s374/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_2(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Opt_2(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s404/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_3(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s373/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_Mux3_4(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_0_arith_op_1(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_0(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s395/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_1(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s417/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s402/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_2(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Opt_2(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s414/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_3(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s377/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_Mux3_4(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_1_1_arith_op_1(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_0_mux2_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_2_1_mux2_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_0(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s401/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_1(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s380/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s382/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_2(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Opt_2(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s394/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_3(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s406/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_Mux3_4(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_0_arith_op_1(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_0(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s379/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_1(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s385/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s400/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_2(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Opt_2(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s413/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_3(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s411/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_Mux3_4(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_2_1_arith_op_1(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_0_mux2_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux1(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux1_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux2(int input0, int input1, int blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateless_alu_3_1_mux2_ctrl_local, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_0(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s381/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_1(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s399/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s384/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_2(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Opt_2(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s407/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_3(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s398/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_Mux3_4(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_0_arith_op_1(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_0(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s418/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_1(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s372/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s375/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_2(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Opt_2(int op1, int enable, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s409/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_3(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s393/* len = 17 */);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_Mux3_4(int op1, int op2, int op3, int choice, int& _out);
extern void blue_decrease_equivalent_7_canonicalizer_equivalent_0_sub_stateless_alu_arith_4_2_stateful_alu_3_1_arith_op_1(int operand1, int operand2, int opcode, int& _out);
}

#endif
