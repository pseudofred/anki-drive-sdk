use num_enum::{IntoPrimitive, TryFromPrimitive};
use scroll::{self, ctx, Pread, Pwrite};
use std::ops::Add;

pub const ANKI_VEHICLE_MSG_MAX_SIZE: usize = 20;
pub const ANKI_VEHICLE_MSG_PAYLOAD_MAX_SIZE: usize = 18;
pub const ANKI_VEHICLE_MSG_BASE_SIZE: usize = 2;

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[non_exhaustive]
#[repr(u8)]
pub enum AnkiVehicleMsgType {
    Unknown = 0x0,
    // BLE Connections
    C2VDisconnect = 0x0d,

    // Ping request / response
    C2CPingRequest = 0x16,
    V2CPingResponse = 0x17,

    // Messages for checking vehicle version info
    C2VVersionRequest = 0x18,
    V2CVersionResponse = 0x19,

    // Battery level
    C2VBatteryLevelRequest = 0x1a,
    V2CBatteryLevelResponse = 0x1b,

    // Lights
    C2VSetLights = 0x1d,

    // Driving Commands
    C2VSetSpeed = 0x24,
    C2VChangeLane = 0x25,
    C2VCancelLaneChange = 0x26,

    // Vehicle position updates
    V2CLocalisationPositionUpdate = 0x27,
    V2CLocalisationTransitionUpdate = 0x29,
    V2CLocalisationIntersectionUpdate = 0x2a,
    V2CVehicleDelocalized = 0x2b,
    C2VSetOffsetFromRoadCentre = 0x2c,
    V2COffsetFromRoadCentreUpdate = 0x2d,

    // Turn Command
    C2VTurn = 0x32,

    // Light Patterns
    C2VLightsPattern = 0x33,

    // Vehicle Configuration Parameters
    C2VSetConfigParams = 0x45,

    // SDK Mode
    C2VSDKMode = 0x90,
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsg<'a> {
    size: u8,
    pub msg_id: AnkiVehicleMsgType,
    payload: &'a [u8],
}

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleMsg<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        if data.len() > ANKI_VEHICLE_MSG_MAX_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let size: u8 = data.gread_with::<u8>(offset, ctx)?;
        let msg_id: AnkiVehicleMsgType = data
            .gread_with::<u8>(offset, ctx)?
            .try_into()
            .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown);
        let payload: &'a [u8];
        if data.len() > ANKI_VEHICLE_MSG_BASE_SIZE {
            payload = data.gread_with::<&'a [u8]>(offset, data.len() - 2)?;
        } else {
            payload = &[]
        }

        Ok((
            AnkiVehicleMsg {
                size,
                msg_id,
                payload,
            },
            *offset,
        ))
    }
}

impl<'a> ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsg<'a> {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_BASE_SIZE + self.payload.len() {
            return Err((scroll::Error::Custom(
                "Incorrect size of byte array for anki vehicle message".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        if self.payload.len() > 0 {
            data.gwrite::<&'a [u8]>(self.payload, offset)?;
        }

        Ok(*offset)
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgVersionResponse {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    pub version: u16,
}

pub const ANKI_VEHICLE_MSG_VERSION_RESPONSE_SIZE: usize = 4;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleMsgVersionResponse {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_VERSION_RESPONSE_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let size: u8 = data.gread_with::<u8>(offset, ctx)?;
        let msg_id: AnkiVehicleMsgType = data
            .gread_with::<u8>(offset, ctx)?
            .try_into()
            .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown);
        let version: u16 = data.gread_with::<u16>(offset, ctx)?;

        Ok((
            AnkiVehicleMsgVersionResponse {
                size,
                msg_id,
                version,
            },
            *offset,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgBatteryLevelResponse {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    pub battery_level: u16,
}

pub const ANKI_VEHICLE_MSG_BATTERY_LEVEL_RESPONSE_SIZE: usize = 4;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleMsgBatteryLevelResponse {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_BATTERY_LEVEL_RESPONSE_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let size: u8 = data.gread_with::<u8>(offset, ctx)?;
        let msg_id: AnkiVehicleMsgType = data
            .gread_with::<u8>(offset, ctx)?
            .try_into()
            .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown);
        let battery_level: u16 = data.gread_with::<u16>(offset, ctx)?;

        Ok((
            AnkiVehicleMsgBatteryLevelResponse {
                size,
                msg_id,
                battery_level,
            },
            *offset,
        ))
    }
}

pub const ANKI_VEHICLE_SDK_OPTION_OVERRIDE_LOCALIZATION: u8 = 0x1;

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgSdkMode {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    on: u8,
    flags: u8,
}

pub const ANKI_VEHICLE_MSG_SDK_MODE_SIZE: usize = 4;

impl ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsgSdkMode {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_BATTERY_LEVEL_RESPONSE_SIZE {
            return Err((scroll::Error::Custom(
                "Not enough space available in byte array".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u8>(self.on, offset, ctx)?;
        data.gwrite_with::<u8>(self.flags, offset, ctx)?;

        Ok(*offset)
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgSetSpeed {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    speed_mm_per_sec: i16,
    accel_mm_per_sec2: i16,
    respect_road_piece_speed_limit: u8,
}

pub const ANKI_VEHICLE_MSG_SET_SPEED_SIZE: usize = 7;

impl ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsgSetSpeed {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_SET_SPEED_SIZE {
            return Err((scroll::Error::Custom(
                "Not enough space available in byte array".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<i16>(self.speed_mm_per_sec, offset, ctx)?;
        data.gwrite_with::<i16>(self.accel_mm_per_sec2, offset, ctx)?;
        data.gwrite_with::<u8>(self.respect_road_piece_speed_limit, offset, ctx)?;

        Ok(*offset)
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum VehicleTurn {
    None = 0,
    Left = 1,
    Right = 2,
    UTurn = 3,
    UTurnJump = 4,
}

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum VehicleTurnTrigger {
    // Run immediately
    Immediate = 0,
    // Run at the next intersection
    Intersection = 1,
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgTurn {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    turn_type: VehicleTurn,
    trigger: VehicleTurnTrigger,
}

pub const ANKI_VEHICLE_MSG_TURN_SIZE: usize = 4;

impl ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsgTurn {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_TURN_SIZE {
            return Err((scroll::Error::Custom(
                "Not enough space available in byte array".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u8>(
            self.turn_type
                .try_into()
                .unwrap_or_else(|_| VehicleTurn::None.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u8>(
            self.trigger
                .try_into()
                .unwrap_or_else(|_| VehicleTurnTrigger::Immediate.into()),
            offset,
            ctx,
        )?;

        Ok(*offset)
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgSetOffsetFromRoadCentre {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    offset_mm: f32,
}

pub const ANKI_VEHICLE_MSG_SET_OFFSET_FROM_ROAD_CENTRE_SIZE: usize = 6;

impl ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsgSetOffsetFromRoadCentre {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_SET_OFFSET_FROM_ROAD_CENTRE_SIZE {
            return Err((scroll::Error::Custom(
                "Not enough space available in byte array".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<f32>(self.offset_mm, offset, ctx)?;

        Ok(*offset)
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgChangeLane {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    horizontal_speed_mm_per_sec: u16,
    horizontal_accel_mm_per_sec2: u16,
    offset_from_road_centre_mm: f32,
    hop_intent: u8,
    tag: u8,
}

pub const ANKI_VEHICLE_MSG_CHANGE_LANE_SIZE: usize = 12;

impl ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsgChangeLane {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_CHANGE_LANE_SIZE {
            return Err((scroll::Error::Custom(
                "Not enough space available in byte array".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u16>(self.horizontal_speed_mm_per_sec, offset, ctx)?;
        data.gwrite_with::<u16>(self.horizontal_accel_mm_per_sec2, offset, ctx)?;
        data.gwrite_with::<f32>(self.offset_from_road_centre_mm, offset, ctx)?;
        data.gwrite_with::<u8>(self.hop_intent, offset, ctx)?;
        data.gwrite_with::<u8>(self.tag, offset, ctx)?;

        Ok(*offset)
    }
}

pub const PARSE_FLAGS_MASK_NUM_BITS: u8 = 0x0f;
pub const PARSE_FLAGS_MASK_INVERTED_COLOR: u8 = 0x80;
pub const PARSE_FLAGS_MASK_REVERSE_PARSING: u8 = 0x40;
pub const PARSE_FLAGS_MASK_REVERSE_DRIVING: u8 = 0x20;

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgLocalisationPositionUpdate {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    pub location_id: u8,
    pub road_piece_id: u8,
    pub offset_from_road_centre_mm: f32,
    pub speed_mm_per_sec: u16,
    pub parsing_flags: u8,

    /* ACK commands received */
    pub last_recv_lane_change_cmd_id: u8,
    pub last_exec_lane_change_cmd_id: u8,
    pub last_desired_lane_change_speed_mm_per_sec: u16,
    pub last_desired_speed_mm_per_sec: u16,
}

pub const ANKI_VEHICLE_MSG_LOCALISATION_POSITION_UPDATE_SIZE: usize = 17;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleMsgLocalisationPositionUpdate {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_LOCALISATION_POSITION_UPDATE_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let size: u8 = data.gread_with::<u8>(offset, ctx)?;
        let msg_id: AnkiVehicleMsgType = data
            .gread_with::<u8>(offset, ctx)?
            .try_into()
            .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown);
        let location_id: u8 = data.gread_with::<u8>(offset, ctx)?;
        let road_piece_id: u8 = data.gread_with::<u8>(offset, ctx)?;
        let offset_from_road_centre_mm: f32 = data.gread_with::<f32>(offset, ctx)?;
        let speed_mm_per_sec: u16 = data.gread_with::<u16>(offset, ctx)?;
        let parsing_flags: u8 = data.gread_with::<u8>(offset, ctx)?;
        let last_recv_lane_change_cmd_id: u8 = data.gread_with::<u8>(offset, ctx)?;
        let last_exec_lane_change_cmd_id: u8 = data.gread_with::<u8>(offset, ctx)?;
        let last_desired_lane_change_speed_mm_per_sec: u16 = data.gread_with::<u16>(offset, ctx)?;
        let last_desired_speed_mm_per_sec: u16 = data.gread_with::<u16>(offset, ctx)?;

        Ok((
            AnkiVehicleMsgLocalisationPositionUpdate {
                size,
                msg_id,
                location_id,
                road_piece_id,
                offset_from_road_centre_mm,
                speed_mm_per_sec,
                parsing_flags,
                last_recv_lane_change_cmd_id,
                last_exec_lane_change_cmd_id,
                last_desired_lane_change_speed_mm_per_sec,
                last_desired_speed_mm_per_sec,
            },
            *offset,
        ))
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
#[allow(unused)]
enum AnkiVehicleDrivingDirection {
    Forward = 0,
    Reverse = 1,
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgLocalisationTransitionUpdate {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    pub road_piece_idx: i8,
    pub road_piece_idx_prev: i8,
    pub offset_from_road_centre_mm: f32,

    /* ACK commands received */
    pub last_recv_lane_change_id: u8,
    pub last_exec_lane_change_id: u8,
    pub last_desired_lane_change_speed_mm_per_sec: u16,
    pub ave_follow_line_drift_pixels: i8,
    pub had_lane_change_activity: u8,

    /* track grade detection */
    pub uphill_counter: u8,
    pub downhill_counter: u8,

    /* wheel displacement (cm) since last transition bar */
    pub left_wheel_dist_cm: u8,
    pub right_wheel_dist_cm: u8,
}

pub const ANKI_VEHICLE_MSG_LOCALISATION_TRANSITION_UPDATE_SIZE: usize = 18;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleMsgLocalisationTransitionUpdate {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_LOCALISATION_TRANSITION_UPDATE_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let size: u8 = data.gread_with::<u8>(offset, ctx)?;
        let msg_id: AnkiVehicleMsgType = data
            .gread_with::<u8>(offset, ctx)?
            .try_into()
            .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown);
        let road_piece_idx: i8 = data.gread_with::<i8>(offset, ctx)?;
        let road_piece_idx_prev: i8 = data.gread_with::<i8>(offset, ctx)?;
        let offset_from_road_centre_mm: f32 = data.gread_with::<f32>(offset, ctx)?;
        let last_recv_lane_change_id: u8 = data.gread_with::<u8>(offset, ctx)?;
        let last_exec_lane_change_id: u8 = data.gread_with::<u8>(offset, ctx)?;
        let last_desired_lane_change_speed_mm_per_sec: u16 = data.gread_with::<u16>(offset, ctx)?;
        let ave_follow_line_drift_pixels: i8 = data.gread_with::<i8>(offset, ctx)?;
        let had_lane_change_activity: u8 = data.gread_with::<u8>(offset, ctx)?;
        let uphill_counter: u8 = data.gread_with::<u8>(offset, ctx)?;
        let downhill_counter: u8 = data.gread_with::<u8>(offset, ctx)?;
        let left_wheel_dist_cm: u8 = data.gread_with::<u8>(offset, ctx)?;
        let right_wheel_dist_cm: u8 = data.gread_with::<u8>(offset, ctx)?;

        Ok((
            AnkiVehicleMsgLocalisationTransitionUpdate {
                size,
                msg_id,
                road_piece_idx,
                road_piece_idx_prev,
                offset_from_road_centre_mm,
                last_recv_lane_change_id,
                last_exec_lane_change_id,
                last_desired_lane_change_speed_mm_per_sec,
                ave_follow_line_drift_pixels,
                had_lane_change_activity,
                uphill_counter,
                downhill_counter,
                left_wheel_dist_cm,
                right_wheel_dist_cm,
            },
            *offset,
        ))
    }
}

#[derive(Debug, PartialEq, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum IntersectionCode {
    None = 0,
    EntryFirst = 1,
    ExitFirst = 2,
    EntrySecond = 3,
    ExitSecond = 4,
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgLocalisationIntersectionUpdate {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    pub road_piece_idx: i8,
    pub offset_from_road_centre_mm: f32,

    pub intersection_code: IntersectionCode,
    pub is_exiting: u8,
    pub mm_since_last_transition_bar: u16,
    pub mm_since_last_intersection_code: u16,
}

pub const ANKI_VEHICLE_MSG_LOCALISATION_INTERSECTION_UPDATE_SIZE: usize = 13;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleMsgLocalisationIntersectionUpdate {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_LOCALISATION_INTERSECTION_UPDATE_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let size: u8 = data.gread_with::<u8>(offset, ctx)?;
        let msg_id: AnkiVehicleMsgType = data
            .gread_with::<u8>(offset, ctx)?
            .try_into()
            .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown);
        let road_piece_idx: i8 = data.gread_with::<i8>(offset, ctx)?;
        let offset_from_road_centre_mm: f32 = data.gread_with::<f32>(offset, ctx)?;
        let intersection_code: IntersectionCode = data
            .gread_with::<u8>(offset, ctx)?
            .try_into()
            .unwrap_or_else(|_| IntersectionCode::None);
        let is_exiting: u8 = data.gread_with::<u8>(offset, ctx)?;
        let mm_since_last_transition_bar: u16 = data.gread_with::<u16>(offset, ctx)?;
        let mm_since_last_intersection_code: u16 = data.gread_with::<u16>(offset, ctx)?;

        Ok((
            AnkiVehicleMsgLocalisationIntersectionUpdate {
                size,
                msg_id,
                road_piece_idx,
                offset_from_road_centre_mm,
                intersection_code,
                is_exiting,
                mm_since_last_transition_bar,
                mm_since_last_intersection_code,
            },
            *offset,
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgOffsetFromRoadCentreUpdate {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    pub offset_from_road_centre_mm: f32,
    pub lane_change_id: u8,
}

pub const ANKI_VEHICLE_MSG_OFFSET_FROM_ROAD_CENTRE_UPDATE_SIZE: usize = 7;

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for AnkiVehicleMsgOffsetFromRoadCentreUpdate {
    type Error = scroll::Error;
    fn try_from_ctx(data: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_OFFSET_FROM_ROAD_CENTRE_UPDATE_SIZE {
            return Err((scroll::Error::Custom("Incorrect num of bytes".to_string())).into());
        }

        let offset = &mut 0;
        let size: u8 = data.gread_with::<u8>(offset, ctx)?;
        let msg_id: AnkiVehicleMsgType = data
            .gread_with::<u8>(offset, ctx)?
            .try_into()
            .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown);
        let offset_from_road_centre_mm: f32 = data.gread_with::<f32>(offset, ctx)?;
        let lane_change_id: u8 = data.gread_with::<u8>(offset, ctx)?;

        Ok((
            AnkiVehicleMsgOffsetFromRoadCentreUpdate {
                size,
                msg_id,
                offset_from_road_centre_mm,
                lane_change_id,
            },
            *offset,
        ))
    }
}

// TODO: Work out what this is used for. Think it is for the helper macros below.
#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
#[allow(unused)]
enum Light {
    Headlights = 0,
    BrakeLights = 1,
    FrontLights = 2,
    Engine = 3,
}

// TODO: Helper macros for parsing lights bits

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgSetLights {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    light_mask: u8, // Valid and value bits for lights (see above)
}

pub const ANKI_VEHICLE_MSG_SET_LIGHTS_SIZE: usize = 3;

impl ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsgSetLights {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_SET_LIGHTS_SIZE {
            return Err((scroll::Error::Custom(
                "Not enough space available in byte array".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u8>(self.light_mask, offset, ctx)?;

        Ok(*offset)
    }
}

// TODO: Check type requirements of these below
pub const ANKI_VEHICLE_MAX_LIGHT_INTENSITY: u8 = 14;
pub const ANKI_VEHICLE_MAX_LIGHT_TIME: u8 = 11;

#[derive(Debug, PartialEq, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum LightChannel {
    Red = 0,
    Tail = 1,
    Blue = 2,
    Green = 3,
    FrontL = 4,
    FrontR = 5,
    Count = 6,
}

#[derive(Debug, PartialEq, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum LightEffect {
    // Simply set the light intensity to 'start' value
    Steady = 0,
    // Fade intensity from 'start' to 'end'
    Fade = 1,
    // Fade intensity from 'start' to 'end' and back to 'start'
    Throb = 2,
    // Turn on LED between time 'start' and time 'end' inclusive
    Flash = 3,
    // Flash the LED erratically - ignoring start/end
    Random = 4,
    Count = 5,
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleLightConfig {
    channel: LightChannel,
    effect: LightEffect,
    start: u8,
    end: u8,
    cycles_per_10_sec: u8,
}

const LIGHT_CHANNEL_COUNT_MAX: usize = 3;
pub const ANKI_VEHICLE_LIGHT_CONFIG_SIZE: usize = 5;

impl ctx::TryIntoCtx<scroll::Endian> for &AnkiVehicleLightConfig {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        // TODO: This might break if a bigger size data is inputted.
        if data.len() < ANKI_VEHICLE_LIGHT_CONFIG_SIZE || data.len() > ANKI_VEHICLE_MSG_MAX_SIZE {
            return Err((scroll::Error::Custom(
                "Invalid space requirements in byte array. data_len:"
                    .to_string()
                    .add(&*(data.len().to_string())),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(
            self.channel
                .clone()
                .try_into()
                .unwrap_or_else(|_| LightChannel::Tail.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u8>(
            self.effect
                .clone()
                .try_into()
                .unwrap_or_else(|_| LightEffect::Steady.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u8>(self.start, offset, ctx)?;
        data.gwrite_with::<u8>(self.end, offset, ctx)?;
        data.gwrite_with::<u8>(self.cycles_per_10_sec, offset, ctx)?;

        Ok(*offset)
    }
}

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgLightsPattern {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    channel_count: u8,
    channel_config: [Option<AnkiVehicleLightConfig>; LIGHT_CHANNEL_COUNT_MAX],
}

pub const ANKI_VEHICLE_MSG_LIGHTS_PATTERN_SIZE: usize =
    (LIGHT_CHANNEL_COUNT_MAX * ANKI_VEHICLE_LIGHT_CONFIG_SIZE) + 3;

impl ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsgLightsPattern {
    type Error = scroll::Error;
    fn try_into_ctx<'a>(
        self,
        data: &'a mut [u8],
        ctx: scroll::Endian,
    ) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_LIGHTS_PATTERN_SIZE {
            return Err((scroll::Error::Custom(
                "Not enough space available in byte array".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u8>(self.channel_count, offset, ctx)?;

        for i in 0..LIGHT_CHANNEL_COUNT_MAX {
            // TODO: This could panic if wrong arguments entered.
            let config = self.channel_config.get(i).unwrap().as_ref();
            match config {
                None => {
                    data.gwrite_with::<&'a [u8]>(
                        &[0u8; ANKI_VEHICLE_LIGHT_CONFIG_SIZE as usize],
                        offset,
                        (),
                    )?;
                }
                Some(c) => {
                    data.gwrite_with::<&AnkiVehicleLightConfig>(c, offset, ctx)?;
                }
            }
        }

        Ok(*offset)
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum TrackMaterial {
    Plastic = 0,
    Vinyl = 1,
}

pub const SUPERCODE_NONE: u8 = 0;
pub const SUPERCODE_BOOST_JUMP: u8 = 1;
pub const SUPERCODE_ALL: u8 = SUPERCODE_BOOST_JUMP;

#[derive(Debug, PartialEq)]
pub struct AnkiVehicleMsgSetConfigParams {
    size: u8,
    msg_id: AnkiVehicleMsgType,
    super_code_parse_mask: u8,
    track_material: TrackMaterial,
}

pub const ANKI_VEHICLE_MSG_SET_CONFIG_PARAMS_SIZE: usize = 4;

impl ctx::TryIntoCtx<scroll::Endian> for AnkiVehicleMsgSetConfigParams {
    type Error = scroll::Error;
    fn try_into_ctx(self, data: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        if data.len() != ANKI_VEHICLE_MSG_SET_CONFIG_PARAMS_SIZE {
            return Err((scroll::Error::Custom(
                "Not enough space available in byte array".to_string(),
            ))
            .into());
        }

        let offset = &mut 0;
        data.gwrite_with::<u8>(self.size, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.msg_id
                .try_into()
                .unwrap_or_else(|_| AnkiVehicleMsgType::Unknown.into()),
            offset,
            ctx,
        )?;
        data.gwrite_with::<u8>(self.super_code_parse_mask, offset, ctx)?;
        data.gwrite_with::<u8>(
            self.track_material
                .try_into()
                .unwrap_or_else(|_| TrackMaterial::Plastic.into()),
            offset,
            ctx,
        )?;

        Ok(*offset)
    }
}

pub fn anki_vehicle_msg_set_sdk_mode(on: u8, flags: u8) -> AnkiVehicleMsgSdkMode {
    AnkiVehicleMsgSdkMode {
        size: ANKI_VEHICLE_MSG_SDK_MODE_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VSDKMode,
        on,
        flags,
    }
}

pub fn anki_vehicle_msg_set_speed(
    speed_mm_per_sec: i16,
    accel_mm_per_sec2: i16,
) -> AnkiVehicleMsgSetSpeed {
    AnkiVehicleMsgSetSpeed {
        size: ANKI_VEHICLE_MSG_SET_SPEED_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VSetSpeed,
        speed_mm_per_sec,
        accel_mm_per_sec2,
        respect_road_piece_speed_limit: 0,
    }
}

pub fn anki_vehicle_msg_set_offset_from_road_centre(
    offset_mm: f32,
) -> AnkiVehicleMsgSetOffsetFromRoadCentre {
    AnkiVehicleMsgSetOffsetFromRoadCentre {
        size: ANKI_VEHICLE_MSG_SET_OFFSET_FROM_ROAD_CENTRE_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VSetOffsetFromRoadCentre,
        offset_mm,
    }
}

pub fn anki_vehicle_msg_change_lane(
    horizontal_speed_mm_per_sec: u16,
    horizontal_accel_mm_per_sec2: u16,
    offset_from_road_centre_mm: f32,
) -> AnkiVehicleMsgChangeLane {
    AnkiVehicleMsgChangeLane {
        size: ANKI_VEHICLE_MSG_CHANGE_LANE_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VChangeLane,
        horizontal_speed_mm_per_sec,
        horizontal_accel_mm_per_sec2,
        offset_from_road_centre_mm,
        hop_intent: 0,
        tag: 0,
    }
}

pub fn anki_vehicle_msg_set_lights(mask: u8) -> AnkiVehicleMsgSetLights {
    AnkiVehicleMsgSetLights {
        size: ANKI_VEHICLE_MSG_SET_LIGHTS_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VSetLights,
        light_mask: mask,
    }
}

pub fn anki_vehicle_light_config(
    channel: LightChannel,
    effect: LightEffect,
    start: u8,
    end: u8,
    cycles_per_min: u16,
) -> AnkiVehicleLightConfig {
    AnkiVehicleLightConfig {
        channel,
        effect,
        start,
        end,
        cycles_per_10_sec: (cycles_per_min / 6) as u8,
    }
}

pub fn anki_vehicle_msg_lights_pattern(
    channel: LightChannel,
    effect: LightEffect,
    start: u8,
    end: u8,
    cycles_per_min: u16,
) -> AnkiVehicleMsgLightsPattern {
    AnkiVehicleMsgLightsPattern {
        size: ANKI_VEHICLE_MSG_LIGHTS_PATTERN_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VLightsPattern,
        channel_count: 1,
        channel_config: [
            Some(AnkiVehicleLightConfig {
                channel,
                effect,
                start,
                end,
                cycles_per_10_sec: (cycles_per_min / 6) as u8,
            }),
            None,
            None,
        ],
    }
}

impl AnkiVehicleMsgLightsPattern {
    pub fn append(&mut self, config: AnkiVehicleLightConfig) -> u8 {
        if self.channel_count >= 3 {
            return 0;
        }
        self.channel_config[self.channel_count as usize] = Some(config);
        self.channel_count += 1;
        self.channel_count
    }
}

pub const ANKI_VEHICLE_MSG_PING_SIZE: usize = ANKI_VEHICLE_MSG_BASE_SIZE;

pub fn anki_vehicle_msg_ping<'a>() -> AnkiVehicleMsg<'a> {
    AnkiVehicleMsg {
        size: ANKI_VEHICLE_MSG_BASE_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2CPingRequest,
        payload: &[],
    }
}

pub const ANKI_VEHICLE_MSG_DISCONNECT_SIZE: usize = ANKI_VEHICLE_MSG_BASE_SIZE;

pub fn anki_vehicle_msg_disconnect() -> AnkiVehicleMsg<'static> {
    AnkiVehicleMsg {
        size: ANKI_VEHICLE_MSG_BASE_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VDisconnect,
        payload: &[],
    }
}

pub const ANKI_VEHICLE_MSG_VERSION_REQUEST_SIZE: usize = ANKI_VEHICLE_MSG_BASE_SIZE;

pub fn anki_vehicle_msg_get_version() -> AnkiVehicleMsg<'static> {
    AnkiVehicleMsg {
        size: ANKI_VEHICLE_MSG_BASE_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VVersionRequest,
        payload: &[],
    }
}

pub const ANKI_VEHICLE_MSG_BATTERY_LEVEL_REQUEST_SIZE: usize = ANKI_VEHICLE_MSG_BASE_SIZE;

pub fn anki_vehicle_msg_get_battery_level() -> AnkiVehicleMsg<'static> {
    AnkiVehicleMsg {
        size: ANKI_VEHICLE_MSG_BASE_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VBatteryLevelRequest,
        payload: &[],
    }
}

pub const ANKI_VEHICLE_MSG_CANCEL_LANE_CHANGE_SIZE: usize = ANKI_VEHICLE_MSG_BASE_SIZE;

pub fn anki_vehicle_msg_cancel_lane_change() -> AnkiVehicleMsg<'static> {
    AnkiVehicleMsg {
        size: ANKI_VEHICLE_MSG_BASE_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VCancelLaneChange,
        payload: &[],
    }
}

pub fn anki_vehicle_msg_turn(
    turn_type: VehicleTurn,
    trigger: VehicleTurnTrigger,
) -> AnkiVehicleMsgTurn {
    AnkiVehicleMsgTurn {
        size: ANKI_VEHICLE_MSG_TURN_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VTurn,
        turn_type,
        trigger,
    }
}

pub fn anki_vehicle_msg_turn_180() -> AnkiVehicleMsgTurn {
    AnkiVehicleMsgTurn {
        size: ANKI_VEHICLE_MSG_TURN_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VTurn,
        turn_type: VehicleTurn::UTurn,
        trigger: VehicleTurnTrigger::Immediate,
    }
}

pub fn anki_vehicle_msg_set_config_params(
    super_code_parse_mask: u8,
    track_material: TrackMaterial,
) -> AnkiVehicleMsgSetConfigParams {
    AnkiVehicleMsgSetConfigParams {
        size: ANKI_VEHICLE_MSG_SET_CONFIG_PARAMS_SIZE as u8 - 1,
        msg_id: AnkiVehicleMsgType::C2VSetConfigParams,
        super_code_parse_mask,
        track_material,
    }
}

#[cfg(test)]
mod tests {
    use scroll::{Pread, BE};

    use super::*;

    #[test]
    fn anki_vehicle_msg_version_response_struct_test() {
        let data: &[u8; ANKI_VEHICLE_MSG_VERSION_RESPONSE_SIZE] = &[
            0x3,
            AnkiVehicleMsgType::V2CVersionResponse as u8,
            0xAB,
            0xCD,
        ];
        let msg: AnkiVehicleMsgVersionResponse = AnkiVehicleMsgVersionResponse {
            size: 3,
            msg_id: AnkiVehicleMsgType::V2CVersionResponse,
            version: 0xABCD,
        };
        let test_msg = data
            .gread_with::<AnkiVehicleMsgVersionResponse>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, msg);
        assert_eq!(msg, test_msg)
    }

    #[test]
    fn anki_vehicle_msg_battery_level_response_struct_test() {
        let data: &[u8; ANKI_VEHICLE_MSG_BATTERY_LEVEL_RESPONSE_SIZE] = &[
            0x3,
            AnkiVehicleMsgType::V2CBatteryLevelResponse as u8,
            0xAB,
            0xCD,
        ];
        let msg: AnkiVehicleMsgBatteryLevelResponse = AnkiVehicleMsgBatteryLevelResponse {
            size: 3,
            msg_id: AnkiVehicleMsgType::V2CBatteryLevelResponse,
            battery_level: 0xABCD,
        };
        let test_msg = data
            .gread_with::<AnkiVehicleMsgBatteryLevelResponse>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, msg);
        assert_eq!(msg, test_msg)
    }

    #[test]
    fn anki_vehicle_msg_localisation_position_update_struct_test() {
        let data: &[u8; ANKI_VEHICLE_MSG_LOCALISATION_POSITION_UPDATE_SIZE] = &[
            16,
            AnkiVehicleMsgType::V2CLocalisationPositionUpdate as u8,
            0xA,
            0xB,
            66,
            200,
            0,
            0,
            0xCD,
            0xEF,
            1,
            2,
            3,
            0x44,
            0x55,
            0x66,
            0x77,
        ];
        let msg: AnkiVehicleMsgLocalisationPositionUpdate =
            AnkiVehicleMsgLocalisationPositionUpdate {
                size: 16,
                msg_id: AnkiVehicleMsgType::V2CLocalisationPositionUpdate,
                location_id: 0xA,
                road_piece_id: 0xB,
                offset_from_road_centre_mm: 100.0,
                speed_mm_per_sec: 0xCDEF,
                parsing_flags: 1,
                last_recv_lane_change_cmd_id: 2,
                last_exec_lane_change_cmd_id: 3,
                last_desired_lane_change_speed_mm_per_sec: 0x4455,
                last_desired_speed_mm_per_sec: 0x6677,
            };
        let test_msg = data
            .gread_with::<AnkiVehicleMsgLocalisationPositionUpdate>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, msg);
        assert_eq!(msg, test_msg)
    }

    #[test]
    fn anki_vehicle_msg_localisation_transition_update_struct_test() {
        let data: &[u8; ANKI_VEHICLE_MSG_LOCALISATION_TRANSITION_UPDATE_SIZE] = &[
            17,
            AnkiVehicleMsgType::V2CLocalisationTransitionUpdate as u8,
            0xA,
            0xB,
            66,
            200,
            0,
            0,
            0xC,
            0xD,
            0x7E,
            0xF0,
            1,
            0x1,
            0x2,
            0x3,
            0x4,
            0x5,
        ];
        let msg: AnkiVehicleMsgLocalisationTransitionUpdate =
            AnkiVehicleMsgLocalisationTransitionUpdate {
                size: 17,
                msg_id: AnkiVehicleMsgType::V2CLocalisationTransitionUpdate,
                road_piece_idx: 0xA,
                road_piece_idx_prev: 0xB,
                offset_from_road_centre_mm: 100.0,
                last_recv_lane_change_id: 0xC,
                last_exec_lane_change_id: 0xD,
                last_desired_lane_change_speed_mm_per_sec: 0x7EF0,
                ave_follow_line_drift_pixels: 1,
                had_lane_change_activity: 0x1,
                uphill_counter: 0x2,
                downhill_counter: 0x3,
                left_wheel_dist_cm: 0x4,
                right_wheel_dist_cm: 0x5,
            };
        let test_msg = data
            .gread_with::<AnkiVehicleMsgLocalisationTransitionUpdate>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, msg);
        assert_eq!(msg, test_msg)
    }

    #[test]
    fn anki_vehicle_msg_localisation_intersection_update_struct_test() {
        let data: &[u8; ANKI_VEHICLE_MSG_LOCALISATION_INTERSECTION_UPDATE_SIZE] = &[
            12,
            AnkiVehicleMsgType::V2CLocalisationIntersectionUpdate as u8,
            1,
            66,
            200,
            0,
            0,
            IntersectionCode::EntryFirst as u8,
            0xB,
            0xCD,
            0xEF,
            0x12,
            0x34,
        ];
        let msg: AnkiVehicleMsgLocalisationIntersectionUpdate =
            AnkiVehicleMsgLocalisationIntersectionUpdate {
                size: 12,
                msg_id: AnkiVehicleMsgType::V2CLocalisationIntersectionUpdate,
                road_piece_idx: 1,
                offset_from_road_centre_mm: 100.0,
                intersection_code: IntersectionCode::EntryFirst,
                is_exiting: 0xB,
                mm_since_last_transition_bar: 0xCDEF,
                mm_since_last_intersection_code: 0x1234,
            };
        let test_msg = data
            .gread_with::<AnkiVehicleMsgLocalisationIntersectionUpdate>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, msg);
        assert_eq!(msg, test_msg)
    }

    #[test]
    fn anki_vehicle_msg_offset_from_road_centre_update_struct_test() {
        let data: &[u8; ANKI_VEHICLE_MSG_OFFSET_FROM_ROAD_CENTRE_UPDATE_SIZE] = &[
            6,
            AnkiVehicleMsgType::V2COffsetFromRoadCentreUpdate as u8,
            66,
            200,
            0,
            0,
            0xAB,
        ];
        let msg: AnkiVehicleMsgOffsetFromRoadCentreUpdate =
            AnkiVehicleMsgOffsetFromRoadCentreUpdate {
                size: 6,
                msg_id: AnkiVehicleMsgType::V2COffsetFromRoadCentreUpdate,
                offset_from_road_centre_mm: 100.0,
                lane_change_id: 0xAB,
            };
        let test_msg = data
            .gread_with::<AnkiVehicleMsgOffsetFromRoadCentreUpdate>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, msg);
        assert_eq!(msg, test_msg)
    }
}
