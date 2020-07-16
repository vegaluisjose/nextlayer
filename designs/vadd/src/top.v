module top (
    input  logic          clock,
    input  logic          reset
);

    localparam integer AXI_ID_WIDTH = 1;
    localparam integer AXI_ADDR_WIDTH = 64;
    localparam integer AXI_DATA_WIDTH = 32;
    localparam integer AXI_STRB_WIDTH = (AXI_DATA_WIDTH/8);

    wire [AXI_ID_WIDTH-1:0]    axi_awid;
    wire [AXI_ADDR_WIDTH-1:0]  axi_awaddr;
    wire [7:0]                 axi_awlen;
    wire [2:0]                 axi_awsize;
    wire [1:0]                 axi_awburst;
    wire                       axi_awlock;
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
    wire                       axi_arlock;
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

    krnl_vadd_rtl # (
        .C_M_AXI_GMEM_ID_WIDTH(AXI_ID_WIDTH),
        .C_M_AXI_GMEM_ADDR_WIDTH(AXI_ADDR_WIDTH),
        .C_M_AXI_GMEM_DATA_WIDTH(AXI_DATA_WIDTH)
    ) vadd (
        .ap_clk                  (clock),
        .ap_rst_n                (~reset),
        // mem port
        .m_axi_gmem_AWVALID      (axi_awvalid),
        .m_axi_gmem_AWREADY      (axi_awready),
        .m_axi_gmem_AWADDR       (axi_awaddr),
        .m_axi_gmem_AWID         (axi_awid),
        .m_axi_gmem_AWLEN        (axi_awlen),
        .m_axi_gmem_AWSIZE       (axi_awsize),
        .m_axi_gmem_AWBURST      (axi_awburst),
        .m_axi_gmem_AWLOCK       (axi_awlock),
        .m_axi_gmem_AWCACHE      (axi_awcache),
        .m_axi_gmem_AWPROT       (axi_awprot),
        .m_axi_gmem_AWQOS        (), // unused
        .m_axi_gmem_AWREGION     (), // unused
        .m_axi_gmem_WVALID       (axi_wvalid),
        .m_axi_gmem_WREADY       (axi_wready),
        .m_axi_gmem_WDATA        (axi_wdata),
        .m_axi_gmem_WSTRB        (axi_wstrb),
        .m_axi_gmem_WLAST        (axi_wlast),
        .m_axi_gmem_ARVALID      (axi_arvalid),
        .m_axi_gmem_ARREADY      (axi_arready),
        .m_axi_gmem_ARADDR       (axi_araddr),
        .m_axi_gmem_ARID         (axi_arid),
        .m_axi_gmem_ARLEN        (axi_arlen),
        .m_axi_gmem_ARSIZE       (axi_arsize),
        .m_axi_gmem_ARBURST      (axi_arburst),
        .m_axi_gmem_ARLOCK       (axi_arlock),
        .m_axi_gmem_ARCACHE      (axi_arcache),
        .m_axi_gmem_ARPROT       (axi_arprot),
        .m_axi_gmem_ARQOS        (), // unused
        .m_axi_gmem_ARREGION     (), // unused
        .m_axi_gmem_RVALID       (axi_rvalid),
        .m_axi_gmem_RREADY       (axi_rready),
        .m_axi_gmem_RDATA        (axi_rdata),
        .m_axi_gmem_RLAST        (axi_rlast),
        .m_axi_gmem_RID          (axi_rid),
        .m_axi_gmem_RRESP        (axi_rresp),
        .m_axi_gmem_BVALID       (axi_bvalid),
        .m_axi_gmem_BREADY       (axi_bready),
        .m_axi_gmem_BRESP        (axi_bresp),
        .m_axi_gmem_BID          (axi_bid),
        // no need for connecting control
        // since we can write directly to
        // those mmio registers
        .s_axi_control_AWVALID   (),
        .s_axi_control_AWREADY   (),
        .s_axi_control_AWADDR    (),
        .s_axi_control_WVALID    (),
        .s_axi_control_WREADY    (),
        .s_axi_control_WDATA     (),
        .s_axi_control_WSTRB     (),
        .s_axi_control_ARVALID   (),
        .s_axi_control_ARREADY   (),
        .s_axi_control_ARADDR    (),
        .s_axi_control_RVALID    (),
        .s_axi_control_RREADY    (),
        .s_axi_control_RDATA     (),
        .s_axi_control_RRESP     (),
        .s_axi_control_BVALID    (),
        .s_axi_control_BREADY    (),
        .s_axi_control_BRESP     ()
    );

    axi_ram # (
         .DATA_WIDTH(AXI_DATA_WIDTH),
         .ADDR_WIDTH(AXI_ADDR_WIDTH),
         .ID_WIDTH(AXI_ID_WIDTH)
    ) ram (
        .clk                     (clock),
        .rst                     (reset),
        .s_axi_awid              (axi_awid),
        .s_axi_awaddr            (axi_awaddr),
        .s_axi_awlen             (axi_awlen),
        .s_axi_awsize            (axi_awsize),
        .s_axi_awburst           (axi_awburst),
        .s_axi_awlock            (axi_awlock),
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
        .s_axi_araddr            (axi_araddr),
        .s_axi_arlen             (axi_arlen),
        .s_axi_arsize            (axi_arsize),
        .s_axi_arburst           (axi_arburst),
        .s_axi_arlock            (axi_arlock),
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

endmodule
