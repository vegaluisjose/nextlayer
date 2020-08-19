module top (
    input  logic          clock,
    input  logic          reset
);

    logic [31:0] cycles;

    always_ff @(posedge clock)
        if (reset)
            cycles <= 0;
        else
            cycles <= cycles + 1'b1;

    fifo # (
        .DEPTH_WIDTH(10),
        .DATA_WIDTH(128)
    ) fifo (
        .clk(clock),
        .rst(reset),
        .wr_data_i('d3),
        .wr_en_i(cycles == 'd5),
        .rd_en_i(1'b0),
        .full_o(),
        .empty_o()
    );

endmodule
