module testbench (
   input         clock,
   input         reset,
   output [31:0] count,
   input         update,
   input [31:0]  value
);

    function void modify;
        input logic [31:0] value;
        logic [31:0] tmp;
        begin
            tmp = 0;
            tmp = testbench.cnt;
            testbench.cnt = value;
        end
    endfunction

    logic [31:0] cnt;

    always_ff @(posedge clock) begin
        if (reset) begin
            cnt <= 32'd0;
        end
        else begin
            cnt <= cnt + 1'b1;
        end
    end

    assign count = cnt;

    always_comb begin
        if (update) begin
            modify(value);
        end
    end

endmodule
