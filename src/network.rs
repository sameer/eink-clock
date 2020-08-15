use futures::stream::TryStreamExt;
use ipnetwork::IpNetwork;
use rtnetlink::{new_connection, Error, Handle};

use crate::{KINDLE_INTERFACE, PI_IP_ADDRESS};

pub fn setup_if_down() -> Result<(), Error> {
    tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let (connection, handle, _) = new_connection().unwrap();
            tokio::spawn(connection);
            let ip_network = IpNetwork::new(PI_IP_ADDRESS, 24).unwrap();
            link_down(&handle, ip_network).await;
            add_address(&handle, ip_network).await;
            link_up(&handle, ip_network).await
        })
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
        handle.link().set(link.header.index).up().execute().await?
    }
    Ok(())
}

async fn link_down(handle: &Handle, ip: IpNetwork) -> Result<(), Error> {
    let mut links = handle
        .link()
        .get()
        .set_name_filter(KINDLE_INTERFACE.to_string())
        .execute();
    if let Some(link) = links.try_next().await? {
        handle.link().set(link.header.index).down().execute().await?
    }
    Ok(())
}
