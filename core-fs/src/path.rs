use crate::error::PathParseError;
use arcstr::ArcStr;
use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum Drive {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl TryFrom<char> for Drive {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if !c.is_ascii() {
            return Err(());
        }
        let c = c.to_ascii_uppercase();
        let c = c as u8;
        if c < b'A' {
            return Err(());
        }
        if c > b'Z' {
            return Err(());
        }

        let index = c - b'A';

        Ok(num::FromPrimitive::from_u8(index).ok_or(())?)
    }
}

impl Display for Drive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Root {
    UNC,          // \\
    Drive(Drive), // C:\
    Device,       // \\.\
    RawDevice,    // \\?\
}

impl Display for Root {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Root::UNC => write!(f, "\\\\"),
            Root::Drive(drive) => write!(f, "{}:\\", drive),
            Root::Device => write!(f, "\\\\.\\"),
            Root::RawDevice => write!(f, "\\\\?\\"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DirectoryPath {
    pub parts: Vec<ArcStr>,
}

impl Display for DirectoryPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, part) in self.parts.iter().enumerate() {
            if i != 0 {
                write!(f, "\\")?;
            }
            write!(f, "{}", part)?
        }
        Ok(())
    }
}

impl DirectoryPath {
    pub fn parse(input: &str, as_relative: bool) -> Result<Self, PathParseError> {
        let mut res = Vec::new();
        let mut component = String::new();

        // TODO: sometimes not all the normalization is needed, for example when \\?\ prefix is used
        // need to investigate...

        fn flush_component(as_relative: bool, res: &mut Vec<ArcStr>, component: &mut String) {
            if !component.is_empty() {
                if component == "." {
                    component.clear();
                } else if component == ".." {
                    let popped = res.pop();
                    if as_relative {
                        if popped.is_none() {
                            res.push(arcstr::literal!(".."))
                        }
                    }
                    component.clear();
                } else {
                    res.push(ArcStr::from(component.as_str()));
                    component.clear();
                }
            }
        }

        for c in input.chars() {
            if is_separator(c) {
                flush_component(as_relative, &mut res, &mut component)
            } else {
                component.push(c)
            }
        }

        flush_component(as_relative, &mut res, &mut component);

        Ok(Self { parts: res })
    }

    pub fn iter(&self) -> impl Iterator<Item = &ArcStr> {
        self.parts.iter()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(unused)]
pub enum DosDevice {
    Aux,
    Com,
    Con,
    Lpt,
    Nul,
    Prn,
    ConIn,
    ConOut,
}

impl Display for DosDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let device = match self {
            DosDevice::Aux => "AUX",
            DosDevice::Com => "COM",
            DosDevice::Con => "CON",
            DosDevice::Lpt => "LPT",
            DosDevice::Nul => "NUL",
            DosDevice::Prn => "PRN",
            DosDevice::ConIn => "CONIN$",
            DosDevice::ConOut => "CONOUT$",
        };
        write!(f, "{}", device)
    }
}

// see https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file
// and https://docs.microsoft.com/en-us/dotnet/standard/io/file-path-formats

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum WindowsPath {
    /// Standard windows absolute path
    ///
    /// Depending on the value of the Root can represent:
    /// - a normal abs path: C:\file.txt
    /// - a UNC path: \\share\help\file.txt
    /// - a dos device path: \\.\secret_name
    /// - raw NT path: \\?\RawName
    Absolute(Root, DirectoryPath),
    /// Path relative to the disk of the cwd
    ///
    /// For example: \file.txt
    CurrentDiskRelative(DirectoryPath),
    /// Path relative to the current directory
    ///
    /// For example: file.txt
    CwdRelative(DirectoryPath),
    /// Path relative to the disk-specific working directory
    ///
    /// For example: C:file.txt
    ///
    /// This relatively obscure feature of windows
    ///
    /// wine impl may be useful: https://github.com/wine-mirror/wine/blob/c1e793f1119de0c0ef7d4bd6d9fefbafdb5dbbe5/dlls/ntdll/path.c#L546
    DiskRelative(Drive, DirectoryPath),
    #[allow(unused)] // parsing those is TODO
    DosDevice(DosDevice),
}

impl Display for WindowsPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowsPath::Absolute(root, path) => write!(f, "{}{}", root, path),
            WindowsPath::CurrentDiskRelative(path) => write!(f, "\\{}", path),
            WindowsPath::CwdRelative(path) => write!(f, "{}", path),
            WindowsPath::DiskRelative(drive, path) => write!(f, "{}:{}", drive, path),
            WindowsPath::DosDevice(device) => write!(f, "{}", device),
        }
    }
}

pub struct AbsolutePath {
    pub root: Root,
    pub path: DirectoryPath,
}

impl Display for AbsolutePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.root, self.path)
    }
}

fn is_separator(c: char) -> bool {
    c == '/' || c == '\\'
}

impl WindowsPath {
    /// Parses and normalizes path
    ///
    /// TODO: this currently does not handle DOS device names (like AUX, CON, COM, etc, etc)
    pub fn parse(input: &str) -> Result<Self, PathParseError> {
        if input.trim().is_empty() {
            return Err(PathParseError::EmptyPath);
        }

        // first determine the path type

        let mut chars = input.chars();
        let first = chars.next().unwrap();
        if is_separator(first) {
            if let Some(second) = chars.next() {
                if is_separator(second) {
                    // it can be either a UCN path or some kind of device path
                    return if let Some(third) = chars.next() {
                        let mut root = Root::UNC;

                        if third == '?' {
                            if let Some(fourth) = chars.next() {
                                if is_separator(fourth) {
                                    root = Root::RawDevice
                                }
                            }
                        } else if third == '.' {
                            if let Some(fourth) = chars.next() {
                                if is_separator(fourth) {
                                    root = Root::Device
                                }
                            }
                        };

                        let rest = match root {
                            Root::UNC => &input[2..],                      // skip the \\
                            Root::RawDevice | Root::Device => &input[4..], // skip the \\?\ or \\.\
                            _ => unreachable!(),
                        };

                        Ok(WindowsPath::Absolute(
                            root,
                            DirectoryPath::parse(rest, false)?,
                        ))
                    } else {
                        // WTF is \\ ???
                        Err(PathParseError::EmptyUncPath)
                    };
                }
            }
            // else it's just a slash
            // it's just the root of the current disk
            Ok(WindowsPath::CurrentDiskRelative(DirectoryPath::parse(
                &input[1..], /* skip the first slash */
                false,
            )?))
        } else {
            if let Some(second) = chars.next() {
                if second == ':' {
                    // aha, a drive separator
                    // must be an absolute path using some drive as a root or a drive-relative path
                    let drive =
                        Drive::try_from(first).map_err(|_| PathParseError::InvalidDriveLetter)?;
                    let root = Root::Drive(drive.clone());

                    let third = chars.next();
                    return if let Some(third) = third {
                        if is_separator(third) {
                            // an absolute path
                            Ok(WindowsPath::Absolute(
                                root,
                                DirectoryPath::parse(
                                    &input[3..], /* skip the drive letter, the drive separator and the first directory separator (C:\) */
                                    false,
                                )?,
                            ))
                        } else {
                            // ew, a disk-relative path
                            Ok(WindowsPath::DiskRelative(
                                drive,
                                DirectoryPath::parse(
                                    &input[2..], /* skip the drive letter and the drive separator (C:) */
                                    true,
                                )?,
                            ))
                        }
                    } else {
                        // only two characters and the second one is :
                        // this smth like C:
                        // so it must refer to the disk root
                        Ok(WindowsPath::Absolute(
                            root,
                            DirectoryPath::parse("", false)?,
                        ))
                    };
                } // else - must be relative
            }
            // else - only one char path
            // must be relative
            Ok(WindowsPath::CwdRelative(DirectoryPath::parse(input, true)?))
        }
    }

    /// Resolves path to a pair of a root directory and a path inside it
    #[allow(unused)]
    pub fn resolve(self, current_directory: &AbsolutePath) -> AbsolutePath {
        match self {
            WindowsPath::Absolute(root, path) => AbsolutePath { root, path },
            WindowsPath::CwdRelative(path) => {
                let mut new_path = current_directory.path.clone();
                for element in path.iter() {
                    // assumption: "." elements are already handled by the parser, no need to handle them here
                    if element == ".." {
                        new_path.parts.pop();
                    } else {
                        new_path.parts.push(element.clone());
                    }
                }
                AbsolutePath {
                    root: current_directory.root.clone(),
                    path: new_path,
                }
            }
            WindowsPath::CurrentDiskRelative(_) => {
                todo!("Current-disk relative path resolving")
            }
            WindowsPath::DiskRelative(_, _) => {
                todo!("Disk-relative path resolving")
            }
            WindowsPath::DosDevice(_) => {
                todo!("Dos device path resolving")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::path::{DirectoryPath, Drive, Root, WindowsPath};

    #[test]
    fn display_path() {
        assert_eq!(format!("{}", Drive::C), "C");
        assert_eq!(format!("{}", Root::Drive(Drive::C)), "C:\\");
        assert_eq!(
            format!(
                "{}",
                DirectoryPath {
                    parts: vec![arcstr::literal!("hello"), arcstr::literal!("world.txt")]
                }
            ),
            "hello\\world.txt"
        );
        assert_eq!(
            format!(
                "{}",
                WindowsPath::Absolute(
                    Root::Drive(Drive::C),
                    DirectoryPath {
                        parts: vec![arcstr::literal!("hello"), arcstr::literal!("world.txt")]
                    }
                )
            ),
            "C:\\hello\\world.txt"
        );
    }

    fn test_roundtrip(path: &str) {
        test_trip(path, path)
    }

    fn test_trip(orig: &str, expected: &str) {
        let parsed = WindowsPath::parse(orig).unwrap();
        let stringified = format!("{}", parsed);
        assert_eq!(stringified, expected);
    }

    #[test]
    fn parse_absolute() {
        test_roundtrip("C:\\");
        test_roundtrip("C:\\hello.txt");
        test_roundtrip("C:\\path\\with\\directories");
        test_trip("C:\\.\\kek", "C:\\kek");
        test_trip("c:\\..\\kek", "C:\\kek");
        test_trip("h:\\directory\\..\\kek", "H:\\kek");
        test_trip("Z:\\directory\\more\\..\\kek", "Z:\\directory\\kek");
    }

    #[test]
    fn parse_disk_relative() {
        test_roundtrip("C:hello.txt");
        test_roundtrip("C:dir\\hello.txt");
        test_roundtrip("C:..\\dir\\hello.txt");
    }

    #[test]
    fn parse_current_disk_relative() {
        test_roundtrip("\\hello.txt");
        test_roundtrip("\\dir\\hello.txt");
        test_trip("\\..\\dir\\hello.txt", "\\dir\\hello.txt");
    }
    #[test]
    fn parse_unc() {
        test_roundtrip("\\\\hello.txt");
        test_roundtrip("\\\\share\\hello.txt");
        test_roundtrip("\\\\?bla\\dir\\hello.txt");
        // a fucking wild corned case...
        // TODO: what to do?
        test_trip("\\\\..\\dir\\hello.txt", "\\\\dir\\hello.txt");
    }
}
