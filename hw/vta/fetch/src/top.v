module top (
    input  logic          clock,
    input  logic          reset
);

    localparam integer AXI_ID_WIDTH = 1;
    localparam integer AXI_ADDR_WIDTH = 32;
    localparam integer AXI_DATA_WIDTH = 128;
    localparam integer AXI_STRB_WIDTH = (AXI_DATA_WIDTH/8);

    wire                       load_valid;
    wire [AXI_DATA_WIDTH-1:0]  load_data;
    wire                       load_full;

    wire                       gemm_valid;
    wire [AXI_DATA_WIDTH-1:0]  gemm_data;
    wire                       gemm_full;

    wire                       store_valid;
    wire [AXI_DATA_WIDTH-1:0]  store_data;
    wire                       store_full;

    wire [AXI_ID_WIDTH-1:0]    axi_awid;
    wire [AXI_ADDR_WIDTH-1:0]  axi_awaddr;
    wire [7:0]                 axi_awlen;
    wire [2:0]                 axi_awsize;
    wire [1:0]                 axi_awburst;
    wire [1:0]                 axi_awlock;
    wire [3:0]                 axi_awcache;
    wire [2:0]                 axi_awprot;
    wire                       axi_awvalid;
    wire                       axi_awready;
    wire [AXI_DATA_WIDTH-1:0]  axi_wdata;
    wire [AXI_STRB_WIDTH-1:0]  axi_wstrb;
    wire                       axi_wlast;
    wire                       axi_wvalid;
    wire                       axi_wready;
    wire [AXI_ID_WIDTH-1:0]    axi_bid;
    wire [1:0]                 axi_bresp;
    wire                       axi_bvalid;
    wire                       axi_bready;
    wire [AXI_ID_WIDTH-1:0]    axi_arid;
    wire [AXI_ADDR_WIDTH-1:0]  axi_araddr;
    wire [7:0]                 axi_arlen;
    wire [2:0]                 axi_arsize;
    wire [1:0]                 axi_arburst;
    wire [1:0]                 axi_arlock;
    wire [3:0]                 axi_arcache;
    wire [2:0]                 axi_arprot;
    wire                       axi_arvalid;
    wire                       axi_arready;
    wire [AXI_ID_WIDTH-1:0]    axi_rid;
    wire [AXI_DATA_WIDTH-1:0]  axi_rdata;
    wire [1:0]                 axi_rresp;
    wire                       axi_rlast;
    wire                       axi_rvalid;
    wire                       axi_rready;

    axi_ram # (
         .DATA_WIDTH(AXI_DATA_WIDTH),
         .ADDR_WIDTH(AXI_ADDR_WIDTH/4), // 2^64 leads to overflow
         .ID_WIDTH(AXI_ID_WIDTH)
    ) imem (
        .clk                     (clock),
        .rst                     (reset),
        .s_axi_awid              (axi_awid),
        .s_axi_awaddr            (axi_awaddr[AXI_ADDR_WIDTH/4-1:0]),
        .s_axi_awlen             (axi_awlen),
        .s_axi_awsize            (axi_awsize),
        .s_axi_awburst           (axi_awburst),
        .s_axi_awlock            (axi_awlock[0]),
        .s_axi_awcache           (axi_awcache),
        .s_axi_awprot            (axi_awprot),
        .s_axi_awvalid           (axi_awvalid),
        .s_axi_awready           (axi_awready),
        .s_axi_wdata             (axi_wdata),
        .s_axi_wstrb             (axi_wstrb),
        .s_axi_wlast             (axi_wlast),
        .s_axi_wvalid            (axi_wvalid),
        .s_axi_wready            (axi_wready),
        .s_axi_bid               (axi_bid),
        .s_axi_bresp             (axi_bresp),
        .s_axi_bvalid            (axi_bvalid),
        .s_axi_bready            (axi_bready),
        .s_axi_arid              (axi_arid),
        .s_axi_araddr            (axi_araddr[AXI_ADDR_WIDTH/4-1:0]),
        .s_axi_arlen             (axi_arlen),
        .s_axi_arsize            (axi_arsize),
        .s_axi_arburst           (axi_arburst),
        .s_axi_arlock            (axi_arlock[0]),
        .s_axi_arcache           (axi_arcache),
        .s_axi_arprot            (axi_arprot),
        .s_axi_arvalid           (axi_arvalid),
        .s_axi_arready           (axi_arready),
        .s_axi_rid               (axi_rid),
        .s_axi_rdata             (axi_rdata),
        .s_axi_rresp             (axi_rresp),
        .s_axi_rlast             (axi_rlast),
        .s_axi_rvalid            (axi_rvalid),
        .s_axi_rready            (axi_rready)
    );

    fetch fetch (
        .ap_clk                      (clock),
        .ap_rst_n                    (~reset),
        // mem port
        .m_axi_ins_port_AWVALID      (axi_awvalid),
        .m_axi_ins_port_AWREADY      (axi_awready),
        .m_axi_ins_port_AWADDR       (axi_awaddr),
        .m_axi_ins_port_AWID         (axi_awid),
        .m_axi_ins_port_AWLEN        (axi_awlen),
        .m_axi_ins_port_AWSIZE       (axi_awsize),
        .m_axi_ins_port_AWBURST      (axi_awburst),
        .m_axi_ins_port_AWLOCK       (axi_awlock),
        .m_axi_ins_port_AWCACHE      (axi_awcache),
        .m_axi_ins_port_AWPROT       (axi_awprot),
        .m_axi_ins_port_AWQOS        (), // unused
        .m_axi_ins_port_AWREGION     (), // unused
        .m_axi_ins_port_WVALID       (axi_wvalid),
        .m_axi_ins_port_WREADY       (axi_wready),
        .m_axi_ins_port_WDATA        (axi_wdata),
        .m_axi_ins_port_WSTRB        (axi_wstrb),
        .m_axi_ins_port_WLAST        (axi_wlast),
        .m_axi_ins_port_ARVALID      (axi_arvalid),
        .m_axi_ins_port_ARREADY      (axi_arready),
        .m_axi_ins_port_ARADDR       (axi_araddr),
        .m_axi_ins_port_ARID         (axi_arid),
        .m_axi_ins_port_ARLEN        (axi_arlen),
        .m_axi_ins_port_ARSIZE       (axi_arsize),
        .m_axi_ins_port_ARBURST      (axi_arburst),
        .m_axi_ins_port_ARLOCK       (axi_arlock),
        .m_axi_ins_port_ARCACHE      (axi_arcache),
        .m_axi_ins_port_ARPROT       (axi_arprot),
        .m_axi_ins_port_ARQOS        (), // unused
        .m_axi_ins_port_ARREGION     (), // unused
        .m_axi_ins_port_RVALID       (axi_rvalid),
        .m_axi_ins_port_RREADY       (axi_rready),
        .m_axi_ins_port_RDATA        (axi_rdata),
        .m_axi_ins_port_RLAST        (axi_rlast),
        .m_axi_ins_port_RID          (axi_rid),
        .m_axi_ins_port_RRESP        (axi_rresp),
        .m_axi_ins_port_BVALID       (axi_bvalid),
        .m_axi_ins_port_BREADY       (axi_bready),
        .m_axi_ins_port_BRESP        (axi_bresp),
        .m_axi_ins_port_BID          (axi_bid),
        // no need for connecting control
        // since we can write directly to
        // those mmio registers
        .s_axi_CONTROL_BUS_AWVALID   ('d0),
        .s_axi_CONTROL_BUS_AWREADY   (),
        .s_axi_CONTROL_BUS_AWADDR    ('d0),
        .s_axi_CONTROL_BUS_WVALID    ('d0),
        .s_axi_CONTROL_BUS_WREADY    (),
        .s_axi_CONTROL_BUS_WDATA     ('d0),
        .s_axi_CONTROL_BUS_WSTRB     ('d0),
        .s_axi_CONTROL_BUS_ARVALID   ('d0),
        .s_axi_CONTROL_BUS_ARREADY   (),
        .s_axi_CONTROL_BUS_ARADDR    ('d0),
        .s_axi_CONTROL_BUS_RVALID    (),
        .s_axi_CONTROL_BUS_RREADY    ('d0),
        .s_axi_CONTROL_BUS_RDATA     (),
        .s_axi_CONTROL_BUS_RRESP     (),
        .s_axi_CONTROL_BUS_BVALID    (),
        .s_axi_CONTROL_BUS_BREADY    ('d0),
        .s_axi_CONTROL_BUS_BRESP     (),
        // load fifo
        .load_queue_V_V_TDATA        (load_data),
        .load_queue_V_V_TVALID       (load_valid),
        .load_queue_V_V_TREADY       (~load_full),
        // gemm fifo
        .gemm_queue_V_V_TDATA        (gemm_data),
        .gemm_queue_V_V_TVALID       (gemm_valid),
        .gemm_queue_V_V_TREADY       (~gemm_full),
        // store fifo
        .store_queue_V_V_TDATA       (store_data),
        .store_queue_V_V_TVALID      (store_valid),
        .store_queue_V_V_TREADY      (~store_full),
        // interrupt
        .interrupt                   ()
    );


    fifo # (
        .DEPTH_WIDTH(10),
        .DATA_WIDTH(128)
    ) load_queue (
        .clk(clock),
        .rst(reset),
        .wr_data_i(load_data),
        .wr_en_i(load_valid),
        .rd_en_i(1'b0),
        .full_o(load_full),
        .empty_o()
    );

    fifo # (
        .DEPTH_WIDTH(10),
        .DATA_WIDTH(128)
    ) gemm_queue (
        .clk(clock),
        .rst(reset),
        .wr_data_i(gemm_data),
        .wr_en_i(gemm_valid),
        .rd_en_i(1'b0),
        .full_o(gemm_full),
        .empty_o()
    );

    fifo # (
        .DEPTH_WIDTH(10),
        .DATA_WIDTH(128)
    ) store_queue (
        .clk(clock),
        .rst(reset),
        .wr_data_i(store_data),
        .wr_en_i(store_valid),
        .rd_en_i(1'b0),
        .full_o(store_full),
        .empty_o()
    );

endmodule
