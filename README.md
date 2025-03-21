![rustc](https://img.shields.io/badge/rustc-1.85.0-blue.svg)
[![codecov](https://codecov.io/gh/averageeucplayer/lost-metrics-sniffer/graph/badge.svg?token=HHRGYYUNM2)](https://codecov.io/gh/averageeucplayer/lost-metrics-sniffer)
![CI](https://github.com/averageeucplayer/lost-metrics-sniffer/actions/workflows/ci.yml/badge.svg)

# 🔮 Lost Metrics Sniffer

## 🏗️ Architecture

The Lost Metrics Sniffer project is structured as a Cargo workspace, consisting of four main projects:

### **windivert_sniffer**

This DLL library is responsible for connecting to and listening for network packets using the WindDivert API.

While it currently handles the packet capture part of the process (i.e., it connects to the network interface and listens for incoming packets), it is not fully implemented.

The library still requires the logic to parse the raw byte data of the captured packets, convert them into tagged unions, and send them through an MPSC channel for further processing.

### **fake_sniffer**

It serves as a mock implementation of the packet sniffing service, mainly used for testing purposes.

It implements the same ABI stable interface as **windivert_sniffer**, but instead of processing real network packets, it simulates the behavior.

This enables the testing of the system without relying on the actual packet capture functionality, providing a way to validate that the other parts of the system are functioning as expected.

### **shared**

This project defines the ABI stable interface that must be implemented by DLL.

The interface includes the data structures, traits, and service methods required for the interaction between different parts of the system.

### **test-client**

This project serves as a testing framework that interacts with the ABI interface exposed by the DLL.

## 📦 Installation & Setup

### 1️⃣ **Clone the Repository**

```sh
git clone https://github.com/averageeucplayer/lost-metrics-sniffer.git
```

### 2️⃣ Add to Cargo.toml

```toml
[dependencies]
lost-metrics-sniffer = { git = "https://github.com/averageeucplayer/lost-metrics-sniffer" }
```

### 3️⃣ Build DLL and copy it to the consuming crate

```sh
cargo build --workspace
```

```rust
let src = "../lost-metrics-sniffer/target/debug/fake_sniffer.dll";
let current_executable = std::env::current_exe().unwrap();
let dest = current_executable.parent().unwrap().to_path_buf().join("fake_sniffer.dll");
fs::copy(src, dest).unwrap();
```

### 4️⃣ Use service

```rust
use lost_metrics_sniffer::PacketSnifferServiceWrapper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use either fake or windivert sniffer
    let mut service = PacketSnifferServiceWrapper::fake()?;
    // let mut service = PacketSnifferServiceWrapper::windivert()?;
    
    let port = 80;
    let rx = service.start(port)?;

    while let Ok(packet) = rx.recv() {
        println!("Received: {:?}", packet);
    }

    Ok(())
}
```