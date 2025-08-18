//! Low-level system metrics collection using direct /proc filesystem access
//!
//! This module provides precise system metrics by reading directly from the Linux
//! /proc filesystem, matching the behavior of the standard uptime command.

use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead, BufReader};
use utmpx::{close_database, read_next_entry, sys::UtType};

/// System metrics collector using low-level /proc filesystem access
#[derive(Debug, Clone, PartialEq)]
pub struct SystemMetrics {
    /// System uptime in seconds (floating point for precision)
    uptime_seconds: f64,
    /// System idle time in seconds
    idle_time: f64,
    /// Load averages (1min, 5min, 15min)
    load_avg: (f64, f64, f64),
    /// Number of unique logged-in users
    user_count: usize,
    /// System boot time as UNIX timestamp
    boot_time: u64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            uptime_seconds: 0.0,
            idle_time: 0.0,
            load_avg: (0.0, 0.0, 0.0),
            user_count: 0,
            boot_time: 0,
        }
    }
}

impl SystemMetrics {
    /// Creates a new SystemMetrics instance by reading from /proc filesystem
    pub fn new() -> io::Result<Self> {
        let mut metrics = Self::default();

        // Read uptime and idle time from /proc/uptime
        metrics.read_uptime()?;

        // Read load averages from /proc/loadavg
        metrics.read_loadavg()?;

        // Read user count from utmp database
        metrics.read_users();

        // Calculate boot time from uptime
        metrics.calculate_boot_time()?;

        Ok(metrics)
    }

    /// Read uptime and idle time from /proc/uptime
    fn read_uptime(&mut self) -> io::Result<()> {
        let content = fs::read_to_string("/proc/uptime")?;
        let parts: Vec<&str> = content.trim().split_whitespace().collect();

        if parts.len() >= 2 {
            self.uptime_seconds = parts[0].parse().unwrap_or(0.0);
            self.idle_time = parts[1].parse().unwrap_or(0.0);
        }

        Ok(())
    }

    /// Read load averages from /proc/loadavg
    fn read_loadavg(&mut self) -> io::Result<()> {
        let content = fs::read_to_string("/proc/loadavg")?;
        let parts: Vec<&str> = content.trim().split_whitespace().collect();

        if parts.len() >= 3 {
            let load1 = parts[0].parse().unwrap_or(0.0);
            let load5 = parts[1].parse().unwrap_or(0.0);
            let load15 = parts[2].parse().unwrap_or(0.0);
            self.load_avg = (load1, load5, load15);
        }

        Ok(())
    }

    /// Count unique users from the utmp database
    fn read_users(&mut self) {
        let mut unique_users: HashSet<Vec<u8>> = HashSet::new();

        while let Ok(utmp) = read_next_entry() {
            // UtType::USER_PROCESS is a logged in user
            if matches!(utmp.ut_type, UtType::USER_PROCESS) {
                // Take ut_user up to the first null byte
                let user_bytes: Vec<u8> = utmp
                    .ut_user
                    .iter()
                    .take_while(|&&c| c != 0)
                    .map(|&c| c as u8)
                    .collect();

                unique_users.insert(user_bytes);
            }
        }

        close_database();
        self.user_count = unique_users.len();
    }

    /// Calculate boot time from current time minus uptime
    fn calculate_boot_time(&mut self) -> io::Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.boot_time = now.saturating_sub(self.uptime_seconds as u64);
        Ok(())
    }

    /// Get uptime in seconds with decimal precision
    pub fn uptime_seconds(&self) -> f64 {
        self.uptime_seconds
    }

    /// Get idle time in seconds
    pub fn idle_time(&self) -> f64 {
        self.idle_time
    }

    /// Get load averages as (1min, 5min, 15min)
    pub fn load_averages(&self) -> (f64, f64, f64) {
        self.load_avg
    }

    /// Get number of unique users
    pub fn user_count(&self) -> usize {
        self.user_count
    }

    /// Get system boot time as UNIX timestamp
    pub fn boot_time(&self) -> u64 {
        self.boot_time
    }

    /// Refresh all metrics
    pub fn refresh(&mut self) -> io::Result<()> {
        self.read_uptime()?;
        self.read_loadavg()?;
        self.read_users();
        self.calculate_boot_time()?;
        Ok(())
    }
}
