use std::io::{Write, Read};

enum MdioType
{
    Send,
    Receive
}

struct MdioHeader
{
    mdio_type: MdioType,
    phy_address: u16,
    reg_address: u16,
}

fn to_buffer(header: MdioHeader) -> [u8; 2]
{
    let mut out: u16 = 0;

    // start bits
    out |= 0b01 << 14;
    // type
    out |= match header.mdio_type
    {
        MdioType::Send => 0b01 << 12,
        MdioType::Receive => 0b10 << 12,
    };
    // PHY
    out |= (header.phy_address & 0b11111) << 7;
    // REG
    out |= (header.reg_address & 0b11111) << 2;
    // Turnaround bits
    out |= match header.mdio_type
    {
        MdioType::Send => 0b10,
        MdioType::Receive => 0b00
    };

    [(out >> 8) as u8, (out & 0xff) as u8]
}

fn write<T>(file: &mut T, phy_address: u16, reg_address: u16, data: &[u8])
    where T: Write
{
    let header = to_buffer(MdioHeader
    {
        mdio_type: MdioType::Send,
        phy_address: phy_address,
        reg_address: reg_address
    });
    for buf in data.chunks(2)
    {
        assert_eq!(buf.len(), 2);
        file.write(&header).unwrap();
        file.write(buf).unwrap();
    }
}

fn read<T>(file: &mut T, phy_address: u16, reg_address: u16, buffer: &mut [u8])
    where T: Read + Write
{
    let header = to_buffer(MdioHeader
    {
        mdio_type: MdioType::Send,
        phy_address: phy_address,
        reg_address: reg_address
    });
    for mut buf in buffer.chunks_mut(2)
    {
        assert_eq!(buf.len(), 2);
        file.write(&header).unwrap();
        file.read_exact(&mut buf).unwrap();
    }
}


fn main() {
    let device_name = std::env::args().nth(1).expect("provide device name");
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&device_name)
        .unwrap();
    let data: [u8; 2] = [3, 3];
    write(&mut file, 3, 3, &data);
    let mut retval: [u8; 4] = [2; 4];
    read(&mut file, 3, 3, &mut retval);
    println!("{:?}", retval);
}

