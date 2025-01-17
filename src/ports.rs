//! Well known ADS ports as defined
//! [here](https://infosys.beckhoff.com/content/1033/tc3_ads_intro/116159883.html?id=6824734840428332798).

#![allow(missing_docs)]

use crate::AmsPort;

pub const AMS_ROUTER: AmsPort = 1;
pub const AMS_DEBUGGER: AmsPort = 2;
pub const TCOM_SERVER: AmsPort = 10;
pub const TCOM_SERVER_TASK: AmsPort = 11;
pub const TCOM_SERVER_PASSIVE: AmsPort = 12;
pub const TCAT_DEBUGGER: AmsPort = 20;
pub const TCAT_DEBUGGER_TASK: AmsPort = 21;
pub const LICENSE_SERVER: AmsPort = 30;
pub const LOGGER: AmsPort = 100;
pub const EVENT_LOGGER: AmsPort = 110;
pub const APPLICATION: AmsPort = 120;
pub const EVENT_LOGGER_USER: AmsPort = 130;
pub const EVENT_LOGGER_REALTIME: AmsPort = 131;
pub const EVENT_LOGGER_PUBLISHER: AmsPort = 132;
pub const RING0_REALTIME: AmsPort = 200;
pub const RING0_TRACE: AmsPort = 290;
pub const RING0_IO: AmsPort = 300;
pub const RING0_PLC: AmsPort = 400; // legacy
pub const RING0_NC: AmsPort = 500;
pub const RING0_NC_SAF: AmsPort = 501;
pub const RING0_NC_SVB: AmsPort = 511;
pub const NC_INSTANCE: AmsPort = 520;
pub const RING_ISG: AmsPort = 550;
pub const RING0_CNC: AmsPort = 600;
pub const RING0_LINE: AmsPort = 700;
pub const RING0_TC2_PLC: AmsPort = 800;
pub const TC2_PLC_SYSTEM1: AmsPort = 801;
pub const TC2_PLC_SYSTEM2: AmsPort = 811;
pub const TC2_PLC_SYSTEM3: AmsPort = 821;
pub const TC2_PLC_SYSTEM4: AmsPort = 831;
pub const RING0_TC3_PLC: AmsPort = 850;
pub const TC3_PLC_SYSTEM1: AmsPort = 851;
pub const TC3_PLC_SYSTEM2: AmsPort = 852;
pub const TC3_PLC_SYSTEM3: AmsPort = 853;
pub const TC3_PLC_SYSTEM4: AmsPort = 854; // and following
pub const CAMSHAFT_CONTROLLER: AmsPort = 900;
pub const CAM_TOOL: AmsPort = 950;
pub const RING0_IO_PORTS: AmsPort = 1000; // to 1199
pub const RING0_USER: AmsPort = 2000;
pub const CRESTRON_SERVER: AmsPort = 2500;
pub const SYSTEM_SERVICE: AmsPort = 10000;
pub const TCPIP_SERVER: AmsPort = 10201;
pub const SYSTEM_MANAGER: AmsPort = 10300;
pub const SMS_SERVER: AmsPort = 10400;
pub const MODBUS_SERVER: AmsPort = 10500;
pub const AMS_LOGGER: AmsPort = 10502;
pub const XML_DATA_SERVER: AmsPort = 10600;
pub const AUTO_CONFIG: AmsPort = 10700;
pub const PLC_CONTROL: AmsPort = 10800;
pub const FTP_CLIENT: AmsPort = 10900;
pub const NC_CONTROL: AmsPort = 11000;
pub const NC_INTERPRETER: AmsPort = 11500;
pub const GST_INTERPRETER: AmsPort = 11600;
pub const STRECKE_CONTROL: AmsPort = 12000;
pub const CAM_CONTROL: AmsPort = 13000;
pub const SCOPE_SERVER: AmsPort = 14000;
pub const COND_MONITORING: AmsPort = 14100;
pub const SINE_CH1: AmsPort = 15000;
pub const CONTROL_NET: AmsPort = 16000;
pub const OPC_SERVER: AmsPort = 17000;
pub const OPC_CLIENT: AmsPort = 17500;
pub const MAIL_SERVER: AmsPort = 18000;
pub const VIRTUAL_COM: AmsPort = 19000;
pub const MGMT_SERVER: AmsPort = 19100;
pub const MIELE_HOME_SERVER: AmsPort = 19200;
pub const CP_LINK3: AmsPort = 19300;
pub const VISION_SERVICE: AmsPort = 19500;
pub const DATABASE_SERVER: AmsPort = 21372;
pub const FIAS_SERVER: AmsPort = 25013;
pub const BANG_OLUFSEN_SERVER: AmsPort = 25015;
