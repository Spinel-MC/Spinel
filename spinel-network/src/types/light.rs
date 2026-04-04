use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub struct LightData {
    pub sky_light_mask: Vec<i64>,
    pub block_light_mask: Vec<i64>,
    pub empty_sky_light_mask: Vec<i64>,
    pub empty_block_light_mask: Vec<i64>,
    pub sky_light_arrays: Vec<Vec<u8>>,
    pub block_light_arrays: Vec<Vec<u8>>,
}

fn encode_bitset<W: Write>(w: &mut W, data: &[i64]) -> io::Result<()> {
    VarIntWrapper(data.len() as i32).encode(w)?;
    for &val in data {
        val.encode(w)?;
    }
    Ok(())
}

fn decode_bitset<R: Read>(r: &mut R) -> io::Result<Vec<i64>> {
    let len = VarIntWrapper::decode(r)?.0 as usize;
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(i64::decode(r)?);
    }
    Ok(vec)
}

impl DataType for LightData {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        encode_bitset(w, &self.sky_light_mask)?;
        encode_bitset(w, &self.block_light_mask)?;
        encode_bitset(w, &self.empty_sky_light_mask)?;
        encode_bitset(w, &self.empty_block_light_mask)?;

        VarIntWrapper(self.sky_light_arrays.len() as i32).encode(w)?;
        for arr in &self.sky_light_arrays {
            VarIntWrapper(arr.len() as i32).encode(w)?;
            w.write_all(arr)?;
        }

        VarIntWrapper(self.block_light_arrays.len() as i32).encode(w)?;
        for arr in &self.block_light_arrays {
            VarIntWrapper(arr.len() as i32).encode(w)?;
            w.write_all(arr)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let sky_light_mask = decode_bitset(r)?;
        let block_light_mask = decode_bitset(r)?;
        let empty_sky_light_mask = decode_bitset(r)?;
        let empty_block_light_mask = decode_bitset(r)?;

        let sky_len = VarIntWrapper::decode(r)?.0 as usize;
        let mut sky_light_arrays = Vec::with_capacity(sky_len);
        for _ in 0..sky_len {
            let len = VarIntWrapper::decode(r)?.0 as usize;
            let mut buf = vec![0u8; len];
            r.read_exact(&mut buf)?;
            sky_light_arrays.push(buf);
        }

        let block_len = VarIntWrapper::decode(r)?.0 as usize;
        let mut block_light_arrays = Vec::with_capacity(block_len);
        for _ in 0..block_len {
            let len = VarIntWrapper::decode(r)?.0 as usize;
            let mut buf = vec![0u8; len];
            r.read_exact(&mut buf)?;
            block_light_arrays.push(buf);
        }

        Ok(LightData {
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_arrays,
            block_light_arrays,
        })
    }
}
