# nexus7: Rust SDK for HCL Transpilation

## Overview

nexus7 is a Rust SDK designed to transpile Rust code into HashiCorp Configuration Language (HCL). This project enables developers to write their infrastructure code in Rust while still leveraging Terraform or OpenTofu as backends for infrastructure management.

## Features

- Idiomatic Rust approach with struct and type-driven design
- Rust to HCL transpilation using strongly-typed structs
- Leverages Rust's trait system for extensible and composable infrastructure definitions
- Compile-time validation of infrastructure configurations
- Compatible with Terraform and OpenTofu backends
- Modular architecture allowing easy addition of new providers and resource types

## Scope and Current Limitations

nexus7 was initially developed as a way to build infrastructure for [pocketsizefund](https://github.com/pocketsizefund/pocketsizefund). As such, in its early stages, it focuses primarily on the infrastructure components used by pocketsizefund. However, our long-term goal is to expand its capabilities to cover a broader range of infrastructure needs.

Current limitations:
- Only supports AWS as the cloud provider
- Limited to EC2 instances for compute resources
- Restricted to t2.micro instance type

While these limitations reflect our current focus, we are committed to growing nexus7's capabilities over time to support a wider array of cloud providers, resource types, and configuration options.

## Getting Started

### Prerequisites

- Rust 1.54.0 or later
- Cargo package manager
- Terraform or OpenTofu (for applying the generated HCL)

### Installation

1. Add nexus7 to your Cargo.toml:
   ```toml
   [dependencies]
   nexus7 = "0.1.0"
   ```

2. Run cargo build to fetch and compile the SDK:
   ```
   cargo build
   ```

## Usage

1. Write your infrastructure code in Rust using the nexus7 SDK.
2. Use the SDK to transpile your Rust code to HCL.
3. Apply the generated HCL using Terraform or OpenTofu.

Example:

```rust
use nexus7::{Resource, Provider};

// Define a resource
let resource = Resource::new("aws_instance", "example")
    .with_provider(Provider::AWS)
    .with_attribute("ami", "ami-0c94855ba95c71c99")
    .with_attribute("instance_type", "t2.micro");

// Transpile the resource to HCL
let hcl = resource.to_hcl();

// Apply the HCL using Terraform or OpenTofu
// ...
```

## Contributing

We welcome contributions to the nexus7 project! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to submit pull requests, report issues, and suggest improvements.

## Testing

To run the test suite:

```
cargo test
```

## Documentation

For detailed documentation, please refer to the `docs/` directory or visit our [online documentation](https://nexus7-docs.example.com).

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For questions, suggestions, or collaboration opportunities, please contact the project maintainers at [contact@nexus7.example.com](mailto:contact@nexus7.example.com).

## Acknowledgments

[List any individuals, organizations, or resources that have contributed to or inspired the project]