// Copyright © 2020 Starcat LLC
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(non_camel_case_types)]
//! ADS1299 constants

use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt;

pub enum SystemCommands {
    WAKEUP = 0x02,
    STANDBY = 0x04,
    RESET = 0x06,
    START = 0x08,
    STOP = 0x0a,
}

pub enum ReadCommands {
    RDATAC = 0x10,
    SDATAC = 0x11,
    RDATA = 0x12,
}

pub enum RegisterCommands {
    RREG = 0x20,
    WREG = 0x40,
}

pub enum DeviceSettings {
    ID = 0x00,
}

pub enum GlobalSettings {
    CONFIG1 = 0x01,
    CONFIG2 = 0x02,
    CONFIG3 = 0x03,
    LOFF = 0x04,
}

pub enum ChannelSettings {
    CHnSET = 0x04,
    CH1SET,
    CH2SET,
    CH3SET,
    CH4SET,
    CH5SET,
    CH6SET,
    CH7SET,
    CH8SET,
    BIAS_SENSP = 0x0d,
    BIAS_SENSN = 0x0e,
    LOFF_SENSP = 0x0f,
    LOFF_SENSN = 0x10,
    LOFF_FLIP = 0x11,
}

pub enum LeadOffStatus {
    LOFF_STATP = 0x12,
    LOFF_STATN = 0x13,
}

pub enum Speed {
    HIGH_RES_16k_SPS = 0x00,
    HIGH_RES_8k_SPS = 0x01,
    HIGH_RES_4k_SPS = 0x02,
    HIGH_RES_2k_SPS = 0x03,
    HIGH_RES_1k_SPS = 0x04,
    HIGH_RES_500_SPS = 0x05,
    HIGH_RES_250_SPS = 0x06,
}

impl From<u32> for Speed {
    fn from(num: u32) -> Self {
        match num {
            250 => Speed::HIGH_RES_250_SPS,
            500 => Speed::HIGH_RES_500_SPS,
            1000 => Speed::HIGH_RES_1k_SPS,
            2000 => Speed::HIGH_RES_2k_SPS,
            4000 => Speed::HIGH_RES_4k_SPS,
            8000 => Speed::HIGH_RES_8k_SPS,
            16000 => Speed::HIGH_RES_16k_SPS,
            _ => panic!("Invalid speed"),
        }
    }
}

// TODO do the rest of these.  not all of them are classified into enums, like the above.  where
// grouping together into an enum doesn't make sense, use a const

//GPIO = 0x14
pub const MISC1: u8 = 0x15;
//RESP = 0x16
//CONFIG4 = 0x17
//WCT1 = 0x18
//WCT2 = 0x19
//
//DEV_ID7 = 0x80
//DEV_ID6 = 0x40
//DEV_ID5 = 0x20
//DEV_ID3 = 0x08
//DEV_ID2 = 0x04
//DEV_ID1 = 0x02
//DEV_ID0 = 0x01
//
//ID_const = 0x10
//ID_ADS129x = DEV_ID7
//ID_ADS129xR = (DEV_ID7 | DEV_ID6)
//
//ID_4CHAN = 0
//ID_6CHAN = DEV_ID0
//ID_8CHAN = DEV_ID1
//
//ID_ADS1294 = (ID_ADS129x | ID_4CHAN)
//ID_ADS1296 = (ID_ADS129x | ID_6CHAN)
//ID_ADS1298 = (ID_ADS129x | ID_8CHAN)
//ID_ADS1294R = (ID_ADS129xR | ID_4CHAN)
//ID_ADS1296R = (ID_ADS129xR | ID_6CHAN)
//ID_ADS1298R = (ID_ADS129xR | ID_8CHAN)
//ID_ADS1299 = (DEV_ID3 | DEV_ID2 | DEV_ID1)
//
//HR = 0x80
//DAISY_EN = 0x40
//CLK_EN = 0x20

//WCT_CHOP = 0x20
pub const INT_TEST: u8 = 0x10;
pub const TEST_AMP: u8 = 0x04;
pub const TEST_FREQ1: u8 = 0x02;
pub const TEST_FREQ0: u8 = 0x01;
//
pub const CONFIG2_const: u8 = 0xC0;
pub const CONFIG1_const: u8 = 0x90;
pub const INT_TEST_4HZ: u8 = INT_TEST;
pub const INT_TEST_8HZ: u8 = (INT_TEST | TEST_FREQ0);
pub const INT_TEST_DC: u8 = (INT_TEST | TEST_FREQ1 | TEST_FREQ0);
//
//PD_REFBUF = 0x80
//VREF_4V = 0x20
//RLD_MEAS = 0x10
//RLDREF_INT = 0x08
//PD_RLD = 0x04
//RLD_LOFF_SENS = 0x02
//RLD_STAT = 0x01
//
//CONFIG3_const = 0x60
//
//COMP_TH2 = 0x80
//COMP_TH1 = 0x40
//COMP_TH0 = 0x20
//VLEAD_OFF_EN = 0x10
//ILEAD_OFF1 = 0x08
//ILEAD_OFF0 = 0x04
//FLEAD_OFF1 = 0x02
//FLEAD_OFF0 = 0x01
//
//LOFF_const = 0x00
//
//COMP_TH_95 = 0x00
//COMP_TH_92_5 = COMP_TH0
//COMP_TH_90 = COMP_TH1
//COMP_TH_87_5 = (COMP_TH1 | COMP_TH0)
//COMP_TH_85 = COMP_TH2
//COMP_TH_80 = (COMP_TH2 | COMP_TH0)
//COMP_TH_75 = (COMP_TH2 | COMP_TH1)
//COMP_TH_70 = (COMP_TH2 | COMP_TH1 | COMP_TH0)
//
//ILEAD_OFF_6nA = 0x00
//ILEAD_OFF_12nA = ILEAD_OFF0
//ILEAD_OFF_18nA = ILEAD_OFF1
//ILEAD_OFF_24nA = (ILEAD_OFF1 | ILEAD_OFF0)
//
//FLEAD_OFF_AC = FLEAD_OFF0
//FLEAD_OFF_DC = (FLEAD_OFF1 | FLEAD_OFF0)
//
pub const PDn: u8 = 0x80;
//GAINn2 = 0x40
//GAINn1 = 0x20
//GAINn0 = 0x10
//SRB2n0 = 0x08

pub const MUXn2: u8 = 0x04;
pub const MUXn1: u8 = 0x02;
pub const MUXn0: u8 = 0x01;

//
//CHnSET_const = 0x00
//

// http://www.ti.com/lit/ds/symlink/ads1299.pdf  pg 50
#[derive(Debug, Copy, Clone)]
pub enum Gain {
    X1 = 0b0,
    X2 = 0b001,
    X4 = 0b010,
    X6 = 0b011,
    X8 = 0b100,
    X12 = 0b101,
    X24 = 0b110,
}

impl fmt::Display for Gain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::X1 => "X1",
            Self::X2 => "X2",
            Self::X4 => "X4",
            Self::X6 => "X6",
            Self::X8 => "X8",
            Self::X12 => "X12",
            Self::X24 => "X24",
        };
        write!(f, "Gain {}", s)
    }
}

impl From<u32> for Gain {
    fn from(num: u32) -> Self {
        match num {
            1 => Gain::X1,
            2 => Gain::X2,
            4 => Gain::X4,
            6 => Gain::X6,
            8 => Gain::X8,
            12 => Gain::X12,
            24 => Gain::X24,
            _ => panic!("Invalid gain"),
        }
    }
}

//
//# ADS1298
//ADS1298_GAIN_1X = GAINn0
//ADS1298_GAIN_2X = GAINn1
//ADS1298_GAIN_3X = (GAINn1 | GAINn0)
//ADS1298_GAIN_4X = GAINn2
//ADS1298_GAIN_6X = 0x00
//ADS1298_GAIN_8X = (GAINn2 | GAINn0)
//ADS1298_GAIN_12X = (GAINn2 | GAINn1)
//
pub const ELECTRODE_INPUT: u8 = 0x00;
pub const SHORTED: u8 = 0x01;
//RLD_INPUT = MUXn1
pub const MVDD: u8 = (MUXn1 | MUXn0);
pub const TEMP: u8 = MUXn2;
pub const TEST_SIGNAL: u8 = (MUXn2 | MUXn0);
pub const BIAS_DRP: u8 = (MUXn2 | MUXn1);
pub const BIAS_DRN: u8 = (MUXn2 | MUXn1 | MUXn0);
//
//PD_1 = 0x80
//GAIN12 = 0x40
//GAIN11 = 0x20
//GAIN10 = 0x10
//MUX12 = 0x04
//MUX11 = 0x02
//MUX10 = 0x01
//
//BIAS8P = 0x80
//BIAS7P = 0x40
//BIAS6P = 0x20
//BIAS5P = 0x10
//BIAS4P = 0x08
//BIAS3P = 0x04
//BIAS2P = 0x02
//BIAS1P = 0x01
//
//CH1SET_const = 0x00
//
//PD_2 = 0x80
//GAIN22 = 0x40
//GAIN21 = 0x20
//GAIN20 = 0x10
//MUX22 = 0x04
//MUX21 = 0x02
//MUX20 = 0x01
//
//CH2SET_const = 0x00
//
//PD_3 = 0x80
//GAIN32 = 0x40
//GAIN31 = 0x20
//GAIN30 = 0x10
//MUX32 = 0x04
//MUX31 = 0x02
//MUX30 = 0x01
//
//CH3SET_const = 0x00
//
//PD_4 = 0x80
//GAIN42 = 0x40
//GAIN41 = 0x20
//GAIN40 = 0x10
//MUX42 = 0x04
//MUX41 = 0x02
//MUX40 = 0x01
//
//CH4SET_const = 0x00
//
//PD_5 = 0x80
//GAIN52 = 0x40
//GAIN51 = 0x20
//GAIN50 = 0x10
//MUX52 = 0x04
//MUX51 = 0x02
//MUX50 = 0x01
//
//CH5SET_const = 0x00
//
//PD_6 = 0x80
//GAIN62 = 0x40
//GAIN61 = 0x20
//GAIN60 = 0x10
//MUX62 = 0x04
//MUX61 = 0x02
//MUX60 = 0x01
//
//CH6SET_const = 0x00
//
//PD_7 = 0x80
//GAIN72 = 0x40
//GAIN71 = 0x20
//GAIN70 = 0x10
//MUX72 = 0x04
//MUX71 = 0x02
//MUX70 = 0x01
//
//CH7SET_const = 0x00
//
//PD_8 = 0x80
//GAIN82 = 0x40
//GAIN81 = 0x20
//GAIN80 = 0x10
//MUX82 = 0x04
//MUX81 = 0x02
//MUX80 = 0x01
//
//CH8SET_const = 0x00
//
//RLD8P = 0x80
//RLD7P = 0x40
//RLD6P = 0x20
//RLD5P = 0x10
//RLD4P = 0x08
//RLD3P = 0x04
//RLD2P = 0x02
//RLD1P = 0x01
//
//RLD_SENSP_const = 0x00
//
//RLD8N = 0x80
//RLD7N = 0x40
//RLD6N = 0x20
//RLD5N = 0x10
//RLD4N = 0x08
//RLD3N = 0x04
//RLD2N = 0x02
//RLD1N = 0x01
//
//RLD_SENSN_const = 0x00
//
//LOFF8P = 0x80
//LOFF7P = 0x40
//LOFF6P = 0x20
//LOFF5P = 0x10
//LOFF4P = 0x08
//LOFF3P = 0x04
//LOFF2P = 0x02
//LOFF1P = 0x01
//
//LOFF_SENSP_const = 0x00
//
//LOFF8N = 0x80
//LOFF7N = 0x40
//LOFF6N = 0x20
//LOFF5N = 0x10
//LOFF4N = 0x08
//LOFF3N = 0x04
//LOFF2N = 0x02
//LOFF1N = 0x01
//
//LOFF_SENSN_const = 0x00
//
//LOFF_FLIP8 = 0x80
//LOFF_FLIP7 = 0x40
//LOFF_FLIP6 = 0x20
//LOFF_FLIP5 = 0x10
//LOFF_FLIP4 = 0x08
//LOFF_FLIP3 = 0x04
//LOFF_FLIP2 = 0x02
//LOFF_FLIP1 = 0x01
//
//LOFF_FLIP_const = 0x00
//
//IN8P_OFF = 0x80
//IN7P_OFF = 0x40
//IN6P_OFF = 0x20
//IN5P_OFF = 0x10
//IN4P_OFF = 0x08
//IN3P_OFF = 0x04
//IN2P_OFF = 0x02
//IN1P_OFF = 0x01
//
//LOFF_STATP_const = 0x00
//
//IN8N_OFF = 0x80
//IN7N_OFF = 0x40
//IN6N_OFF = 0x20
//IN5N_OFF = 0x10
//IN4N_OFF = 0x08
//IN3N_OFF = 0x04
//IN2N_OFF = 0x02
//IN1N_OFF = 0x01
//
//LOFF_STATN_const = 0x00
//
//GPIOD4 = 0x80
//GPIOD3 = 0x40
//GPIOD2 = 0x20
//GPIOD1 = 0x10
//GPIOC4 = 0x08
//GPIOC3 = 0x04
//GPIOC2 = 0x02
//GPIOC1 = 0x01
//
//GPIO_const = 0x00
//
//PACEE1 = 0x10
//PACEE0 = 0x08
//PACEO1 = 0x04
//PACEO0 = 0x02
//PD_PACE = 0x01
//
//PACE_const = 0x00
//
//PACEE_CHAN2 = 0x00
//PACEE_CHAN4 = PACEE0
//PACEE_CHAN6 = PACEE1
//PACEE_CHAN8 = (PACEE1 | PACEE0)
//
//PACEO_CHAN1 = 0x00
//PACEO_CHAN3 = PACEE0
//PACEO_CHAN5 = PACEE1
//PACEO_CHAN7 = (PACEE1 | PACEE0)
//
//RESP_DEMOD_EN1 = 0x80
//RESP_MOD_EN1 = 0x40
//RESP_PH2 = 0x10
//RESP_PH1 = 0x08
//RESP_PH0 = 0x04
//RESP_CTRL1 = 0x02
//RESP_CTRL0 = 0x01
//
//RESP_const = 0x20
//
//RESP_PH_22_5 = 0x00
//RESP_PH_45 = RESP_PH0
//RESP_PH_67_5 = RESP_PH1
//RESP_PH_90 = (RESP_PH1 | RESP_PH0)
//RESP_PH_112_5 = RESP_PH2
//RESP_PH_135 = (RESP_PH2 | RESP_PH0)
//RESP_PH_157_5 = (RESP_PH2 | RESP_PH1)
//
//RESP_NONE = 0x00
//RESP_EXT = RESP_CTRL0
//RESP_INT_SIG_INT = RESP_CTRL1
//RESP_INT_SIG_EXT = (RESP_CTRL1 | RESP_CTRL0)
//
//RESP_FREQ2 = 0x80
//RESP_FREQ1 = 0x40
//RESP_FREQ0 = 0x20
//SINGLE_SHOT = 0x08
//WCT_TO_RLD = 0x04
//PD_LOFF_COMP = 0x02
//
//CONFIG4_const = 0x00
//
//RESP_FREQ_64k_Hz = 0x00
//RESP_FREQ_32k_Hz = RESP_FREQ0
//RESP_FREQ_16k_Hz = RESP_FREQ1
//RESP_FREQ_8k_Hz = (RESP_FREQ1 | RESP_FREQ0)
//RESP_FREQ_4k_Hz = RESP_FREQ2
//RESP_FREQ_2k_Hz = (RESP_FREQ2 | RESP_FREQ0)
//RESP_FREQ_1k_Hz = (RESP_FREQ2 | RESP_FREQ1)
//RESP_FREQ_500_Hz = (RESP_FREQ2 | RESP_FREQ1 | RESP_FREQ0)
//
//aVF_CH6 = 0x80
//aVL_CH5 = 0x40
//aVR_CH7 = 0x20
//avR_CH4 = 0x10
//PD_WCTA = 0x08
//WCTA2 = 0x04
//WCTA1 = 0x02
//WCTA0 = 0x01
//
//WCT1_const = 0x00
//
//WCTA_CH1P = 0x00
//WCTA_CH1N = WCTA0
//WCTA_CH2P = WCTA1
//WCTA_CH2N = (WCTA1 | WCTA0)
//WCTA_CH3P = WCTA2
//WCTA_CH3N = (WCTA2 | WCTA0)
//WCTA_CH4P = (WCTA2 | WCTA1)
//WCTA_CH4N = (WCTA2 | WCTA1 | WCTA0)
//
//PD_WCTC = 0x80
//PD_WCTB = 0x40
//WCTB2 = 0x20
//WCTB1 = 0x10
//WCTB0 = 0x08
//WCTC2 = 0x04
//WCTC1 = 0x02
//WCTC0 = 0x01
//
//WCT2_const = 0x00
//
//WCTB_CH1P = 0x00
//WCTB_CH1N = WCTB0
//WCTB_CH2P = WCTB1
//WCTB_CH2N = (WCTB1 | WCTB0)
//WCTB_CH3P = WCTB2
//WCTB_CH3N = (WCTB2 | WCTB0)
//WCTB_CH4P = (WCTB2 | WCTB1)
//WCTB_CH4N = (WCTB2 | WCTB1 | WCTB0)
//
//WCTC_CH1P = 0x00
//WCTC_CH1N = WCTC0
//WCTC_CH2P = WCTC1
//WCTC_CH2N = (WCTC1 | WCTC0)
//WCTC_CH3P = WCTC2
//WCTC_CH3N = (WCTC2 | WCTC0)
//WCTC_CH4P = (WCTC2 | WCTC1)
//WCTC_CH4N = (WCTC2 | WCTC1 | WCTC0)
//
pub const MISC1_const: u8 = 0;
pub const SRB1: u8 = 0x20;
