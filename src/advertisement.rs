use scroll::ctx::StrCtx;
use scroll::{self, ctx, Pread};

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleAdvLocalName<'a> {
    pub state: u8,
    pub version: u16,
    _reserved: &'a [u8],
    pub name: &'a str, // UTF8: 12 bytes + NULL
}

pub const ANKI_VEHICLE_ADV_LOCAL_NAME_SIZE: usize = 21;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleAdvLocalName<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        // TODO: This might break if a bigger size data is inputted.
        if data.len() < ANKI_VEHICLE_ADV_LOCAL_NAME_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let state: u8 = data.gread_with::<u8>(offset, ctx)?;
        let version: u16 = data.gread_with::<u16>(offset, ctx)?;
        let _reserved: &'a [u8] = data.gread_with::<&'a [u8]>(offset, 5)?;
        let name: &str = data.gread_with::<&str>(offset, StrCtx::Length(13))?;

        Ok((
            AnkiVehicleAdvLocalName {
                state,
                version,
                _reserved,
                name,
            },
            *offset,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleAdvMfgData {
    pub identifier: u32,
    pub model_id: u8,
    _reserved: u8,
    pub product_id: u16,
}

pub const ANKI_VEHICLE_ADV_MFG_DATA_SIZE: usize = 8;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleAdvMfgData {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        // TODO: This might break if a bigger size data is inputted.
        if data.len() < ANKI_VEHICLE_ADV_MFG_DATA_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let identifier: u32 = data.gread_with::<u32>(offset, ctx)?;
        let model_id: u8 = data.gread_with::<u8>(offset, ctx)?;
        let _reserved: u8 = data.gread_with::<u8>(offset, ctx)?;
        let product_id: u16 = data.gread_with::<u16>(offset, ctx)?;

        Ok((
            AnkiVehicleAdvMfgData {
                identifier,
                model_id,
                _reserved,
                product_id,
            },
            *offset,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleAdv<'a> {
    pub flags: u8,
    pub tx_power: u8,
    pub mfg_data: AnkiVehicleAdvMfgData,
    pub local_name: AnkiVehicleAdvLocalName<'a>,
    pub service_id: &'a [u8],
}

pub const ANKI_VEHICLE_ADV_SIZE: usize =
    2 + ANKI_VEHICLE_ADV_MFG_DATA_SIZE + ANKI_VEHICLE_ADV_LOCAL_NAME_SIZE + 16;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleAdv<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        if data.len() != ANKI_VEHICLE_ADV_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let flags: u8 = data.gread_with::<u8>(offset, ctx)?;
        let tx_power: u8 = data.gread_with::<u8>(offset, ctx)?;
        let mfg_data: AnkiVehicleAdvMfgData =
            data.gread_with::<AnkiVehicleAdvMfgData>(offset, ctx)?;
        let local_name: AnkiVehicleAdvLocalName =
            data.gread_with::<AnkiVehicleAdvLocalName>(offset, ctx)?;
        let service_id: &'a [u8] = data.gread_with::<&'a [u8]>(offset, 16)?;

        Ok((
            AnkiVehicleAdv {
                flags,
                tx_power,
                mfg_data,
                local_name,
                service_id,
            },
            *offset,
        ))
    }
}

#[cfg(test)]
mod tests {
    use scroll::{Pread, BE};

    use super::*;

    #[test]
    fn anki_vehicle_adv_local_name_struct_test() {
        let data: &[u8; ANKI_VEHICLE_ADV_LOCAL_NAME_SIZE] = &[
            0xAB, 0xCD, 0xEF, 0x1, 0x2, 0x3, 0x4, 0x5, 'l' as u8, 'o' as u8, 'c' as u8, 'a' as u8,
            'l' as u8, 'n' as u8, 'a' as u8, 'm' as u8, 'e' as u8, 't' as u8, 'e' as u8, 's' as u8,
            't' as u8,
        ];
        let local_name: AnkiVehicleAdvLocalName = AnkiVehicleAdvLocalName {
            state: 0xAB,
            version: 0xCDEF,
            _reserved: &[0x1, 0x2, 0x3, 0x4, 0x5],
            name: "localnametest",
        };
        let test_local_name = data
            .gread_with::<AnkiVehicleAdvLocalName>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_local_name, local_name);
        assert_eq!(local_name, test_local_name)
    }

    #[test]
    fn anki_vehicle_adv_mfg_data_struct_test() {
        let data: &[u8; ANKI_VEHICLE_ADV_MFG_DATA_SIZE] =
            &[0x89, 0xAB, 0xCD, 0xEF, 0xAB, 0x12, 0xCD, 0xEF];
        let mfg_data: AnkiVehicleAdvMfgData = AnkiVehicleAdvMfgData {
            identifier: 0x89ABCDEF,
            model_id: 0xAB,
            _reserved: 0x12,
            product_id: 0xCDEF,
        };
        let test_mfg_data = data
            .gread_with::<AnkiVehicleAdvMfgData>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_mfg_data, mfg_data);
        assert_eq!(mfg_data, test_mfg_data)
    }

    #[test]
    fn anki_vehicle_adv_struct_test() {
        let data: &[u8; ANKI_VEHICLE_ADV_SIZE] = &[
            0x12, 0x34, 0x89, 0xAB, 0xCD, 0xEF, 0xAB, 0x56, 0xCD, 0xEF, 0xAB, 0xCD, 0xEF, 0x1, 0x2,
            0x3, 0x4, 0x5, 'l' as u8, 'o' as u8, 'c' as u8, 'a' as u8, 'l' as u8, 'n' as u8,
            'a' as u8, 'm' as u8, 'e' as u8, 't' as u8, 'e' as u8, 's' as u8, 't' as u8, 0x0, 0x1,
            0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
        ];
        let adv: AnkiVehicleAdv = AnkiVehicleAdv {
            flags: 0x12,
            tx_power: 0x34,
            mfg_data: AnkiVehicleAdvMfgData {
                identifier: 0x89ABCDEF,
                model_id: 0xAB,
                _reserved: 0x56,
                product_id: 0xCDEF,
            },
            local_name: AnkiVehicleAdvLocalName {
                state: 0xAB,
                version: 0xCDEF,
                _reserved: &[0x1, 0x2, 0x3, 0x4, 0x5],
                name: "localnametest",
            },
            service_id: &[
                0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
            ],
        };
        let test_adv = data.gread_with::<AnkiVehicleAdv>(&mut 0, BE).unwrap();
        println!("T:{:?} == G:{:?}", test_adv, adv);
        assert_eq!(adv, test_adv)
    }
}
