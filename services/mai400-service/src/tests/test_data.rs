/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![allow(dead_code)]
#![allow(clippy::unreadable_literal)]

use super::*;

pub static RAW_READ: [u8; 238] = [
    0x90, 0xEB, 0x3, 0x93, 0x3C, 0x74, 0x47, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x44, 0x1, 0x0,
    0x0, 0x4, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x1, 0x1, 0x80, 0x1, 0x80, 0x1, 0x80,
    0xA7, 0xFA, 0x69, 0x0, 0xEF, 0xFC, 0x7A, 0xFB, 0xE9, 0xB5, 0x37, 0xC0, 0xA, 0x34, 0x78, 0x27,
    0x86, 0xB5, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1B, 0x2F, 0xDD, 0x3D, 0x8C, 0xB7,
    0x53, 0xBC, 0xF9, 0xB3, 0xCC, 0x3D, 0x7F, 0xF1, 0x76, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xFF, 0x7F,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xFF, 0x7F, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0xFF, 0x7F, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xBF, 0xE9, 0x54, 0xBE,
    0x34, 0x56, 0xAD, 0x40, 0x2A, 0x56, 0xAD, 0x40, 0x19, 0x7C, 0x19, 0x7C, 0x19, 0x7C, 0xE, 0x80,
    0x8D, 0xFD, 0x8D, 0xFD, 0xBD, 0x26, 0x91, 0xEA, 0x34, 0x0, 0xA6, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x8, 0x8, 0x1,
    0x1, 0x8, 0x8, 0x1, 0x1, 0xD6, 0xE0, 0x91, 0xEA, 0x11, 0x0, 0xC8, 0x0, 0x1, 0x0, 0xFB, 0xFF,
    0x10, 0x1, 0x26, 0x0, 0x1D, 0x0, 0x16, 0x0, 0x13, 0x82, 0x93,
];

pub const STD: StandardTelemetry = StandardTelemetry {
    tlm_counter: 3,
    gps_time: 1198800019,
    time_subsec: 0,
    cmd_valid_cntr: 2,
    cmd_invalid_cntr: 0,
    cmd_invalid_chksum_cntr: 0,
    last_command: 0x44,
    acs_mode: 1,
    css: [0, 4, 1, 0, 0, 4],
    eclipse_flag: 1,
    sun_vec_b: [-32767, -32767, -32767],
    i_b_field_meas: [-1369, 105, -785],
    bd: [-0.0000017433042, 0.00000012922179, -0.0000009995265],
    rws_speed_cmd: [0, 0, 0],
    rws_speed_tach: [0, 0, 0],
    rwa_torque_cmd: [0.0, 0.0, 0.0],
    gc_rwa_torque_cmd: [0, 0, 0],
    torque_coil_cmd: [0.108, -0.012922179, 0.099952646],
    gc_torque_coil_cmd: [127, -15, 118],
    qbo_cmd: [0, 0, 0, 32767],
    qbo_hat: [0, 0, 0, 32767],
    angle_to_go: 0.0,
    q_error: [0, 0, 0, 32767],
    omega_b: [0.0, 0.0, 0.0],
    rotating_variable_a: 0xBE54E9BF,
    rotating_variable_b: 0x40AD5634,
    rotating_variable_c: 0x40AD562A,
    nb: [31769, 31769, 31769],
    neci: [-32754, -627, -627],
};

pub const IREHS: IREHSTelemetry = IREHSTelemetry {
    thermopiles_a: [0, 0, 0, 0],
    thermopiles_b: [0, 0, 0, 0],
    temp_a: [0, 0, 0, 0],
    temp_b: [0, 0, 0, 0],
    dip_angle_a: 0,
    dip_angle_b: 0,
    solution_degraded: [
        ThermopileFlags::NO_COMM,
        ThermopileFlags::NO_COMM,
        ThermopileFlags::DIP_ANGLE_LIMIT,
        ThermopileFlags::DIP_ANGLE_LIMIT,
        ThermopileFlags::NO_COMM,
        ThermopileFlags::NO_COMM,
        ThermopileFlags::DIP_ANGLE_LIMIT,
        ThermopileFlags::DIP_ANGLE_LIMIT,
    ],
};
pub const IMU: RawIMU = RawIMU {
    accel: [1, -5, 272],
    gyro: [38, 29, 22],
    gyro_temp: 19,
};

pub const ROTATING: RotatingTelemetry = RotatingTelemetry {
    b_field_igrf: [-0.000015042761, 0.000000205466, 0.000022202008],
    sun_vec_eph: [0.18260002, -0.90207666, -0.39104345],
    sc_pos_eci: [6720.9653, 669.7314, 669.72473],
    sc_vel_eci: [-0.20792292, 5.416773, 5.416768],
    kepler_elem: KeplerElem {
        semi_major_axis: 6787.47,
        eccentricity: 0.0,
        inclination: 45.0,
        raan: 0.0,
        arg_parigee: 0.0,
        true_anomoly: 0.0,
    },
    k_bdot: [-100000.0, -100000.0, -100000.0],
    kp: [-1.11, -1.1, -0.25],
    kd: [-12.1, -12.0, -4.0],
    k_unload: [-5000000.0, -5000000.0, -5000000.0],
    css_bias: [0; 6],
    mag_bias: [1028, 0, 0],
    rws_volt: 0,
    rws_press: 0,
    att_det_mode: 24,
    rws_reset_cntr: [1, 1, 2],
    sun_mag_aligned: 0,
    minor_version: 24,
    mai_sn: 120,
    orbit_prop_mode: 0,
    acs_op_mode: 169,
    proc_reset_cntr: 4,
    major_version: 2,
    ads_op_mode: 0,
    css_gain: [1.0; 6],
    mag_gain: [1.0; 3],
    orbit_epoch: 511358571,
    true_anomoly_epoch: 0.0,
    orbit_epoch_next: 1,
    sc_pos_eci_epoch: [6787.47, 0.0, 0.0],
    sc_vel_eci_epoch: [0.0, 5.418765, 5.418765],
    qb_x_wheel_speed: 0,
    qb_x_filter_gain: 1.0,
    qb_x_dipole_gain: 1.0,
    dipole_gain: [1.0; 3],
    wheel_speed_bias: [16256; 3],
    cos_sun_mag_align_thresh: 0.99,
    unload_ang_thresh: 0.05,
    q_sat: 0.3,
    rwa_trq_max: 0.2,
    rws_motor_current: [4, 4, 4004],
    rws_motor_temp: 12,
};

pub const REQUEST_RESET: [u8; 40] = [
    0x90, 0xEB, 0x5A, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0xD5, 0x1,
];

pub const CONFIRM_RESET: [u8; 40] = [
    0x90, 0xEB, 0xF1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x6C, 0x2,
];

// set_mode(0x01, [0x02, 0x03, 0x04, 0x05])
pub const SET_MODE_ACQUISITION: [u8; 40] = [
    0x90, 0xEB, 0x0, 0x1, 0x2, 0x0, 0x3, 0x0, 0x4, 0x0, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x8A, 0x1,
];

// set_mode(0x01, [0x00, 0x00, 0x00, 0x00])
pub const SET_MODE_ACQUISITION_DEFAULT: [u8; 40] = [
    0x90, 0xEB, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x7C, 0x1,
];

// set_mode_sun(0x07, 1, 2.2)
pub const SET_MODE_NORMAL_SUN: [u8; 40] = [
    0x90, 0xEB, 0x0, 0x7, 0x1, 0x0, 0xCD, 0xCC, 0xC, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x68, 0x3,
];

// set_mode_sun(0x08, 1, 2.2)
pub const SET_MODE_LATLONG_SUN: [u8; 40] = [
    0x90, 0xEB, 0x0, 0x8, 0x1, 0x0, 0xCD, 0xCC, 0xC, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x69, 0x3,
];

// set_mode_sun(0x08, 0, 0.0)
pub const SET_MODE_SUN_DEFAULT: [u8; 40] = [
    0x90, 0xEB, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x83, 0x1,
];

// set_gps_time(1198800018)
pub const SET_GPS_TIME: [u8; 40] = [
    0x90, 0xEB, 0x44, 0x92, 0x3C, 0x74, 0x47, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x48, 0x3,
];

// set_rv([1.1, 2.2, 3.3], [4.4, 5.5, 6.6], 1198800018)
pub const SET_RV: [u8; 40] = [
    0x90, 0xEB, 0x41, 0xCD, 0xCC, 0x8C, 0x3F, 0xCD, 0xCC, 0xC, 0x40, 0x33, 0x33, 0x53, 0x40, 0xCD,
    0xCC, 0x8C, 0x40, 0x0, 0x0, 0xB0, 0x40, 0x33, 0x33, 0xD3, 0x40, 0x92, 0x3C, 0x74, 0x47, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x55, 0xD,
];
