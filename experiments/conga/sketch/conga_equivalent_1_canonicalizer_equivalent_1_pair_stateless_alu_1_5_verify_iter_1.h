#ifndef CONGA_EQUIVALENT_1_CANONICALIZER_EQUIVALENT_1_PAIR_STATELESS_ALU_1_5_VERIFY_ITER_1_H
#define CONGA_EQUIVALENT_1_CANONICALIZER_EQUIVALENT_1_PAIR_STATELESS_ALU_1_5_VERIFY_ITER_1_H

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
  int  state_1;
  StateGroup(){}
  static StateGroup* create(  int  state_0_,   int  state_1_);
  ~StateGroup(){
  }
  void operator delete(void* p){ free(p); }
};
class StateAndPacket{
  public:
  int  pkt_0;
  int  pkt_1;
  int  pkt_2;
  int  pkt_3;
  int  pkt_4;
  int  state_group_0_state_0;
  int  state_group_0_state_1;
  StateAndPacket(){}
  static StateAndPacket* create(  int  pkt_0_,   int  pkt_1_,   int  pkt_2_,   int  pkt_3_,   int  pkt_4_,   int  state_group_0_state_0_,   int  state_group_0_state_1_);
  ~StateAndPacket(){
  }
  void operator delete(void* p){ free(p); }
};
extern void main__Wrapper(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int state_group_0_state_0, int state_group_0_state_1);
extern void main__WrapperNospec(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int state_group_0_state_0, int state_group_0_state_1);
extern void glblInit_constant_vector__ANONYMOUS_s232(int* constant_vector__ANONYMOUS_s231/* len = 4 */);
extern void _main(int pkt_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int state_group_0_state_0, int state_group_0_state_1, int* constant_vector__ANONYMOUS_s229/* len = 4 */);
extern void pipeline(int state_and_packet_pkt_0_s282, int state_and_packet_pkt_1_s283, int state_and_packet_pkt_2_s284, int state_and_packet_pkt_3_s285, int state_and_packet_pkt_4_s286, int state_and_packet_state_group_0_state_0_s287, int state_and_packet_state_group_0_state_1_s288, int& _out_pkt_0_s289, int& _out_pkt_1_s290, int& _out_pkt_2_s291, int& _out_pkt_3_s292, int& _out_pkt_4_s293, int& _out_state_group_0_state_0_s294, int& _out_state_group_0_state_1_s295, int* constant_vector__ANONYMOUS_s227/* len = 4 */);
extern void program(int state_and_packet_pkt_0_s268, int state_and_packet_pkt_1_s269, int state_and_packet_pkt_2_s270, int state_and_packet_pkt_3_s271, int state_and_packet_pkt_4_s272, int state_and_packet_state_group_0_state_0_s273, int state_and_packet_state_group_0_state_1_s274, int& _out_pkt_0_s275, int& _out_pkt_1_s276, int& _out_pkt_2_s277, int& _out_pkt_3_s278, int& _out_pkt_4_s279, int& _out_state_group_0_state_0_s280, int& _out_state_group_0_state_1_s281);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s208/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s204/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s213/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s222/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4(int input0, int input1, int input2, int input3, int input4, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s210/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_0_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_1_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_2_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_3_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_operand_mux_0_0_4_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0(int& state_group_state_0_s302, int& state_group_state_1_s303, int output_mux_0, int pkt_1, int pkt_2, int pkt_3, int pkt_4, int pkt_5, int Mux3_0, int Mux3_1, int Mux3_10, int Mux3_11, int Mux3_12, int Mux3_13, int Mux3_14, int Mux3_15, int Mux3_16, int Mux3_17, int Mux3_18, int Mux3_19, int Mux3_2, int Mux3_20, int Mux3_21, int Mux3_22, int Mux3_23, int Mux3_24, int Mux3_25, int Mux3_26, int Mux3_27, int Mux3_3, int Mux3_4, int Mux3_5, int Mux3_6, int Mux3_7, int Mux3_8, int Mux3_9, int Opt_0, int Opt_1, int Opt_2, int Opt_3, int Opt_4, int Opt_5, int Opt_6, int Opt_7, int arith_op_0, int arith_op_1, int arith_op_2, int arith_op_3, int arith_op_4, int arith_op_5, int arith_op_6, int arith_op_7, int const_0, int const_1, int const_10, int const_11, int const_12, int const_13, int const_14, int const_15, int const_16, int const_17, int const_18, int const_19, int const_2, int const_3, int const_4, int const_5, int const_6, int const_7, int const_8, int const_9, int output_mux, int rel_op_0, int rel_op_1, int rel_op_2, int rel_op_3, int& _out_state_0_s304, int& _out_state_1_s305, int* constant_vector__ANONYMOUS_s228/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_0(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_0_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_1(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_1_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_2(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_2_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_3(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_3_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_4(int input0, int input1, int input2, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_output_mux_phv_0_4_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux1_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux2_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_0_mux3_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux1_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux2_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_1_mux3_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux1_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux2_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_2_mux3_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux1_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux2_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_3_mux3_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux1_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux2_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3(int input0, int input1, int input2, int input3, int input4, int conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateless_alu_0_4_mux3_ctrl_local, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_0(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_1(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_2(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s225/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_3(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_4(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_5(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s207/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_1(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_0(int op1, int enable, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_2(int const, int& _out, int* constant_vector__ANONYMOUS_s214/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_6(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_3(int const, int& _out, int* constant_vector__ANONYMOUS_s215/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_7(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_0(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_1(int op1, int enable, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_4(int const, int& _out, int* constant_vector__ANONYMOUS_s205/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_8(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_5(int const, int& _out, int* constant_vector__ANONYMOUS_s212/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_9(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_1(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_2(int op1, int enable, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_6(int const, int& _out, int* constant_vector__ANONYMOUS_s203/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_10(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_7(int const, int& _out, int* constant_vector__ANONYMOUS_s216/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_11(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_2(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_3(int op1, int enable, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_8(int const, int& _out, int* constant_vector__ANONYMOUS_s226/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_12(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_9(int const, int& _out, int* constant_vector__ANONYMOUS_s206/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_13(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_3(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_14(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_15(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_16(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_10(int const, int& _out, int* constant_vector__ANONYMOUS_s209/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_2(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_17(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_18(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_19(int op1, int op2, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_11(int const, int& _out, int* constant_vector__ANONYMOUS_s219/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_rel_op_3(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_4(int op1, int enable, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_12(int const, int& _out, int* constant_vector__ANONYMOUS_s211/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_20(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_13(int const, int& _out, int* constant_vector__ANONYMOUS_s224/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_21(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_4(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_5(int op1, int enable, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_14(int const, int& _out, int* constant_vector__ANONYMOUS_s221/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_22(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_15(int const, int& _out, int* constant_vector__ANONYMOUS_s220/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_23(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_5(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_6(int op1, int enable, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_16(int const, int& _out, int* constant_vector__ANONYMOUS_s217/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_24(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_17(int const, int& _out, int* constant_vector__ANONYMOUS_s218/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_25(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_6(int operand1, int operand2, int opcode, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Opt_7(int op1, int enable, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_18(int const, int& _out, int* constant_vector__ANONYMOUS_s202/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_26(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_C_19(int const, int& _out, int* constant_vector__ANONYMOUS_s223/* len = 4 */);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_Mux3_27(int op1, int op2, int op3, int choice, int& _out);
extern void conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_stateful_alu_0_0_arith_op_7(int operand1, int operand2, int opcode, int& _out);
}

#endif
