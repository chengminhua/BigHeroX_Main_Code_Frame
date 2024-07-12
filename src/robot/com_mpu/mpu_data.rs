use bevy::prelude::*;
use glam::{I16Vec3, I16Vec4, Vec3Swizzles, Vec4Swizzles};
use primitive_byte_iter::ByteIter;

pub const MPU_DATA_HEADER: [u8; 2] = [0x55, 0xAA];
pub const MPU_DATA_BYTES_LENGTH: usize = 2 + 40 + 1;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Component)]
pub struct MPURawData {
    pub quat: I16Vec4,
    pub gyro: I16Vec3,
    pub acc: I16Vec3,
}

impl MPURawData {
    pub fn from_raw_parts<'a, T: Iterator<Item = &'a u8>>(data: T) -> Option<Self> {
        // 读取
        let mut byte_iter = ByteIter::new(data);
        let _ = byte_iter
            .next_u16_be()
            .map(|val| val.to_be_bytes())
            .filter(|header| *header == MPU_DATA_HEADER)?;
        let data: Vec<i16> = std::iter::repeat(0i16)
            .take(20)
            .filter_map(|_| byte_iter.next_i16_be())
            .collect::<Vec<_>>();
        let set_sum = byte_iter.next_u8()?;
        // 处理
        let sum = data
            .iter()
            .flat_map(|val| val.to_be_bytes())
            .fold(0xffu8, |val, now| val.wrapping_add(now));
        // let sum = 0;
        let mut actual_data = data
            .into_iter()
            .enumerate()
            .filter_map(|(count, value)| Some(value).filter(|_| count % 2 == 0));
        // Check SUM
        Some(MPURawData {
            quat: {
                let w = actual_data.next()?;
                I16Vec4::new(
                    actual_data.next()?,
                    actual_data.next()?,
                    actual_data.next()?,
                    w,
                )
            },
            gyro: I16Vec3::new(
                actual_data.next()?,
                actual_data.next()?,
                actual_data.next()?,
            ),
            acc: I16Vec3::new(
                actual_data.next()?,
                actual_data.next()?,
                actual_data.next()?,
            ),
        })
        .filter(|_| sum == set_sum)
    }

    pub fn generate_bytes(&self) -> [u8; MPU_DATA_BYTES_LENGTH] {
        let data_bytes: Vec<u8> = MPU_DATA_HEADER
            .into_iter()
            .chain(
                self.quat
                    .wxyz()
                    .to_array()
                    .into_iter()
                    .chain(self.gyro.xyz().to_array())
                    .chain(self.acc.xyz().to_array())
                    .flat_map(|val| [val, 0])
                    .flat_map(|val| val.to_be_bytes()),
            )
            .collect();
        let sum = data_bytes
            .iter()
            .fold(0u8, |val, now| val.wrapping_add(*now));
        let data_vec = data_bytes.into_iter().chain([sum]).collect::<Vec<_>>();
        let mut data = [0; MPU_DATA_BYTES_LENGTH];
        data.copy_from_slice(data_vec.as_slice());
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_empty() {
        #[rustfmt::skip]
        let origin_slice: [u8; MPU_DATA_BYTES_LENGTH] = [0x55, 0xAA,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0xFF,
        ];
        let empty_data = MPURawData {
            quat: I16Vec4::new(0, 0, 0, 0),
            gyro: I16Vec3::new(0, 0, 0),
            acc: I16Vec3::new(0, 0, 0),
        };
        let new_data = MPURawData::from_raw_parts(origin_slice.iter()).expect("");
        assert_eq!(new_data, empty_data);
        let new_bytes = new_data.generate_bytes();
        assert_eq!(new_bytes, origin_slice);
    }
}
