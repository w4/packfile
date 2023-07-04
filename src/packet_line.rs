use std::fmt::Write;

use bytes::{BufMut, BytesMut};

use crate::{low_level::PackFile, Error};

/// A wrapper containing every possible type of message that can be sent to a Git client.
pub enum PktLine<'a> {
    /// General data sent to a client, generally a UTF-8 encoded string.
    Data(&'a [u8]),
    /// Similar to a data packet, but used during packfile sending to indicate this
    /// packet is a block of data by appending a byte containing the u8 `1`.
    SidebandData(PackFile<'a>),
    /// Similar to a data packet, but used during packfile sending to indicate this
    /// packet is a status message by appending a byte containing the u8 `2`.
    SidebandMsg(&'a [u8]),
    /// Indicates the end of a response.
    Flush,
    /// Separates sections of a response.
    Delimiter,
    /// Indicates the end of the response, allowing the client to send another request.
    ResponseEnd,
}

impl PktLine<'_> {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, buf), err))]
    pub fn encode_to(&self, buf: &mut BytesMut) -> Result<(), Error> {
        match self {
            Self::Data(data) => {
                write!(buf, "{:04x}", data.len() + 4)?;
                buf.extend_from_slice(data);
            }
            Self::SidebandData(packfile) => {
                // split the buf off so the cost of counting the bytes to put in the
                // data line prefix is just the cost of `unsplit` (an atomic decrement)
                let mut data_buf = buf.split_off(buf.len());

                data_buf.put_u8(1); // sideband, 1 = data
                packfile.encode_to(&mut data_buf)?;

                // write into the buf not the data buf so it's at the start of the msg
                write!(buf, "{:04x}", data_buf.len() + 4)?;
                buf.unsplit(data_buf);
            }
            Self::SidebandMsg(msg) => {
                write!(buf, "{:04x}", msg.len() + 4 + 1)?;
                buf.put_u8(2); // sideband, 2 = msg
                buf.extend_from_slice(msg);
            }
            Self::Flush => buf.extend_from_slice(b"0000"),
            Self::Delimiter => buf.extend_from_slice(b"0001"),
            Self::ResponseEnd => buf.extend_from_slice(b"0002"),
        }

        Ok(())
    }
}

impl<'a> From<&'a str> for PktLine<'a> {
    fn from(val: &'a str) -> Self {
        PktLine::Data(val.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use bytes::BytesMut;

    #[test]
    fn test_pkt_line() {
        let mut buffer = BytesMut::new();
        super::PktLine::from("agent=git/2.32.0\n")
            .encode_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.as_ref(), b"0015agent=git/2.32.0\n");
    }
}
