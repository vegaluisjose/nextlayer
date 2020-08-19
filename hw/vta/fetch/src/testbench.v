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
            tmp[0+:32] = testbench.dut.fetch.fetch_CONTROL_BUS_s_axi_U.int_insns_V;
            tmp[mask*32+:32] = value;
            testbench.dut.fetch.fetch_CONTROL_BUS_s_axi_U.int_insns_V = tmp[0+:32];
        end
    endfunction

    function int read_reg_0;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.fetch.fetch_CONTROL_BUS_s_axi_U.int_insns_V;
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
            tmp[0+:32] = testbench.dut.imem.mem[addr];
            tmp[mask*32+:32] = value;
            testbench.dut.imem.mem[addr] = tmp[0+:32];
        end
    endfunction

    function int read_mem_0;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.imem.mem[addr];
            return tmp[mask*32+:32];
        end
    endfunction

    function void write_mem_1;
        input int value;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.load_queue.fifo_ram.mem[addr];
            tmp[mask*32+:32] = value;
            testbench.dut.load_queue.fifo_ram.mem[addr] = tmp[0+:32];
        end
    endfunction

    function int read_mem_1;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.load_queue.fifo_ram.mem[addr];
            return tmp[mask*32+:32];
        end
    endfunction

    function void write_mem_2;
        input int value;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.gemm_queue.fifo_ram.mem[addr];
            tmp[mask*32+:32] = value;
            testbench.dut.gemm_queue.fifo_ram.mem[addr] = tmp[0+:32];
        end
    endfunction

    function int read_mem_2;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.gemm_queue.fifo_ram.mem[addr];
            return tmp[mask*32+:32];
        end
    endfunction

    function void write_mem_3;
        input int value;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.store_queue.fifo_ram.mem[addr];
            tmp[mask*32+:32] = value;
            testbench.dut.store_queue.fifo_ram.mem[addr] = tmp[0+:32];
        end
    endfunction

    function int read_mem_3;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.store_queue.fifo_ram.mem[addr];
            return tmp[mask*32+:32];
        end
    endfunction

    always_comb begin
        case(opcode)
            32'd0 : out = 32'hdeadbeef;
            32'd1 : begin
                case(id)
                    32'd0 : write_reg_0(in, mask);
                    default : $error("invalid id");
                endcase
            end
            32'd2 : begin
                case(id)
                    32'd0 : out = read_reg_0(mask);
                    default : $error("invalid id");
                endcase
            end
            32'd3 : begin
                case(id)
                    32'd0 : write_mem_0(in, addr, mask);
                    32'd1 : write_mem_1(in, addr, mask);
                    32'd2 : write_mem_2(in, addr, mask);
                    32'd3 : write_mem_3(in, addr, mask);
                    default : $error("invalid id");
                endcase
            end
            32'd4 : begin
                case(id)
                    32'd0 : out = read_mem_0(addr, mask);
                    32'd1 : out = read_mem_1(addr, mask);
                    32'd2 : out = read_mem_2(addr, mask);
                    32'd3 : out = read_mem_3(addr, mask);
                    default : $error("invalid id");
                endcase
            end
            default : $error("invalid opcode");
        endcase
    end

    top dut (.clock(clock), .reset(reset));

endmodule
