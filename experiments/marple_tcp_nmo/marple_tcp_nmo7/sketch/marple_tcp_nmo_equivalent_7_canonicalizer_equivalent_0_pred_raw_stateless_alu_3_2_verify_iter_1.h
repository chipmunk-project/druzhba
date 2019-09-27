#ifndef MARPLE_TCP_NMO_EQUIVALENT_7_CANONICALIZER_EQUIVALENT_0_PRED_RAW_STATELESS_ALU_3_2_VERIFY_ITER_1_H
#define MARPLE_TCP_NMO_EQUIVALENT_7_CANONICALIZER_EQUIVALENT_0_PRED_RAW_STATELESS_ALU_3_2_VERIFY_ITER_1_H

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
  int  state_group_0_state_0;
  int  state_group_1_state_0;
  StateAndPacket(){}
  static StateAndPacket* create(  int  pkt_0_,   int  state_group_0_state_0_,   int  state_group_1_state_0_);
  ~StateAndPacket(){
  }
  void operator delete(void* p){ free(p); }
};
extern void main__Wrapper(int pkt_0, int state_group_0_state_0, int state_group_1_state_0);
extern void main__WrapperNospec(int pkt_0, int state_group_0_state_0, int state_group_1_state_0);
extern void glblInit_constant_vector__ANONYMOUS_s212(int* constant_vector__ANONYMOUS_s211/* len = 4 */);
extern void _main(int pkt_0, int state_group_0_state_0, int state_group_1_state_0, int* constant_vector__ANONYMOUS_s204/* len = 4 */);
extern void pipeline(int state_and_packet_pkt_0_s234, int state_and_packet_state_group_0_state_0_s235, int state_and_packet_state_group_1_state_0_s236, int& _out_pkt_0_s237, int& _out_state_group_0_state_0_s238, int& _out_state_group_1_state_0_s239, int* constant_vector__ANONYMOUS_s203/* len = 4 */);
extern void program(int state_and_packet_pkt_0_s228, int state_and_packet_state_group_0_state_0_s229, int state_and_packet_state_group_1_state_0_s230, int& _out_pkt_0_s231, int& _out_state_group_0_state_0_s232, int& _out_state_group_1_state_0_s233);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s189/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s184/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_0_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_0_1_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0(int& state_group_state_0_s273, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s274, int* constant_vector__ANONYMOUS_s207/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1(int& state_group_state_0_s270, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s271, int* constant_vector__ANONYMOUS_s210/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_0(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_1(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_0_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s197/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s193/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_0_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_1_1_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0(int& state_group_state_0_s267, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s268, int* constant_vector__ANONYMOUS_s208/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1(int& state_group_state_0_s264, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s265, int* constant_vector__ANONYMOUS_s206/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_0(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_1(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_1_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s200/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1(int input0, int input1, int opcode_hole_local, int immediate_operand_hole_local, int mux1_ctrl_hole_local, int mux2_ctrl_hole_local, int mux3_ctrl_hole_local, int& _out, int* constant_vector__ANONYMOUS_s186/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_0_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_0(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_operand_mux_2_1_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0(int& state_group_state_0_s261, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s262, int* constant_vector__ANONYMOUS_s205/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1(int& state_group_state_0_s258, int output_mux_0, int pkt_0, int pkt_1, int Mux3_0, int Mux3_1, int Opt_0, int Opt_1, int const_0, int const_1, int output_mux, int rel_op_0, int& _out_state_0_s259, int* constant_vector__ANONYMOUS_s209/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_0(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_0_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_1(int input0, int input1, int input2, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_output_mux_phv_2_1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux2_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_0_mux3_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux2_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_0_1_mux3_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Opt_0(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s194/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Opt_1(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s199/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Opt_0(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s190/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Opt_1(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s198/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_0_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux2_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_0_mux3_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux2_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_1_1_mux3_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Opt_0(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s188/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Opt_1(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s192/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Opt_0(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s201/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Opt_1(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s185/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_1_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux2_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_0_mux3_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux1(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux1_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux2(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux2_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux3(int input0, int input1, int marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateless_alu_2_1_mux3_ctrl_local, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Opt_0(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s187/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Opt_1(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s191/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_0_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Opt_0(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_C_0(int const, int& _out, int* constant_vector__ANONYMOUS_s195/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Mux3_0(int op1, int op2, int op3, int choice, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_rel_op_0(int operand1, int operand2, int opcode, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Opt_1(int op1, int enable, int& _out);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_C_1(int const, int& _out, int* constant_vector__ANONYMOUS_s196/* len = 4 */);
extern void marple_tcp_nmo_equivalent_7_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_stateful_alu_2_1_Mux3_1(int op1, int op2, int op3, int choice, int& _out);
}

#endif
