use bluer::{
    l2cap::{SocketAddr, Stream},
    Address, AddressType,
};
use log::debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const WIIMOTE_ADDR: [u8; 6] = [0x2C, 0x10, 0xC1, 0xE1, 0x61, 0xAC];

fn main() {
    env_logger::init_from_env("LOG");
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run());
}
async fn run() {
    debug!("create session");
    let session = bluer::Session::new().await.unwrap();
    debug!("get adapter");
    let adapter = session.default_adapter().await.unwrap();
    debug!("done");
    adapter.set_powered(true).await.unwrap();

    // debug!("get device");
    // let device = adapter.device(Address::new(WIIMOTE_ADDR)).unwrap();
    // debug!("get device uuids");
    // let uuids = device.uuids().await.unwrap().unwrap_or_default();
    // debug!("uuids: {uuids:?}");

    // for s in device.services().await.unwrap() {
    //     debug!("\tservice {}", s.id());
    //     for c in s.characteristics().await.unwrap() {
    //         debug!("\tch {}", c.id())
    //     }
    // }

    let mut control_pipe = Stream::connect(SocketAddr::new(
        Address(WIIMOTE_ADDR),
        AddressType::LePublic,
        0x13,
    ))
    .await
    .unwrap();

    println!(
        "Local address: {:?}",
        control_pipe.as_ref().local_addr().unwrap()
    );
    println!("Remote address: {:?}", control_pipe.peer_addr().unwrap());
    println!("Send MTU: {:?}", control_pipe.as_ref().send_mtu());
    println!("Recv MTU: {}", control_pipe.as_ref().recv_mtu().unwrap());
    println!("Security: {:?}", control_pipe.as_ref().security().unwrap());
    println!("Flow control: {:?}", control_pipe.as_ref().flow_control());

    // control_pipe.write_all(&[0x12, 0x04]).await.unwrap();

    let mut buf = [0u8; 1024];
    loop {
        let size = control_pipe.read(&mut buf).await.unwrap();
        debug!("recv {:?}", &buf[..size]);
    }

    debug!("exit");
}
