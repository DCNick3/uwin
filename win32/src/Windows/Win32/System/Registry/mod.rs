#![doc = r" do not edit! File auto-generated with win32-bindgen"]
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    unused_assignments,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub const AGP_FLAG_NO_1X_RATE: i32 = 1i32;
pub const AGP_FLAG_NO_2X_RATE: i32 = 2i32;
pub const AGP_FLAG_NO_4X_RATE: i32 = 4i32;
pub const AGP_FLAG_NO_8X_RATE: i32 = 8i32;
pub const AGP_FLAG_NO_FW_ENABLE: i32 = 512i32;
pub const AGP_FLAG_NO_SBA_ENABLE: i32 = 256i32;
pub const AGP_FLAG_REVERSE_INITIALIZATION: i32 = 128i32;
pub const AGP_FLAG_SPECIAL_RESERVE: i32 = 1015808i32;
pub const AGP_FLAG_SPECIAL_TARGET: i32 = 1048575i32;
pub const APMMENUSUSPEND_DISABLED: u32 = 0u32;
pub const APMMENUSUSPEND_ENABLED: u32 = 1u32;
pub const APMMENUSUSPEND_NOCHANGE: u32 = 128u32;
pub const APMMENUSUSPEND_UNDOCKED: u32 = 2u32;
pub const APMTIMEOUT_DISABLED: u32 = 0u32;
pub const BIF_RAWDEVICENEEDSDRIVER: u32 = 2u32;
pub const BIF_SHOWSIMILARDRIVERS: u32 = 1u32;
pub const CONFIGFLAG_BOOT_DEVICE: u32 = 262144u32;
pub const CONFIGFLAG_CANTSTOPACHILD: u32 = 128u32;
pub const CONFIGFLAG_DISABLED: u32 = 1u32;
pub const CONFIGFLAG_FAILEDINSTALL: u32 = 64u32;
pub const CONFIGFLAG_FINISHINSTALL_ACTION: u32 = 131072u32;
pub const CONFIGFLAG_FINISHINSTALL_UI: u32 = 65536u32;
pub const CONFIGFLAG_FINISH_INSTALL: u32 = 1024u32;
pub const CONFIGFLAG_IGNORE_BOOT_LC: u32 = 8u32;
pub const CONFIGFLAG_MANUAL_INSTALL: u32 = 4u32;
pub const CONFIGFLAG_NEEDS_CLASS_CONFIG: u32 = 524288u32;
pub const CONFIGFLAG_NEEDS_FORCED_CONFIG: u32 = 2048u32;
pub const CONFIGFLAG_NETBOOT_CARD: u32 = 4096u32;
pub const CONFIGFLAG_NET_BOOT: u32 = 16u32;
pub const CONFIGFLAG_NOREMOVEEXIT: u32 = 512u32;
pub const CONFIGFLAG_OKREMOVEROM: u32 = 256u32;
pub const CONFIGFLAG_PARTIAL_LOG_CONF: u32 = 8192u32;
pub const CONFIGFLAG_REINSTALL: u32 = 32u32;
pub const CONFIGFLAG_REMOVED: u32 = 2u32;
pub const CONFIGFLAG_SUPPRESS_SURPRISE: u32 = 16384u32;
pub const CONFIGFLAG_VERIFY_HARDWARE: u32 = 32768u32;
pub const CSCONFIGFLAG_BITS: u32 = 7u32;
pub const CSCONFIGFLAG_DISABLED: u32 = 1u32;
pub const CSCONFIGFLAG_DO_NOT_CREATE: u32 = 2u32;
pub const CSCONFIGFLAG_DO_NOT_START: u32 = 4u32;
pub const DMSTATEFLAG_APPLYTOALL: u32 = 1u32;
pub const DOSOPTF_ALWAYSUSE: i32 = 4i32;
pub const DOSOPTF_DEFAULT: i32 = 1i32;
pub const DOSOPTF_INDOSSTART: i32 = 64i32;
pub const DOSOPTF_MULTIPLE: i32 = 128i32;
pub const DOSOPTF_NEEDSETUP: i32 = 32i32;
pub const DOSOPTF_PROVIDESUMB: i32 = 16i32;
pub const DOSOPTF_SUPPORTED: i32 = 2i32;
pub const DOSOPTF_USESPMODE: i32 = 8i32;
pub const DOSOPTGF_DEFCLEAN: i32 = 1i32;
pub const DRIVERSIGN_BLOCKING: u32 = 2u32;
pub const DRIVERSIGN_NONE: u32 = 0u32;
pub const DRIVERSIGN_WARNING: u32 = 1u32;
pub struct DSKTLSYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
    pub wResult: u16,
}
impl ::core::marker::Copy for DSKTLSYSTEMTIME {}
impl ::core::clone::Clone for DSKTLSYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSKTLSYSTEMTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSKTLSYSTEMTIME")
            .field("wYear", &self.wYear)
            .field("wMonth", &self.wMonth)
            .field("wDayOfWeek", &self.wDayOfWeek)
            .field("wDay", &self.wDay)
            .field("wHour", &self.wHour)
            .field("wMinute", &self.wMinute)
            .field("wSecond", &self.wSecond)
            .field("wMilliseconds", &self.wMilliseconds)
            .field("wResult", &self.wResult)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DSKTLSYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear
            && self.wMonth == other.wMonth
            && self.wDayOfWeek == other.wDayOfWeek
            && self.wDay == other.wDay
            && self.wHour == other.wHour
            && self.wMinute == other.wMinute
            && self.wSecond == other.wSecond
            && self.wMilliseconds == other.wMilliseconds
            && self.wResult == other.wResult
    }
}
impl ::core::cmp::Eq for DSKTLSYSTEMTIME {}
impl FromIntoMemory for DSKTLSYSTEMTIME {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 18);
        let f_wYear = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wMonth = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_wDayOfWeek = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_wDay = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_wHour = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_wMinute = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_wSecond = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_wMilliseconds = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_wResult = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        Self {
            wYear: f_wYear,
            wMonth: f_wMonth,
            wDayOfWeek: f_wDayOfWeek,
            wDay: f_wDay,
            wHour: f_wHour,
            wMinute: f_wMinute,
            wSecond: f_wSecond,
            wMilliseconds: f_wMilliseconds,
            wResult: f_wResult,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 18);
        FromIntoMemory::into_bytes(self.wYear, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wMonth, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.wDayOfWeek, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.wDay, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.wHour, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.wMinute, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.wSecond, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.wMilliseconds, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.wResult, &mut into[16..16 + 2]);
    }
    fn size() -> usize {
        18
    }
}
pub const DTRESULTFIX: u32 = 1u32;
pub const DTRESULTOK: u32 = 0u32;
pub const DTRESULTPART: u32 = 3u32;
pub const DTRESULTPROB: u32 = 2u32;
pub const EISAFLAG_NO_IO_MERGE: u32 = 1u32;
pub const EISAFLAG_SLOT_IO_FIRST: u32 = 2u32;
pub const EISA_NO_MAX_FUNCTION: u32 = 255u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HKEY(pub PtrDiffRepr);
impl HKEY {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HKEY {}
impl ::core::hash::Hash for HKEY {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HKEY").field(&self.0).finish()
    }
}
impl FromIntoMemory for HKEY {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub const HKEY_CLASSES_ROOT: HKEY = HKEY(-2147483648i32 as _);
pub const HKEY_CURRENT_CONFIG: HKEY = HKEY(-2147483643i32 as _);
pub const HKEY_CURRENT_USER: HKEY = HKEY(-2147483647i32 as _);
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = HKEY(-2147483641i32 as _);
pub const HKEY_DYN_DATA: HKEY = HKEY(-2147483642i32 as _);
pub const HKEY_LOCAL_MACHINE: HKEY = HKEY(-2147483646i32 as _);
pub const HKEY_PERFORMANCE_DATA: HKEY = HKEY(-2147483644i32 as _);
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = HKEY(-2147483552i32 as _);
pub const HKEY_PERFORMANCE_TEXT: HKEY = HKEY(-2147483568i32 as _);
pub const HKEY_USERS: HKEY = HKEY(-2147483645i32 as _);
pub const IT_COMPACT: u32 = 0u32;
pub const IT_CUSTOM: u32 = 3u32;
pub const IT_PORTABLE: u32 = 2u32;
pub const IT_TYPICAL: u32 = 1u32;
pub const LASTGOOD_OPERATION: u32 = 255u32;
pub const LASTGOOD_OPERATION_DELETE: u32 = 1u32;
pub const LASTGOOD_OPERATION_NOPOSTPROC: u32 = 0u32;
pub const MF_FLAGS_CREATE_BUT_NO_SHOW_DISABLED: u32 = 8u32;
pub const MF_FLAGS_EVEN_IF_NO_RESOURCE: u32 = 1u32;
pub const MF_FLAGS_FILL_IN_UNKNOWN_RESOURCE: u32 = 4u32;
pub const MF_FLAGS_NO_CREATE_IF_NO_RESOURCE: u32 = 2u32;
pub const NUM_EISA_RANGES: u32 = 4u32;
pub const NUM_RESOURCE_MAP: u32 = 256u32;
pub const PCIC_DEFAULT_IRQMASK: u32 = 20152u32;
pub const PCIC_DEFAULT_NUMSOCKETS: u32 = 0u32;
pub const PCI_OPTIONS_USE_BIOS: i32 = 1i32;
pub const PCI_OPTIONS_USE_IRQ_STEERING: i32 = 2i32;
pub const PCMCIA_DEF_MEMBEGIN: u32 = 786432u32;
pub const PCMCIA_DEF_MEMEND: u32 = 16777215u32;
pub const PCMCIA_DEF_MEMLEN: u32 = 4096u32;
pub const PCMCIA_DEF_MIN_REGION: u32 = 65536u32;
pub const PCMCIA_OPT_AUTOMEM: i32 = 4i32;
pub const PCMCIA_OPT_HAVE_SOCKET: i32 = 1i32;
pub const PCMCIA_OPT_NO_APMREMOVE: i32 = 32i32;
pub const PCMCIA_OPT_NO_AUDIO: i32 = 16i32;
pub const PCMCIA_OPT_NO_SOUND: i32 = 8i32;
pub const PIR_OPTION_DEFAULT: u32 = 15u32;
pub const PIR_OPTION_ENABLED: u32 = 1u32;
pub const PIR_OPTION_MSSPEC: u32 = 4u32;
pub const PIR_OPTION_REALMODE: u32 = 8u32;
pub const PIR_OPTION_REGISTRY: u32 = 2u32;
pub const PIR_STATUS_DISABLED: u32 = 2u32;
pub const PIR_STATUS_ENABLED: u32 = 1u32;
pub const PIR_STATUS_ERROR: u32 = 0u32;
pub const PIR_STATUS_MAX: u32 = 3u32;
pub const PIR_STATUS_MINIPORT_COMPATIBLE: u32 = 1u32;
pub const PIR_STATUS_MINIPORT_ERROR: u32 = 4u32;
pub const PIR_STATUS_MINIPORT_INVALID: u32 = 7u32;
pub const PIR_STATUS_MINIPORT_MAX: u32 = 8u32;
pub const PIR_STATUS_MINIPORT_NOKEY: u32 = 5u32;
pub const PIR_STATUS_MINIPORT_NONE: u32 = 3u32;
pub const PIR_STATUS_MINIPORT_NORMAL: u32 = 0u32;
pub const PIR_STATUS_MINIPORT_OVERRIDE: u32 = 2u32;
pub const PIR_STATUS_MINIPORT_SUCCESS: u32 = 6u32;
pub const PIR_STATUS_TABLE_BAD: u32 = 5u32;
pub const PIR_STATUS_TABLE_ERROR: u32 = 4u32;
pub const PIR_STATUS_TABLE_MAX: u32 = 7u32;
pub const PIR_STATUS_TABLE_MSSPEC: u32 = 1u32;
pub const PIR_STATUS_TABLE_NONE: u32 = 3u32;
pub const PIR_STATUS_TABLE_REALMODE: u32 = 2u32;
pub const PIR_STATUS_TABLE_REGISTRY: u32 = 0u32;
pub const PIR_STATUS_TABLE_SUCCESS: u32 = 6u32;
pub type PQUERYHANDLER = StdCallFnPtr<
    (
        MutPtr<::core::ffi::c_void>,
        MutPtr<val_context>,
        u32,
        MutPtr<::core::ffi::c_void>,
        MutPtr<u32>,
        u32,
    ),
    u32,
>;
pub const PROVIDER_KEEPS_VALUE_LENGTH: u32 = 1u32;
pub const REGDF_CONFLICTDMA: u32 = 524288u32;
pub const REGDF_CONFLICTIO: u32 = 65536u32;
pub const REGDF_CONFLICTIRQ: u32 = 262144u32;
pub const REGDF_CONFLICTMEM: u32 = 131072u32;
pub const REGDF_GENFORCEDCONFIG: u32 = 32u32;
pub const REGDF_MAPIRQ2TO9: u32 = 1048576u32;
pub const REGDF_NEEDFULLCONFIG: u32 = 16u32;
pub const REGDF_NODETCONFIG: u32 = 32768u32;
pub const REGDF_NOTDETDMA: u32 = 8u32;
pub const REGDF_NOTDETIO: u32 = 1u32;
pub const REGDF_NOTDETIRQ: u32 = 4u32;
pub const REGDF_NOTDETMEM: u32 = 2u32;
pub const REGDF_NOTVERIFIED: u32 = 2147483648u32;
pub const REGSTR_DATA_NETOS_IPX: &'static str = "IPX";
pub const REGSTR_DATA_NETOS_NDIS: &'static str = "NDIS";
pub const REGSTR_DATA_NETOS_ODI: &'static str = "ODI";
pub const REGSTR_DEFAULT_INSTANCE: &'static str = "0000";
pub const REGSTR_KEY_ACPIENUM: &'static str = "ACPI";
pub const REGSTR_KEY_APM: &'static str = "*PNP0C05";
pub const REGSTR_KEY_BIOSENUM: &'static str = "BIOS";
pub const REGSTR_KEY_CLASS: &'static str = "Class";
pub const REGSTR_KEY_CONFIG: &'static str = "Config";
pub const REGSTR_KEY_CONTROL: &'static str = "Control";
pub const REGSTR_KEY_CRASHES: &'static str = "Crashes";
pub const REGSTR_KEY_CURRENT: &'static str = "Current";
pub const REGSTR_KEY_CURRENT_ENV: &'static str = "\\Windows 4.0";
pub const REGSTR_KEY_DANGERS: &'static str = "Dangers";
pub const REGSTR_KEY_DEFAULT: &'static str = "Default";
pub const REGSTR_KEY_DETMODVARS: &'static str = "DetModVars";
pub const REGSTR_KEY_DEVICEPARAMETERS: &'static str = "Device Parameters";
pub const REGSTR_KEY_DEVICE_PROPERTIES: &'static str = "Properties";
pub const REGSTR_KEY_DISPLAY_CLASS: &'static str = "Display";
pub const REGSTR_KEY_DOSOPTCDROM: &'static str = "CD-ROM";
pub const REGSTR_KEY_DOSOPTMOUSE: &'static str = "MOUSE";
pub const REGSTR_KEY_DRIVERPARAMETERS: &'static str = "Driver Parameters";
pub const REGSTR_KEY_DRIVERS: &'static str = "\\Drivers";
pub const REGSTR_KEY_EBDAUTOEXECBATKEYBOARD: &'static str = "EBDAutoexecBatKeyboard";
pub const REGSTR_KEY_EBDAUTOEXECBATLOCAL: &'static str = "EBDAutoexecBatLocale";
pub const REGSTR_KEY_EBDCONFIGSYSKEYBOARD: &'static str = "EBDConfigSysKeyboard";
pub const REGSTR_KEY_EBDCONFIGSYSLOCAL: &'static str = "EBDConfigSysLocale";
pub const REGSTR_KEY_EBDFILESKEYBOARD: &'static str = "EBDFilesKeyboard";
pub const REGSTR_KEY_EBDFILESLOCAL: &'static str = "EBDFilesLocale";
pub const REGSTR_KEY_EISAENUM: &'static str = "EISA";
pub const REGSTR_KEY_ENUM: &'static str = "Enum";
pub const REGSTR_KEY_EXPLORER: &'static str = "Explorer";
pub const REGSTR_KEY_FILTERS: &'static str = "Filters";
pub const REGSTR_KEY_INIUPDATE: &'static str = "IniUpdate";
pub const REGSTR_KEY_ISAENUM: &'static str = "ISAPnP";
pub const REGSTR_KEY_JOYCURR: &'static str = "CurrentJoystickSettings";
pub const REGSTR_KEY_JOYSETTINGS: &'static str = "JoystickSettings";
pub const REGSTR_KEY_KEYBOARD_CLASS: &'static str = "Keyboard";
pub const REGSTR_KEY_KNOWNDOCKINGSTATES: &'static str = "Hardware Profiles";
pub const REGSTR_KEY_LOGCONFIG: &'static str = "LogConfig";
pub const REGSTR_KEY_LOGON: &'static str = "\\Logon";
pub const REGSTR_KEY_LOWER_FILTER_LEVEL_DEFAULT: &'static str = "*Lower";
pub const REGSTR_KEY_MEDIA_CLASS: &'static str = "MEDIA";
pub const REGSTR_KEY_MODEM_CLASS: &'static str = "Modem";
pub const REGSTR_KEY_MODES: &'static str = "Modes";
pub const REGSTR_KEY_MONITOR_CLASS: &'static str = "Monitor";
pub const REGSTR_KEY_MOUSE_CLASS: &'static str = "Mouse";
pub const REGSTR_KEY_NDISINFO: &'static str = "NDISInfo";
pub const REGSTR_KEY_NETWORK: &'static str = "Network";
pub const REGSTR_KEY_NETWORKPROVIDER: &'static str = "\\NetworkProvider";
pub const REGSTR_KEY_NETWORK_PERSISTENT: &'static str = "\\Persistent";
pub const REGSTR_KEY_NETWORK_RECENT: &'static str = "\\Recent";
pub const REGSTR_KEY_OVERRIDE: &'static str = "Override";
pub const REGSTR_KEY_PCIENUM: &'static str = "PCI";
pub const REGSTR_KEY_PCMCIA: &'static str = "PCMCIA\\";
pub const REGSTR_KEY_PCMCIAENUM: &'static str = "PCMCIA";
pub const REGSTR_KEY_PCMCIA_CLASS: &'static str = "PCMCIA";
pub const REGSTR_KEY_PCMTD: &'static str = "MTD-";
pub const REGSTR_KEY_PCUNKNOWN: &'static str = "UNKNOWN_MANUFACTURER";
pub const REGSTR_KEY_POL_COMPUTERS: &'static str = "Computers";
pub const REGSTR_KEY_POL_DEFAULT: &'static str = ".default";
pub const REGSTR_KEY_POL_USERGROUPDATA: &'static str = "GroupData\\UserGroups\\Priority";
pub const REGSTR_KEY_POL_USERGROUPS: &'static str = "UserGroups";
pub const REGSTR_KEY_POL_USERS: &'static str = "Users";
pub const REGSTR_KEY_PORTS_CLASS: &'static str = "ports";
pub const REGSTR_KEY_PRINTERS: &'static str = "Printers";
pub const REGSTR_KEY_PRINT_PROC: &'static str = "\\Print Processors";
pub const REGSTR_KEY_ROOTENUM: &'static str = "Root";
pub const REGSTR_KEY_RUNHISTORY: &'static str = "RunHistory";
pub const REGSTR_KEY_SCSI_CLASS: &'static str = "SCSIAdapter";
pub const REGSTR_KEY_SETUP: &'static str = "\\Setup";
pub const REGSTR_KEY_SHARES: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Network\\LanMan";
pub const REGSTR_KEY_SYSTEM: &'static str = "System";
pub const REGSTR_KEY_SYSTEMBOARD: &'static str = "*PNP0C01";
pub const REGSTR_KEY_UPPER_FILTER_LEVEL_DEFAULT: &'static str = "*Upper";
pub const REGSTR_KEY_USER: &'static str = "User";
pub const REGSTR_KEY_VPOWERDENUM: &'static str = "VPOWERD";
pub const REGSTR_KEY_WINOLDAPP: &'static str = "WinOldApp";
pub const REGSTR_MACHTYPE_ATT_PC: &'static str = "AT&T PC";
pub const REGSTR_MACHTYPE_HP_VECTRA: &'static str = "HP Vectra";
pub const REGSTR_MACHTYPE_IBMPC: &'static str = "IBM PC";
pub const REGSTR_MACHTYPE_IBMPCAT: &'static str = "IBM PC/AT";
pub const REGSTR_MACHTYPE_IBMPCCONV: &'static str = "IBM PC Convertible";
pub const REGSTR_MACHTYPE_IBMPCJR: &'static str = "IBM PCjr";
pub const REGSTR_MACHTYPE_IBMPCXT: &'static str = "IBM PC/XT";
pub const REGSTR_MACHTYPE_IBMPCXT_286: &'static str = "IBM PC/XT 286";
pub const REGSTR_MACHTYPE_IBMPS1: &'static str = "IBM PS/1";
pub const REGSTR_MACHTYPE_IBMPS2_25: &'static str = "IBM PS/2-25";
pub const REGSTR_MACHTYPE_IBMPS2_30: &'static str = "IBM PS/2-30";
pub const REGSTR_MACHTYPE_IBMPS2_30_286: &'static str = "IBM PS/2-30 286";
pub const REGSTR_MACHTYPE_IBMPS2_50: &'static str = "IBM PS/2-50";
pub const REGSTR_MACHTYPE_IBMPS2_50Z: &'static str = "IBM PS/2-50Z";
pub const REGSTR_MACHTYPE_IBMPS2_55SX: &'static str = "IBM PS/2-55SX";
pub const REGSTR_MACHTYPE_IBMPS2_60: &'static str = "IBM PS/2-60";
pub const REGSTR_MACHTYPE_IBMPS2_65SX: &'static str = "IBM PS/2-65SX";
pub const REGSTR_MACHTYPE_IBMPS2_70: &'static str = "IBM PS/2-70";
pub const REGSTR_MACHTYPE_IBMPS2_70_80: &'static str = "IBM PS/2-70/80";
pub const REGSTR_MACHTYPE_IBMPS2_80: &'static str = "IBM PS/2-80";
pub const REGSTR_MACHTYPE_IBMPS2_90: &'static str = "IBM PS/2-90";
pub const REGSTR_MACHTYPE_IBMPS2_P70: &'static str = "IBM PS/2-P70";
pub const REGSTR_MACHTYPE_PHOENIX_PCAT: &'static str = "Phoenix PC/AT Compatible";
pub const REGSTR_MACHTYPE_UNKNOWN: &'static str = "Unknown";
pub const REGSTR_MACHTYPE_ZENITH_PC: &'static str = "Zenith PC";
pub const REGSTR_MAX_VALUE_LENGTH: u32 = 256u32;
pub const REGSTR_PATH_ADDRARB: &'static str =
    "System\\CurrentControlSet\\Services\\Arbitrators\\AddrArb";
pub const REGSTR_PATH_AEDEBUG: &'static str =
    "Software\\Microsoft\\Windows NT\\CurrentVersion\\AeDebug";
pub const REGSTR_PATH_APPEARANCE: &'static str = "Control Panel\\Appearance";
pub const REGSTR_PATH_APPPATCH: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\AppPatches";
pub const REGSTR_PATH_APPPATHS: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\App Paths";
pub const REGSTR_PATH_BIOSINFO: &'static str = "System\\CurrentControlSet\\Control\\BiosInfo";
pub const REGSTR_PATH_BUSINFORMATION: &'static str =
    "System\\CurrentControlSet\\Control\\PnP\\BusInformation";
pub const REGSTR_PATH_CDFS: &'static str = "System\\CurrentControlSet\\Control\\FileSystem\\CDFS";
pub const REGSTR_PATH_CHECKBADAPPS: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps";
pub const REGSTR_PATH_CHECKBADAPPS400: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps400";
pub const REGSTR_PATH_CHECKDISK: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive";
pub const REGSTR_PATH_CHECKDISKSET: &'static str = "Settings";
pub const REGSTR_PATH_CHECKDISKUDRVS: &'static str = "NoUnknownDDErrDrvs";
pub const REGSTR_PATH_CHECKVERDLLS: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\CheckVerDLLs";
pub const REGSTR_PATH_CHILD_PREFIX: &'static str = "Child";
pub const REGSTR_PATH_CHKLASTCHECK: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastCheck";
pub const REGSTR_PATH_CHKLASTSURFAN: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastSurfaceAnalysis";
pub const REGSTR_PATH_CLASS: &'static str = "System\\CurrentControlSet\\Services\\Class";
pub const REGSTR_PATH_CLASS_NT: &'static str = "System\\CurrentControlSet\\Control\\Class";
pub const REGSTR_PATH_CODEPAGE: &'static str = "System\\CurrentControlSet\\Control\\Nls\\Codepage";
pub const REGSTR_PATH_CODEVICEINSTALLERS: &'static str =
    "System\\CurrentControlSet\\Control\\CoDeviceInstallers";
pub const REGSTR_PATH_COLORS: &'static str = "Control Panel\\Colors";
pub const REGSTR_PATH_COMPUTRNAME: &'static str =
    "System\\CurrentControlSet\\Control\\ComputerName\\ComputerName";
pub const REGSTR_PATH_CONTROLPANEL: &'static str = "Control Panel";
pub const REGSTR_PATH_CONTROLSFOLDER: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Controls Folder";
pub const REGSTR_PATH_CRITICALDEVICEDATABASE: &'static str =
    "System\\CurrentControlSet\\Control\\CriticalDeviceDatabase";
pub const REGSTR_PATH_CURRENTCONTROLSET: &'static str = "System\\CurrentControlSet";
pub const REGSTR_PATH_CURRENT_CONTROL_SET: &'static str = "System\\CurrentControlSet\\Control";
pub const REGSTR_PATH_CURSORS: &'static str = "Control Panel\\Cursors";
pub const REGSTR_PATH_CVNETWORK: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Network";
pub const REGSTR_PATH_DESKTOP: &'static str = "Control Panel\\Desktop";
pub const REGSTR_PATH_DETECT: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Detect";
pub const REGSTR_PATH_DEVICEINSTALLER: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Device Installer";
pub const REGSTR_PATH_DEVICE_CLASSES: &'static str =
    "System\\CurrentControlSet\\Control\\DeviceClasses";
pub const REGSTR_PATH_DIFX: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\DIFX";
pub const REGSTR_PATH_DISPLAYSETTINGS: &'static str = "Display\\Settings";
pub const REGSTR_PATH_DMAARB: &'static str =
    "System\\CurrentControlSet\\Services\\Arbitrators\\DMAArb";
pub const REGSTR_PATH_DRIVERSIGN: &'static str = "Software\\Microsoft\\Driver Signing";
pub const REGSTR_PATH_DRIVERSIGN_POLICY: &'static str =
    "Software\\Policies\\Microsoft\\Windows NT\\Driver Signing";
pub const REGSTR_PATH_ENUM: &'static str = "Enum";
pub const REGSTR_PATH_ENVIRONMENTS: &'static str =
    "System\\CurrentControlSet\\Control\\Print\\Environments";
pub const REGSTR_PATH_EVENTLABELS: &'static str = "AppEvents\\EventLabels";
pub const REGSTR_PATH_EXPLORER: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer";
pub const REGSTR_PATH_FAULT: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Fault";
pub const REGSTR_PATH_FILESYSTEM: &'static str = "System\\CurrentControlSet\\Control\\FileSystem";
pub const REGSTR_PATH_FILESYSTEM_NOVOLTRACK: &'static str =
    "System\\CurrentControlSet\\Control\\FileSystem\\NoVolTrack";
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR: &'static str =
    "HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor";
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR0: &'static str =
    "HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor\\0";
pub const REGSTR_PATH_FONTS: &'static str = "Display\\Fonts";
pub const REGSTR_PATH_GRPCONV: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\GrpConv";
pub const REGSTR_PATH_HACKINIFILE: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\HackIniFiles";
pub const REGSTR_PATH_HWPROFILES: &'static str = "System\\CurrentControlSet\\Hardware Profiles";
pub const REGSTR_PATH_HWPROFILESCURRENT: &'static str =
    "System\\CurrentControlSet\\Hardware Profiles\\Current";
pub const REGSTR_PATH_ICONS: &'static str = "Control Panel\\Icons";
pub const REGSTR_PATH_IDCONFIGDB: &'static str = "System\\CurrentControlSet\\Control\\IDConfigDB";
pub const REGSTR_PATH_INSTALLEDFILES: &'static str =
    "System\\CurrentControlSet\\Control\\InstalledFiles";
pub const REGSTR_PATH_IOARB: &'static str =
    "System\\CurrentControlSet\\Services\\Arbitrators\\IOArb";
pub const REGSTR_PATH_IOS: &'static str = "System\\CurrentControlSet\\Services\\VxD\\IOS";
pub const REGSTR_PATH_IRQARB: &'static str =
    "System\\CurrentControlSet\\Services\\Arbitrators\\IRQArb";
pub const REGSTR_PATH_KEYBOARD: &'static str = "Control Panel\\Keyboard";
pub const REGSTR_PATH_KNOWN16DLLS: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\Known16DLLs";
pub const REGSTR_PATH_KNOWNDLLS: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\KnownDLLs";
pub const REGSTR_PATH_KNOWNVXDS: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\KnownVxDs";
pub const REGSTR_PATH_LASTBACKUP: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastBackup";
pub const REGSTR_PATH_LASTCHECK: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastCheck";
pub const REGSTR_PATH_LASTGOOD: &'static str = "System\\LastKnownGoodRecovery\\LastGood";
pub const REGSTR_PATH_LASTGOODTMP: &'static str = "System\\LastKnownGoodRecovery\\LastGood.Tmp";
pub const REGSTR_PATH_LASTOPTIMIZE: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastOptimize";
pub const REGSTR_PATH_LOOKSCHEMES: &'static str = "Control Panel\\Appearance\\Schemes";
pub const REGSTR_PATH_METRICS: &'static str = "Control Panel\\Desktop\\WindowMetrics";
pub const REGSTR_PATH_MONITORS: &'static str =
    "System\\CurrentControlSet\\Control\\Print\\Monitors";
pub const REGSTR_PATH_MOUSE: &'static str = "Control Panel\\Mouse";
pub const REGSTR_PATH_MSDOSOPTS: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSOptions";
pub const REGSTR_PATH_MULTIMEDIA_AUDIO: &'static str = "Software\\Microsoft\\Multimedia\\Audio";
pub const REGSTR_PATH_MULTI_FUNCTION: &'static str = "MF";
pub const REGSTR_PATH_NCPSERVER: &'static str =
    "System\\CurrentControlSet\\Services\\NcpServer\\Parameters";
pub const REGSTR_PATH_NETEQUIV: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Equivalent";
pub const REGSTR_PATH_NETWORK_USERSETTINGS: &'static str = "Network";
pub const REGSTR_PATH_NEWDOSBOX: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSSpecialConfig";
pub const REGSTR_PATH_NONDRIVERSIGN: &'static str = "Software\\Microsoft\\Non-Driver Signing";
pub const REGSTR_PATH_NONDRIVERSIGN_POLICY: &'static str =
    "Software\\Policies\\Microsoft\\Windows NT\\Non-Driver Signing";
pub const REGSTR_PATH_NOSUGGMSDOS: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\NoMSDOSWarn";
pub const REGSTR_PATH_NT_CURRENTVERSION: &'static str =
    "Software\\Microsoft\\Windows NT\\CurrentVersion";
pub const REGSTR_PATH_NWREDIR: &'static str = "System\\CurrentControlSet\\Services\\VxD\\NWREDIR";
pub const REGSTR_PATH_PCIIR: &'static str =
    "System\\CurrentControlSet\\Control\\Pnp\\PciIrqRouting";
pub const REGSTR_PATH_PER_HW_ID_STORAGE: &'static str =
    "Software\\Microsoft\\Windows NT\\CurrentVersion\\PerHwIdStorage";
pub const REGSTR_PATH_PIFCONVERT: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\PIFConvert";
pub const REGSTR_PATH_POLICIES: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Policies";
pub const REGSTR_PATH_PRINT: &'static str = "System\\CurrentControlSet\\Control\\Print";
pub const REGSTR_PATH_PRINTERS: &'static str =
    "System\\CurrentControlSet\\Control\\Print\\Printers";
pub const REGSTR_PATH_PROPERTYSYSTEM: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\PropertySystem";
pub const REGSTR_PATH_PROVIDERS: &'static str =
    "System\\CurrentControlSet\\Control\\Print\\Providers";
pub const REGSTR_PATH_PWDPROVIDER: &'static str = "System\\CurrentControlSet\\Control\\PwdProvider";
pub const REGSTR_PATH_REALMODENET: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Real Mode Net";
pub const REGSTR_PATH_REINSTALL: &'static str =
    "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Reinstall";
pub const REGSTR_PATH_RELIABILITY: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Reliability";
pub const REGSTR_PATH_RELIABILITY_POLICY: &'static str =
    "Software\\Policies\\Microsoft\\Windows NT\\Reliability";
pub const REGSTR_PATH_RELIABILITY_POLICY_REPORTSNAPSHOT: &'static str = "ReportSnapshot";
pub const REGSTR_PATH_RELIABILITY_POLICY_SHUTDOWNREASONUI: &'static str = "ShutdownReasonUI";
pub const REGSTR_PATH_RELIABILITY_POLICY_SNAPSHOT: &'static str = "Snapshot";
pub const REGSTR_PATH_ROOT: &'static str = "Enum\\Root";
pub const REGSTR_PATH_RUN: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
pub const REGSTR_PATH_RUNONCE: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce";
pub const REGSTR_PATH_RUNONCEEX: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnceEx";
pub const REGSTR_PATH_RUNSERVICES: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\RunServices";
pub const REGSTR_PATH_RUNSERVICESONCE: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\RunServicesOnce";
pub const REGSTR_PATH_SCHEMES: &'static str = "AppEvents\\Schemes";
pub const REGSTR_PATH_SCREENSAVE: &'static str = "Control Panel\\Desktop";
pub const REGSTR_PATH_SERVICES: &'static str = "System\\CurrentControlSet\\Services";
pub const REGSTR_PATH_SETUP: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion";
pub const REGSTR_PATH_SHUTDOWN: &'static str = "System\\CurrentControlSet\\Control\\Shutdown";
pub const REGSTR_PATH_SOUND: &'static str = "Control Panel\\Sound";
pub const REGSTR_PATH_SYSTEMENUM: &'static str = "System\\CurrentControlSet\\Enum";
pub const REGSTR_PATH_SYSTRAY: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\SysTray";
pub const REGSTR_PATH_TIMEZONE: &'static str =
    "System\\CurrentControlSet\\Control\\TimeZoneInformation";
pub const REGSTR_PATH_UNINSTALL: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
pub const REGSTR_PATH_UPDATE: &'static str = "System\\CurrentControlSet\\Control\\Update";
pub const REGSTR_PATH_VCOMM: &'static str = "System\\CurrentControlSet\\Services\\VxD\\VCOMM";
pub const REGSTR_PATH_VMM: &'static str = "System\\CurrentControlSet\\Services\\VxD\\VMM";
pub const REGSTR_PATH_VMM32FILES: &'static str = "System\\CurrentControlSet\\Control\\VMM32Files";
pub const REGSTR_PATH_VNETSUP: &'static str = "System\\CurrentControlSet\\Services\\VxD\\VNETSUP";
pub const REGSTR_PATH_VOLUMECACHE: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VolumeCaches";
pub const REGSTR_PATH_VPOWERD: &'static str = "System\\CurrentControlSet\\Services\\VxD\\VPOWERD";
pub const REGSTR_PATH_VXD: &'static str = "System\\CurrentControlSet\\Services\\VxD";
pub const REGSTR_PATH_WARNVERDLLS: &'static str =
    "System\\CurrentControlSet\\Control\\SessionManager\\WarnVerDLLs";
pub const REGSTR_PATH_WINBOOT: &'static str = "System\\CurrentControlSet\\Control\\WinBoot";
pub const REGSTR_PATH_WINDOWSAPPLETS: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Applets";
pub const REGSTR_PATH_WINLOGON: &'static str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Winlogon";
pub const REGSTR_PATH_WMI_SECURITY: &'static str =
    "System\\CurrentControlSet\\Control\\Wmi\\Security";
pub const REGSTR_PCI_DUAL_IDE: &'static str = "PCIDualIDE";
pub const REGSTR_PCI_OPTIONS: &'static str = "Options";
pub const REGSTR_VALUE_DEFAULTLOC: &'static str = "UseDefaultNetLocation";
pub const REGSTR_VALUE_ENABLE: &'static str = "Enable";
pub const REGSTR_VALUE_LOWPOWERACTIVE: &'static str = "ScreenSaveLowPowerActive";
pub const REGSTR_VALUE_LOWPOWERTIMEOUT: &'static str = "ScreenSaveLowPowerTimeout";
pub const REGSTR_VALUE_NETPATH: &'static str = "NetworkPath";
pub const REGSTR_VALUE_POWEROFFACTIVE: &'static str = "ScreenSavePowerOffActive";
pub const REGSTR_VALUE_POWEROFFTIMEOUT: &'static str = "ScreenSavePowerOffTimeout";
pub const REGSTR_VALUE_SCRPASSWORD: &'static str = "ScreenSave_Data";
pub const REGSTR_VALUE_USESCRPASSWORD: &'static str = "ScreenSaveUsePassword";
pub const REGSTR_VALUE_VERBOSE: &'static str = "Verbose";
pub const REGSTR_VAL_ACDRIVESPINDOWN: &'static str = "ACDriveSpinDown";
pub const REGSTR_VAL_ACSPINDOWNPREVIOUS: &'static str = "ACSpinDownPrevious";
pub const REGSTR_VAL_ACTIVESERVICE: &'static str = "ActiveService";
pub const REGSTR_VAL_ADDRESS: &'static str = "Address";
pub const REGSTR_VAL_AEDEBUG_AUTO: &'static str = "Auto";
pub const REGSTR_VAL_AEDEBUG_DEBUGGER: &'static str = "Debugger";
pub const REGSTR_VAL_ALPHANUMPWDS: &'static str = "AlphanumPwds";
pub const REGSTR_VAL_APISUPPORT: &'static str = "APISupport";
pub const REGSTR_VAL_APMACTIMEOUT: &'static str = "APMACTimeout";
pub const REGSTR_VAL_APMBATTIMEOUT: &'static str = "APMBatTimeout";
pub const REGSTR_VAL_APMBIOSVER: &'static str = "APMBiosVer";
pub const REGSTR_VAL_APMFLAGS: &'static str = "APMFlags";
pub const REGSTR_VAL_APMMENUSUSPEND: &'static str = "APMMenuSuspend";
pub const REGSTR_VAL_APMSHUTDOWNPOWER: &'static str = "APMShutDownPower";
pub const REGSTR_VAL_APPINSTPATH: &'static str = "AppInstallPath";
pub const REGSTR_VAL_ASKFORCONFIG: &'static str = "AskForConfig";
pub const REGSTR_VAL_ASKFORCONFIGFUNC: &'static str = "AskForConfigFunc";
pub const REGSTR_VAL_ASYNCFILECOMMIT: &'static str = "AsyncFileCommit";
pub const REGSTR_VAL_AUDIO_BITMAP: &'static str = "bitmap";
pub const REGSTR_VAL_AUDIO_ICON: &'static str = "icon";
pub const REGSTR_VAL_AUTHENT_AGENT: &'static str = "AuthenticatingAgent";
pub const REGSTR_VAL_AUTOEXEC: &'static str = "Autoexec.Bat";
pub const REGSTR_VAL_AUTOINSNOTE: &'static str = "AutoInsertNotification";
pub const REGSTR_VAL_AUTOLOGON: &'static str = "AutoLogon";
pub const REGSTR_VAL_AUTOMOUNT: &'static str = "AutoMountDrives";
pub const REGSTR_VAL_AUTOSTART: &'static str = "AutoStart";
pub const REGSTR_VAL_BASICPROPERTIES: &'static str = "BasicProperties";
pub const REGSTR_VAL_BASICPROPERTIES_32: &'static str = "BasicProperties32";
pub const REGSTR_VAL_BATDRIVESPINDOWN: &'static str = "BatDriveSpinDown";
pub const REGSTR_VAL_BATSPINDOWNPREVIOUS: &'static str = "BatSpinDownPrevious";
pub const REGSTR_VAL_BEHAVIOR_ON_FAILED_VERIFY: &'static str = "BehaviorOnFailedVerify";
pub const REGSTR_VAL_BIOSDATE: &'static str = "BIOSDate";
pub const REGSTR_VAL_BIOSNAME: &'static str = "BIOSName";
pub const REGSTR_VAL_BIOSVERSION: &'static str = "BIOSVersion";
pub const REGSTR_VAL_BITSPERPIXEL: &'static str = "BitsPerPixel";
pub const REGSTR_VAL_BOOTCONFIG: &'static str = "BootConfig";
pub const REGSTR_VAL_BOOTCOUNT: &'static str = "BootCount";
pub const REGSTR_VAL_BOOTDIR: &'static str = "BootDir";
pub const REGSTR_VAL_BPP: &'static str = "BPP";
pub const REGSTR_VAL_BT: &'static str = "6005BT";
pub const REGSTR_VAL_BUFFAGETIMEOUT: &'static str = "BufferAgeTimeout";
pub const REGSTR_VAL_BUFFIDLETIMEOUT: &'static str = "BufferIdleTimeout";
pub const REGSTR_VAL_BUSTYPE: &'static str = "BusType";
pub const REGSTR_VAL_CAPABILITIES: &'static str = "Capabilities";
pub const REGSTR_VAL_CARDSPECIFIC: &'static str = "CardSpecific";
pub const REGSTR_VAL_CDCACHESIZE: &'static str = "CacheSize";
pub const REGSTR_VAL_CDCOMPATNAMES: &'static str = "MSCDEXCompatNames";
pub const REGSTR_VAL_CDEXTERRORS: &'static str = "ExtendedErrors";
pub const REGSTR_VAL_CDNOREADAHEAD: &'static str = "NoReadAhead";
pub const REGSTR_VAL_CDPREFETCH: &'static str = "Prefetch";
pub const REGSTR_VAL_CDPREFETCHTAIL: &'static str = "PrefetchTail";
pub const REGSTR_VAL_CDRAWCACHE: &'static str = "RawCache";
pub const REGSTR_VAL_CDROM: &'static str = "GenCD";
pub const REGSTR_VAL_CDROMCLASSNAME: &'static str = "CDROM";
pub const REGSTR_VAL_CDSHOWVERSIONS: &'static str = "ShowVersions";
pub const REGSTR_VAL_CDSVDSENSE: &'static str = "SVDSense";
pub const REGSTR_VAL_CHECKSUM: &'static str = "CurrentChecksum";
pub const REGSTR_VAL_CLASS: &'static str = "Class";
pub const REGSTR_VAL_CLASSDESC: &'static str = "ClassDesc";
pub const REGSTR_VAL_CLASSGUID: &'static str = "ClassGUID";
pub const REGSTR_VAL_CMDRIVFLAGS: &'static str = "CMDrivFlags";
pub const REGSTR_VAL_CMENUMFLAGS: &'static str = "CMEnumFlags";
pub const REGSTR_VAL_COINSTALLERS_32: &'static str = "CoInstallers32";
pub const REGSTR_VAL_COMINFO: &'static str = "ComInfo";
pub const REGSTR_VAL_COMMENT: &'static str = "Comment";
pub const REGSTR_VAL_COMPATIBLEIDS: &'static str = "CompatibleIDs";
pub const REGSTR_VAL_COMPRESSIONMETHOD: &'static str = "CompressionAlgorithm";
pub const REGSTR_VAL_COMPRESSIONTHRESHOLD: &'static str = "CompressionThreshold";
pub const REGSTR_VAL_COMPUTERNAME: &'static str = "ComputerName";
pub const REGSTR_VAL_COMPUTRNAME: &'static str = "ComputerName";
pub const REGSTR_VAL_COMVERIFYBASE: &'static str = "COMVerifyBase";
pub const REGSTR_VAL_CONFIG: &'static str = "ConfigPath";
pub const REGSTR_VAL_CONFIGFLAGS: &'static str = "ConfigFlags";
pub const REGSTR_VAL_CONFIGMG: &'static str = "CONFIGMG";
pub const REGSTR_VAL_CONFIGSYS: &'static str = "Config.Sys";
pub const REGSTR_VAL_CONNECTION_TYPE: &'static str = "ConnectionType";
pub const REGSTR_VAL_CONTAINERID: &'static str = "ContainerID";
pub const REGSTR_VAL_CONTIGFILEALLOC: &'static str = "ContigFileAllocSize";
pub const REGSTR_VAL_CONVMEM: &'static str = "ConvMem";
pub const REGSTR_VAL_CPU: &'static str = "CPU";
pub const REGSTR_VAL_CRASHFUNCS: &'static str = "CrashFuncs";
pub const REGSTR_VAL_CSCONFIGFLAGS: &'static str = "CSConfigFlags";
pub const REGSTR_VAL_CURCONFIG: &'static str = "CurrentConfig";
pub const REGSTR_VAL_CURDRVLET: &'static str = "CurrentDriveLetterAssignment";
pub const REGSTR_VAL_CURRENTCONFIG: &'static str = "CurrentConfig";
pub const REGSTR_VAL_CURRENT_BUILD: &'static str = "CurrentBuildNumber";
pub const REGSTR_VAL_CURRENT_CSDVERSION: &'static str = "CSDVersion";
pub const REGSTR_VAL_CURRENT_TYPE: &'static str = "CurrentType";
pub const REGSTR_VAL_CURRENT_USER: &'static str = "Current User";
pub const REGSTR_VAL_CURRENT_VERSION: &'static str = "CurrentVersion";
pub const REGSTR_VAL_CUSTOMCOLORS: &'static str = "CustomColors";
pub const REGSTR_VAL_CUSTOM_PROPERTY_CACHE_DATE: &'static str = "CustomPropertyCacheDate";
pub const REGSTR_VAL_CUSTOM_PROPERTY_HW_ID_KEY: &'static str = "CustomPropertyHwIdKey";
pub const REGSTR_VAL_DEFAULT: &'static str = "Default";
pub const REGSTR_VAL_DETCONFIG: &'static str = "DetConfig";
pub const REGSTR_VAL_DETECT: &'static str = "Detect";
pub const REGSTR_VAL_DETECTFUNC: &'static str = "DetectFunc";
pub const REGSTR_VAL_DETFLAGS: &'static str = "DetFlags";
pub const REGSTR_VAL_DETFUNC: &'static str = "DetFunc";
pub const REGSTR_VAL_DEVDESC: &'static str = "DeviceDesc";
pub const REGSTR_VAL_DEVICEDRIVER: &'static str = "DeviceDriver";
pub const REGSTR_VAL_DEVICEPATH: &'static str = "DevicePath";
pub const REGSTR_VAL_DEVICE_CHARACTERISTICS: &'static str = "DeviceCharacteristics";
pub const REGSTR_VAL_DEVICE_EXCLUSIVE: &'static str = "Exclusive";
pub const REGSTR_VAL_DEVICE_INSTANCE: &'static str = "DeviceInstance";
pub const REGSTR_VAL_DEVICE_SECURITY_DESCRIPTOR: &'static str = "Security";
pub const REGSTR_VAL_DEVICE_TYPE: &'static str = "DeviceType";
pub const REGSTR_VAL_DEVLOADER: &'static str = "DevLoader";
pub const REGSTR_VAL_DEVTYPE: &'static str = "DeviceType";
pub const REGSTR_VAL_DIRECTHOST: &'static str = "DirectHost";
pub const REGSTR_VAL_DIRTYSHUTDOWN: &'static str = "DirtyShutdown";
pub const REGSTR_VAL_DIRTYSHUTDOWNTIME: &'static str = "DirtyShutdownTime";
pub const REGSTR_VAL_DISABLECOUNT: &'static str = "DisableCount";
pub const REGSTR_VAL_DISABLEPWDCACHING: &'static str = "DisablePwdCaching";
pub const REGSTR_VAL_DISABLEREGTOOLS: &'static str = "DisableRegistryTools";
pub const REGSTR_VAL_DISCONNECT: &'static str = "Disconnect";
pub const REGSTR_VAL_DISK: &'static str = "GenDisk";
pub const REGSTR_VAL_DISKCLASSNAME: &'static str = "DiskDrive";
pub const REGSTR_VAL_DISPCPL_NOAPPEARANCEPAGE: &'static str = "NoDispAppearancePage";
pub const REGSTR_VAL_DISPCPL_NOBACKGROUNDPAGE: &'static str = "NoDispBackgroundPage";
pub const REGSTR_VAL_DISPCPL_NODISPCPL: &'static str = "NoDispCPL";
pub const REGSTR_VAL_DISPCPL_NOSCRSAVPAGE: &'static str = "NoDispScrSavPage";
pub const REGSTR_VAL_DISPCPL_NOSETTINGSPAGE: &'static str = "NoDispSettingsPage";
pub const REGSTR_VAL_DISPLAY: &'static str = "display";
pub const REGSTR_VAL_DISPLAYFLAGS: &'static str = "DisplayFlags";
pub const REGSTR_VAL_DOCKED: &'static str = "CurrentDockedState";
pub const REGSTR_VAL_DOCKSTATE: &'static str = "DockState";
pub const REGSTR_VAL_DOES_POLLING: &'static str = "PollingSupportNeeded";
pub const REGSTR_VAL_DONTLOADIFCONFLICT: &'static str = "DontLoadIfConflict";
pub const REGSTR_VAL_DONTUSEMEM: &'static str = "DontAllocLastMem";
pub const REGSTR_VAL_DOSCP: &'static str = "OEMCP";
pub const REGSTR_VAL_DOSOPTFLAGS: &'static str = "Flags";
pub const REGSTR_VAL_DOSOPTGLOBALFLAGS: &'static str = "GlobalFlags";
pub const REGSTR_VAL_DOSOPTTIP: &'static str = "TipText";
pub const REGSTR_VAL_DOSPAGER: &'static str = "DOSPager";
pub const REGSTR_VAL_DOS_SPOOL_MASK: &'static str = "DOSSpoolMask";
pub const REGSTR_VAL_DOUBLEBUFFER: &'static str = "DoubleBuffer";
pub const REGSTR_VAL_DPI: &'static str = "dpi";
pub const REGSTR_VAL_DPILOGICALX: &'static str = "DPILogicalX";
pub const REGSTR_VAL_DPILOGICALY: &'static str = "DPILogicalY";
pub const REGSTR_VAL_DPIPHYSICALX: &'static str = "DPIPhysicalX";
pub const REGSTR_VAL_DPIPHYSICALY: &'static str = "DPIPhysicalY";
pub const REGSTR_VAL_DPMS: &'static str = "DPMS";
pub const REGSTR_VAL_DRIVER: &'static str = "Driver";
pub const REGSTR_VAL_DRIVERCACHEPATH: &'static str = "DriverCachePath";
pub const REGSTR_VAL_DRIVERDATE: &'static str = "DriverDate";
pub const REGSTR_VAL_DRIVERDATEDATA: &'static str = "DriverDateData";
pub const REGSTR_VAL_DRIVERVERSION: &'static str = "DriverVersion";
pub const REGSTR_VAL_DRIVESPINDOWN: &'static str = "DriveSpinDown";
pub const REGSTR_VAL_DRIVEWRITEBEHIND: &'static str = "DriveWriteBehind";
pub const REGSTR_VAL_DRIVE_SPINDOWN: &'static str = "NoDispSpinDown";
pub const REGSTR_VAL_DRV: &'static str = "drv";
pub const REGSTR_VAL_DRVDESC: &'static str = "DriverDesc";
pub const REGSTR_VAL_DYNAMIC: &'static str = "Dynamic";
pub const REGSTR_VAL_EISA_FLAGS: &'static str = "EISAFlags";
pub const REGSTR_VAL_EISA_FUNCTIONS: &'static str = "EISAFunctions";
pub const REGSTR_VAL_EISA_FUNCTIONS_MASK: &'static str = "EISAFunctionsMask";
pub const REGSTR_VAL_EISA_RANGES: &'static str = "EISARanges";
pub const REGSTR_VAL_EISA_SIMULATE_INT15: &'static str = "EISASimulateInt15";
pub const REGSTR_VAL_EJECT_PRIORITY: &'static str = "EjectPriority";
pub const REGSTR_VAL_ENABLEINTS: &'static str = "EnableInts";
pub const REGSTR_VAL_ENUMERATOR: &'static str = "Enumerator";
pub const REGSTR_VAL_ENUMPROPPAGES: &'static str = "EnumPropPages";
pub const REGSTR_VAL_ENUMPROPPAGES_32: &'static str = "EnumPropPages32";
pub const REGSTR_VAL_ESDI: &'static str = "ESDI\\";
pub const REGSTR_VAL_EXISTS: &'static str = "Exists";
pub const REGSTR_VAL_EXTMEM: &'static str = "ExtMem";
pub const REGSTR_VAL_FAULT_LOGFILE: &'static str = "LogFile";
pub const REGSTR_VAL_FIFODEPTH: &'static str = "FIFODepth";
pub const REGSTR_VAL_FILESHARING: &'static str = "FileSharing";
pub const REGSTR_VAL_FIRSTINSTALLDATETIME: &'static str = "FirstInstallDateTime";
pub const REGSTR_VAL_FIRSTNETDRIVE: &'static str = "FirstNetworkDrive";
pub const REGSTR_VAL_FLOP: &'static str = "FLOP\\";
pub const REGSTR_VAL_FLOPPY: &'static str = "FLOPPY";
pub const REGSTR_VAL_FONTSIZE: &'static str = "FontSize";
pub const REGSTR_VAL_FORCECL: &'static str = "ForceChangeLine";
pub const REGSTR_VAL_FORCEDCONFIG: &'static str = "ForcedConfig";
pub const REGSTR_VAL_FORCEFIFO: &'static str = "ForceFIFO";
pub const REGSTR_VAL_FORCELOAD: &'static str = "ForceLoadPD";
pub const REGSTR_VAL_FORCEPMIO: &'static str = "ForcePMIO";
pub const REGSTR_VAL_FORCEREBOOT: &'static str = "ForceReboot";
pub const REGSTR_VAL_FORCERMIO: &'static str = "ForceRMIO";
pub const REGSTR_VAL_FREESPACERATIO: &'static str = "FreeSpaceRatio";
pub const REGSTR_VAL_FRIENDLYNAME: &'static str = "FriendlyName";
pub const REGSTR_VAL_FSFILTERCLASS: &'static str = "FSFilterClass";
pub const REGSTR_VAL_FULLTRACE: &'static str = "FullTrace";
pub const REGSTR_VAL_FUNCDESC: &'static str = "FunctionDesc";
pub const REGSTR_VAL_GAPTIME: &'static str = "GapTime";
pub const REGSTR_VAL_GRB: &'static str = "grb";
pub const REGSTR_VAL_HARDWAREID: &'static str = "HardwareID";
pub const REGSTR_VAL_HIDESHAREPWDS: &'static str = "HideSharePwds";
pub const REGSTR_VAL_HRES: &'static str = "HRes";
pub const REGSTR_VAL_HWDETECT: &'static str = "HardwareDetect";
pub const REGSTR_VAL_HWMECHANISM: &'static str = "HWMechanism";
pub const REGSTR_VAL_HWREV: &'static str = "HWRevision";
pub const REGSTR_VAL_ID: &'static str = "CurrentID";
pub const REGSTR_VAL_IDE_FORCE_SERIALIZE: &'static str = "ForceSerialization";
pub const REGSTR_VAL_IDE_NO_SERIALIZE: &'static str = "IDENoSerialize";
pub const REGSTR_VAL_INFNAME: &'static str = "InfName";
pub const REGSTR_VAL_INFPATH: &'static str = "InfPath";
pub const REGSTR_VAL_INFSECTION: &'static str = "InfSection";
pub const REGSTR_VAL_INFSECTIONEXT: &'static str = "InfSectionExt";
pub const REGSTR_VAL_INHIBITRESULTS: &'static str = "InhibitResults";
pub const REGSTR_VAL_INSICON: &'static str = "Icon";
pub const REGSTR_VAL_INSTALLER: &'static str = "Installer";
pub const REGSTR_VAL_INSTALLER_32: &'static str = "Installer32";
pub const REGSTR_VAL_INSTALLTYPE: &'static str = "InstallType";
pub const REGSTR_VAL_INT13: &'static str = "Int13";
pub const REGSTR_VAL_ISAPNP: &'static str = "ISAPNP";
pub const REGSTR_VAL_ISAPNP_RDP_OVERRIDE: &'static str = "RDPOverRide";
pub const REGSTR_VAL_JOYCALLOUT: &'static str = "JoystickCallout";
pub const REGSTR_VAL_JOYNCONFIG: &'static str = "Joystick%dConfiguration";
pub const REGSTR_VAL_JOYNOEMCALLOUT: &'static str = "Joystick%dOEMCallout";
pub const REGSTR_VAL_JOYNOEMNAME: &'static str = "Joystick%dOEMName";
pub const REGSTR_VAL_JOYOEMCAL1: &'static str = "OEMCal1";
pub const REGSTR_VAL_JOYOEMCAL10: &'static str = "OEMCal10";
pub const REGSTR_VAL_JOYOEMCAL11: &'static str = "OEMCal11";
pub const REGSTR_VAL_JOYOEMCAL12: &'static str = "OEMCal12";
pub const REGSTR_VAL_JOYOEMCAL2: &'static str = "OEMCal2";
pub const REGSTR_VAL_JOYOEMCAL3: &'static str = "OEMCal3";
pub const REGSTR_VAL_JOYOEMCAL4: &'static str = "OEMCal4";
pub const REGSTR_VAL_JOYOEMCAL5: &'static str = "OEMCal5";
pub const REGSTR_VAL_JOYOEMCAL6: &'static str = "OEMCal6";
pub const REGSTR_VAL_JOYOEMCAL7: &'static str = "OEMCal7";
pub const REGSTR_VAL_JOYOEMCAL8: &'static str = "OEMCal8";
pub const REGSTR_VAL_JOYOEMCAL9: &'static str = "OEMCal9";
pub const REGSTR_VAL_JOYOEMCALCAP: &'static str = "OEMCalCap";
pub const REGSTR_VAL_JOYOEMCALLOUT: &'static str = "OEMCallout";
pub const REGSTR_VAL_JOYOEMCALWINCAP: &'static str = "OEMCalWinCap";
pub const REGSTR_VAL_JOYOEMDATA: &'static str = "OEMData";
pub const REGSTR_VAL_JOYOEMNAME: &'static str = "OEMName";
pub const REGSTR_VAL_JOYOEMPOVLABEL: &'static str = "OEMPOVLabel";
pub const REGSTR_VAL_JOYOEMRLABEL: &'static str = "OEMRLabel";
pub const REGSTR_VAL_JOYOEMTESTBUTTONCAP: &'static str = "OEMTestButtonCap";
pub const REGSTR_VAL_JOYOEMTESTBUTTONDESC: &'static str = "OEMTestButtonDesc";
pub const REGSTR_VAL_JOYOEMTESTMOVECAP: &'static str = "OEMTestMoveCap";
pub const REGSTR_VAL_JOYOEMTESTMOVEDESC: &'static str = "OEMTestMoveDesc";
pub const REGSTR_VAL_JOYOEMTESTWINCAP: &'static str = "OEMTestWinCap";
pub const REGSTR_VAL_JOYOEMULABEL: &'static str = "OEMULabel";
pub const REGSTR_VAL_JOYOEMVLABEL: &'static str = "OEMVLabel";
pub const REGSTR_VAL_JOYOEMXYLABEL: &'static str = "OEMXYLabel";
pub const REGSTR_VAL_JOYOEMZLABEL: &'static str = "OEMZLabel";
pub const REGSTR_VAL_JOYUSERVALUES: &'static str = "JoystickUserValues";
pub const REGSTR_VAL_LASTALIVEBT: &'static str = "LastAliveBT";
pub const REGSTR_VAL_LASTALIVEINTERVAL: &'static str = "TimeStampInterval";
pub const REGSTR_VAL_LASTALIVEPMPOLICY: &'static str = "LastAlivePMPolicy";
pub const REGSTR_VAL_LASTALIVESTAMP: &'static str = "LastAliveStamp";
pub const REGSTR_VAL_LASTALIVESTAMPFORCED: &'static str = "LastAliveStampForced";
pub const REGSTR_VAL_LASTALIVESTAMPINTERVAL: &'static str = "LastAliveStampInterval";
pub const REGSTR_VAL_LASTALIVESTAMPPOLICYINTERVAL: &'static str = "LastAliveStampPolicyInterval";
pub const REGSTR_VAL_LASTALIVEUPTIME: &'static str = "LastAliveUptime";
pub const REGSTR_VAL_LASTBOOTPMDRVS: &'static str = "LastBootPMDrvs";
pub const REGSTR_VAL_LASTCOMPUTERNAME: &'static str = "LastComputerName";
pub const REGSTR_VAL_LASTPCIBUSNUM: &'static str = "LastPCIBusNum";
pub const REGSTR_VAL_LAST_UPDATE_TIME: &'static str = "LastUpdateTime";
pub const REGSTR_VAL_LEGALNOTICECAPTION: &'static str = "LegalNoticeCaption";
pub const REGSTR_VAL_LEGALNOTICETEXT: &'static str = "LegalNoticeText";
pub const REGSTR_VAL_LICENSINGINFO: &'static str = "LicensingInfo";
pub const REGSTR_VAL_LINKED: &'static str = "Linked";
pub const REGSTR_VAL_LOADHI: &'static str = "LoadHi";
pub const REGSTR_VAL_LOADRMDRIVERS: &'static str = "LoadRMDrivers";
pub const REGSTR_VAL_LOCATION_INFORMATION: &'static str = "LocationInformation";
pub const REGSTR_VAL_LOCATION_INFORMATION_OVERRIDE: &'static str = "LocationInformationOverride";
pub const REGSTR_VAL_LOWERFILTERS: &'static str = "LowerFilters";
pub const REGSTR_VAL_LOWER_FILTER_DEFAULT_LEVEL: &'static str = "LowerFilterDefaultLevel";
pub const REGSTR_VAL_LOWER_FILTER_LEVELS: &'static str = "LowerFilterLevels";
pub const REGSTR_VAL_MACHINETYPE: &'static str = "MachineType";
pub const REGSTR_VAL_MANUFACTURER: &'static str = "Manufacturer";
pub const REGSTR_VAL_MAP: &'static str = "Map";
pub const REGSTR_VAL_MATCHINGDEVID: &'static str = "MatchingDeviceId";
pub const REGSTR_VAL_MAXCONNECTIONS: &'static str = "MaxConnections";
pub const REGSTR_VAL_MAXLIP: &'static str = "MaxLIP";
pub const REGSTR_VAL_MAXRES: &'static str = "MaxResolution";
pub const REGSTR_VAL_MAXRETRY: &'static str = "MaxRetry";
pub const REGSTR_VAL_MAX_HCID_LEN: u32 = 1024u32;
pub const REGSTR_VAL_MEDIA: &'static str = "MediaPath";
pub const REGSTR_VAL_MFG: &'static str = "Mfg";
pub const REGSTR_VAL_MF_FLAGS: &'static str = "MFFlags";
pub const REGSTR_VAL_MINIPORT_STAT: &'static str = "MiniportStatus";
pub const REGSTR_VAL_MINPWDLEN: &'static str = "MinPwdLen";
pub const REGSTR_VAL_MINRETRY: &'static str = "MinRetry";
pub const REGSTR_VAL_MODE: &'static str = "Mode";
pub const REGSTR_VAL_MODEL: &'static str = "Model";
pub const REGSTR_VAL_MSDOSMODE: &'static str = "MSDOSMode";
pub const REGSTR_VAL_MSDOSMODEDISCARD: &'static str = "Discard";
pub const REGSTR_VAL_MUSTBEVALIDATED: &'static str = "MustBeValidated";
pub const REGSTR_VAL_NAMECACHECOUNT: &'static str = "NameCache";
pub const REGSTR_VAL_NAMENUMERICTAIL: &'static str = "NameNumericTail";
pub const REGSTR_VAL_NCP_BROWSEMASTER: &'static str = "BrowseMaster";
pub const REGSTR_VAL_NCP_USEPEERBROWSING: &'static str = "Use_PeerBrowsing";
pub const REGSTR_VAL_NCP_USESAP: &'static str = "Use_Sap";
pub const REGSTR_VAL_NDP: &'static str = "NDP";
pub const REGSTR_VAL_NETCARD: &'static str = "Netcard";
pub const REGSTR_VAL_NETCLEAN: &'static str = "NetClean";
pub const REGSTR_VAL_NETOSTYPE: &'static str = "NetOSType";
pub const REGSTR_VAL_NETSETUP_DISABLE: &'static str = "NoNetSetup";
pub const REGSTR_VAL_NETSETUP_NOCONFIGPAGE: &'static str = "NoNetSetupConfigPage";
pub const REGSTR_VAL_NETSETUP_NOIDPAGE: &'static str = "NoNetSetupIDPage";
pub const REGSTR_VAL_NETSETUP_NOSECURITYPAGE: &'static str = "NoNetSetupSecurityPage";
pub const REGSTR_VAL_NOCMOSORFDPT: &'static str = "NoCMOSorFDPT";
pub const REGSTR_VAL_NODISPLAYCLASS: &'static str = "NoDisplayClass";
pub const REGSTR_VAL_NOENTIRENETWORK: &'static str = "NoEntireNetwork";
pub const REGSTR_VAL_NOFILESHARING: &'static str = "NoFileSharing";
pub const REGSTR_VAL_NOFILESHARINGCTRL: &'static str = "NoFileSharingControl";
pub const REGSTR_VAL_NOIDE: &'static str = "NoIDE";
pub const REGSTR_VAL_NOINSTALLCLASS: &'static str = "NoInstallClass";
pub const REGSTR_VAL_NONSTANDARD_ATAPI: &'static str = "NonStandardATAPI";
pub const REGSTR_VAL_NOPRINTSHARING: &'static str = "NoPrintSharing";
pub const REGSTR_VAL_NOPRINTSHARINGCTRL: &'static str = "NoPrintSharingControl";
pub const REGSTR_VAL_NOUSECLASS: &'static str = "NoUseClass";
pub const REGSTR_VAL_NOWORKGROUPCONTENTS: &'static str = "NoWorkgroupContents";
pub const REGSTR_VAL_OLDMSDOSVER: &'static str = "OldMSDOSVer";
pub const REGSTR_VAL_OLDWINDIR: &'static str = "OldWinDir";
pub const REGSTR_VAL_OPTIMIZESFN: &'static str = "OptimizeSFN";
pub const REGSTR_VAL_OPTIONS: &'static str = "Options";
pub const REGSTR_VAL_OPTORDER: &'static str = "Order";
pub const REGSTR_VAL_P1284MDL: &'static str = "Model";
pub const REGSTR_VAL_P1284MFG: &'static str = "Manufacturer";
pub const REGSTR_VAL_PATHCACHECOUNT: &'static str = "PathCache";
pub const REGSTR_VAL_PCCARD_POWER: &'static str = "EnablePowerManagement";
pub const REGSTR_VAL_PCI: &'static str = "PCI";
pub const REGSTR_VAL_PCIBIOSVER: &'static str = "PCIBIOSVer";
pub const REGSTR_VAL_PCICIRQMAP: &'static str = "PCICIRQMap";
pub const REGSTR_VAL_PCICOPTIONS: &'static str = "PCICOptions";
pub const REGSTR_VAL_PCMCIA_ALLOC: &'static str = "AllocMemWin";
pub const REGSTR_VAL_PCMCIA_ATAD: &'static str = "ATADelay";
pub const REGSTR_VAL_PCMCIA_MEM: &'static str = "Memory";
pub const REGSTR_VAL_PCMCIA_OPT: &'static str = "Options";
pub const REGSTR_VAL_PCMCIA_SIZ: &'static str = "MinRegionSize";
pub const REGSTR_VAL_PCMTDRIVER: &'static str = "MTD";
pub const REGSTR_VAL_PCSSDRIVER: &'static str = "Driver";
pub const REGSTR_VAL_PHYSICALDEVICEOBJECT: &'static str = "PhysicalDeviceObject";
pub const REGSTR_VAL_PMODE_INT13: &'static str = "PModeInt13";
pub const REGSTR_VAL_PNPBIOSVER: &'static str = "PnPBIOSVer";
pub const REGSTR_VAL_PNPSTRUCOFFSET: &'static str = "PnPStrucOffset";
pub const REGSTR_VAL_POLICY: &'static str = "Policy";
pub const REGSTR_VAL_POLLING: &'static str = "Polling";
pub const REGSTR_VAL_PORTNAME: &'static str = "PortName";
pub const REGSTR_VAL_PORTSUBCLASS: &'static str = "PortSubClass";
pub const REGSTR_VAL_PREFREDIR: &'static str = "PreferredRedir";
pub const REGSTR_VAL_PRESERVECASE: &'static str = "PreserveCase";
pub const REGSTR_VAL_PRESERVELONGNAMES: &'static str = "PreserveLongNames";
pub const REGSTR_VAL_PRINTERS_HIDETABS: &'static str = "NoPrinterTabs";
pub const REGSTR_VAL_PRINTERS_MASK: &'static str = "PrintersMask";
pub const REGSTR_VAL_PRINTERS_NOADD: &'static str = "NoAddPrinter";
pub const REGSTR_VAL_PRINTERS_NODELETE: &'static str = "NoDeletePrinter";
pub const REGSTR_VAL_PRINTSHARING: &'static str = "PrintSharing";
pub const REGSTR_VAL_PRIORITY: &'static str = "Priority";
pub const REGSTR_VAL_PRIVATE: &'static str = "Private";
pub const REGSTR_VAL_PRIVATEFUNC: &'static str = "PrivateFunc";
pub const REGSTR_VAL_PRIVATEPROBLEM: &'static str = "PrivateProblem";
pub const REGSTR_VAL_PRODUCTID: &'static str = "ProductId";
pub const REGSTR_VAL_PRODUCTTYPE: &'static str = "ProductType";
pub const REGSTR_VAL_PROFILEFLAGS: &'static str = "ProfileFlags";
pub const REGSTR_VAL_PROPERTIES: &'static str = "Properties";
pub const REGSTR_VAL_PROTINIPATH: &'static str = "ProtIniPath";
pub const REGSTR_VAL_PROVIDER_NAME: &'static str = "ProviderName";
pub const REGSTR_VAL_PWDEXPIRATION: &'static str = "PwdExpiration";
pub const REGSTR_VAL_PWDPROVIDER_CHANGEORDER: &'static str = "ChangeOrder";
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWD: &'static str = "ChangePassword";
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWDHWND: &'static str = "ChangePasswordHwnd";
pub const REGSTR_VAL_PWDPROVIDER_DESC: &'static str = "Description";
pub const REGSTR_VAL_PWDPROVIDER_GETPWDSTATUS: &'static str = "GetPasswordStatus";
pub const REGSTR_VAL_PWDPROVIDER_ISNP: &'static str = "NetworkProvider";
pub const REGSTR_VAL_PWDPROVIDER_PATH: &'static str = "ProviderPath";
pub const REGSTR_VAL_RDINTTHRESHOLD: &'static str = "RDIntThreshold";
pub const REGSTR_VAL_READAHEADTHRESHOLD: &'static str = "ReadAheadThreshold";
pub const REGSTR_VAL_READCACHING: &'static str = "ReadCaching";
pub const REGSTR_VAL_REALNETSTART: &'static str = "RealNetStart";
pub const REGSTR_VAL_REASONCODE: &'static str = "ReasonCode";
pub const REGSTR_VAL_REFRESHRATE: &'static str = "RefreshRate";
pub const REGSTR_VAL_REGITEMDELETEMESSAGE: &'static str = "Removal Message";
pub const REGSTR_VAL_REGORGANIZATION: &'static str = "RegisteredOrganization";
pub const REGSTR_VAL_REGOWNER: &'static str = "RegisteredOwner";
pub const REGSTR_VAL_REINSTALL_DEVICEINSTANCEIDS: &'static str = "DeviceInstanceIds";
pub const REGSTR_VAL_REINSTALL_DISPLAYNAME: &'static str = "DisplayName";
pub const REGSTR_VAL_REINSTALL_STRING: &'static str = "ReinstallString";
pub const REGSTR_VAL_REMOTE_PATH: &'static str = "RemotePath";
pub const REGSTR_VAL_REMOVABLE: &'static str = "Removable";
pub const REGSTR_VAL_REMOVAL_POLICY: &'static str = "RemovalPolicy";
pub const REGSTR_VAL_REMOVEROMOKAY: &'static str = "RemoveRomOkay";
pub const REGSTR_VAL_REMOVEROMOKAYFUNC: &'static str = "RemoveRomOkayFunc";
pub const REGSTR_VAL_RESERVED_DEVNODE: &'static str = "HTREE\\RESERVED\\0";
pub const REGSTR_VAL_RESOLUTION: &'static str = "Resolution";
pub const REGSTR_VAL_RESOURCES: &'static str = "Resources";
pub const REGSTR_VAL_RESOURCE_MAP: &'static str = "ResourceMap";
pub const REGSTR_VAL_RESOURCE_PICKER_EXCEPTIONS: &'static str = "ResourcePickerExceptions";
pub const REGSTR_VAL_RESOURCE_PICKER_TAGS: &'static str = "ResourcePickerTags";
pub const REGSTR_VAL_RESTRICTRUN: &'static str = "RestrictRun";
pub const REGSTR_VAL_RESUMERESET: &'static str = "ResumeReset";
pub const REGSTR_VAL_REVISION: &'static str = "Revision";
pub const REGSTR_VAL_REVLEVEL: &'static str = "RevisionLevel";
pub const REGSTR_VAL_ROOT_DEVNODE: &'static str = "HTREE\\ROOT\\0";
pub const REGSTR_VAL_RUNLOGINSCRIPT: &'static str = "ProcessLoginScript";
pub const REGSTR_VAL_SCANNER: &'static str = "SCANNER";
pub const REGSTR_VAL_SCAN_ONLY_FIRST: &'static str = "ScanOnlyFirstDrive";
pub const REGSTR_VAL_SCSI: &'static str = "SCSI\\";
pub const REGSTR_VAL_SCSILUN: &'static str = "SCSILUN";
pub const REGSTR_VAL_SCSITID: &'static str = "SCSITargetID";
pub const REGSTR_VAL_SEARCHMODE: &'static str = "SearchMode";
pub const REGSTR_VAL_SEARCHOPTIONS: &'static str = "SearchOptions";
pub const REGSTR_VAL_SECCPL_NOADMINPAGE: &'static str = "NoAdminPage";
pub const REGSTR_VAL_SECCPL_NOPROFILEPAGE: &'static str = "NoProfilePage";
pub const REGSTR_VAL_SECCPL_NOPWDPAGE: &'static str = "NoPwdPage";
pub const REGSTR_VAL_SECCPL_NOSECCPL: &'static str = "NoSecCPL";
pub const REGSTR_VAL_SERVICE: &'static str = "Service";
pub const REGSTR_VAL_SETUPFLAGS: &'static str = "SetupFlags";
pub const REGSTR_VAL_SETUPMACHINETYPE: &'static str = "SetupMachineType";
pub const REGSTR_VAL_SETUPN: &'static str = "SetupN";
pub const REGSTR_VAL_SETUPNPATH: &'static str = "SetupNPath";
pub const REGSTR_VAL_SETUPPROGRAMRAN: &'static str = "SetupProgramRan";
pub const REGSTR_VAL_SHARES_FLAGS: &'static str = "Flags";
pub const REGSTR_VAL_SHARES_PATH: &'static str = "Path";
pub const REGSTR_VAL_SHARES_REMARK: &'static str = "Remark";
pub const REGSTR_VAL_SHARES_RO_PASS: &'static str = "Parm2";
pub const REGSTR_VAL_SHARES_RW_PASS: &'static str = "Parm1";
pub const REGSTR_VAL_SHARES_TYPE: &'static str = "Type";
pub const REGSTR_VAL_SHARE_IRQ: &'static str = "ForceIRQSharing";
pub const REGSTR_VAL_SHELLVERSION: &'static str = "ShellVersion";
pub const REGSTR_VAL_SHOWDOTS: &'static str = "ShowDots";
pub const REGSTR_VAL_SHOWREASONUI: &'static str = "ShutdownReasonUI";
pub const REGSTR_VAL_SHUTDOWNREASON: &'static str = "ShutdownReason";
pub const REGSTR_VAL_SHUTDOWNREASON_CODE: &'static str = "ShutdownReasonCode";
pub const REGSTR_VAL_SHUTDOWNREASON_COMMENT: &'static str = "ShutdownReasonComment";
pub const REGSTR_VAL_SHUTDOWNREASON_PROCESS: &'static str = "ShutdownReasonProcess";
pub const REGSTR_VAL_SHUTDOWNREASON_USERNAME: &'static str = "ShutdownReasonUserName";
pub const REGSTR_VAL_SHUTDOWN_FLAGS: &'static str = "ShutdownFlags";
pub const REGSTR_VAL_SHUTDOWN_IGNORE_PREDEFINED: &'static str = "ShutdownIgnorePredefinedReasons";
pub const REGSTR_VAL_SHUTDOWN_STATE_SNAPSHOT: &'static str = "ShutdownStateSnapshot";
pub const REGSTR_VAL_SILENTINSTALL: &'static str = "SilentInstall";
pub const REGSTR_VAL_SLSUPPORT: &'static str = "SLSupport";
pub const REGSTR_VAL_SOFTCOMPATMODE: &'static str = "SoftCompatMode";
pub const REGSTR_VAL_SRCPATH: &'static str = "SourcePath";
pub const REGSTR_VAL_SRVNAMECACHE: &'static str = "ServerNameCache";
pub const REGSTR_VAL_SRVNAMECACHECOUNT: &'static str = "ServerNameCacheMax";
pub const REGSTR_VAL_SRVNAMECACHENETPROV: &'static str = "ServerNameCacheNumNets";
pub const REGSTR_VAL_START_ON_BOOT: &'static str = "StartOnBoot";
pub const REGSTR_VAL_STAT: &'static str = "Status";
pub const REGSTR_VAL_STATICDRIVE: &'static str = "StaticDrive";
pub const REGSTR_VAL_STATICVXD: &'static str = "StaticVxD";
pub const REGSTR_VAL_STDDOSOPTION: &'static str = "StdOption";
pub const REGSTR_VAL_SUBMODEL: &'static str = "Submodel";
pub const REGSTR_VAL_SUPPORTBURST: &'static str = "SupportBurst";
pub const REGSTR_VAL_SUPPORTLFN: &'static str = "SupportLFN";
pub const REGSTR_VAL_SUPPORTTUNNELLING: &'static str = "SupportTunnelling";
pub const REGSTR_VAL_SYMBOLIC_LINK: &'static str = "SymbolicLink";
pub const REGSTR_VAL_SYNCDATAXFER: &'static str = "SyncDataXfer";
pub const REGSTR_VAL_SYSDM: &'static str = "SysDM";
pub const REGSTR_VAL_SYSDMFUNC: &'static str = "SysDMFunc";
pub const REGSTR_VAL_SYSTEMCPL_NOCONFIGPAGE: &'static str = "NoConfigPage";
pub const REGSTR_VAL_SYSTEMCPL_NODEVMGRPAGE: &'static str = "NoDevMgrPage";
pub const REGSTR_VAL_SYSTEMCPL_NOFILESYSPAGE: &'static str = "NoFileSysPage";
pub const REGSTR_VAL_SYSTEMCPL_NOVIRTMEMPAGE: &'static str = "NoVirtMemPage";
pub const REGSTR_VAL_SYSTEMROOT: &'static str = "SystemRoot";
pub const REGSTR_VAL_SYSTRAYBATFLAGS: &'static str = "PowerFlags";
pub const REGSTR_VAL_SYSTRAYPCCARDFLAGS: &'static str = "PCMCIAFlags";
pub const REGSTR_VAL_SYSTRAYSVCS: &'static str = "Services";
pub const REGSTR_VAL_TABLE_STAT: &'static str = "TableStatus";
pub const REGSTR_VAL_TAPE: &'static str = "TAPE";
pub const REGSTR_VAL_TRANSITION: &'static str = "Transition";
pub const REGSTR_VAL_TRANSPORT: &'static str = "Transport";
pub const REGSTR_VAL_TZACTBIAS: &'static str = "ActiveTimeBias";
pub const REGSTR_VAL_TZBIAS: &'static str = "Bias";
pub const REGSTR_VAL_TZDLTBIAS: &'static str = "DaylightBias";
pub const REGSTR_VAL_TZDLTFLAG: &'static str = "DaylightFlag";
pub const REGSTR_VAL_TZDLTNAME: &'static str = "DaylightName";
pub const REGSTR_VAL_TZDLTSTART: &'static str = "DaylightStart";
pub const REGSTR_VAL_TZNOAUTOTIME: &'static str = "DisableAutoDaylightTimeSet";
pub const REGSTR_VAL_TZNOCHANGEEND: &'static str = "NoChangeEnd";
pub const REGSTR_VAL_TZNOCHANGESTART: &'static str = "NoChangeStart";
pub const REGSTR_VAL_TZSTDBIAS: &'static str = "StandardBias";
pub const REGSTR_VAL_TZSTDNAME: &'static str = "StandardName";
pub const REGSTR_VAL_TZSTDSTART: &'static str = "StandardStart";
pub const REGSTR_VAL_UI_NUMBER: &'static str = "UINumber";
pub const REGSTR_VAL_UI_NUMBER_DESC_FORMAT: &'static str = "UINumberDescFormat";
pub const REGSTR_VAL_UNDOCK_WITHOUT_LOGON: &'static str = "UndockWithoutLogon";
pub const REGSTR_VAL_UNINSTALLER_COMMANDLINE: &'static str = "UninstallString";
pub const REGSTR_VAL_UNINSTALLER_DISPLAYNAME: &'static str = "DisplayName";
pub const REGSTR_VAL_UPGRADE: &'static str = "Upgrade";
pub const REGSTR_VAL_UPPERFILTERS: &'static str = "UpperFilters";
pub const REGSTR_VAL_UPPER_FILTER_DEFAULT_LEVEL: &'static str = "UpperFilterDefaultLevel";
pub const REGSTR_VAL_UPPER_FILTER_LEVELS: &'static str = "UpperFilterLevels";
pub const REGSTR_VAL_USERSETTINGS: &'static str = "AdapterSettings";
pub const REGSTR_VAL_USER_NAME: &'static str = "UserName";
pub const REGSTR_VAL_USRDRVLET: &'static str = "UserDriveLetterAssignment";
pub const REGSTR_VAL_VDD: &'static str = "vdd";
pub const REGSTR_VAL_VER: &'static str = "Ver";
pub const REGSTR_VAL_VERIFYKEY: &'static str = "VerifyKey";
pub const REGSTR_VAL_VIRTUALHDIRQ: &'static str = "VirtualHDIRQ";
pub const REGSTR_VAL_VOLIDLETIMEOUT: &'static str = "VolumeIdleTimeout";
pub const REGSTR_VAL_VPOWERDFLAGS: &'static str = "Flags";
pub const REGSTR_VAL_VRES: &'static str = "VRes";
pub const REGSTR_VAL_VXDGROUPS: &'static str = "VXDGroups";
pub const REGSTR_VAL_WAITFORUNDOCK: &'static str = "WaitForUndock";
pub const REGSTR_VAL_WAITFORUNDOCKFUNC: &'static str = "WaitForUndockFunc";
pub const REGSTR_VAL_WIN31FILESYSTEM: &'static str = "Win31FileSystem";
pub const REGSTR_VAL_WIN31PROVIDER: &'static str = "Win31Provider";
pub const REGSTR_VAL_WINBOOTDIR: &'static str = "WinbootDir";
pub const REGSTR_VAL_WINCP: &'static str = "ACP";
pub const REGSTR_VAL_WINDIR: &'static str = "WinDir";
pub const REGSTR_VAL_WINOLDAPP_DISABLED: &'static str = "Disabled";
pub const REGSTR_VAL_WINOLDAPP_NOREALMODE: &'static str = "NoRealMode";
pub const REGSTR_VAL_WORKGROUP: &'static str = "Workgroup";
pub const REGSTR_VAL_WRAPPER: &'static str = "Wrapper";
pub const REGSTR_VAL_WRINTTHRESHOLD: &'static str = "WRIntThreshold";
pub const REGSTR_VAL_WRKGRP_FORCEMAPPING: &'static str = "WrkgrpForceMapping";
pub const REGSTR_VAL_WRKGRP_REQUIRED: &'static str = "WrkgrpRequired";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REG_CREATE_KEY_DISPOSITION(pub u32);
pub const REG_CREATED_NEW_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(1u32);
pub const REG_OPENED_EXISTING_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(2u32);
impl ::core::marker::Copy for REG_CREATE_KEY_DISPOSITION {}
impl ::core::clone::Clone for REG_CREATE_KEY_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_CREATE_KEY_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REG_CREATE_KEY_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_CREATE_KEY_DISPOSITION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for REG_CREATE_KEY_DISPOSITION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const REG_KEY_INSTDEV: &'static str = "Installed";
pub const REG_MUI_STRING_TRUNCATE: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REG_NOTIFY_FILTER(pub u32);
pub const REG_NOTIFY_CHANGE_NAME: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(1u32);
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(2u32);
pub const REG_NOTIFY_CHANGE_LAST_SET: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(4u32);
pub const REG_NOTIFY_CHANGE_SECURITY: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(8u32);
pub const REG_NOTIFY_THREAD_AGNOSTIC: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(268435456u32);
impl ::core::marker::Copy for REG_NOTIFY_FILTER {}
impl ::core::clone::Clone for REG_NOTIFY_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_NOTIFY_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REG_NOTIFY_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_NOTIFY_FILTER").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_NOTIFY_FILTER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_NOTIFY_FILTER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_NOTIFY_FILTER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for REG_NOTIFY_FILTER {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REG_OPEN_CREATE_OPTIONS(pub u32);
pub const REG_OPTION_RESERVED: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
pub const REG_OPTION_NON_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
pub const REG_OPTION_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(1u32);
pub const REG_OPTION_CREATE_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(2u32);
pub const REG_OPTION_BACKUP_RESTORE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(4u32);
pub const REG_OPTION_OPEN_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(8u32);
pub const REG_OPTION_DONT_VIRTUALIZE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(16u32);
impl ::core::marker::Copy for REG_OPEN_CREATE_OPTIONS {}
impl ::core::clone::Clone for REG_OPEN_CREATE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_OPEN_CREATE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REG_OPEN_CREATE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_OPEN_CREATE_OPTIONS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for REG_OPEN_CREATE_OPTIONS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const REG_PROCESS_APPKEY: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REG_RESTORE_KEY_FLAGS(pub i32);
pub const REG_FORCE_RESTORE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(8i32);
pub const REG_WHOLE_HIVE_VOLATILE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(1i32);
impl ::core::marker::Copy for REG_RESTORE_KEY_FLAGS {}
impl ::core::clone::Clone for REG_RESTORE_KEY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_RESTORE_KEY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REG_RESTORE_KEY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_RESTORE_KEY_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for REG_RESTORE_KEY_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REG_SAM_FLAGS(pub u32);
pub const KEY_QUERY_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(1u32);
pub const KEY_SET_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(2u32);
pub const KEY_CREATE_SUB_KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(4u32);
pub const KEY_ENUMERATE_SUB_KEYS: REG_SAM_FLAGS = REG_SAM_FLAGS(8u32);
pub const KEY_NOTIFY: REG_SAM_FLAGS = REG_SAM_FLAGS(16u32);
pub const KEY_CREATE_LINK: REG_SAM_FLAGS = REG_SAM_FLAGS(32u32);
pub const KEY_WOW64_32KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(512u32);
pub const KEY_WOW64_64KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(256u32);
pub const KEY_WOW64_RES: REG_SAM_FLAGS = REG_SAM_FLAGS(768u32);
pub const KEY_READ: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
pub const KEY_WRITE: REG_SAM_FLAGS = REG_SAM_FLAGS(131078u32);
pub const KEY_EXECUTE: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
pub const KEY_ALL_ACCESS: REG_SAM_FLAGS = REG_SAM_FLAGS(983103u32);
impl ::core::marker::Copy for REG_SAM_FLAGS {}
impl ::core::clone::Clone for REG_SAM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_SAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REG_SAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_SAM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REG_SAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_SAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_SAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_SAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_SAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for REG_SAM_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REG_SAVE_FORMAT(pub u32);
pub const REG_STANDARD_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(1u32);
pub const REG_LATEST_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(2u32);
pub const REG_NO_COMPRESSION: REG_SAVE_FORMAT = REG_SAVE_FORMAT(4u32);
impl ::core::marker::Copy for REG_SAVE_FORMAT {}
impl ::core::clone::Clone for REG_SAVE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_SAVE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REG_SAVE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_SAVE_FORMAT").field(&self.0).finish()
    }
}
impl FromIntoMemory for REG_SAVE_FORMAT {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const REG_SECURE_CONNECTION: u32 = 1u32;
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REG_VALUE_TYPE(pub u32);
pub const REG_NONE: REG_VALUE_TYPE = REG_VALUE_TYPE(0u32);
pub const REG_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(1u32);
pub const REG_EXPAND_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(2u32);
pub const REG_BINARY: REG_VALUE_TYPE = REG_VALUE_TYPE(3u32);
pub const REG_DWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
pub const REG_DWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
pub const REG_DWORD_BIG_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(5u32);
pub const REG_LINK: REG_VALUE_TYPE = REG_VALUE_TYPE(6u32);
pub const REG_MULTI_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(7u32);
pub const REG_RESOURCE_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(8u32);
pub const REG_FULL_RESOURCE_DESCRIPTOR: REG_VALUE_TYPE = REG_VALUE_TYPE(9u32);
pub const REG_RESOURCE_REQUIREMENTS_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(10u32);
pub const REG_QWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
pub const REG_QWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
impl ::core::marker::Copy for REG_VALUE_TYPE {}
impl ::core::clone::Clone for REG_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REG_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_VALUE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for REG_VALUE_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const RRF_NOEXPAND: u32 = 268435456u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RRF_RT(pub u32);
pub const RRF_RT_ANY: RRF_RT = RRF_RT(65535u32);
pub const RRF_RT_DWORD: RRF_RT = RRF_RT(24u32);
pub const RRF_RT_QWORD: RRF_RT = RRF_RT(72u32);
pub const RRF_RT_REG_BINARY: RRF_RT = RRF_RT(8u32);
pub const RRF_RT_REG_DWORD: RRF_RT = RRF_RT(16u32);
pub const RRF_RT_REG_EXPAND_SZ: RRF_RT = RRF_RT(4u32);
pub const RRF_RT_REG_MULTI_SZ: RRF_RT = RRF_RT(32u32);
pub const RRF_RT_REG_NONE: RRF_RT = RRF_RT(1u32);
pub const RRF_RT_REG_QWORD: RRF_RT = RRF_RT(64u32);
pub const RRF_RT_REG_SZ: RRF_RT = RRF_RT(2u32);
impl ::core::marker::Copy for RRF_RT {}
impl ::core::clone::Clone for RRF_RT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RRF_RT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RRF_RT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RRF_RT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RRF_RT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RRF_RT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RRF_RT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RRF_RT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RRF_RT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for RRF_RT {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const RRF_SUBKEY_WOW6432KEY: u32 = 131072u32;
pub const RRF_SUBKEY_WOW6464KEY: u32 = 65536u32;
pub const RRF_WOW64_MASK: u32 = 196608u32;
pub const RRF_ZEROONFAILURE: u32 = 536870912u32;
pub const SUF_BATCHINF: i32 = 4i32;
pub const SUF_CLEAN: i32 = 8i32;
pub const SUF_EXPRESS: i32 = 2i32;
pub const SUF_FIRSTTIME: i32 = 1i32;
pub const SUF_INSETUP: i32 = 16i32;
pub const SUF_NETHDBOOT: i32 = 64i32;
pub const SUF_NETRPLBOOT: i32 = 128i32;
pub const SUF_NETSETUP: i32 = 32i32;
pub const SUF_SBSCOPYOK: i32 = 256i32;
pub struct VALENTA {
    pub ve_valuename: PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: PtrRepr,
    pub ve_type: REG_VALUE_TYPE,
}
impl ::core::marker::Copy for VALENTA {}
impl ::core::clone::Clone for VALENTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VALENTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VALENTA")
            .field("ve_valuename", &self.ve_valuename)
            .field("ve_valuelen", &self.ve_valuelen)
            .field("ve_valueptr", &self.ve_valueptr)
            .field("ve_type", &self.ve_type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VALENTA {
    fn eq(&self, other: &Self) -> bool {
        self.ve_valuename == other.ve_valuename
            && self.ve_valuelen == other.ve_valuelen
            && self.ve_valueptr == other.ve_valueptr
            && self.ve_type == other.ve_type
    }
}
impl ::core::cmp::Eq for VALENTA {}
impl FromIntoMemory for VALENTA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_ve_valuename = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ve_valuelen = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ve_valueptr = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ve_type = <REG_VALUE_TYPE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            ve_valuename: f_ve_valuename,
            ve_valuelen: f_ve_valuelen,
            ve_valueptr: f_ve_valueptr,
            ve_type: f_ve_type,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.ve_valuename, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ve_valuelen, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ve_valueptr, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ve_type, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct VALENTW {
    pub ve_valuename: PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: PtrRepr,
    pub ve_type: REG_VALUE_TYPE,
}
impl ::core::marker::Copy for VALENTW {}
impl ::core::clone::Clone for VALENTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VALENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VALENTW")
            .field("ve_valuename", &self.ve_valuename)
            .field("ve_valuelen", &self.ve_valuelen)
            .field("ve_valueptr", &self.ve_valueptr)
            .field("ve_type", &self.ve_type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VALENTW {
    fn eq(&self, other: &Self) -> bool {
        self.ve_valuename == other.ve_valuename
            && self.ve_valuelen == other.ve_valuelen
            && self.ve_valueptr == other.ve_valueptr
            && self.ve_type == other.ve_type
    }
}
impl ::core::cmp::Eq for VALENTW {}
impl FromIntoMemory for VALENTW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_ve_valuename = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ve_valuelen = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ve_valueptr = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ve_type = <REG_VALUE_TYPE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            ve_valuename: f_ve_valuename,
            ve_valuelen: f_ve_valuelen,
            ve_valueptr: f_ve_valueptr,
            ve_type: f_ve_type,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.ve_valuename, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ve_valuelen, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ve_valueptr, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ve_type, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const VPDF_DISABLEPWRMGMT: u32 = 1u32;
pub const VPDF_DISABLEPWRSTATUSPOLL: u32 = 8u32;
pub const VPDF_DISABLERINGRESUME: u32 = 16u32;
pub const VPDF_FORCEAPM10MODE: u32 = 2u32;
pub const VPDF_SHOWMULTIBATT: u32 = 32u32;
pub const VPDF_SKIPINTELSLCHECK: u32 = 4u32;
pub struct provider_info {
    pub pi_R0_1val: PQUERYHANDLER,
    pub pi_R0_allvals: PQUERYHANDLER,
    pub pi_R3_1val: PQUERYHANDLER,
    pub pi_R3_allvals: PQUERYHANDLER,
    pub pi_flags: u32,
    pub pi_key_context: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for provider_info {}
impl ::core::clone::Clone for provider_info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for provider_info {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("provider_info")
            .field("pi_R0_1val", &self.pi_R0_1val)
            .field("pi_R0_allvals", &self.pi_R0_allvals)
            .field("pi_R3_1val", &self.pi_R3_1val)
            .field("pi_R3_allvals", &self.pi_R3_allvals)
            .field("pi_flags", &self.pi_flags)
            .field("pi_key_context", &self.pi_key_context)
            .finish()
    }
}
impl ::core::cmp::PartialEq for provider_info {
    fn eq(&self, other: &Self) -> bool {
        self.pi_R0_1val == other.pi_R0_1val
            && self.pi_R0_allvals == other.pi_R0_allvals
            && self.pi_R3_1val == other.pi_R3_1val
            && self.pi_R3_allvals == other.pi_R3_allvals
            && self.pi_flags == other.pi_flags
            && self.pi_key_context == other.pi_key_context
    }
}
impl ::core::cmp::Eq for provider_info {}
impl FromIntoMemory for provider_info {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_pi_R0_1val = <PQUERYHANDLER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pi_R0_allvals = <PQUERYHANDLER as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pi_R3_1val = <PQUERYHANDLER as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pi_R3_allvals = <PQUERYHANDLER as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_pi_flags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_pi_key_context =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            pi_R0_1val: f_pi_R0_1val,
            pi_R0_allvals: f_pi_R0_allvals,
            pi_R3_1val: f_pi_R3_1val,
            pi_R3_allvals: f_pi_R3_allvals,
            pi_flags: f_pi_flags,
            pi_key_context: f_pi_key_context,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.pi_R0_1val, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pi_R0_allvals, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pi_R3_1val, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pi_R3_allvals, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.pi_flags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.pi_key_context, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct pvalueA {
    pub pv_valuename: PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: MutPtr<::core::ffi::c_void>,
    pub pv_type: u32,
}
impl ::core::marker::Copy for pvalueA {}
impl ::core::clone::Clone for pvalueA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pvalueA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pvalueA")
            .field("pv_valuename", &self.pv_valuename)
            .field("pv_valuelen", &self.pv_valuelen)
            .field("pv_value_context", &self.pv_value_context)
            .field("pv_type", &self.pv_type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for pvalueA {
    fn eq(&self, other: &Self) -> bool {
        self.pv_valuename == other.pv_valuename
            && self.pv_valuelen == other.pv_valuelen
            && self.pv_value_context == other.pv_value_context
            && self.pv_type == other.pv_type
    }
}
impl ::core::cmp::Eq for pvalueA {}
impl FromIntoMemory for pvalueA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_pv_valuename = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pv_valuelen = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pv_value_context =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pv_type = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            pv_valuename: f_pv_valuename,
            pv_valuelen: f_pv_valuelen,
            pv_value_context: f_pv_value_context,
            pv_type: f_pv_type,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.pv_valuename, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pv_valuelen, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pv_value_context, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pv_type, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct pvalueW {
    pub pv_valuename: PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: MutPtr<::core::ffi::c_void>,
    pub pv_type: u32,
}
impl ::core::marker::Copy for pvalueW {}
impl ::core::clone::Clone for pvalueW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pvalueW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pvalueW")
            .field("pv_valuename", &self.pv_valuename)
            .field("pv_valuelen", &self.pv_valuelen)
            .field("pv_value_context", &self.pv_value_context)
            .field("pv_type", &self.pv_type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for pvalueW {
    fn eq(&self, other: &Self) -> bool {
        self.pv_valuename == other.pv_valuename
            && self.pv_valuelen == other.pv_valuelen
            && self.pv_value_context == other.pv_value_context
            && self.pv_type == other.pv_type
    }
}
impl ::core::cmp::Eq for pvalueW {}
impl FromIntoMemory for pvalueW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_pv_valuename = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_pv_valuelen = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_pv_value_context =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_pv_type = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            pv_valuename: f_pv_valuename,
            pv_valuelen: f_pv_valuelen,
            pv_value_context: f_pv_value_context,
            pv_type: f_pv_type,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.pv_valuename, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.pv_valuelen, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.pv_value_context, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.pv_type, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct val_context {
    pub valuelen: i32,
    pub value_context: MutPtr<::core::ffi::c_void>,
    pub val_buff_ptr: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for val_context {}
impl ::core::clone::Clone for val_context {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for val_context {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("val_context")
            .field("valuelen", &self.valuelen)
            .field("value_context", &self.value_context)
            .field("val_buff_ptr", &self.val_buff_ptr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for val_context {
    fn eq(&self, other: &Self) -> bool {
        self.valuelen == other.valuelen
            && self.value_context == other.value_context
            && self.val_buff_ptr == other.val_buff_ptr
    }
}
impl ::core::cmp::Eq for val_context {}
impl FromIntoMemory for val_context {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_valuelen = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_value_context =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_val_buff_ptr =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            valuelen: f_valuelen,
            value_context: f_value_context,
            val_buff_ptr: f_val_buff_ptr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.valuelen, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.value_context, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.val_buff_ptr, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub trait Api {
    #[doc = "RegCloseKey from ADVAPI32"]
    fn RegCloseKey(&self, h_key: HKEY) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCloseKey")
    }
    #[doc = "RegConnectRegistryA from ADVAPI32"]
    fn RegConnectRegistryA(
        &self,
        lp_machine_name: PCSTR,
        h_key: HKEY,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegConnectRegistryA")
    }
    #[doc = "RegConnectRegistryExA from ADVAPI32"]
    fn RegConnectRegistryExA(
        &self,
        lp_machine_name: PCSTR,
        h_key: HKEY,
        flags: u32,
        phk_result: MutPtr<HKEY>,
    ) -> i32 {
        todo!("RegConnectRegistryExA")
    }
    #[doc = "RegConnectRegistryExW from ADVAPI32"]
    fn RegConnectRegistryExW(
        &self,
        lp_machine_name: PCWSTR,
        h_key: HKEY,
        flags: u32,
        phk_result: MutPtr<HKEY>,
    ) -> i32 {
        todo!("RegConnectRegistryExW")
    }
    #[doc = "RegConnectRegistryW from ADVAPI32"]
    fn RegConnectRegistryW(
        &self,
        lp_machine_name: PCWSTR,
        h_key: HKEY,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegConnectRegistryW")
    }
    #[doc = "RegCopyTreeA from ADVAPI32"]
    fn RegCopyTreeA(
        &self,
        h_key_src: HKEY,
        lp_sub_key: PCSTR,
        h_key_dest: HKEY,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCopyTreeA")
    }
    #[doc = "RegCopyTreeW from ADVAPI32"]
    fn RegCopyTreeW(
        &self,
        h_key_src: HKEY,
        lp_sub_key: PCWSTR,
        h_key_dest: HKEY,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCopyTreeW")
    }
    #[doc = "RegCreateKeyA from ADVAPI32"]
    fn RegCreateKeyA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCreateKeyA")
    }
    #[doc = "RegCreateKeyExA from ADVAPI32"]
    fn RegCreateKeyExA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        reserved: u32,
        lp_class: PCSTR,
        dw_options: REG_OPEN_CREATE_OPTIONS,
        sam_desired: REG_SAM_FLAGS,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        phk_result: MutPtr<HKEY>,
        lpdw_disposition: MutPtr<REG_CREATE_KEY_DISPOSITION>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCreateKeyExA")
    }
    #[doc = "RegCreateKeyExW from ADVAPI32"]
    fn RegCreateKeyExW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        reserved: u32,
        lp_class: PCWSTR,
        dw_options: REG_OPEN_CREATE_OPTIONS,
        sam_desired: REG_SAM_FLAGS,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        phk_result: MutPtr<HKEY>,
        lpdw_disposition: MutPtr<REG_CREATE_KEY_DISPOSITION>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCreateKeyExW")
    }
    #[doc = "RegCreateKeyTransactedA from ADVAPI32"]
    fn RegCreateKeyTransactedA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        reserved: u32,
        lp_class: PCSTR,
        dw_options: REG_OPEN_CREATE_OPTIONS,
        sam_desired: REG_SAM_FLAGS,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        phk_result: MutPtr<HKEY>,
        lpdw_disposition: MutPtr<REG_CREATE_KEY_DISPOSITION>,
        h_transaction: super::super::Foundation::HANDLE,
        p_extended_paremeter: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCreateKeyTransactedA")
    }
    #[doc = "RegCreateKeyTransactedW from ADVAPI32"]
    fn RegCreateKeyTransactedW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        reserved: u32,
        lp_class: PCWSTR,
        dw_options: REG_OPEN_CREATE_OPTIONS,
        sam_desired: REG_SAM_FLAGS,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        phk_result: MutPtr<HKEY>,
        lpdw_disposition: MutPtr<REG_CREATE_KEY_DISPOSITION>,
        h_transaction: super::super::Foundation::HANDLE,
        p_extended_paremeter: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCreateKeyTransactedW")
    }
    #[doc = "RegCreateKeyW from ADVAPI32"]
    fn RegCreateKeyW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegCreateKeyW")
    }
    #[doc = "RegDeleteKeyA from ADVAPI32"]
    fn RegDeleteKeyA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteKeyA")
    }
    #[doc = "RegDeleteKeyExA from ADVAPI32"]
    fn RegDeleteKeyExA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        sam_desired: u32,
        reserved: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteKeyExA")
    }
    #[doc = "RegDeleteKeyExW from ADVAPI32"]
    fn RegDeleteKeyExW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        sam_desired: u32,
        reserved: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteKeyExW")
    }
    #[doc = "RegDeleteKeyTransactedA from ADVAPI32"]
    fn RegDeleteKeyTransactedA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        sam_desired: u32,
        reserved: u32,
        h_transaction: super::super::Foundation::HANDLE,
        p_extended_parameter: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteKeyTransactedA")
    }
    #[doc = "RegDeleteKeyTransactedW from ADVAPI32"]
    fn RegDeleteKeyTransactedW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        sam_desired: u32,
        reserved: u32,
        h_transaction: super::super::Foundation::HANDLE,
        p_extended_parameter: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteKeyTransactedW")
    }
    #[doc = "RegDeleteKeyValueA from ADVAPI32"]
    fn RegDeleteKeyValueA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        lp_value_name: PCSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteKeyValueA")
    }
    #[doc = "RegDeleteKeyValueW from ADVAPI32"]
    fn RegDeleteKeyValueW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        lp_value_name: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteKeyValueW")
    }
    #[doc = "RegDeleteKeyW from ADVAPI32"]
    fn RegDeleteKeyW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteKeyW")
    }
    #[doc = "RegDeleteTreeA from ADVAPI32"]
    fn RegDeleteTreeA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteTreeA")
    }
    #[doc = "RegDeleteTreeW from ADVAPI32"]
    fn RegDeleteTreeW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteTreeW")
    }
    #[doc = "RegDeleteValueA from ADVAPI32"]
    fn RegDeleteValueA(
        &self,
        h_key: HKEY,
        lp_value_name: PCSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteValueA")
    }
    #[doc = "RegDeleteValueW from ADVAPI32"]
    fn RegDeleteValueW(
        &self,
        h_key: HKEY,
        lp_value_name: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDeleteValueW")
    }
    #[doc = "RegDisablePredefinedCache from ADVAPI32"]
    fn RegDisablePredefinedCache(&self) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDisablePredefinedCache")
    }
    #[doc = "RegDisablePredefinedCacheEx from ADVAPI32"]
    fn RegDisablePredefinedCacheEx(&self) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDisablePredefinedCacheEx")
    }
    #[doc = "RegDisableReflectionKey from ADVAPI32"]
    fn RegDisableReflectionKey(&self, h_base: HKEY) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegDisableReflectionKey")
    }
    #[doc = "RegEnableReflectionKey from ADVAPI32"]
    fn RegEnableReflectionKey(&self, h_base: HKEY) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegEnableReflectionKey")
    }
    #[doc = "RegEnumKeyA from ADVAPI32"]
    fn RegEnumKeyA(
        &self,
        h_key: HKEY,
        dw_index: u32,
        lp_name: PSTR,
        cch_name: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegEnumKeyA")
    }
    #[doc = "RegEnumKeyExA from ADVAPI32"]
    fn RegEnumKeyExA(
        &self,
        h_key: HKEY,
        dw_index: u32,
        lp_name: PSTR,
        lpcch_name: MutPtr<u32>,
        lp_reserved: MutPtr<u32>,
        lp_class: PSTR,
        lpcch_class: MutPtr<u32>,
        lpft_last_write_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegEnumKeyExA")
    }
    #[doc = "RegEnumKeyExW from ADVAPI32"]
    fn RegEnumKeyExW(
        &self,
        h_key: HKEY,
        dw_index: u32,
        lp_name: PWSTR,
        lpcch_name: MutPtr<u32>,
        lp_reserved: MutPtr<u32>,
        lp_class: PWSTR,
        lpcch_class: MutPtr<u32>,
        lpft_last_write_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegEnumKeyExW")
    }
    #[doc = "RegEnumKeyW from ADVAPI32"]
    fn RegEnumKeyW(
        &self,
        h_key: HKEY,
        dw_index: u32,
        lp_name: PWSTR,
        cch_name: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegEnumKeyW")
    }
    #[doc = "RegEnumValueA from ADVAPI32"]
    fn RegEnumValueA(
        &self,
        h_key: HKEY,
        dw_index: u32,
        lp_value_name: PSTR,
        lpcch_value_name: MutPtr<u32>,
        lp_reserved: MutPtr<u32>,
        lp_type: MutPtr<u32>,
        lp_data: MutPtr<u8>,
        lpcb_data: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegEnumValueA")
    }
    #[doc = "RegEnumValueW from ADVAPI32"]
    fn RegEnumValueW(
        &self,
        h_key: HKEY,
        dw_index: u32,
        lp_value_name: PWSTR,
        lpcch_value_name: MutPtr<u32>,
        lp_reserved: MutPtr<u32>,
        lp_type: MutPtr<u32>,
        lp_data: MutPtr<u8>,
        lpcb_data: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegEnumValueW")
    }
    #[doc = "RegFlushKey from ADVAPI32"]
    fn RegFlushKey(&self, h_key: HKEY) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegFlushKey")
    }
    #[doc = "RegGetKeySecurity from ADVAPI32"]
    fn RegGetKeySecurity(
        &self,
        h_key: HKEY,
        security_information: u32,
        p_security_descriptor: MutPtr<super::super::Security::SECURITY_DESCRIPTOR>,
        lpcb_security_descriptor: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegGetKeySecurity")
    }
    #[doc = "RegGetValueA from ADVAPI32"]
    fn RegGetValueA(
        &self,
        hkey: HKEY,
        lp_sub_key: PCSTR,
        lp_value: PCSTR,
        dw_flags: RRF_RT,
        pdw_type: MutPtr<u32>,
        pv_data: MutPtr<::core::ffi::c_void>,
        pcb_data: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegGetValueA")
    }
    #[doc = "RegGetValueW from ADVAPI32"]
    fn RegGetValueW(
        &self,
        hkey: HKEY,
        lp_sub_key: PCWSTR,
        lp_value: PCWSTR,
        dw_flags: RRF_RT,
        pdw_type: MutPtr<u32>,
        pv_data: MutPtr<::core::ffi::c_void>,
        pcb_data: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegGetValueW")
    }
    #[doc = "RegLoadAppKeyA from ADVAPI32"]
    fn RegLoadAppKeyA(
        &self,
        lp_file: PCSTR,
        phk_result: MutPtr<HKEY>,
        sam_desired: u32,
        dw_options: u32,
        reserved: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegLoadAppKeyA")
    }
    #[doc = "RegLoadAppKeyW from ADVAPI32"]
    fn RegLoadAppKeyW(
        &self,
        lp_file: PCWSTR,
        phk_result: MutPtr<HKEY>,
        sam_desired: u32,
        dw_options: u32,
        reserved: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegLoadAppKeyW")
    }
    #[doc = "RegLoadKeyA from ADVAPI32"]
    fn RegLoadKeyA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        lp_file: PCSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegLoadKeyA")
    }
    #[doc = "RegLoadKeyW from ADVAPI32"]
    fn RegLoadKeyW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        lp_file: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegLoadKeyW")
    }
    #[doc = "RegLoadMUIStringA from ADVAPI32"]
    fn RegLoadMUIStringA(
        &self,
        h_key: HKEY,
        psz_value: PCSTR,
        psz_out_buf: PSTR,
        cb_out_buf: u32,
        pcb_data: MutPtr<u32>,
        flags: u32,
        psz_directory: PCSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegLoadMUIStringA")
    }
    #[doc = "RegLoadMUIStringW from ADVAPI32"]
    fn RegLoadMUIStringW(
        &self,
        h_key: HKEY,
        psz_value: PCWSTR,
        psz_out_buf: PWSTR,
        cb_out_buf: u32,
        pcb_data: MutPtr<u32>,
        flags: u32,
        psz_directory: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegLoadMUIStringW")
    }
    #[doc = "RegNotifyChangeKeyValue from ADVAPI32"]
    fn RegNotifyChangeKeyValue(
        &self,
        h_key: HKEY,
        b_watch_subtree: super::super::Foundation::BOOL,
        dw_notify_filter: REG_NOTIFY_FILTER,
        h_event: super::super::Foundation::HANDLE,
        f_asynchronous: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegNotifyChangeKeyValue")
    }
    #[doc = "RegOpenCurrentUser from ADVAPI32"]
    fn RegOpenCurrentUser(
        &self,
        sam_desired: u32,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOpenCurrentUser")
    }
    #[doc = "RegOpenKeyA from ADVAPI32"]
    fn RegOpenKeyA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOpenKeyA")
    }
    #[doc = "RegOpenKeyExA from ADVAPI32"]
    fn RegOpenKeyExA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        ul_options: u32,
        sam_desired: REG_SAM_FLAGS,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOpenKeyExA")
    }
    #[doc = "RegOpenKeyExW from ADVAPI32"]
    fn RegOpenKeyExW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        ul_options: u32,
        sam_desired: REG_SAM_FLAGS,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOpenKeyExW")
    }
    #[doc = "RegOpenKeyTransactedA from ADVAPI32"]
    fn RegOpenKeyTransactedA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        ul_options: u32,
        sam_desired: REG_SAM_FLAGS,
        phk_result: MutPtr<HKEY>,
        h_transaction: super::super::Foundation::HANDLE,
        p_extended_paremeter: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOpenKeyTransactedA")
    }
    #[doc = "RegOpenKeyTransactedW from ADVAPI32"]
    fn RegOpenKeyTransactedW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        ul_options: u32,
        sam_desired: REG_SAM_FLAGS,
        phk_result: MutPtr<HKEY>,
        h_transaction: super::super::Foundation::HANDLE,
        p_extended_paremeter: MutPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOpenKeyTransactedW")
    }
    #[doc = "RegOpenKeyW from ADVAPI32"]
    fn RegOpenKeyW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOpenKeyW")
    }
    #[doc = "RegOpenUserClassesRoot from ADVAPI32"]
    fn RegOpenUserClassesRoot(
        &self,
        h_token: super::super::Foundation::HANDLE,
        dw_options: u32,
        sam_desired: u32,
        phk_result: MutPtr<HKEY>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOpenUserClassesRoot")
    }
    #[doc = "RegOverridePredefKey from ADVAPI32"]
    fn RegOverridePredefKey(
        &self,
        h_key: HKEY,
        h_new_h_key: HKEY,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegOverridePredefKey")
    }
    #[doc = "RegQueryInfoKeyA from ADVAPI32"]
    fn RegQueryInfoKeyA(
        &self,
        h_key: HKEY,
        lp_class: PSTR,
        lpcch_class: MutPtr<u32>,
        lp_reserved: MutPtr<u32>,
        lpc_sub_keys: MutPtr<u32>,
        lpcb_max_sub_key_len: MutPtr<u32>,
        lpcb_max_class_len: MutPtr<u32>,
        lpc_values: MutPtr<u32>,
        lpcb_max_value_name_len: MutPtr<u32>,
        lpcb_max_value_len: MutPtr<u32>,
        lpcb_security_descriptor: MutPtr<u32>,
        lpft_last_write_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryInfoKeyA")
    }
    #[doc = "RegQueryInfoKeyW from ADVAPI32"]
    fn RegQueryInfoKeyW(
        &self,
        h_key: HKEY,
        lp_class: PWSTR,
        lpcch_class: MutPtr<u32>,
        lp_reserved: MutPtr<u32>,
        lpc_sub_keys: MutPtr<u32>,
        lpcb_max_sub_key_len: MutPtr<u32>,
        lpcb_max_class_len: MutPtr<u32>,
        lpc_values: MutPtr<u32>,
        lpcb_max_value_name_len: MutPtr<u32>,
        lpcb_max_value_len: MutPtr<u32>,
        lpcb_security_descriptor: MutPtr<u32>,
        lpft_last_write_time: MutPtr<super::super::Foundation::FILETIME>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryInfoKeyW")
    }
    #[doc = "RegQueryMultipleValuesA from ADVAPI32"]
    fn RegQueryMultipleValuesA(
        &self,
        h_key: HKEY,
        val_list: MutPtr<VALENTA>,
        num_vals: u32,
        lp_value_buf: PSTR,
        ldw_totsize: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryMultipleValuesA")
    }
    #[doc = "RegQueryMultipleValuesW from ADVAPI32"]
    fn RegQueryMultipleValuesW(
        &self,
        h_key: HKEY,
        val_list: MutPtr<VALENTW>,
        num_vals: u32,
        lp_value_buf: PWSTR,
        ldw_totsize: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryMultipleValuesW")
    }
    #[doc = "RegQueryReflectionKey from ADVAPI32"]
    fn RegQueryReflectionKey(
        &self,
        h_base: HKEY,
        b_is_reflection_disabled: MutPtr<super::super::Foundation::BOOL>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryReflectionKey")
    }
    #[doc = "RegQueryValueA from ADVAPI32"]
    fn RegQueryValueA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        lp_data: PSTR,
        lpcb_data: MutPtr<i32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryValueA")
    }
    #[doc = "RegQueryValueExA from ADVAPI32"]
    fn RegQueryValueExA(
        &self,
        h_key: HKEY,
        lp_value_name: PCSTR,
        lp_reserved: MutPtr<u32>,
        lp_type: MutPtr<REG_VALUE_TYPE>,
        lp_data: MutPtr<u8>,
        lpcb_data: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryValueExA")
    }
    #[doc = "RegQueryValueExW from ADVAPI32"]
    fn RegQueryValueExW(
        &self,
        h_key: HKEY,
        lp_value_name: PCWSTR,
        lp_reserved: MutPtr<u32>,
        lp_type: MutPtr<REG_VALUE_TYPE>,
        lp_data: MutPtr<u8>,
        lpcb_data: MutPtr<u32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryValueExW")
    }
    #[doc = "RegQueryValueW from ADVAPI32"]
    fn RegQueryValueW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        lp_data: PWSTR,
        lpcb_data: MutPtr<i32>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegQueryValueW")
    }
    #[doc = "RegRenameKey from ADVAPI32"]
    fn RegRenameKey(
        &self,
        h_key: HKEY,
        lp_sub_key_name: PCWSTR,
        lp_new_key_name: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegRenameKey")
    }
    #[doc = "RegReplaceKeyA from ADVAPI32"]
    fn RegReplaceKeyA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        lp_new_file: PCSTR,
        lp_old_file: PCSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegReplaceKeyA")
    }
    #[doc = "RegReplaceKeyW from ADVAPI32"]
    fn RegReplaceKeyW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        lp_new_file: PCWSTR,
        lp_old_file: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegReplaceKeyW")
    }
    #[doc = "RegRestoreKeyA from ADVAPI32"]
    fn RegRestoreKeyA(
        &self,
        h_key: HKEY,
        lp_file: PCSTR,
        dw_flags: REG_RESTORE_KEY_FLAGS,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegRestoreKeyA")
    }
    #[doc = "RegRestoreKeyW from ADVAPI32"]
    fn RegRestoreKeyW(
        &self,
        h_key: HKEY,
        lp_file: PCWSTR,
        dw_flags: REG_RESTORE_KEY_FLAGS,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegRestoreKeyW")
    }
    #[doc = "RegSaveKeyA from ADVAPI32"]
    fn RegSaveKeyA(
        &self,
        h_key: HKEY,
        lp_file: PCSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSaveKeyA")
    }
    #[doc = "RegSaveKeyExA from ADVAPI32"]
    fn RegSaveKeyExA(
        &self,
        h_key: HKEY,
        lp_file: PCSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        flags: REG_SAVE_FORMAT,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSaveKeyExA")
    }
    #[doc = "RegSaveKeyExW from ADVAPI32"]
    fn RegSaveKeyExW(
        &self,
        h_key: HKEY,
        lp_file: PCWSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
        flags: REG_SAVE_FORMAT,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSaveKeyExW")
    }
    #[doc = "RegSaveKeyW from ADVAPI32"]
    fn RegSaveKeyW(
        &self,
        h_key: HKEY,
        lp_file: PCWSTR,
        lp_security_attributes: ConstPtr<super::super::Security::SECURITY_ATTRIBUTES>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSaveKeyW")
    }
    #[doc = "RegSetKeySecurity from ADVAPI32"]
    fn RegSetKeySecurity(
        &self,
        h_key: HKEY,
        security_information: u32,
        p_security_descriptor: ConstPtr<super::super::Security::SECURITY_DESCRIPTOR>,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSetKeySecurity")
    }
    #[doc = "RegSetKeyValueA from ADVAPI32"]
    fn RegSetKeyValueA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        lp_value_name: PCSTR,
        dw_type: u32,
        lp_data: ConstPtr<::core::ffi::c_void>,
        cb_data: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSetKeyValueA")
    }
    #[doc = "RegSetKeyValueW from ADVAPI32"]
    fn RegSetKeyValueW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        lp_value_name: PCWSTR,
        dw_type: u32,
        lp_data: ConstPtr<::core::ffi::c_void>,
        cb_data: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSetKeyValueW")
    }
    #[doc = "RegSetValueA from ADVAPI32"]
    fn RegSetValueA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
        dw_type: REG_VALUE_TYPE,
        lp_data: PCSTR,
        cb_data: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSetValueA")
    }
    #[doc = "RegSetValueExA from ADVAPI32"]
    fn RegSetValueExA(
        &self,
        h_key: HKEY,
        lp_value_name: PCSTR,
        reserved: u32,
        dw_type: REG_VALUE_TYPE,
        lp_data: ConstPtr<u8>,
        cb_data: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSetValueExA")
    }
    #[doc = "RegSetValueExW from ADVAPI32"]
    fn RegSetValueExW(
        &self,
        h_key: HKEY,
        lp_value_name: PCWSTR,
        reserved: u32,
        dw_type: REG_VALUE_TYPE,
        lp_data: ConstPtr<u8>,
        cb_data: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSetValueExW")
    }
    #[doc = "RegSetValueW from ADVAPI32"]
    fn RegSetValueW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
        dw_type: REG_VALUE_TYPE,
        lp_data: PCWSTR,
        cb_data: u32,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegSetValueW")
    }
    #[doc = "RegUnLoadKeyA from ADVAPI32"]
    fn RegUnLoadKeyA(
        &self,
        h_key: HKEY,
        lp_sub_key: PCSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegUnLoadKeyA")
    }
    #[doc = "RegUnLoadKeyW from ADVAPI32"]
    fn RegUnLoadKeyW(
        &self,
        h_key: HKEY,
        lp_sub_key: PCWSTR,
    ) -> super::super::Foundation::WIN32_ERROR {
        todo!("RegUnLoadKeyW")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
