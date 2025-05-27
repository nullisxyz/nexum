# nexum-apdu: Smart Card APDU in Rust

nexum-apdu is a comprehensive toolkit for smart card communication in Rust using the APDU (Application Protocol Data Unit) protocol. Whether you're building a payment terminal, identity verification system, or HSM integration, nexum-apdu provides the foundation for secure card communications.

[![docs.rs](https://img.shields.io/docsrs/nexum-apdu-core/latest)](https://docs.rs/nexum-apdu-core)
[![Crates.io](https://img.shields.io/crates/v/nexum-apdu-core)](https://crates.io/crates/nexum-apdu-core)

Unlock the cryptographic conversations that smart cards are dying to have with nexum-apdu as your trusted interpreter.

## Installation

The easiest way to get started is to add the core crate:

```sh
cargo add nexum-apdu-core
```

For PC/SC reader support, add:

```sh
cargo add nexum-apdu-transport-pcsc
```

For GlobalPlatform card management:

```sh
cargo add nexum-apdu-globalplatform
```

For APDU command/response pair macro support:

```sh
cargo add nexum-apdu-macros
```

## Quick Start

```rust
use nexum_apdu_core::prelude::*;
use nexum_apdu_transport_pcsc::{PcscDeviceManager, PcscConfig};

fn main() -> Result<(), Error> {
    // Create a transport
    let manager = PcscDeviceManager::new()?;
    let readers = manager.list_readers()?;
    let reader = readers.iter().find(|r| r.has_card()).expect("No card present");
    let transport = manager.open_reader(reader.name())?;

    // Create an executor with default processors (GET RESPONSE handler)
    let mut executor = CardExecutor::new_with_defaults(transport);

    // Create a SELECT command to select a payment application
    let aid = [0xA0, 0x00, 0x00, 0x00, 0x03, 0x10, 0x10];
    let select_cmd = Command::new_with_data(0x00, 0xA4, 0x04, 0x00, aid.to_vec());

    // Execute the command
    let response = executor.transmit(&select_cmd)?;

    // Parse the response
    if response.is_success() {
        println!("Application selected successfully");
        if let Some(data) = response.payload() {
            println!("FCI: {}", hex::encode_upper(data));
        }
    } else {
        println!("Failed to select application: {}", response.status());
    }

    Ok(())
}
```

## Overview

This repository contains the following crates:

- [`nexum-apdu-core`]: Core traits and types for APDU operations
- [`nexum-apdu-macros`]: Procedural macros for defining APDU commands and responses
- [`nexum-apdu-transport-pcsc`]: PC/SC transport implementation for card readers
- [`nexum-apdu-globalplatform`]: GlobalPlatform card management functionality

[`nexum-apdu-core`]: https://github.com/nullisxyz/apdu/tree/main/crates/core
[`nexum-apdu-macros`]: https://github.com/nullisxyz/apdu/tree/main/crates/macros
[`nexum-apdu-transport-pcsc`]: https://github.com/nullisxyz/apdu/tree/main/crates/pcsc
[`nexum-apdu-globalplatform`]: https://github.com/nullisxyz/apdu/tree/main/crates/globalplatform

## Features

- 🎯 **Abstracted transport layer** supporting different card reader types
- 🛡️ **Secure channel support** for GlobalPlatform SCP02 protocol
- 🧩 **Modular architecture** allowing use of only what you need
- 📦 **Command processor pipeline** for flexible transformations
- 📝 **Declarative command definitions** with the `apdu_pair!` macro
- 🔄 **Response chaining support** for handling complex responses
- 🧰 **Comprehensive prelude** for streamlined imports

## Documentation & Examples

For detailed documentation on each crate, please check their individual README files:

- [nexum-apdu-core README](./crates/core/README.md) - Core APDU abstractions and types
- [nexum-apdu-macros README](./crates/macros/README.md) - Procedural macros for command/response definition
- [nexum-apdu-transport-pcsc README](./crates/pcsc/README.md) - PC/SC transport implementation
- [nexum-apdu-globalplatform README](./crates/globalplatform/README.md) - GlobalPlatform operations

### Example Applications

Check out these examples to see nexum-apdu in action:

- [Connect to a reader](./crates/pcsc/examples/connect.rs) - Basic card connection and communication
- [List available readers](./crates/pcsc/examples/list_readers.rs) - Enumerate connected card readers
- [APDU shell](./crates/pcsc/examples/apdu_shell.rs) - Interactive APDU command interpreter
- [Monitor card events](./crates/pcsc/examples/monitor_events.rs) - Track card insertion/removal events
- [Select an application by AID](./crates/pcsc/examples/select_aid.rs) - Select applications using their AID
- [Install a CAP file](./crates/globalplatform/examples/install_cap.rs) - Install Java Card applications

For more examples, see the `examples` directory in each crate.

## Architecture

nexum-apdu is built around three main architectural layers:

### Transport Layer

The `CardTransport` trait handles the low-level communication with cards, providing a clean abstraction over different physical transport mechanisms.

### Command Processor Layer

Command processors can transform, secure, or log APDU commands before they reach the transport layer, allowing for modular protocol implementations like secure channels.

### Executor Layer

Executors manage the complete command execution flow, combining transports and processors to provide a high-level interface for applications.

## License

Licensed under the [AGPL License](LICENSE) or http://www.gnu.org/licenses/agpl-3.0.html.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in these crates by you shall be licensed as above, without any additional terms or conditions.
