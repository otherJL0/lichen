// SPDX-FileCopyrightText: Copyright © 2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0
use std::collections::{HashMap, HashSet};

use fs_err as fs;

const ZONEINFO_BASE: &str = "/usr/share/zoneinfo";

pub struct Registry {
    timezones: Vec<String>,
    timezones_lookup: HashMap<String, Vec<usize>>,
}

impl Registry {
    pub fn new() -> Result<Self, std::io::Error> {
        let timezones_table = Self::load_timezones_table()?;
        let mut timezones: Vec<String> = timezones_table
            .values()
            .flat_map(|zones| zones.clone())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        timezones.sort();

        let timezone_index_lookup: HashMap<String, usize> =
            timezones.iter().enumerate().map(|(i, v)| (v.clone(), i)).collect();
        let mut timezones_lookup: HashMap<String, Vec<usize>> = HashMap::with_capacity(timezones_table.capacity());

        for (code2, timezones) in timezones_table {
            let result: Vec<usize> = timezones
                .iter()
                .map(|timezone| timezone_index_lookup[timezone])
                .collect();
            timezones_lookup.insert(code2, result);
        }

        Ok(Self {
            timezones,
            timezones_lookup,
        })
    }

    fn load_timezones_table() -> Result<HashMap<String, Vec<String>>, std::io::Error> {
        let zone_tab = format!("{ZONEINFO_BASE}/zone1970.tab");
        let contents = fs::read_to_string(zone_tab)?;
        let mut timezones_lookup: HashMap<String, Vec<String>> = HashMap::new();

        for line in contents.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let row: Vec<&str> = line.split('\t').collect();
            let timezone = row[2];
            for code2 in row[0].split(',') {
                timezones_lookup
                    .entry(code2.to_string())
                    .or_default()
                    .push(timezone.to_string());
            }
        }
        Ok(timezones_lookup)
    }

    pub fn all_timezones(self) -> Vec<String> {
        self.timezones
    }
}
