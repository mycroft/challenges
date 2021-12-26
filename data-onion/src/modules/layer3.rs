use crate::modules::ascii85::decode_ascii85;

/*
The payload for this layer is encoded as a stream of raw
network data, as if the solution was being received over the
internet. The data is a series of IPv4 packets with User
Datagram Protocol (UDP) inside. Extract the payload data
from inside each packet, and combine them together to form
the solution.

Each packet has three segments: the IPv4 header, the UDP
header, and the data section. So the first 20 bytes of the
payload will be the IPv4 header of the first packet. The
next 8 bytes will be the UDP header of the first packet.
This is followed by a variable-length data section for the
first packet. After the data section you will find the
second packet, starting with another 20 byte IPv4 header,
and so on.

You will need to read the specifications for IPv4 and UDP in
order to parse the data. The official specification for IPv4
is RFC 791 (https://tools.ietf.org/html/rfc791) and for UDP
it is RFC 768 (https://tools.ietf.org/html/rfc768). The
Wikipedia pages for these two protocols are also good, and
probably easier to read than the RFCs.

However, the payload contains extra packets that are not
part of the solution. Discard these corrupted and irrelevant
packets when forming the solution.

Each valid packet of the solution has the following
properties. Discard packets that do not have all of these
properties.

 - The packet was sent FROM any port of 10.1.1.10
 - The packet was sent TO port 42069 of 10.1.1.200
 - The IPv4 header checksum is correct
 - The UDP header checksum is correct

WARNING: Failing to do this properly WILL cause the next
layer to be unsolveable. If you include incorrect packets in
your solution, the result may be readable and look correct,
but its payload WILL be corrupted in ways that are
impossible to detect. Trust me.

The packets appear in the correct order. No reordering is
necessary.
*/

struct Ipv4Header {
    version: u8,       // on 4 bits
    header_length: u8, // on 4 bits
    service_type: u8,
    total_length: u8,
    ident: u8,
    indicator: u8, // on 3 bits
    offset: u8,    // on 13 bits
    lifetime: u8,
    protocol: u8,
    chksum: u16,
    src_address: u32,
    dst_address: u32,
}

struct UdpHeader {
    src_port: u16,
    dest_port: u16,
    length: u16,
    chksum: u16,
}

// Parse ipv4 & check checksum. Return Ok with required fields for UPD checks
fn parse_ipv4(buffer: &[u8]) -> (u32, u32, u8, u16, bool) {
    let mut sum: u32 = 0;
    let mut is_valid = true;

    for idx in 0..10 {
        let idx = idx * 2;
        let mut v: u32 = (buffer[idx] as u32) << 8;

        v += buffer[idx + 1] as u32;

        sum += v as u32;
    }

    while sum > 0xffff {
        let c = sum >> 16;
        sum += c;
        sum &= 0xffff;
    }

    if sum != 0xffff {
        // println!("Invalid IPv4 checksum check: {} != 0xffff", sum);
        is_valid = false;
    }

    let src_addr: u32 = (buffer[12] as u32) << 24
        | (buffer[13] as u32) << 16
        | (buffer[14] as u32) << 8
        | buffer[15] as u32;
    let dst_addr: u32 = (buffer[16] as u32) << 24
        | (buffer[17] as u32) << 16
        | (buffer[18] as u32) << 8
        | buffer[19] as u32;
    let protocol = buffer[9];
    let length = (buffer[2] as u16) << 8 | buffer[3] as u16;

    (src_addr, dst_addr, protocol, length, is_valid)
}

fn parse_udp(buffer: &[u8]) -> (u16, u16, u16, u16) {
    let src_port: u16 = (buffer[0] as u16) << 8 | (buffer[1] as u16);
    let dst_port: u16 = (buffer[2] as u16) << 8 | (buffer[3] as u16);
    let length: u16 = (buffer[4] as u16) << 8 | (buffer[5] as u16);
    let chksum: u16 = (buffer[6] as u16) << 8 | (buffer[7] as u16);

    (src_port, dst_port, length, chksum)
}

fn check_udp(buffer: &[u8]) -> bool {
    let mut total: u32 = 0;

    // src ip
    total += ((buffer[12] as u16) << 8 | (buffer[13] as u16)) as u32;
    total += ((buffer[14] as u16) << 8 | (buffer[15] as u16)) as u32;
    // dst ip
    total += ((buffer[16] as u16) << 8 | (buffer[17] as u16)) as u32;
    total += ((buffer[18] as u16) << 8 | (buffer[19] as u16)) as u32;

    // 8 bit protocol
    total += buffer[9] as u32;

    // 16 bit total udp length
    total += ((buffer[24] as u16) << 8 | (buffer[25] as u16)) as u32;

    // src port, dst port
    total += ((buffer[20] as u16) << 8 | (buffer[21] as u16)) as u32;
    total += ((buffer[22] as u16) << 8 | (buffer[23] as u16)) as u32;

    // total length, again?
    total += ((buffer[24] as u16) << 8 | (buffer[25] as u16)) as u32;
    total += ((buffer[26] as u16) << 8 | (buffer[27] as u16)) as u32;

    // adding data
    let mut len = (buffer[24] as u16) << 8 | (buffer[25] as u16);
    len -= 8;
    let mut idx = 20 + 8;

    while len > 0 {
        total += (buffer[idx] as u32) << 8;
        if len == 1 {
            break;
        }
        idx += 1;
        total += buffer[idx] as u32;

        idx += 1;
        len -= 2;
    }

    while total > 0xffff {
        let c = total >> 16;
        total += c;
        total &= 0xffff;
    }

    total == 0xffff
}

pub fn decode_layer3(orig: &String) -> String {
    let decoded = decode_ascii85(&orig);
    let mut idx = 0;
    let mut res = vec![];

    loop {
        // Read a packet
        if decoded.len() < idx + 28 {
            break;
        }

        // IPv4 header: Read 20 bytes
        let res_ip = parse_ipv4(&decoded[idx..]);

        let res_udp = parse_udp(&decoded[idx + 20..]);
        let data_len = (res_udp.2 - 8) as usize;

        // Verify UDP checksum
        let res_chk_udp = check_udp(&decoded[idx..]);
        /*
               - The packet was sent FROM any port of 10.1.1.10
               - The packet was sent TO port 42069 of 10.1.1.200
               - The IPv4 header checksum is correct
               - The UDP header checksum is correct
        */
        if res_ip.4
            && res_chk_udp
            && res_ip.0 == 0x0a01010a
            && res_ip.1 == 0x0a0101c8
            && res_udp.1 == 42069
        {
            let mut data = decoded[idx + 28..idx + 28 + data_len].to_vec();
            res.append(&mut data);
        }

        //        println!("0x{:08x}:{} -> 0x{:08x}:{} len: {}",
        //            res_ip.0, res_udp.0, res_ip.1, res_udp.1, data_len);

        idx += res_ip.3 as usize;
    }

    String::from_utf8(res).unwrap()
}
