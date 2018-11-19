# pnet-futures

## Futures on top of non-blocking libpnet sockets

### Example usage:

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
        let mapped = transport_stream.map(|(p, a)| println!("oh look we have a packet: {:#?}", p));
        tokio::run(mapped)


## TODO:

* Non-blocking calls for TransportReceiver