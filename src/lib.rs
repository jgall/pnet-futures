#[macro_use]
extern crate futures;
use futures::{stream::Stream, Async, Future, Poll};
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::DataLinkReceiver;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::udp::MutableUdpPacket;
use pnet::packet::{MutablePacket, Packet};
use pnet::transport;
use pnet::transport::Ipv4TransportChannelNBIterator;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::{Ipv4, Ipv6};
use pnet::transport::TransportReceiver;
use pnet::transport::{transport_channel, udp_packet_iter};
use std::env;
use std::io::{self, Write};
use std::net::IpAddr;
use std::process;

pub struct TransportStream<'a, T: 'a> {
    //tr: pnet::transport::TransportReceiver,
    inner: transport::Ipv4TransportChannelNBIterator,
    pd: std::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> TransportStream<'a, T> {
    fn new(mut receiver: pnet::transport::TransportReceiver) -> Self {
        Self {
            inner: transport::ipv4_packet_nb_iter(receiver),
            pd: std::marker::PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Parts<T> {
    /// The socket
    pub socket: u32,
    /// The buffer
    pub buffer: T,

    _priv: (),
}

pub struct ToPacket<T> {
    content: Vec<u8>,
    pd: std::marker::PhantomData<T>,
}

impl<'a> ToPacket<Ipv4Packet<'a>> {
    pub fn to_packet(&'a mut self) -> Ipv4Packet<'a> {
        Ipv4Packet::new(self.content.as_slice()).unwrap()
    }
}

impl<'a, T: 'a> Stream for TransportStream<'a, T> {
    type Item = (ToPacket<Ipv4Packet<'a>>, std::net::IpAddr);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let next = self.inner.next();

        match next {
            Some(Ok(p)) => Ok(Async::Ready(Some({
                use pnet::packet::*;
                let (packet, addr) = p;
                let packet_content = packet.packet();
                (
                    ToPacket::<Ipv4Packet<'a>> {
                        content: packet_content.to_vec(),
                        pd: std::marker::PhantomData,
                    },
                    addr,
                )
            }))),
            Some(Err(e)) => Err(e),
            None => Ok(Async::NotReady),
        }
    }
    // fn poll(&mut self) -> Poll<Self::Item, io::Error> {
    //     let packet = self.inner.recv_chan.next();
    //     let packet = packet.unwrap();
    //     Ok(Async::Ready(Box::new(packet))
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
