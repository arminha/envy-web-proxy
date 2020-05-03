/*
Copyright (C) 2018  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use xmltree::Element;

use std::io::Read;
use std::str::FromStr;

use crate::message::error::ParseError;
use crate::message::util;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScannerState {
    Idle,
    BusyWithScanJob,
}

impl FromStr for ScannerState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<ScannerState, ParseError> {
        match s {
            "Idle" => Ok(ScannerState::Idle),
            "BusyWithScanJob" => Ok(ScannerState::BusyWithScanJob),
            _ => Err(ParseError::unknown_enum_value("ScannerState", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AdfState {
    Empty,
    Loaded,
}

impl FromStr for AdfState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<AdfState, ParseError> {
        match s {
            "Empty" => Ok(AdfState::Empty),
            "Loaded" => Ok(AdfState::Loaded),
            _ => Err(ParseError::unknown_enum_value("AdfState", s)),
        }
    }
}

#[derive(Debug)]
pub struct ScanStatus {
    scanner_state: ScannerState,
    adf_state: AdfState,
}

impl ScanStatus {
    pub fn new(scanner_state: ScannerState, adf_state: AdfState) -> ScanStatus {
        ScanStatus {
            scanner_state,
            adf_state,
        }
    }

    pub fn scanner_state(&self) -> ScannerState {
        self.scanner_state
    }

    pub fn is_idle(&self) -> bool {
        self.scanner_state == ScannerState::Idle
    }

    pub fn adf_state(&self) -> AdfState {
        self.adf_state
    }

    pub fn read_xml<R: Read>(r: R) -> Result<ScanStatus, ParseError> {
        let element = Element::parse(r)?;
        let scanner_state: ScannerState = util::parse_child_value(&element, "ScannerState")?;
        let adf_state: AdfState = util::parse_child_value(&element, "AdfState")?;
        Ok(ScanStatus::new(scanner_state, adf_state))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const SCAN_STATUS_IDLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>Idle</ScannerState>
            <AdfState>Empty</AdfState>
            </ScanStatus>"#;

    const SCAN_STATUS_BUSY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>BusyWithScanJob</ScannerState>
            <AdfState>Empty</AdfState>
            </ScanStatus>"#;

    const SCAN_STATUS_LOADED: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>Idle</ScannerState>
            <AdfState>Loaded</AdfState>
            </ScanStatus>"#;

    #[test]
    fn read_scan_status_xml() {
        fn check_parse_scan_status(s: &str, scanner_state: ScannerState, adf_state: AdfState) {
            let status = s.as_bytes();
            let scan_status = ScanStatus::read_xml(status).expect("parsing failed");
            assert_eq!(scanner_state, scan_status.scanner_state());
            assert_eq!(adf_state, scan_status.adf_state());
        }
        check_parse_scan_status(SCAN_STATUS_IDLE, ScannerState::Idle, AdfState::Empty);
        check_parse_scan_status(
            SCAN_STATUS_BUSY,
            ScannerState::BusyWithScanJob,
            AdfState::Empty,
        );
        check_parse_scan_status(SCAN_STATUS_LOADED, ScannerState::Idle, AdfState::Loaded);
    }
}
