extern crate core;

use scroll::Pwrite;

use crate::protocol::{
    anki_vehicle_msg_change_lane, anki_vehicle_msg_set_sdk_mode,
    AnkiVehicleMsgBatteryLevelResponse, AnkiVehicleMsgChangeLane,
    AnkiVehicleMsgLocalisationIntersectionUpdate, AnkiVehicleMsgLocalisationPositionUpdate,
    AnkiVehicleMsgLocalisationTransitionUpdate, AnkiVehicleMsgOffsetFromRoadCentreUpdate,
    AnkiVehicleMsgSdkMode, AnkiVehicleMsgVersionResponse, IntersectionCode,
    ANKI_VEHICLE_MSG_CHANGE_LANE_SIZE, ANKI_VEHICLE_MSG_SDK_MODE_SIZE,
    ANKI_VEHICLE_SDK_OPTION_OVERRIDE_LOCALIZATION,
};

pub mod advertisement;
pub mod protocol;
pub mod vehicle_gatt_profile;

pub struct AnkiVehicle<'a> {
    name: &'a str,
    bt_address: &'a str,

    version: u16,
    battery_level: u16,
    sdk_mode_on: bool,

    // Position Info
    speed_mm_per_sec: u16,
    offset_from_road_centre_mm: f32,
    location_id: u8,
    // Driving State Info
    parsing_flags: u8,

    // Additional Speed Info
    last_desired_speed_mm_per_sec: u16,
    last_desired_lane_change_speed_mm_per_sec: u16,

    // Transition Info
    road_piece_idx_prev: i8,
    road_piece_idx: i8,
    uphill_counter: u8,
    downhill_counter: u8,
    left_wheel_dist_cm: u8,
    right_wheel_dist_cm: u8,

    // Intersection Info
    intersection_code: IntersectionCode,
    is_exiting_intersection: u8,
    mm_since_last_transition_bar: u16,
    mm_since_last_intersection_code: u16,
    //TODO: Lighting
}

impl<'a> AnkiVehicle<'a> {
    pub fn new(mut self, name: &'a str, bt_address: &'a str) {
        self.name = name;
        self.bt_address = bt_address;
    }

    pub fn configure(self) -> Vec<Vec<u8>> {
        let mut commands: Vec<Vec<u8>> = Vec::new();

        let msg: AnkiVehicleMsgChangeLane = anki_vehicle_msg_change_lane(300, 2500, 0.0);
        let mut lane_reset = [0u8; ANKI_VEHICLE_MSG_CHANGE_LANE_SIZE]; // TODO: fix lifetimes problem
        let offset = lane_reset
            .pwrite_with::<AnkiVehicleMsgChangeLane>(msg, 0, scroll::LE)
            .expect("Failed to write AnkiVehicleMsgChangeLane as bytes");

        commands.push(lane_reset[..offset].to_vec());

        if self.sdk_mode_on {
            return commands;
        }

        let msg: AnkiVehicleMsgSdkMode =
            anki_vehicle_msg_set_sdk_mode(1, ANKI_VEHICLE_SDK_OPTION_OVERRIDE_LOCALIZATION);
        let mut sdk_mode = [0u8; ANKI_VEHICLE_MSG_SDK_MODE_SIZE]; // TODO: fix lifetimes problem
        let offset = sdk_mode
            .pwrite_with::<AnkiVehicleMsgSdkMode>(msg, 0, scroll::LE)
            .expect("Failed to write AnkiVehicleMsgSdkMode as bytes");

        commands.insert(0, sdk_mode[..offset].to_vec());

        commands
    }

    pub fn process_battery_level_response(&mut self, data: AnkiVehicleMsgBatteryLevelResponse) {
        self.battery_level = data.battery_level;
    }

    pub fn process_version_response(&mut self, data: AnkiVehicleMsgVersionResponse) {
        self.version = data.version;
    }

    pub fn process_position_update(&mut self, data: AnkiVehicleMsgLocalisationPositionUpdate) {
        self.location_id = data.location_id;
        self.offset_from_road_centre_mm = data.offset_from_road_centre_mm;
        self.speed_mm_per_sec = data.speed_mm_per_sec;
        self.parsing_flags = data.parsing_flags;
        self.last_desired_lane_change_speed_mm_per_sec =
            data.last_desired_lane_change_speed_mm_per_sec;
        self.last_desired_speed_mm_per_sec = data.last_desired_speed_mm_per_sec;
    }

    pub fn process_transition_update(&mut self, data: AnkiVehicleMsgLocalisationTransitionUpdate) {
        self.road_piece_idx = data.road_piece_idx;
        self.road_piece_idx_prev = data.road_piece_idx_prev;
        self.offset_from_road_centre_mm = data.offset_from_road_centre_mm;
        self.last_desired_lane_change_speed_mm_per_sec =
            data.last_desired_lane_change_speed_mm_per_sec;
        self.uphill_counter = data.uphill_counter;
        self.downhill_counter = data.downhill_counter;
        self.left_wheel_dist_cm = data.left_wheel_dist_cm;
        self.right_wheel_dist_cm = data.right_wheel_dist_cm;
    }

    pub fn process_intersection_update(
        &mut self,
        data: AnkiVehicleMsgLocalisationIntersectionUpdate,
    ) {
        self.offset_from_road_centre_mm = data.offset_from_road_centre_mm;
        self.intersection_code = data.intersection_code;
        self.is_exiting_intersection = data.is_exiting;
        self.mm_since_last_transition_bar = data.mm_since_last_transition_bar;
        self.mm_since_last_intersection_code = data.mm_since_last_intersection_code;
    }

    pub fn process_offset_from_road_centre_update(
        &mut self,
        data: AnkiVehicleMsgOffsetFromRoadCentreUpdate,
    ) {
        self.offset_from_road_centre_mm = data.offset_from_road_centre_mm;
    }
}

#[cfg(test)]
mod tests {
    use scroll::{Pread, Pwrite, BE};

    use crate::protocol::{
        AnkiVehicleMsgType, LightChannel, LightEffect, VehicleTurn, VehicleTurnTrigger,
        ANKI_VEHICLE_LIGHT_CONFIG_SIZE, ANKI_VEHICLE_MSG_BATTERY_LEVEL_REQUEST_SIZE,
        ANKI_VEHICLE_MSG_BATTERY_LEVEL_RESPONSE_SIZE, ANKI_VEHICLE_MSG_CANCEL_LANE_CHANGE_SIZE,
        ANKI_VEHICLE_MSG_CHANGE_LANE_SIZE, ANKI_VEHICLE_MSG_DISCONNECT_SIZE,
        ANKI_VEHICLE_MSG_LIGHTS_PATTERN_SIZE,
        ANKI_VEHICLE_MSG_LOCALISATION_INTERSECTION_UPDATE_SIZE,
        ANKI_VEHICLE_MSG_LOCALISATION_POSITION_UPDATE_SIZE,
        ANKI_VEHICLE_MSG_LOCALISATION_TRANSITION_UPDATE_SIZE,
        ANKI_VEHICLE_MSG_OFFSET_FROM_ROAD_CENTRE_UPDATE_SIZE, ANKI_VEHICLE_MSG_PING_SIZE,
        ANKI_VEHICLE_MSG_SDK_MODE_SIZE, ANKI_VEHICLE_MSG_SET_CONFIG_PARAMS_SIZE,
        ANKI_VEHICLE_MSG_SET_LIGHTS_SIZE, ANKI_VEHICLE_MSG_SET_OFFSET_FROM_ROAD_CENTRE_SIZE,
        ANKI_VEHICLE_MSG_SET_SPEED_SIZE, ANKI_VEHICLE_MSG_TURN_SIZE,
        ANKI_VEHICLE_MSG_VERSION_REQUEST_SIZE, ANKI_VEHICLE_MSG_VERSION_RESPONSE_SIZE,
        SUPERCODE_BOOST_JUMP,
    };

    #[test]
    fn test() {
        use crate::protocol::{anki_vehicle_msg_set_speed, AnkiVehicleMsgSetSpeed};

        let msg: AnkiVehicleMsgSetSpeed = anki_vehicle_msg_set_speed(2, 25);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_SET_SPEED_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgSetSpeed>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgSetSpeed as bytes");
        println!("AnkiVehicleMsgSetSpeed T:{:?}", test_data);
    }

    #[test]
    fn anki_vehicle_msg_struct_read<'a>() {
        use crate::protocol::{anki_vehicle_msg_ping, AnkiVehicleMsg};

        let data: &'a [u8; ANKI_VEHICLE_MSG_PING_SIZE] = &[0x1, 0x16];
        let msg: AnkiVehicleMsg = anki_vehicle_msg_ping();
        let test_msg = data.gread_with::<AnkiVehicleMsg<'a>>(&mut 0, BE).unwrap();
        println!("T:{:?} == G:{:?}", test_msg, msg);
        assert_eq!(msg, test_msg)
    }

    #[test]
    fn anki_vehicle_msg_struct_write<'a>() {
        use crate::protocol::{anki_vehicle_msg_ping, AnkiVehicleMsg};

        let data: &[u8; ANKI_VEHICLE_MSG_PING_SIZE] =
            &[0x1, AnkiVehicleMsgType::C2CPingRequest as u8];
        let msg: AnkiVehicleMsg<'a> = anki_vehicle_msg_ping();
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_PING_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsg<'a>>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgSdkMode as bytes");
        println!("AnkiVehicleMsgSdkMode T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_check_and_read<'a>() {
        use crate::protocol::{AnkiVehicleMsg, AnkiVehicleMsgBatteryLevelResponse};

        let data: &'a [u8; ANKI_VEHICLE_MSG_BATTERY_LEVEL_RESPONSE_SIZE] = &[
            0x3,
            AnkiVehicleMsgType::V2CBatteryLevelResponse as u8,
            0xAB,
            0xCD,
        ];

        let msg = data.gread_with::<AnkiVehicleMsg>(&mut 0, BE).unwrap();
        if msg.msg_id == AnkiVehicleMsgType::V2CBatteryLevelResponse {
            let test_msg = data
                .gread_with::<AnkiVehicleMsgBatteryLevelResponse>(&mut 0, BE)
                .unwrap();
            println!("T:{:?} == G:{:?}", test_msg, data);
            assert_eq!(0xABCD, test_msg.battery_level)
        } else {
            panic!["Message not of type V2CBatteryLevelResponse"]
        }
    }

    #[test]
    fn anki_vehicle_msg_version_response_struct_test() {
        use crate::protocol::AnkiVehicleMsgVersionResponse;

        let data: &[u8; ANKI_VEHICLE_MSG_VERSION_RESPONSE_SIZE] = &[
            0x3,
            AnkiVehicleMsgType::V2CVersionResponse as u8,
            0xAB,
            0xCD,
        ];
        let test_msg = data
            .gread_with::<AnkiVehicleMsgVersionResponse>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, data);
        assert_eq!(0xABCD, test_msg.version)
    }

    #[test]
    fn anki_vehicle_msg_battery_level_response_struct_test() {
        use crate::protocol::AnkiVehicleMsgBatteryLevelResponse;

        let data: &[u8; ANKI_VEHICLE_MSG_BATTERY_LEVEL_RESPONSE_SIZE] = &[
            0x3,
            AnkiVehicleMsgType::V2CBatteryLevelResponse as u8,
            0xAB,
            0xCD,
        ];
        let test_msg = data
            .gread_with::<AnkiVehicleMsgBatteryLevelResponse>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, data);
        assert_eq!(0xABCD, test_msg.battery_level)
    }

    #[test]
    fn anki_vehicle_msg_sdk_mode_test() {
        use crate::protocol::{anki_vehicle_msg_set_sdk_mode, AnkiVehicleMsgSdkMode};

        let data: &[u8; ANKI_VEHICLE_MSG_SDK_MODE_SIZE] =
            &[0x3, AnkiVehicleMsgType::C2VSDKMode as u8, 0x01, 0x00];
        let msg: AnkiVehicleMsgSdkMode = anki_vehicle_msg_set_sdk_mode(1, 0);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_SDK_MODE_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgSdkMode>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgSdkMode as bytes");
        println!("AnkiVehicleMsgSdkMode T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_set_speed_test() {
        use crate::protocol::{anki_vehicle_msg_set_speed, AnkiVehicleMsgSetSpeed};

        let data: &[u8; ANKI_VEHICLE_MSG_SET_SPEED_SIZE] = &[
            0x6,
            AnkiVehicleMsgType::C2VSetSpeed as u8,
            0x7B,
            0xCD,
            0x7B,
            0xCD,
            0x0,
        ];
        let msg: AnkiVehicleMsgSetSpeed = anki_vehicle_msg_set_speed(0x7BCD, 0x7BCD);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_SET_SPEED_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgSetSpeed>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgSetSpeed as bytes");
        println!("AnkiVehicleMsgSetSpeed T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_turn_test() {
        use crate::protocol::{anki_vehicle_msg_turn, AnkiVehicleMsgTurn};

        let data: &[u8; ANKI_VEHICLE_MSG_TURN_SIZE] =
            &[0x3, AnkiVehicleMsgType::C2VTurn as u8, 0x1, 0x1];
        let msg: AnkiVehicleMsgTurn =
            anki_vehicle_msg_turn(VehicleTurn::Left, VehicleTurnTrigger::Intersection);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_TURN_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgTurn>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgTurn as bytes");
        println!("AnkiVehicleMsgTurn T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_set_offset_from_road_centre_test() {
        use crate::protocol::{
            anki_vehicle_msg_set_offset_from_road_centre, AnkiVehicleMsgSetOffsetFromRoadCentre,
        };

        let data: &[u8; ANKI_VEHICLE_MSG_SET_OFFSET_FROM_ROAD_CENTRE_SIZE] = &[
            5,
            AnkiVehicleMsgType::C2VSetOffsetFromRoadCentre as u8,
            66,
            200,
            0,
            0,
        ];
        let msg: AnkiVehicleMsgSetOffsetFromRoadCentre =
            anki_vehicle_msg_set_offset_from_road_centre(100.0);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_SET_OFFSET_FROM_ROAD_CENTRE_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgSetOffsetFromRoadCentre>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgSetOffsetFromRoadCentre as bytes");
        println!(
            "AnkiVehicleMsgSetOffsetFromRoadCentre T:{:?} == G:{:?}",
            test_data, data
        );
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_change_lane_test() {
        use crate::protocol::{anki_vehicle_msg_change_lane, AnkiVehicleMsgChangeLane};

        let data: &[u8; ANKI_VEHICLE_MSG_CHANGE_LANE_SIZE] = &[
            11,
            AnkiVehicleMsgType::C2VChangeLane as u8,
            0,
            10,
            0,
            100,
            65,
            160,
            0,
            0,
            0,
            0,
        ];
        let msg: AnkiVehicleMsgChangeLane = anki_vehicle_msg_change_lane(10, 100, 20.0);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_CHANGE_LANE_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgChangeLane>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgChangeLane as bytes");
        println!("AnkiVehicleMsgChangeLane T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_localisation_position_update_struct_test() {
        use crate::protocol::AnkiVehicleMsgLocalisationPositionUpdate;

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
        let test_msg = data
            .gread_with::<AnkiVehicleMsgLocalisationPositionUpdate>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, data);
        assert_eq!(0xA, test_msg.location_id);
        assert_eq!(0xB, test_msg.road_piece_id);
        assert_eq!(100.0, test_msg.offset_from_road_centre_mm);
        assert_eq!(0xCDEF, test_msg.speed_mm_per_sec);
        assert_eq!(0x1, test_msg.parsing_flags);
        assert_eq!(0x2, test_msg.last_recv_lane_change_cmd_id);
        assert_eq!(0x3, test_msg.last_exec_lane_change_cmd_id);
        assert_eq!(0x4455, test_msg.last_desired_lane_change_speed_mm_per_sec);
        assert_eq!(0x6677, test_msg.last_desired_speed_mm_per_sec);
    }

    #[test]
    fn anki_vehicle_msg_localisation_transition_update_struct_test() {
        use crate::protocol::AnkiVehicleMsgLocalisationTransitionUpdate;

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
        let test_msg = data
            .gread_with::<AnkiVehicleMsgLocalisationTransitionUpdate>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, data);
        assert_eq!(0xA, test_msg.road_piece_idx);
        assert_eq!(0xB, test_msg.road_piece_idx_prev);
        assert_eq!(100.0, test_msg.offset_from_road_centre_mm);
        assert_eq!(0xC, test_msg.last_recv_lane_change_id);
        assert_eq!(0xD, test_msg.last_exec_lane_change_id);
        assert_eq!(0x7EF0, test_msg.last_desired_lane_change_speed_mm_per_sec);
        assert_eq!(1, test_msg.ave_follow_line_drift_pixels);
        assert_eq!(0x1, test_msg.had_lane_change_activity);
        assert_eq!(0x2, test_msg.uphill_counter);
        assert_eq!(0x3, test_msg.downhill_counter);
        assert_eq!(0x4, test_msg.left_wheel_dist_cm);
        assert_eq!(0x5, test_msg.right_wheel_dist_cm);
    }

    #[test]
    fn anki_vehicle_msg_localisation_intersection_update_struct_test() {
        use crate::protocol::{AnkiVehicleMsgLocalisationIntersectionUpdate, IntersectionCode};

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
        let test_msg = data
            .gread_with::<AnkiVehicleMsgLocalisationIntersectionUpdate>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, data);
        assert_eq!(1, test_msg.road_piece_idx);
        assert_eq!(100.0, test_msg.offset_from_road_centre_mm);
        assert_eq!(IntersectionCode::EntryFirst, test_msg.intersection_code);
        assert_eq!(0xB, test_msg.is_exiting);
        assert_eq!(0xCDEF, test_msg.mm_since_last_transition_bar);
        assert_eq!(0x1234, test_msg.mm_since_last_intersection_code);
    }

    #[test]
    fn anki_vehicle_msg_offset_from_road_centre_update_struct_test() {
        use crate::protocol::AnkiVehicleMsgOffsetFromRoadCentreUpdate;

        let data: &[u8; ANKI_VEHICLE_MSG_OFFSET_FROM_ROAD_CENTRE_UPDATE_SIZE] = &[
            6,
            AnkiVehicleMsgType::V2COffsetFromRoadCentreUpdate as u8,
            66,
            200,
            0,
            0,
            0xAB,
        ];
        let test_msg = data
            .gread_with::<AnkiVehicleMsgOffsetFromRoadCentreUpdate>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_msg, data);
        assert_eq!(100.0, test_msg.offset_from_road_centre_mm);
        assert_eq!(0xAB, test_msg.lane_change_id);
    }

    #[test]
    fn anki_vehicle_msg_set_light_test() {
        use crate::protocol::{anki_vehicle_msg_set_lights, AnkiVehicleMsgSetLights};

        let data: &[u8; ANKI_VEHICLE_MSG_SET_LIGHTS_SIZE] =
            &[2, AnkiVehicleMsgType::C2VSetLights as u8, 0xAB];
        let msg: AnkiVehicleMsgSetLights = anki_vehicle_msg_set_lights(0xAB);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_SET_LIGHTS_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgSetLights>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgSetLights as bytes");
        println!("AnkiVehicleMsgSetLights T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_light_config_test() {
        use crate::protocol::{anki_vehicle_light_config, AnkiVehicleLightConfig};

        let data: &[u8; ANKI_VEHICLE_LIGHT_CONFIG_SIZE] = &[
            LightChannel::Tail as u8,
            LightEffect::Flash as u8,
            0xA,
            0xB,
            100,
        ];
        let config: &AnkiVehicleLightConfig =
            &anki_vehicle_light_config(LightChannel::Tail, LightEffect::Flash, 0xA, 0xB, 600);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_LIGHT_CONFIG_SIZE];
        test_data
            .gwrite_with::<&AnkiVehicleLightConfig>(config, &mut 0, BE)
            .expect("Failed to write AnkiVehicleLightConfig as bytes");
        println!("AnkiVehicleLightConfig T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_lights_pattern_test() {
        use crate::protocol::{
            anki_vehicle_light_config, anki_vehicle_msg_lights_pattern, AnkiVehicleLightConfig,
            AnkiVehicleMsgLightsPattern,
        };

        let data: &[u8; ANKI_VEHICLE_MSG_LIGHTS_PATTERN_SIZE] = &[
            17,
            AnkiVehicleMsgType::C2VLightsPattern as u8,
            2,
            LightChannel::FrontL as u8,
            LightEffect::Fade as u8,
            0xA,
            0xB,
            100,
            LightChannel::Tail as u8,
            LightEffect::Flash as u8,
            0xC,
            0xD,
            100,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut config: AnkiVehicleMsgLightsPattern =
            anki_vehicle_msg_lights_pattern(LightChannel::FrontL, LightEffect::Fade, 0xA, 0xB, 600);
        let config2: AnkiVehicleLightConfig =
            anki_vehicle_light_config(LightChannel::Tail, LightEffect::Flash, 0xC, 0xD, 600);
        config.append(config2);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_LIGHTS_PATTERN_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgLightsPattern>(config, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgLightsPattern as bytes");
        println!(
            "AnkiVehicleMsgLightsPattern T:{:?} == G:{:?}",
            test_data, data
        );
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_ping_request_test() {
        use crate::protocol::{anki_vehicle_msg_ping, AnkiVehicleMsg};

        let data: &[u8; ANKI_VEHICLE_MSG_PING_SIZE] =
            &[1, AnkiVehicleMsgType::C2CPingRequest as u8];
        let msg: AnkiVehicleMsg = anki_vehicle_msg_ping();
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_PING_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsg>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsg as bytes");
        println!("AnkiVehicleMsg (Ping) T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_disconnect_test() {
        use crate::protocol::{anki_vehicle_msg_disconnect, AnkiVehicleMsg};

        let data: &[u8; ANKI_VEHICLE_MSG_DISCONNECT_SIZE] =
            &[1, AnkiVehicleMsgType::C2VDisconnect as u8];
        let msg: AnkiVehicleMsg = anki_vehicle_msg_disconnect();
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_DISCONNECT_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsg>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsg as bytes");
        println!(
            "AnkiVehicleMsg (Disconnect) T:{:?} == G:{:?}",
            test_data, data
        );
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_version_request_test() {
        use crate::protocol::{anki_vehicle_msg_get_version, AnkiVehicleMsg};

        let data: &[u8; ANKI_VEHICLE_MSG_VERSION_REQUEST_SIZE] =
            &[1, AnkiVehicleMsgType::C2VVersionRequest as u8];
        let msg: AnkiVehicleMsg = anki_vehicle_msg_get_version();
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_VERSION_REQUEST_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsg>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsg as bytes");
        println!("AnkiVehicleMsg (Version) T:{:?} == G:{:?}", test_data, data);
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_battery_level_request_test() {
        use crate::protocol::{anki_vehicle_msg_get_battery_level, AnkiVehicleMsg};

        let data: &[u8; ANKI_VEHICLE_MSG_BATTERY_LEVEL_REQUEST_SIZE] =
            &[1, AnkiVehicleMsgType::C2VBatteryLevelRequest as u8];
        let msg: AnkiVehicleMsg = anki_vehicle_msg_get_battery_level();
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_BATTERY_LEVEL_REQUEST_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsg>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsg as bytes");
        println!(
            "AnkiVehicleMsg (Battery Level) T:{:?} == G:{:?}",
            test_data, data
        );
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_cancel_lane_change_test() {
        use crate::protocol::{anki_vehicle_msg_cancel_lane_change, AnkiVehicleMsg};

        let data: &[u8; ANKI_VEHICLE_MSG_CANCEL_LANE_CHANGE_SIZE] =
            &[1, AnkiVehicleMsgType::C2VCancelLaneChange as u8];
        let msg: AnkiVehicleMsg = anki_vehicle_msg_cancel_lane_change();
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_CANCEL_LANE_CHANGE_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsg>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsg as bytes");
        println!(
            "AnkiVehicleMsg (Cancel Lane Change) T:{:?} == G:{:?}",
            test_data, data
        );
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_msg_set_config_params_test() {
        use crate::protocol::{
            anki_vehicle_msg_set_config_params, AnkiVehicleMsgSetConfigParams, TrackMaterial,
        };

        let data: &[u8; ANKI_VEHICLE_MSG_SET_CONFIG_PARAMS_SIZE] = &[
            3,
            AnkiVehicleMsgType::C2VSetConfigParams as u8,
            SUPERCODE_BOOST_JUMP,
            TrackMaterial::Plastic as u8,
        ];
        let msg: AnkiVehicleMsgSetConfigParams =
            anki_vehicle_msg_set_config_params(SUPERCODE_BOOST_JUMP, TrackMaterial::Plastic);
        let test_data: &mut [u8] = &mut [0u8; ANKI_VEHICLE_MSG_SET_CONFIG_PARAMS_SIZE];
        test_data
            .gwrite_with::<AnkiVehicleMsgSetConfigParams>(msg, &mut 0, BE)
            .expect("Failed to write AnkiVehicleMsgSetConfigParams as bytes");
        println!(
            "AnkiVehicleMsgSetConfigParams T:{:?} == G:{:?}",
            test_data, data
        );
        assert_eq!(data, test_data)
    }

    #[test]
    fn anki_vehicle_adv_local_name_struct_test() {
        use crate::advertisement::{AnkiVehicleAdvLocalName, ANKI_VEHICLE_ADV_LOCAL_NAME_SIZE};

        let data: &[u8; ANKI_VEHICLE_ADV_LOCAL_NAME_SIZE] = &[
            0xAB, 0xCD, 0xEF, 0x1, 0x2, 0x3, 0x4, 0x5, 'l' as u8, 'o' as u8, 'c' as u8, 'a' as u8,
            'l' as u8, 'n' as u8, 'a' as u8, 'm' as u8, 'e' as u8, 't' as u8, 'e' as u8, 's' as u8,
            't' as u8,
        ];

        let test_local_name = data
            .gread_with::<AnkiVehicleAdvLocalName>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_local_name, data);
        assert_eq!(0xAB, test_local_name.state);
        assert_eq!(0xCDEF, test_local_name.version);
        assert_eq!("localnametest", test_local_name.name);
    }

    #[test]
    fn anki_vehicle_adv_mfg_data_struct_test() {
        use crate::advertisement::{AnkiVehicleAdvMfgData, ANKI_VEHICLE_ADV_MFG_DATA_SIZE};

        let data: &[u8; ANKI_VEHICLE_ADV_MFG_DATA_SIZE] =
            &[0x89, 0xAB, 0xCD, 0xEF, 0xAB, 0x12, 0xCD, 0xEF];

        let test_mfg_data = data
            .gread_with::<AnkiVehicleAdvMfgData>(&mut 0, BE)
            .unwrap();
        println!("T:{:?} == G:{:?}", test_mfg_data, data);
        assert_eq!(0x89ABCDEF, test_mfg_data.identifier);
        assert_eq!(0xAB, test_mfg_data.model_id);
        assert_eq!(0xCDEF, test_mfg_data.product_id);
    }

    #[test]
    fn anki_vehicle_adv_struct_test<'a>() {
        use crate::advertisement::{AnkiVehicleAdv, ANKI_VEHICLE_ADV_SIZE};

        let data: &[u8; ANKI_VEHICLE_ADV_SIZE] = &[
            0x12, 0x34, 0x89, 0xAB, 0xCD, 0xEF, 0xAB, 0x56, 0xCD, 0xEF, 0xAB, 0xCD, 0xEF, 0x1, 0x2,
            0x3, 0x4, 0x5, 'l' as u8, 'o' as u8, 'c' as u8, 'a' as u8, 'l' as u8, 'n' as u8,
            'a' as u8, 'm' as u8, 'e' as u8, 't' as u8, 'e' as u8, 's' as u8, 't' as u8, 0x0, 0x1,
            0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
        ];

        let test_adv = data.gread_with::<AnkiVehicleAdv>(&mut 0, BE).unwrap();
        println!("T:{:?} == G:{:?}", test_adv, data);

        let service_id: &'a [u8] = &[
            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
        ];

        assert_eq!(0x12, test_adv.flags);
        assert_eq!(0x34, test_adv.tx_power);
        assert_eq!(0x89ABCDEF, test_adv.mfg_data.identifier);
        assert_eq!(0xAB, test_adv.mfg_data.model_id);
        assert_eq!(0xCDEF, test_adv.mfg_data.product_id);
        assert_eq!(0xAB, test_adv.local_name.state);
        assert_eq!(0xCDEF, test_adv.local_name.version);
        assert_eq!("localnametest", test_adv.local_name.name);
        assert_eq!(service_id, test_adv.service_id);
    }
}
