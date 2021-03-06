use futures::stream::TryStreamExt;
use ipnetwork::IpNetwork;
use netlink_packet_route::rtnl::AddressMessage;
use rtnetlink::{Error, Handle};

use crate::{KINDLE_INTERFACE, PI_IP_ADDRESS};

pub async fn setup_if_down(handle: &Handle) -> Result<(), Error> {
    let ip_network = IpNetwork::new(PI_IP_ADDRESS, 24).unwrap();
    add_address(&handle, ip_network).await?;
    link_up(&handle).await
}

pub async fn try_recover(handle: &Handle) -> Result<(), Error> {
    let ip_network = IpNetwork::new(PI_IP_ADDRESS, 24).unwrap();
    del_address(&handle, ip_network).await?;
    link_down(&handle, ip_network).await?;
    add_address(&handle, ip_network).await?;
    link_up(&handle).await
}

async fn add_address(handle: &Handle, ip: IpNetwork) -> Result<(), Error> {
    if get_address(handle, ip).await?.is_none() {
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
    }
    Ok(())
}

async fn link_up(handle: &Handle) -> Result<(), Error> {
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

async fn del_address(handle: &Handle, ip: IpNetwork) -> Result<(), Error> {
    if let Some(address) = get_address(handle, ip).await? {
        handle.address().del(address).execute().await
    } else {
        Ok(())
    }
}

async fn get_address(handle: &Handle, ip: IpNetwork) -> Result<Option<AddressMessage>, Error> {
    handle
        .address()
        .get()
        .set_address_filter(ip.ip())
        .set_prefix_length_filter(ip.prefix())
        .execute()
        .try_next()
        .await
}

async fn link_down(handle: &Handle, ip: IpNetwork) -> Result<(), Error> {
    let mut links = handle
        .link()
        .get()
        .set_name_filter(KINDLE_INTERFACE.to_string())
        .execute();
    if let Some(link) = links.try_next().await? {
        handle
            .link()
            .set(link.header.index)
            .down()
            .execute()
            .await?
    }
    Ok(())
}
