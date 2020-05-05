#![no_std]
#![no_main]

use redbpf_probes::net::Transport;
use redbpf_probes::xdp::prelude::*;

program!(0xFFFFFFFE, "GPL");

#[map("tcp_ts")]
static mut tcp_ts: PerfMap<u64> = PerfMap::with_max_entries(1024);

#[xdp("fire_tcp")]
pub fn fire_tcp(ctx: XdpContext) -> XdpResult {
    let ts = bpf_ktime_get_ns();
    if let Ok(transport) = ctx.transport() {
        if let Transport::TCP(_tcphdr) = transport {
            let md = MapData::new(ts);
            unsafe { tcp_ts.insert(&ctx, &md); }
        }
    }

    Ok(XdpAction::Pass)
}
