use std::io::{self, Read, Seek, SeekFrom};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use num_enum::CustomTryInto;

use crate::helpers::read_bytes;

#[derive(Eq, PartialEq, CustomTryInto)]
#[repr(u16)]
pub enum ProfileType {
	None = 0,
	SRgb = 1,
	EmbeddedIccProfile = 2,
}

bitflags! {
	pub struct Flags: u16 {
		const SpecialFixedGamma = 1;
	}
}

pub struct ColorProfileChunk {
	pub profile_type: ProfileType,
	pub flags: Flags,
	pub fixed_gamma: f32,
	pub icc_profile: Vec<u8>,
}

impl ColorProfileChunk {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let profile_type = read
			.read_u16::<LittleEndian>()?
			.try_into_ProfileType()
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
		let flags = Flags::from_bits_truncate(read.read_u16::<LittleEndian>()?);
		let fixed_gamma = read.read_f32::<LittleEndian>()?;
		read.seek(SeekFrom::Current(8))?;

		let icc_profile = if profile_type == ProfileType::EmbeddedIccProfile {
			let icc_profile_length = read.read_u32::<LittleEndian>()? as usize;
			read_bytes(read, icc_profile_length)?
		} else {
			Vec::new()
		};

		Ok(Self {
			profile_type,
			flags,
			fixed_gamma,
			icc_profile,
		})
	}
}
