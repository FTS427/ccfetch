pub mod color;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use color::_BOLD;
use color::_RESET;
pub mod logo;

pub fn split_by_newline_new(logo: &String) -> Vec<String> {
    let mut split_vec: Vec<String> = Vec::new();
    for item in logo.split('\n') {
        split_vec.push(item.to_owned());
    }
    split_vec
}

pub fn format_data(icon: &str, value: &str, color: &str) -> String {
    format!(
        " {color} {key}  {_BOLD}{_RESET} {value}",
        key = icon,
        value = value,
    )
}

pub fn get_device_name_pci(vendor_id: &str, device_id: &str) -> io::Result<Option<String>> {
    let path_hwdata = Path::new("/usr/share/hwdata/pci.ids");
    let path_misc = Path::new("/usr/share/misc/pci.ids");
    let file;
    if path_hwdata.exists() {
        file = File::open(&path_hwdata)?;
    } else {
        file = File::open(&path_misc)?;
    }
    let reader = io::BufReader::new(file);

    let mut device_name = None;
    let mut current_vendor = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if line.split_whitespace().count() >= 2 {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let id = parts[0];
            if id == vendor_id.to_string() {
                current_vendor = Some(parts[1..].join(" "));
                continue;
            }
        }

        if let Some(_) = &current_vendor {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let id = parts[0];
                let name = parts[1..].join(" ");
                if id == device_id.to_string() {
                    device_name = Some(format!("{}", name));
                    break;
                }
            }
        }
    }

    Ok(device_name)
}

pub fn read_pci_devices_and_find_gpu() -> io::Result<Vec<(String, String)>> {
    let pci_devices_path = "/sys/bus/pci/devices";
    let mut devices = Vec::new();

    for entry in fs::read_dir(pci_devices_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let modalias_file_path = path.join("modalias");

            if modalias_file_path.exists() {
                let modalias_content = fs::read_to_string(modalias_file_path)?;
                let modalias_trimmed = modalias_content.trim();

                let class_id = &modalias_trimmed[44..46];

                if class_id == "03" {
                    let vendor_id = &modalias_trimmed[9..13];
                    let device_id = &modalias_trimmed[18..22];

                    devices.push((vendor_id.to_string(), device_id.to_string()));
                }
            }
        }
    }

    Ok(devices)
}

struct Vendor {
    vendor_id: &'static str,
    name: &'static str,
}

const VENDORS: &[Vendor] = &[
    Vendor {
        vendor_id: "106b",
        name: "Apple",
    },
    Vendor {
        vendor_id: "1002",
        name: "AMD",
    },
    Vendor {
        vendor_id: "1022",
        name: "AMD",
    },
    Vendor {
        vendor_id: "8086",
        name: "Intel",
    },
    Vendor {
        vendor_id: "8087",
        name: "Intel",
    },
    Vendor {
        vendor_id: "03e7",
        name: "Intel",
    },
    Vendor {
        vendor_id: "0955",
        name: "NVIDIA",
    },
    Vendor {
        vendor_id: "10de",
        name: "NVIDIA",
    },
    Vendor {
        vendor_id: "12d2",
        name: "NVIDIA",
    },
    Vendor {
        vendor_id: "1ed5",
        name: "Moore Threads",
    },
    Vendor {
        vendor_id: "5143",
        name: "Qualcomm",
    },
    Vendor {
        vendor_id: "14c3",
        name: "MTK",
    },
    Vendor {
        vendor_id: "15ad",
        name: "VMware",
    },
    Vendor {
        vendor_id: "1af4",
        name: "RedHat",
    },
    Vendor {
        vendor_id: "1ab8",
        name: "Parallel",
    },
    Vendor {
        vendor_id: "1414",
        name: "Microsoft",
    },
    Vendor {
        vendor_id: "108e",
        name: "Oracle",
    },
];

pub fn get_gpu_vendor_name(vendor_id: &str) -> &'static str {
    for vendor in VENDORS {
        if vendor.vendor_id == vendor_id {
            return vendor.name;
        }
    }
    "Unknown Vendor"
}
