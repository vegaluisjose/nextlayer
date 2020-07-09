// opcode
// 0 - nop
// 1 - write reg
// 2 - read reg
// 3 - write mem
// 4 - read mem

// all of this will be generated

module testbench (
    input  logic        clock,
    input  logic        reset,
    input  logic [31:0] opcode,
    input  logic [31:0] id,
    input  logic [31:0] mask,
    input  logic [31:0] in,
    output logic [31:0] out
);

    function void write_reg_a;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [31:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:8] = testbench.dut.a;
            tmp[mask*32+:32] = value;
            testbench.dut.a = tmp[0+:8];
        end
    endfunction

    function int read_reg_a;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:8] = testbench.dut.a;
            return tmp[mask*32+:32];
        end
    endfunction

    function void write_reg_b;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [31:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:8] = testbench.dut.b;
            tmp[mask*32+:32] = value;
            testbench.dut.b = tmp[0+:8];
        end
    endfunction

    function int read_reg_b;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:8] = testbench.dut.b;
            return tmp[mask*32+:32];
        end
    endfunction

    function void write_reg_y;
        input logic [31:0] value;
        input logic [31:0] mask;
        logic [31:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:8] = testbench.dut.y;
            tmp[mask*32+:32] = value;
            testbench.dut.b = tmp[0+:8];
        end
    endfunction

    function int read_reg_y;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:8] = testbench.dut.y;
            return tmp[mask*32+:32];
        end
    endfunction

    always_comb begin
        case(opcode)
            32'd0 : out = 32'hdeadbeef;
            32'd1 : begin
                case(id)
                    32'd0 : write_reg_a(in, mask);
                    32'd1 : write_reg_b(in, mask);
                    32'd2 : write_reg_y(in, mask);
                    default : $error("invalid id");
                endcase
            end
            32'd2 : begin
                case(id)
                    32'd0 : out = read_reg_a(mask);
                    32'd1 : out = read_reg_b(mask);
                    32'd2 : out = read_reg_y(mask);
                    default : $error("invalid id");
                endcase
            end
            default : $error("invalid opcode");
        endcase
    end

    adder dut (.clock(clock), .reset(reset));

endmodule
