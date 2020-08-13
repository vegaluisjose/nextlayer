// opcode
// 0 - nop
// 1 - write reg
// 2 - read reg
// 3 - write mem
// 4 - read mem

// all of this will be generated

module testbench (
    input  logic          clock,
    input  logic          reset,
    input  logic [32-1:0] opcode,
    input  logic [32-1:0] id,
    input  logic [32-1:0] mask,
    input  logic [32-1:0] in,
    input  logic [32-1:0] addr,
    output logic [32-1:0] out
);

    function void write_reg_0;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:1] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start;
            tmp[mask*32+:32] = value;
            testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start = tmp[0+:1];
        end
    endfunction

    function int read_reg_0;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:1] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start;
            return tmp[mask*32+:32];
        end
    endfunction
    
    function void write_reg_1;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:1] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_done;
            tmp[mask*32+:32] = value;
            testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_done = tmp[0+:1];
        end
    endfunction

    function int read_reg_1;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:1] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_done;
            return tmp[mask*32+:32];
        end
    endfunction
    
    function void write_reg_2;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [64-1:0] tmp;
        begin
            assert (mask < 2) else $error("mask out of bounds");
            tmp[0+:64] = 0;
            tmp[0+:64] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_a;
            tmp[mask*32+:32] = value;
            testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_a = tmp[0+:64];
        end
    endfunction

    function int read_reg_2;
        input int mask;
        logic [64-1:0] tmp;
        begin
            assert (mask < 2) else $error("mask out of bounds");
            tmp[0+:64] = 0;
            tmp[0+:64] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_a;
            return tmp[mask*32+:32];
        end
    endfunction
    
    function void write_reg_3;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [64-1:0] tmp;
        begin
            assert (mask < 2) else $error("mask out of bounds");
            tmp[0+:64] = 0;
            tmp[0+:64] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_b;
            tmp[mask*32+:32] = value;
            testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_b = tmp[0+:64];
        end
    endfunction

    function int read_reg_3;
        input int mask;
        logic [64-1:0] tmp;
        begin
            assert (mask < 2) else $error("mask out of bounds");
            tmp[0+:64] = 0;
            tmp[0+:64] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_b;
            return tmp[mask*32+:32];
        end
    endfunction
    
    function void write_reg_4;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [64-1:0] tmp;
        begin
            assert (mask < 2) else $error("mask out of bounds");
            tmp[0+:64] = 0;
            tmp[0+:64] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_c;
            tmp[mask*32+:32] = value;
            testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_c = tmp[0+:64];
        end
    endfunction

    function int read_reg_4;
        input int mask;
        logic [64-1:0] tmp;
        begin
            assert (mask < 2) else $error("mask out of bounds");
            tmp[0+:64] = 0;
            tmp[0+:64] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_c;
            return tmp[mask*32+:32];
        end
    endfunction
    
    function void write_reg_5;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_length_r;
            tmp[mask*32+:32] = value;
            testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_length_r = tmp[0+:32];
        end
    endfunction

    function int read_reg_5;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_length_r;
            return tmp[mask*32+:32];
        end
    endfunction

    function void write_mem_0;
        input int value;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.ram.mem[addr];
            tmp[mask*32+:32] = value;
            testbench.dut.ram.mem[addr] = tmp[0+:32];
        end
    endfunction

    function int read_mem_0;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.ram.mem[addr];
            return tmp[mask*32+:32];
        end
    endfunction

    always_comb begin
        case(opcode)
            32'd0 : out = 32'hdeadbeef;
            32'd1 : begin
                case(id)
                    32'd0 : write_reg_0(in, mask);
                    32'd1 : write_reg_1(in, mask);
                    32'd2 : write_reg_2(in, mask);
                    32'd3 : write_reg_3(in, mask);
                    32'd4 : write_reg_4(in, mask);
                    32'd5 : write_reg_5(in, mask);
                    default : $error("invalid id");
                endcase
            end
            32'd2 : begin
                case(id)
                    32'd0 : out = read_reg_0(mask);
                    32'd1 : out = read_reg_1(mask);
                    32'd2 : out = read_reg_2(mask);
                    32'd3 : out = read_reg_3(mask);
                    32'd4 : out = read_reg_4(mask);
                    32'd5 : out = read_reg_5(mask);
                    default : $error("invalid id");
                endcase
            end
            32'd3 : begin
                case(id)
                    32'd0 : write_mem_0(in, addr, mask);
                    default : $error("invalid id");
                endcase
            end
            32'd4 : begin
                case(id)
                    32'd0 : out = read_mem_0(addr, mask);
                    default : $error("invalid id");
                endcase
            end
            default : $error("invalid opcode");
        endcase
    end

    top dut (.clock(clock), .reset(reset));

endmodule
