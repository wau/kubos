use nom::le_u16;

#[derive(Debug, Eq, PartialEq)]
pub struct UdpData {
    pub source: u16,
    pub dest: u16,
    pub data: Vec<u8>,
    pub checksum: bool,
}

fn get_u16(data: &[u8], index: u32) -> Result<u16, String> {
    let index = index as usize;
    let left = u16::from(match data.get(index) {
        Some(u) => *u,
        None => return Err(String::from("Index invalid")),
    });

    let right = u16::from(match data.get(index + 1) {
        Some(u) => *u,
        None => return Err(String::from("Index invalid")),
    });

    Ok((left << 8) | right)
}

fn check(source: u16, dest: u16, len: u16, data: &[u8]) -> Result<u32, String> {
    let mut sum: u32 = u32::from(source) + u32::from(dest) + u32::from(len);
    let mut i = 0;
    let len = len as i32;
    // How do we protect against this one?
    while i < len - 9 {
        sum = sum + u32::from(get_u16(data, i as u32)?);
        i += 2
    }
    while sum >= 0x10000 {
        sum = (sum >> 16) + (sum & 0xffff);
    }
    Ok(!sum & 0xffff)
}

pub fn encode(packet: &UdpData) -> Result<Vec<u8>, String> {
    let mut data: Vec<u8> = vec![];

    let len = packet.data.len() as u16;
    let checksum = check(packet.source, packet.dest, len, &packet.data)?;

    // Source
    data.push((packet.source >> 8) as u8);
    data.push((packet.source & 0xFF) as u8);
    // Dest
    data.push((packet.dest >> 8) as u8);
    data.push((packet.dest & 0xFF) as u8);
    // Payload Length
    println!("encoding len {}", len);
    data.push((len >> 8) as u8);
    data.push((len & 0xFF) as u8);
    // Checksum
    data.push((checksum >> 8) as u8);
    data.push((checksum & 0xFF) as u8);
    // Payload
    data.extend(packet.data.iter().cloned());
    Ok(data)
}

pub fn decode(chunk: &[u8], index: usize) -> Result<UdpData, String> {
    let mut data: Vec<u8> = vec![];
    let index = index as u32;

    // Check for 8 header bytes
    // let offset = index - 1;
    // Let's assume we always have either a complete udp packet or trash
    let index = 0;
    let offset = 0;
    let length = (chunk.len() as u32) - offset;
    if length < 8 {
        return Err(String::from("Header bytes not found"));
    }

    // Check for length of entire datagram
    let len = get_u16(chunk, index + 4)?;
    if length < u32::from(len) {
        return Err(String::from("Incomplete datagram"));
    }

    // Read rest of datagram and parse
    let source = get_u16(chunk, index)?;
    let dest = get_u16(chunk, index + 2)?;
    let checksum = u32::from(get_u16(chunk, index + 6)?);
    data.extend(
        chunk[(index + 8) as usize..(index + u32::from(len) - 1) as usize]
            .iter()
            .cloned(),
    );
    let sum = check(source, dest, len, &data)?;
    println!("checksumming {} v {}", checksum, sum);
    let checksum = sum == checksum;

    Ok(UdpData {
        source,
        dest,
        data,
        checksum,
    })
}

pub fn framed_decode(chunk: &[u8], index: usize) -> Result<UdpData, String> {
    let index = 0 as u32;
    let mut data = vec![];

    let length = (chunk.len() as u32);
    if length < 8 {
        return Err(String::from("Header bytes not found"));
    }

    let source = get_u16(chunk, 0)?;
    let dest = get_u16(chunk, 2)?;
    let len = get_u16(chunk, 4)?;
    let checksum = u32::from(get_u16(chunk, 6)?);
    if u32::from(len) > length {
        return Err(format!("Bad length in header {}", len));
    }
    println!("get data {} .. {}", 8, (len));
    data.extend(chunk[8 as usize..(len) as usize].iter().cloned());

    let sum = check(source, dest, len, &data)?;
    let checksum = sum == checksum;

    Ok(UdpData {
        source,
        dest,
        data,
        checksum,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framed_decode() {
        let data = vec![141, 183, 23, 112, 0, 12, 246, 101, 49, 50, 51, 52];
        let decoded = framed_decode(&data, 0);
        assert_eq!(
            decoded,
            Ok(UdpData {
                source: 36279,
                dest: 6000,
                data: vec![49, 50, 51, 52],
                checksum: true,
            })
        );
    }

    #[test]
    fn test_framed_decode_again() {
        let data = vec![141, 183, 23, 112, 0, 11, 248, 155, 49, 49, 49];
        let decoded = framed_decode(&data, 0);
        assert_eq!(
            decoded,
            Ok(UdpData {
                source: 36279,
                dest: 6000,
                data: vec![49, 49, 49],
                checksum: false,
            })
        );
    }

    #[test]
    fn test_framed_decode_test_data() {
        let data = vec![
            027, 88, 158, 88, 000, 8, 69, 71, 000, 130, 026, 000, 7, 116, 65, 245
        ];
        let payload = vec![0, 130, 26, 0, 7, 116, 65, 245];
        let decoded = framed_decode(&data, 0);
        assert_eq!(true, decoded.is_ok());
        assert_eq!(payload, decoded.unwrap().data);
    }

    // #[test]
    // fn test_framed_decode_test_data() {
    //     let data = vec![
    //         027, 88, 174, 5, 000, 98, 6, 106, 000, 130, 026, 000, 001, 218, 134, 245
    //     ];

    //     let decoded = framed_decode(&data, 0);
    //     assert_eq!(
    //         decoded,
    //         Ok(UdpData {
    //             source: 36279,
    //             dest: 6000,
    //             data: vec![49, 49, 49],
    //             checksum: false,
    //         })
    //     );
    // }

    #[test]
    fn test_encode() {
        let data = UdpData {
            source: 7000,
            dest: 70001,
            data: vec![0, 130, 26, 0, 3, 129, 131, 245],
            checksum: true,
        };
        encode(&data);
    }

    #[test]
    fn test_encode_framed_decode() {
        let data = UdpData {
            source: 7000,
            dest: 70001,
            data: vec![0, 130, 26, 0, 3, 129, 131, 245],
            checksum: true,
        };
        let encoded = encode(&data).unwrap();
        println!("encoded {:?}", encoded);
        let decoded = framed_decode(&encoded, 0).unwrap();

        assert_eq!(decoded, data);
    }

    #[test]
    fn test_encode_framed_decode_test_data() {
        let data = UdpData {
            source: 7000,
            dest: 70001,
            data: vec![0, 130, 26, 0, 7, 116, 65, 245],
            checksum: true,
        };
        let encoded = encode(&data).unwrap();
        println!("encoded {:?}", encoded);
        let decoded = framed_decode(&encoded, 0).unwrap();

        assert_eq!(decoded, data);
    }
}