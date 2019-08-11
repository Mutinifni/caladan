use Packet;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::Read;

pub struct Payload {
    pub work_iterations: u64,
    pub index: u64,
}

use Connection;
use LoadgenProtocol;
use Transport;

#[derive(Clone, Copy)]
pub struct SyntheticProtocol {}

impl LoadgenProtocol for SyntheticProtocol {
    fn gen_req(&self, i: usize, p: &Packet, buf: &mut Vec<u8>) {
        Payload {
            work_iterations: p.work_iterations,
            index: i as u64,
        }
        .serialize_into(buf)
        .unwrap();
    }

    fn read_response(&self, mut sock: &Connection, scratch: &mut [u8]) -> io::Result<usize> {
        sock.read_exact(&mut scratch[..16])?;
        let payload = Payload::deserialize(&mut &scratch[..])?;
        Ok(payload.index as usize)
    }
}

impl SyntheticProtocol {
    pub fn with_args(_matches: &clap::ArgMatches, _tport: Transport) -> Self {
        SyntheticProtocol {}
    }

    pub fn args<'a, 'b>() -> Vec<clap::Arg<'a, 'b>> {
        vec![]
    }
}

impl Payload {
    pub fn serialize_into<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.work_iterations)?;
        writer.write_u64::<BigEndian>(self.index)?;
        Ok(())
    }

    pub fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Payload> {
        let p = Payload {
            work_iterations: reader.read_u64::<BigEndian>()?,
            index: reader.read_u64::<BigEndian>()?,
        };
        return Ok(p);
    }
}
