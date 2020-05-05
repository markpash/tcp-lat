use redbpf::load::Loader;
use redbpf::xdp;
use tokio::stream::StreamExt;
use std::convert::TryInto;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let mut loader = Loader::load_file("xdp_tcp.elf").expect("error loading elf");

    for xdp_prog in loader.xdps_mut() {
        xdp_prog.attach_xdp("enp5s0", xdp::Flags::default()).unwrap();
    }

    while let Some((_map_name, events)) = loader.events.next().await {
        for event in events {
            let thing: [u8; 8] = Box::leak(event)[0..8].try_into().expect("fail");
            let dur = Duration::from_nanos(u64::from_le_bytes(thing));
            println!("{:?}", dur);
        }
    }
    Ok(())
}