// this adder take one operand from a register(a)
// and another from memory(mem). These two storage
// resources are accessed from external langs through
// FFI.
module adder(input clock, input reset);

    wire [7:0] b;

    reg [7:0] a;
    reg [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            a <= 0;
            y <= 0;
        end
        else begin
            y <= a + b;
        end
    end

    reg [32-1:0] mem [256-1:0];

    assign b = mem[4];

endmodule
