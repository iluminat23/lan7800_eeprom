use std::fs;
use std::path::Path;
use std::env;
use std::process::exit;

macro_rules! printreg {
	($img:expr, $offset:expr) => {
		println!("0x{:02X}: 0x{:02X}", $offset, $img[$offset])
	};
	($img:expr, $offset:expr, $desc:expr) => {
		println!("0x{:02X}: 0x{:02X}: {}", $offset, $img[$offset], $desc);
	};
	($img:expr, $offset:expr, $($desc:tt)*) => {
		println!("0x{:02X}: 0x{:02X}: {}", $offset, $img[$offset], format!($($desc)*));
	};
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		eprintln!("Usage: {} FILE", args[0]);
		exit(-1);
	}
	let path = Path::new(&args[1]);
	let img = match fs::read(&path) {
		Err(e) => panic!("Can't read file '{}': {}", path.display(), e),
		Ok(img) => img,
	};

	if img[0] != 0xa5 {
		eprintln!("0x00: {}: Invalid EEPROM signature, expected: 0xA5", img[0]);
		exit(-1);
	} else {
		printreg!(img, 0x00, "EEPROM signature found");
	}

	printreg!(img, 0x01, "MAC Address [7:0]");
	printreg!(img, 0x01, "MAC Address [15:8]");
	printreg!(img, 0x01, "MAC Address [23:16]");
	printreg!(img, 0x01, "MAC Address [31:24]");
	printreg!(img, 0x01, "MAC Address [39:32]");
	printreg!(img, 0x01, "MAC Address [47:40]");
	printreg!(img, 0x07, "Wakeup Enables");
	printreg!(img, 0x08, "RESERVED (0x00)");
	printreg!(img, 0x09, "GPIO PME Flags 0");
	printreg!(img, 0x0a, "GPIO PME Flags 1");
	printreg!(img, 0x0b, "LED Configuration 0");
	printreg!(img, 0x0c, "LED Configuration 1");
	printreg!(img, 0x0d, "LED Configuration 2");
	printreg!(img, 0x0e, "Wakeup Polarity");
	printreg!(img, 0x0f, "RESERVED (0x00)");
	printreg!(img, 0x10, "Full-Speed Polling Interval for Interrupt Endpoint");
	printreg!(img, 0x11, "High-Speed Polling Interval for Interrupt Endpoint");
	printreg!(img, 0x12, "SuperSpeed Polling Interval for Interrupt Endpoint");
	printreg!(img, 0x13, "Configuration Flags 0 [7:0]");
	printreg!(img, 0x14, "Configuration Flags 0 [15:8]");
	printreg!(img, 0x15, "Configuration Flags 0 [23:16]");
	printreg!(img, 0x16, "Configuration Flags 0 [31:24]");
	printreg!(img, 0x17, "Configuration Flags 1 [7:0]");
	printreg!(img, 0x18, "Configuration Flags 1 [15:8]");
	printreg!(img, 0x19, "Configuration Flags 1 [23:16]");
	printreg!(img, 0x1a, "Configuration Flags 1 [31:24]");
	printreg!(img, 0x1b, "Configuration Flags 2 [7:0]");
	printreg!(img, 0x1c, "Configuration Flags 2 [15:8]");
	printreg!(img, 0x1d, "Configuration Flags 2 [23:16]");
	printreg!(img, 0x1e, "Configuration Flags 2 [31:24]");
	printreg!(img, 0x1f, "Configuration Flags 3 [7:0]");
	printreg!(img, 0x20, "Configuration Flags 3 [15:8]");
	printreg!(img, 0x21, "Configuration Flags 3 [23:16]");
	printreg!(img, 0x22, "Configuration Flags 3 [31:24]");
	printreg!(img, 0x23, "Language ID [7:0]");
	printreg!(img, 0x24, "Language ID [15:8]");
	printreg!(img, 0x25, "Manufacturer ID String Descriptor Length (bytes)");
	printreg!(img, 0x26, "Manufacturer ID String Descriptor EEPROM Word Offset");
	printreg!(img, 0x27, "Product Name String Descriptor Length (bytes)");
	printreg!(img, 0x28, "Product Name String Descriptor EEPROM Word Offset");
	printreg!(img, 0x29, "Serial Number String Descriptor Length (bytes)");
	printreg!(img, 0x2a, "Serial Number String Descriptor EEPROM Word Offset");
	printreg!(img, 0x2b, "Configuration String Descriptor Length (bytes)");
	printreg!(img, 0x2c, "Configuration String Descriptor Word Offset");
	printreg!(img, 0x2d, "Interface String Descriptor Length (bytes)");
	printreg!(img, 0x2e, "Interface String Descriptor Word Offset");
	printreg!(img, 0x2f, "Binary Object Store (BOS) Block Length (Bytes)");
	printreg!(img, 0x30, "Binary Object Store (BOS) Block Word Offset");
	printreg!(img, 0x31, "SuperSpeed Device Descriptor Length (bytes)");
	printreg!(img, 0x32, "SuperSpeed Device Descriptor Word Offset");
	printreg!(img, 0x33, "SuperSpeed Configuration Block Length (bytes)");
	printreg!(img, 0x34, "SuperSpeed Configuration Block Word Offset");
	printreg!(img, 0x35, "High-Speed Device Descriptor Length (bytes)");
	printreg!(img, 0x36, "High-Speed Device Descriptor Word Offset");
	printreg!(img, 0x37, "High-Speed Configuration and Interface Descriptor Length (bytes)");
	printreg!(img, 0x38, "High-Speed Configuration and Interface Descriptor Word Offset");
	printreg!(img, 0x39, "Full-Speed Device Descriptor Length (bytes)");
	printreg!(img, 0x3a, "Full-Speed Device Descriptor Word Offset");
	printreg!(img, 0x3b, "Full-Speed Configuration and Interface Descriptor Length (bytes)");
	printreg!(img, 0x3c, "Full-Speed Configuration and Interface Descriptor Word Offset");
	printreg!(img, 0x3d, "Wake Frame Filter 0 Configuration and Mask Length (bytes)");
	printreg!(img, 0x3e, "Wake Frame Filter 0 Configuration and Mask Word Offset");
	printreg!(img, 0x3f, "LTM BELT and Inactivity Timer Length (bytes)");
	printreg!(img, 0x40, "LTM BELT and Inactivity Timer Word Offset");
	printreg!(img, 0x41, "Common Test Bus In Length (bytes)");
	printreg!(img, 0x42, "Common Test Bus In Word Offset (bytes)");
	printreg!(img, 0x43, "RESERVED (0x20)");
	printreg!(img, 0x44, "RESERVED (0x00)");
	printreg!(img, 0x46, "SW Descriptor Length (bytes)");
	printreg!(img, 0x47, "SW Descriptor Word Offset (bytes)");
	printreg!(img, 0x48, "GPIO Configuration");
	printreg!(img, 0x49, "GPIO Configuration");
	printreg!(img, 0x4a, "GPIO Configuration");
	printreg!(img, 0x4b, "GPIO Configuration");
	printreg!(img, 0x4c, "GPIO Configuration");
	printreg!(img, 0x4d, "GPIO Configuration");
	printreg!(img, 0x4e, "GPIO Configuration");
	printreg!(img, 0x4f, "GPIO Configuration");
	printreg!(img, 0x50, "RESERVED (0x00)");
	printreg!(img, 0x51, "RESERVED (0x00)");
	printreg!(img, 0x52, "RESERVED (0x00)");
	printreg!(img, 0x53, "RESERVED (0x00)");
	printreg!(img, 0x54, "RESERVED (0x00)");
	printreg!(img, 0x55, "RESERVED (0x00)");
	printreg!(img, 0x56, "RESERVED (0x00)");
	printreg!(img, 0x57, "RESERVED (0x00)");
	printreg!(img, 0x58, "LED Configuration 3");
	printreg!(img, 0x59, "LED Configuration 4");
	printreg!(img, 0x5a, "RESERVED (0x??)");
	printreg!(img, 0x5b, "RESERVED (0x??)");
	printreg!(img, 0x5c, "RESERVED (0x??)");
	printreg!(img, 0x5d, "RESERVED (0x??)");
	printreg!(img, 0x5e, "RESERVED (0x??)");
	printreg!(img, 0x5f, "RESERVED (0x??)");
	printreg!(img, 0x61, "RESERVED (0x??)");
}
