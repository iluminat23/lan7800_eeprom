use std::fs;
use std::path::Path;
use std::env;
use std::process::exit;

use pretty_hex::*;

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

macro_rules! printbit {
	($val:expr, $bit:expr) => {
		println!("\t{}: {}", $bit, ($val >> $bit) & 1);
	};
	($val:expr, $bit:expr, $desc:expr) => {
		println!("\t{}: {}: {}", $bit, $desc, ($val >> $bit) & 1);
	};
	($val:expr, $bit:expr, $($desc:tt)*) => {
		println!("\t{}: {}: {}", $bit, format!($($desc)*), ($val >> $bit) & 1);
	};
}
fn print_led_control(val: u8) {
	match val & 0b1111 {
		0b0000 => println!("\t\tLink/Activity (default for LED2): b0000"),
		0b0001 => println!("\t\tLink1000/Activity (default for LED0): b0001"),
		0b0010 => println!("\t\tLink100/Activity (default for LED1): b0010"),
		0b0011 => println!("\t\tLink10/Activity: b0011"),
		0b0100 => println!("\t\tLink100/1000/Activity: b0100"),
		0b0101 => println!("\t\tLink10/1000/Activity: b0101"),
		0b0110 => println!("\t\tLink10/100/Activity: b0110"),
		0b1000 => println!("\t\tDuplex/Collision (default for LED3): b1000"),
		0b1001 => println!("\t\tCollision: b1001"),
		0b1010 => println!("\t\tActivity: b1010"),
		0b1100 => println!("\t\tAuto-negotiation Fault: b1100"),
		0b1110 => println!("\t\tForce LED Off (suppresses LED blink after reset/coma): b1110"),
		0b1111 => println!("\t\tForce LED On (suppresses LED blink after reset/coma): b1111"),
		_ => println!("\t\tRESERVED: b{:04b}", val & 0b1111)
	}
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

	println!("\n0x01 - 0x06 MAC Address:");
	println!("\t{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
	         img[0x01], img[0x02], img[0x03], img[0x04], img[0x05], img[0x06]);

	println!("\n0x07 GPIO Wake:");
	for i in 0..8 {
		printbit!(img[0x07], 7-i, "GPIOWK{}", 7-i);
	}

	println!("\n0x09 GPIO PME Flags 0: 0x{:02X}", img[0x09]);
	printbit!(img[0x09], 7, "GPIO PME Enable");
	printbit!(img[0x09], 6, "GPIO PME Configuration");
	printbit!(img[0x09], 5, "GPIO PME Length");
	printbit!(img[0x09], 4, "GPIO PME Polarity");
	printbit!(img[0x09], 3, "GPIO PME Buffer Type");
	printbit!(img[0x09], 2, "PHY Link Change Enable");
	printbit!(img[0x09], 1, "PME Packet Enable");
	printbit!(img[0x09], 0, "PME Perfect DA Enable");

	println!("\n0x0A GPIO PME Flags 1: 0x{:02X}", img[0x0a]);
	println!("\t7-2: RESERVED: b{:06b}", img[0x0a] >> 2);
	printbit!(img[0x0a], 1, "PME Broadcast Packet Enabl");
	printbit!(img[0x0a], 0, "PME WUFF Enable");

	println!("\n0x0B LED Configuration 0: 0x{:02X}", img[0x0b]);
	printbit!(img[0x0b], 3, "LED3 Enable (LED3_EN)");
	printbit!(img[0x0b], 2, "LED2 Enable (LED2_EN)");
	printbit!(img[0x0b], 1, "LED1 Enable (LED1_EN)");
	printbit!(img[0x0b], 0, "LED0 Enable (LED0_EN)");

	println!("\n0x0B LED Configuration 1: 0x{:02X}", img[0x0c]);
	println!("\tLED1 Control: b{:04b}", (img[0x0c] >> 4) & 0b1111);
	print_led_control((img[0x0c] >> 4) & 0b1111);
	println!("\tLED0 Control: b{:04b}", img[0x0c] & 0b1111);
	print_led_control(img[0x0c] & 0b1111);

	println!("\n0x0D LED Configuration 2: 0x{:02X}", img[0x0d]);
	println!("\tLED3 Control: b{:04b}", (img[0x0d] >> 4) & 0b1111);
	print_led_control((img[0x0d] >> 4) & 0b1111);
	println!("\tLED2 Control: b{:04b}", img[0x0d] & 0b1111);
	print_led_control(img[0x0d] & 0b1111);


	println!("\n0x25 Manufacturer ID String Descriptor Length (bytes): 0x{:02X}", img[0x25]);
	println!("\n0x26 Manufacturer ID String Descriptor EEPROM Word Offset: 0x{:02X}", img[0x26]);
	simple_hex(&img);
	let offset: usize = (img[0x26] * 2).into();
	let size: usize = img[0x25].into();
	if img[0x25] != img[offset] {
		eprintln!("\tERROR: Length @0x{:02X}=0x{:02X} and bLength @0x{:02X}=0x{:02X} differ!",
		          0x25, img[0x25], offset, img[offset]);
	}
	let manu = &img[(offset + 2)..(offset + size)];
	println!("\t{}", manu.hex_dump());
	if img[offset + 1] == 0x03 {
		let x = String::from_utf8_lossy(manu);
		println!("\t'{}'", x);
	}
}
