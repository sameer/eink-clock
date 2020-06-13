use futures::stream::TryStreamExt;
use ipnetwork::IpNetwork;
use rtnetlink::{new_connection, Error, Handle};

use crate::{KINDLE_IP_ADDRESS, KINDLE_INTERFACE};

pub fn setup_if_down() -> Result<(), Error> {
    let ip_network = IpNetwork::new(KINDLE_IP_ADDRESS, 24).unwrap();
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    futures::executor::block_on(add_address(&handle, ip_network))?;
    futures::executor::block_on(link_up(&handle, ip_network))
}

async fn add_address(handle: &Handle, ip: IpNetwork) -> Result<(), Error> {
    let mut links = handle
        .link()
        .get()
        .set_name_filter(KINDLE_INTERFACE.to_string())
        .execute();
    if let Some(link) = links.try_next().await? {
        handle
            .address()
            .add(link.header.index, ip.ip(), ip.prefix())
            .execute()
            .await?
    }
    Ok(())
}

async fn link_up(handle: &Handle, ip: IpNetwork) -> Result<(), Error> {
    let mut links = handle
        .link()
        .get()
        .set_name_filter(KINDLE_INTERFACE.to_string())
        .execute();
    if let Some(link) = links.try_next().await? {
        handle
            .link()
            .set(link.header.index)
            .up()
            .execute()
            .await?
    }
    Ok(())
}
