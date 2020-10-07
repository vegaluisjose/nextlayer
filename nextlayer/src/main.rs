use nextlayer::v2::{Interface, Resource, ResourceType};

fn main() {
    let name = String::from("testbench");
    let instance = String::from("top");
    let clock = String::from("clock");
    let reset = String::from("reset");
    let kind = ResourceType::Register;
    let resources = vec![Resource {
        kind,
        id: 0,
        width: 3,
        path: "a".to_string(),
    }];
    let interface = Interface {
        name,
        instance,
        clock,
        reset,
        resources,
    };
    println!("{}", toml::to_string(&interface).unwrap());
}

// use nextlayer::interface::Interface;

// fn main() {
//     let mut interface = Interface::new("top");
//     interface.add_register(
//         0,
//         1,
//         "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_start",
//     );
//     interface.add_register(
//         1,
//         1,
//         "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_ap_done",
//     );
//     interface.add_register(
//         2,
//         64,
//         "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_a",
//     );
//     interface.add_register(
//         3,
//         64,
//         "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_b",
//     );
//     interface.add_register(
//         4,
//         64,
//         "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_c",
//     );
//     interface.add_register(
//         5,
//         32,
//         "vadd.inst_krnl_vadd_rtl_int.inst_krnl_vadd_control_s_axi.int_length_r",
//     );
//     interface.add_memory(0, 32, "ram.mem");
//     println!("{}", interface.emit_module());
// }
