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

    function void write_mem_0;
        input int value;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.fifo.fifo_ram.mem[addr];
            tmp[mask*32+:32] = value;
            testbench.dut.fifo.fifo_ram.mem[addr] = tmp[0+:32];
        end
    endfunction

    function int read_mem_0;
        input int addr;
        input int mask;
        logic [32-1:0] tmp;
        begin
            assert (mask < 1) else $error("mask out of bounds");
            tmp[0+:32] = 0;
            tmp[0+:32] = testbench.dut.fifo.fifo_ram.mem[addr];
            return tmp[mask*32+:32];
        end
    endfunction

    always_comb begin
        case(opcode)
            32'd0 : out = 32'hdeadbeef;
            32'd1 : begin
                case(id)
                    default : $error("invalid id");
                endcase
            end
            32'd2 : begin
                case(id)
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
