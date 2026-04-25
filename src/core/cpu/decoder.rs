use super::CPU;

use super::instructions::arithmetic_16::*;
use super::instructions::arithmetic_8::*;
use super::instructions::bit::*;
use super::instructions::jumps_calls::*;
use super::instructions::load::*;
use super::instructions::logic_8::*;
use super::instructions::misc_control::*;
use super::instructions::shift::*;
use super::instructions::stack::*;



pub fn op_invalid(_cpu: &mut CPU) { }

type OpFn = fn(&mut CPU);

pub static DISPATCH: [OpFn; 256] = [
/* 00 */ op_nop,       op_ld_bc_n16, op_ld_bc_a,  op_inc_bc, op_inc_b,      op_dec_b,      op_ld_b_n8,  op_rlca,
/* 08 */ op_ld_n16_sp, op_add_hl_bc, op_ld_a_bc,  op_dec_bc, op_inc_c,      op_dec_c,      op_ld_c_n8,  op_rrca,
/* 10 */ op_stop,      op_ld_de_n16, op_ld_de_a,  op_inc_de, op_inc_d,      op_dec_d,      op_ld_d_n8,  op_rla,
/* 18 */ op_jr_e8,     op_add_hl_de, op_ld_a_de,  op_dec_de, op_inc_e,      op_dec_e,      op_ld_e_n8,  op_rra,
/* 20 */ op_jr_nz_e8,  op_ld_hl_n16, op_ld_hli_a, op_inc_hl, op_inc_h,      op_dec_h,      op_ld_h_n8,  op_daa,
/* 28 */ op_jr_z_e8,   op_add_hl_hl, op_ld_a_hli, op_dec_hl, op_inc_l,      op_dec_l,      op_ld_l_n8,  op_cpl,
/* 30 */ op_jr_nc_e8,  op_ld_sp_n16, op_ld_hld_a, op_inc_sp, op_inc_hl_ptr, op_dec_hl_ptr, op_ld_hl_n8, op_scf,
/* 38 */ op_jr_c_e8,   op_add_hl_sp, op_ld_a_hld, op_dec_sp, op_inc_a,      op_dec_a,      op_ld_a_n8,  op_ccf,

/* 40 */ op_ld_b_b,       op_ld_b_c, op_ld_b_d, op_ld_b_e, op_ld_b_h, op_ld_b_l, op_ld_b_hl, op_ld_b_a,
/* 48 */ op_ld_c_b,       op_ld_c_c, op_ld_c_d, op_ld_c_e, op_ld_c_h, op_ld_c_l, op_ld_c_hl, op_ld_c_a,
/* 50 */ op_ld_d_b,       op_ld_d_c, op_ld_d_d, op_ld_d_e, op_ld_d_h, op_ld_d_l, op_ld_d_hl, op_ld_d_a,
/* 58 */ op_ld_e_b,       op_ld_e_c, op_ld_e_d, op_ld_e_e, op_ld_e_h, op_ld_e_l, op_ld_e_hl, op_ld_e_a,
/* 60 */ op_ld_h_b,       op_ld_h_c, op_ld_h_d, op_ld_h_e, op_ld_h_h, op_ld_h_l, op_ld_h_hl, op_ld_h_a,
/* 68 */ op_ld_l_b,       op_ld_l_c, op_ld_l_d, op_ld_l_e, op_ld_l_h, op_ld_l_l, op_ld_l_hl, op_ld_l_a,
/* 70 */ op_ld_hl_b,     op_ld_hl_c, op_ld_hl_d, op_ld_hl_e, op_ld_hl_h, op_ld_hl_l, op_halt, op_ld_hl_a,
/* 78 */ op_ld_a_b,       op_ld_a_c, op_ld_a_d, op_ld_a_e, op_ld_a_h, op_ld_a_l, op_ld_a_hl, op_ld_a_a,

/* 80 */ op_add_a_b, op_add_a_c, op_add_a_d, op_add_a_e, op_add_a_h, op_add_a_l, op_add_a_hl, op_add_a_a,
/* 88 */ op_adc_a_b, op_adc_a_c, op_adc_a_d, op_adc_a_e, op_adc_a_h, op_adc_a_l, op_adc_a_hl, op_adc_a_a,
/* 90 */ op_sub_a_b, op_sub_a_c, op_sub_a_d, op_sub_a_e, op_sub_a_h, op_sub_a_l, op_sub_a_hl, op_sub_a_a,
/* 98 */ op_sbc_a_b, op_sbc_a_c, op_sbc_a_d, op_sbc_a_e, op_sbc_a_h, op_sbc_a_l, op_sbc_a_hl, op_sbc_a_a,
/* A0 */ op_and_a_b, op_and_a_c, op_and_a_d, op_and_a_e, op_and_a_h, op_and_a_l, op_and_a_hl, op_and_a_a,
/* A8 */ op_xor_a_b, op_xor_a_c, op_xor_a_d, op_xor_a_e, op_xor_a_h, op_xor_a_l, op_xor_a_hl, op_xor_a_a,
/* B0 */ op_or_a_b, op_or_a_c, op_or_a_d, op_or_a_e, op_or_a_h, op_or_a_l, op_or_a_hl, op_or_a_a,
/* B8 */ op_cp_a_b, op_cp_a_c, op_cp_a_d, op_cp_a_e, op_cp_a_h, op_cp_a_l, op_cp_a_hl, op_cp_a_a,

/* C0 */ op_ret_nz, op_pop_bc, op_jp_nz_n16, op_jp_n16, op_call_nz_n16, op_push_bc, op_add_a_n8, op_rst_00,
/* C8 */ op_ret_z, op_ret, op_jp_z_n16, op_cb_prefix, op_call_z_n16, op_call_n16, op_adc_a_n8, op_rst_08,
/* D0 */ op_ret_nc, op_pop_de, op_jp_nc_n16, op_invalid, op_call_nc_n16, op_push_de, op_sub_a_n8, op_rst_10,
/* D8 */ op_ret_c, op_reti, op_jp_c_n16, op_invalid, op_call_c_n16, op_invalid, op_sbc_a_n8, op_rst_18,
/* E0 */ op_ldh_a_n, op_pop_hl, op_ldh_c_a, op_invalid, op_invalid, op_push_hl, op_and_a_n8, op_rst_20,
/* E8 */ op_add_sp_e8, op_jp_hl, op_ld_n16_a, op_invalid, op_invalid, op_invalid, op_xor_a_n8, op_rst_28,
/* F0 */ op_ldh_n_a, op_pop_af, op_ld_a_c, op_di, op_invalid, op_push_af, op_or_a_n8, op_rst_30,
/* F8 */ op_ld_hl_sp_e8, op_ld_sp_hl, op_ld_a_n16, op_ei, op_invalid, op_invalid, op_cp_a_n8, op_rst_38,
];

pub static CB_DISPATCH: [OpFn; 256] = [
/* 00 */ op_rlc_b, op_rlc_c, op_rlc_d, op_rlc_e, op_rlc_h, op_rlc_l, op_rlc_hl, op_rlc_a,
/* 08 */ op_rrc_b, op_rrc_c, op_rrc_d, op_rrc_e, op_rrc_h, op_rrc_l, op_rrc_hl, op_rrc_a,
/* 10 */ op_rl_b, op_rl_c, op_rl_d, op_rl_e, op_rl_h, op_rl_l, op_rl_hl, op_rl_a,
/* 18 */ op_rr_b, op_rr_c, op_rr_d, op_rr_e, op_rr_h, op_rr_l, op_rr_hl, op_rr_a,
/* 20 */ op_sla_b, op_sla_c, op_sla_d, op_sla_e, op_sla_h, op_sla_l, op_sla_hl, op_sla_a,
/* 28 */ op_sra_b, op_sra_c, op_sra_d, op_sra_e, op_sra_h, op_sra_l, op_sra_hl, op_sra_a,
/* 30 */ op_swap_b, op_swap_c, op_swap_d, op_swap_e, op_swap_h, op_swap_l, op_swap_hl, op_swap_a,
/* 38 */ op_srl_b, op_srl_c, op_srl_d, op_srl_e, op_srl_h, op_srl_l, op_srl_hl, op_srl_a,

/* 40 */ op_bit_0_b, op_bit_0_c, op_bit_0_d, op_bit_0_e, op_bit_0_h, op_bit_0_l, op_bit_0_hl, op_bit_0_a,
/* 48 */ op_bit_1_b, op_bit_1_c, op_bit_1_d, op_bit_1_e, op_bit_1_h, op_bit_1_l, op_bit_1_hl, op_bit_1_a,
/* 50 */ op_bit_2_b, op_bit_2_c, op_bit_2_d, op_bit_2_e, op_bit_2_h, op_bit_2_l, op_bit_2_hl, op_bit_2_a,
/* 58 */ op_bit_3_b, op_bit_3_c, op_bit_3_d, op_bit_3_e, op_bit_3_h, op_bit_3_l, op_bit_3_hl, op_bit_3_a,
/* 60 */ op_bit_4_b, op_bit_4_c, op_bit_4_d, op_bit_4_e, op_bit_4_h, op_bit_4_l, op_bit_4_hl, op_bit_4_a,
/* 68 */ op_bit_5_b, op_bit_5_c, op_bit_5_d, op_bit_5_e, op_bit_5_h, op_bit_5_l, op_bit_5_hl, op_bit_5_a,
/* 70 */ op_bit_6_b, op_bit_6_c, op_bit_6_d, op_bit_6_e, op_bit_6_h, op_bit_6_l, op_bit_6_hl, op_bit_6_a,
/* 78 */ op_bit_7_b, op_bit_7_c, op_bit_7_d, op_bit_7_e, op_bit_7_h, op_bit_7_l, op_bit_7_hl, op_bit_7_a,

/* 80 */ op_res_0_b, op_res_0_c, op_res_0_d, op_res_0_e, op_res_0_h, op_res_0_l, op_res_0_hl, op_res_0_a,
/* 88 */ op_res_1_b, op_res_1_c, op_res_1_d, op_res_1_e, op_res_1_h, op_res_1_l, op_res_1_hl, op_res_1_a,
/* 90 */ op_res_2_b, op_res_2_c, op_res_2_d, op_res_2_e, op_res_2_h, op_res_2_l, op_res_2_hl, op_res_2_a,
/* 98 */ op_res_3_b, op_res_3_c, op_res_3_d, op_res_3_e, op_res_3_h, op_res_3_l, op_res_3_hl, op_res_3_a,
/* A0 */ op_res_4_b, op_res_4_c, op_res_4_d, op_res_4_e, op_res_4_h, op_res_4_l, op_res_4_hl, op_res_4_a,
/* A8 */ op_res_5_b, op_res_5_c, op_res_5_d, op_res_5_e, op_res_5_h, op_res_5_l, op_res_5_hl, op_res_5_a,
/* B0 */ op_res_6_b, op_res_6_c, op_res_6_d, op_res_6_e, op_res_6_h, op_res_6_l, op_res_6_hl, op_res_6_a,
/* B8 */ op_res_7_b, op_res_7_c, op_res_7_d, op_res_7_e, op_res_7_h, op_res_7_l, op_res_7_hl, op_res_7_a,

/* C0 */ op_set_0_b, op_set_0_c, op_set_0_d, op_set_0_e, op_set_0_h, op_set_0_l, op_set_0_hl, op_set_0_a,
/* C8 */ op_set_1_b, op_set_1_c, op_set_1_d, op_set_1_e, op_set_1_h, op_set_1_l, op_set_1_hl, op_set_1_a,
/* D0 */ op_set_2_b, op_set_2_c, op_set_2_d, op_set_2_e, op_set_2_h, op_set_2_l, op_set_2_hl, op_set_2_a,
/* D8 */ op_set_3_b, op_set_3_c, op_set_3_d, op_set_3_e, op_set_3_h, op_set_3_l, op_set_3_hl, op_set_3_a,
/* E0 */ op_set_4_b, op_set_4_c, op_set_4_d, op_set_4_e, op_set_4_h, op_set_4_l, op_set_4_hl, op_set_4_a,
/* E8 */ op_set_5_b, op_set_5_c, op_set_5_d, op_set_5_e, op_set_5_h, op_set_5_l, op_set_5_hl, op_set_5_a,
/* F0 */ op_set_6_b, op_set_6_c, op_set_6_d, op_set_6_e, op_set_6_h, op_set_6_l, op_set_6_hl, op_set_6_a,
/* F8 */ op_set_7_b, op_set_7_c, op_set_7_d, op_set_7_e, op_set_7_h, op_set_7_l, op_set_7_hl, op_set_7_a,
];