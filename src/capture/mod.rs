extern crate pnet;

pub mod errors;
pub mod filter;
pub mod interfaces;
pub mod osi;
pub mod packets;
pub mod results;

use crate::capture::errors::CaptureError;
use crate::capture::osi::*;
use crate::capture::results::CaptureResult;

use pnet::datalink::Channel::*;
use pnet::datalink::{channel, interfaces};
use pnet::datalink::{DataLinkReceiver, DataLinkSender, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

const BUFFER_INIT: u8 = 0u8;
const BUFFER_SIZE: usize = 1600;

/// Creates a tuple of a transmission (Tx) and reception (Rx) channels, to be used to contact or
/// retrieve, respectively, packets from the selected device.
///
/// This function could retrieve, in case of error, an `UnrecognizableDatalinkPacket` error (meaning the
/// packet was unable to be verified), or an `NotEnoughPermissionsToCreateChannel` error (meaning that the
/// current user has not the permissions to create such communication resource.
///
/// # Arguments
///
/// * `device` - The `pnet`'s library `NetworkInterface` reference to the network interface where
/// the capture is to be implemented.
fn create_recv_channel(
    device: &NetworkInterface,
) -> Result<(Box<DataLinkSender>, Box<DataLinkReceiver>), CaptureError> {
    match channel(&device, Default::default()) {
        Ok(Ethernet(tx, rx)) => Ok((tx, rx)),
        Ok(_) => Err(CaptureError::UnrecognizableDatalinkPacket),
        Err(_) => Err(CaptureError::NotEnoughPermissionsToCreateChannel),
    }
}

/// Retrieve a `pnet::datalink::NetworkInterface` structure containing all the informations about
/// the given `interface_name`, as only argument to be accepted.
///
/// This method, as it verifies if the given `interface_name` exists on system, can return a `InterfaceNotFound`
/// error.
///
/// # Arguments
///
/// * `interface_name` - the name of the interface to perform a capture on.
fn get_device_from_name(interface_name: &String) -> Result<NetworkInterface, CaptureError> {
    // Create a matching rule to test if current iteration's interface is the one user wants.
    let match_rule = |interface: &NetworkInterface| interface.name == *interface_name;

    // Retrieve the complete list of interfaces and apply the latter rule on it.
    let interfaces = interfaces();
    let interface = interfaces.into_iter().filter(match_rule).next();

    // If the device was successfully retrieved, then approve it and return it; otherwise, return
    //   `DeviceNotFound` error.
    match interface {
        Some(dev) => Ok(dev),
        None => Err(CaptureError::InterfaceNotFound),
    }
}

pub fn get_interface_channels(
    interface_name: &String,
) -> Result<(Box<DataLinkSender>, Box<DataLinkReceiver>), CaptureError> {
    // Retrieve the interface from the interface name, as chosen by the user.
    let interface_to_match = get_device_from_name(&interface_name);
    let interface = match interface_to_match {
        Ok(dev) => dev,
        Err(e) => return Err(e),
    };

    // Create a communication channel with the device, only in terms of reception (Rx) and return it.
    match create_recv_channel(&interface) {
        Ok((tx, rx)) => Ok((tx, rx)),
        Err(e) => Err(e),
    }
}

// TODO : document this function.
pub fn execute_once(
    (_channel_tx, channel_rx): (&mut Box<DataLinkSender>, &mut Box<DataLinkReceiver>),
) -> Result<(CaptureResult, Vec<u8>), CaptureError> {
    let mut buffer: [u8; BUFFER_SIZE] = [BUFFER_INIT; BUFFER_SIZE];
    let mut _ethernet_frame = MutableEthernetPacket::new(&mut buffer[..]).unwrap();

//    loop {
////        crossbeam_channel.recv()
//
//
//        select!{
//            // ou vem o valor do ethernet channel
//            // ou vem o valor do SIGINT channel
//        }
//
//
//    }



    // thread
    match channel_rx.next() {
        Ok(packet) => {
            match datalink::handle_ethernet_frame(&EthernetPacket::new(packet).unwrap()) {
                Ok(result) => Ok((result, Vec::from(packet))),
                Err(error) => return Err(error),
            }
        }
        Err(_e) => return Err(CaptureError::UnrecognizableDatalinkPacket),
    }

    //
    // -> crossbeam_channel.try_send()

    // /thread
}

// TODO : document this function.
pub fn execute_once_raw(channel_rx: &mut Box<DataLinkReceiver>) -> Result<&[u8], CaptureError> {
    let mut buffer: [u8; BUFFER_SIZE] = [BUFFER_INIT; BUFFER_SIZE];
    let mut _ethernet_frame = MutableEthernetPacket::new(&mut buffer[..]).unwrap();
    match channel_rx.next() {
        Ok(packet) => Ok(packet),
        Err(_e) => return Err(CaptureError::UndefinedPacket),
    }
}
