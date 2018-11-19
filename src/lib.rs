extern crate futures;
use futures::{stream::Stream, Async, Poll};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::transport;
use std::io;

pub struct TransportStream<'a> {
    //tr: pnet::transport::TransportReceiver,
    inner: transport::Ipv4TransportChannelNBIterator<'a>,
}

impl<'a> TransportStream<'a> {
    pub fn new(receiver: &'a mut pnet::transport::TransportReceiver) -> Self {
        Self {
            inner: transport::ipv4_packet_nb_iter(receiver),
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

// You might ask yourself why this struct is necessary:
// the libpnet packet struct contains slice references that cannot be safely passed across threads.
pub struct ToPacket<T> {
    content: Vec<u8>,
    pd: std::marker::PhantomData<T>,
}

impl<'a> ToPacket<Ipv4Packet<'a>> {
    pub fn to_packet(&'a mut self) -> Ipv4Packet<'a> {
        Ipv4Packet::new(self.content.as_slice()).unwrap()
    }
}

impl<'a> Stream for TransportStream<'a> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use pnet::packet::ip::IpNextHeaderProtocols;
    use pnet::transport::TransportChannelType::Layer4;
    use pnet::transport::TransportProtocol::Ipv4;
    use pnet::transport::{transport_channel, udp_packet_nb_iter};
    #[test]
    fn must_run_with_sudo() {
        let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Test1));

        // Create a new transport channel, dealing with layer 4 packets on a test protocol
        // It has a receive buffer of 4096 bytes.
        let (mut tx, mut rx) = match transport_channel(4096, protocol) {
            Ok((tx, rx)) => (tx, rx),
            Err(e) => panic!(
                "An error occurred when creating the transport channel: {}",
                e
            ),
        };
        let transport_stream = TransportStream::new(&mut rx);
        assert_eq!(2 + 2, 4);
    }
}
