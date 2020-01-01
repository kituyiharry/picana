extern crate canparse;
extern crate socketcan;
use hex::FromHex;

use memchr::memchr;
///TODO: Use regexp groups
use socketcan::dump::ParseError;
use socketcan::CANFrame;

//(t_usec, iface, (id, data, remote, error, extended)))) => {
//pub type CanFrameData<'a> = (u64, &'a str, (u32, Vec<u8>, bool, u32, bool));
pub type CanFrameData<'a> = (u64, &'a str, CANFrame);

fn parse_raw(bytes: &[u8], radix: u32) -> Option<u64> {
    ::std::str::from_utf8(bytes)
        .ok()
        .and_then(|s| u64::from_str_radix(s, radix).ok())
}

// A Significant optimization! --- what the heck does memchr do?
pub fn decode_frame_memchr(bytes: &[u8]) -> Result<CanFrameData, ParseError> {
    let mut field_iter = bytes.split(|&c| c == b' ');

    // parse time field
    let f = field_iter.next().ok_or(ParseError::UnexpectedEndOfLine)?;

    if f.len() < 3 || f[0] != b'(' || f[f.len() - 1] != b')' {
        return Err(ParseError::InvalidTimestamp);
    }

    let inner = &f[1..f.len() - 1];

    // split at dot, read both parts
    let dot = match memchr(b'.', inner) {
        Some(position) => position,
        _ => return Err(ParseError::InvalidTimestamp),
    };

    let (num, mant) = inner.split_at(dot);

    // parse number and multiply
    let n_num: u64 = parse_raw(num, 10).ok_or(ParseError::InvalidTimestamp)?;
    let n_mant: u64 = parse_raw(&mant[1..], 10).ok_or(ParseError::InvalidTimestamp)?;
    let t_us = n_num.saturating_mul(1_000_000).saturating_add(n_mant);

    let f = field_iter.next().ok_or(ParseError::UnexpectedEndOfLine)?;

    // device name
    let device = ::std::str::from_utf8(f).map_err(|_| ParseError::InvalidDeviceName)?;

    // parse packet
    let can_raw = field_iter.next().ok_or(ParseError::UnexpectedEndOfLine)?;

    let sep_idx = match memchr(b'#', can_raw) {
        Some(position) => position,
        _ => return Err(ParseError::InvalidCanFrame),
    };

    let (can_id, mut can_data) = can_raw.split_at(sep_idx);

    // cut of linefeed and skip seperator
    // Possibly already removed so.....?
    can_data = &can_data[1..];
    if let Some(&b'\n') = can_data.last() {
        can_data = &can_data[..can_data.len() - 1];
    };

    let rtr = b"R" == can_data;

    let data = if rtr {
        Vec::new()
    } else {
        Vec::from_hex(&can_data).map_err(|_| ParseError::InvalidCanFrame)?
    };
    let frame = CANFrame::new(
        parse_raw(can_id, 16).ok_or(ParseError::InvalidCanFrame)? as u32,
        &data,
        rtr,
        // FIXME: how are error frames saved?
        false,
    )?;

    Ok((
        t_us,
        device,
        frame
        //(
        //frame.id(),
        //frame.data().to_owned(),
        //frame.is_rtr(),
        //frame.err(),
        //frame.is_extended(),
        ))
}

pub fn decode_frame(bytes: &[u8]) -> Result<CanFrameData, ParseError> {
    let mut field_iter = bytes.split(|&c| c == b' ');

    // parse time field
    let f = field_iter.next().ok_or(ParseError::UnexpectedEndOfLine)?;

    if f.len() < 3 || f[0] != b'(' || f[f.len() - 1] != b')' {
        return Err(ParseError::InvalidTimestamp);
    }

    let inner = &f[1..f.len() - 1];

    // split at dot, read both parts
    let dot = inner
        .iter()
        .position(|&c| c == b'.')
        .ok_or(ParseError::InvalidTimestamp)?;

    let (num, mant) = inner.split_at(dot);

    // parse number and multiply
    let n_num: u64 = parse_raw(num, 10).ok_or(ParseError::InvalidTimestamp)?;
    let n_mant: u64 = parse_raw(&mant[1..], 10).ok_or(ParseError::InvalidTimestamp)?;
    let t_us = n_num.saturating_mul(1_000_000).saturating_add(n_mant);

    let f = field_iter.next().ok_or(ParseError::UnexpectedEndOfLine)?;

    // device name
    let device = ::std::str::from_utf8(f).map_err(|_| ParseError::InvalidDeviceName)?;

    // parse packet
    let can_raw = field_iter.next().ok_or(ParseError::UnexpectedEndOfLine)?;

    let sep_idx = can_raw
        .iter()
        .position(|&c| c == b'#')
        .ok_or(ParseError::InvalidCanFrame)?;
    let (can_id, mut can_data) = can_raw.split_at(sep_idx);

    // cut of linefeed and skip seperator
    // Possibly already removed so.....?
    can_data = &can_data[1..];
    if let Some(&b'\n') = can_data.last() {
        can_data = &can_data[..can_data.len() - 1];
    };

    let rtr = b"R" == can_data;

    let data = if rtr {
        Vec::new()
    } else {
        Vec::from_hex(&can_data).map_err(|_| ParseError::InvalidCanFrame)?
    };
    let frame = CANFrame::new(
        parse_raw(can_id, 16).ok_or(ParseError::InvalidCanFrame)? as u32,
        &data,
        rtr,
        // FIXME: how are error frames saved?
        false,
    )?;

    Ok((
        t_us,
        device,
        frame
        //(
        //frame.id(),
        //frame.data().to_owned(),
        //frame.is_rtr(),
        //frame.err(),
        //frame.is_extended(),
        //),
    ))
}
