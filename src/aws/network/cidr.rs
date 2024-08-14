use std::fmt;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    address: Ipv4Addr,
    prefix_length: u8,
}

impl Block {
    pub fn new(address: Ipv4Addr, prefix_length: u8) -> Result<Self, String> {
        if prefix_length > 32 {
            return Err("Prefix length must be between 0 and 32".to_string());
        }
        Ok(Block {
            address,
            prefix_length,
        })
    }

    pub fn address(&self) -> Ipv4Addr {
        self.address
    }

    pub fn prefix_length(&self) -> u8 {
        self.prefix_length
    }

    pub fn network_address(&self) -> Ipv4Addr {
        if self.prefix_length == 0 {
            Ipv4Addr::new(0, 0, 0, 0)
        } else {
            let mask = u32::MAX
                .checked_shl(32 - u32::from(self.prefix_length))
                .unwrap_or(0);
            Ipv4Addr::from(u32::from(self.address) & mask)
        }
    }

    pub fn broadcast_address(&self) -> Ipv4Addr {
        if self.prefix_length == 32 {
            self.address
        } else {
            let mask = u32::MAX.checked_shr(self.prefix_length as u32).unwrap_or(0);
            Ipv4Addr::from(u32::from(self.address) | mask)
        }
    }

    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        let network = u32::from(self.network_address());
        let broadcast = u32::from(self.broadcast_address());
        let ip = u32::from(ip);
        ip >= network && ip <= broadcast
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.address, self.prefix_length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidr_block() {
        let cidr = Block::new(Ipv4Addr::from([192, 168, 1, 0]), 24).unwrap();
        assert_eq!(cidr.network_address(), Ipv4Addr::from([192, 168, 1, 0]));
    }

    #[test]
    fn test_cidr_block_contains() {
        let cidr = Block::new(Ipv4Addr::from([192, 168, 1, 0]), 24).unwrap();
        assert!(cidr.contains(Ipv4Addr::from([192, 168, 1, 1])));
        assert!(cidr.contains(Ipv4Addr::from([192, 168, 1, 255])));
        assert!(cidr.contains(Ipv4Addr::from([192, 168, 1, 254])));
    }

    #[test]
    fn test_cidr_block_to_string() {
        let cidr = Block::new(Ipv4Addr::from([10, 0, 0, 0]), 16).unwrap();
        assert_eq!(cidr.to_string(), "10.0.0.0/16");
    }

    #[test]
    fn test_cidr_block_new_valid() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 0, 0), 16);
        assert!(cidr.is_ok());
    }

    #[test]
    fn test_cidr_block_new_invalid_prefix() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 0, 0), 33);
        assert!(cidr.is_err());
    }

    #[test]
    fn test_cidr_block_broadcast_address() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 0, 0), 24).unwrap();
        assert_eq!(cidr.broadcast_address(), Ipv4Addr::new(192, 168, 0, 255));
    }

    #[test]
    fn test_cidr_block_network_address() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 1, 100), 24).unwrap();
        assert_eq!(cidr.network_address(), Ipv4Addr::new(192, 168, 1, 0));
    }

    #[test]
    fn test_cidr_block_contains_network_address() {
        let cidr = Block::new(Ipv4Addr::new(10, 0, 0, 0), 8).unwrap();
        assert!(cidr.contains(Ipv4Addr::new(10, 0, 0, 0)));
    }

    #[test]
    fn test_cidr_block_contains_broadcast_address() {
        let cidr = Block::new(Ipv4Addr::new(172, 16, 0, 0), 16).unwrap();
        assert!(cidr.contains(Ipv4Addr::new(172, 16, 255, 255)));
    }

    #[test]
    fn test_cidr_block_does_not_contain_outside_ip() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 0, 0), 24).unwrap();
        assert!(!cidr.contains(Ipv4Addr::new(192, 168, 1, 1)));
    }

    #[test]
    fn test_cidr_block_to_string_class_a() {
        let cidr = Block::new(Ipv4Addr::new(10, 0, 0, 0), 8).unwrap();
        assert_eq!(cidr.to_string(), "10.0.0.0/8");
    }

    #[test]
    fn test_cidr_block_to_string_class_b() {
        let cidr = Block::new(Ipv4Addr::new(172, 16, 0, 0), 16).unwrap();
        assert_eq!(cidr.to_string(), "172.16.0.0/16");
    }

    #[test]
    fn test_cidr_block_to_string_class_c() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 0, 0), 24).unwrap();
        assert_eq!(cidr.to_string(), "192.168.0.0/24");
    }

    #[test]
    fn test_cidr_block_contains_edge_cases() {
        let cidr = Block::new(Ipv4Addr::new(10, 0, 0, 0), 31).unwrap();
        assert!(cidr.contains(Ipv4Addr::new(10, 0, 0, 0)));
        assert!(cidr.contains(Ipv4Addr::new(10, 0, 0, 1)));
        assert!(!cidr.contains(Ipv4Addr::new(10, 0, 0, 2)));
    }

    #[test]
    fn test_cidr_block_new_with_host_bits_set() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 1, 100), 24).unwrap();
        assert_eq!(cidr.address(), Ipv4Addr::new(192, 168, 1, 100));
    }

    #[test]
    fn test_cidr_block_contains_entire_range() {
        let cidr = Block::new(Ipv4Addr::new(10, 0, 0, 0), 8).unwrap();
        assert!(cidr.contains(Ipv4Addr::new(10, 0, 0, 0)));
        assert!(cidr.contains(Ipv4Addr::new(10, 128, 0, 0)));
        assert!(cidr.contains(Ipv4Addr::new(10, 255, 255, 255)));
        assert!(!cidr.contains(Ipv4Addr::new(11, 0, 0, 0)));
    }

    #[test]
    fn test_cidr_block_smallest_prefix() {
        let cidr = Block::new(Ipv4Addr::new(0, 0, 0, 0), 0).unwrap();
        // For prefix 0, all IPs are valid
        assert_eq!(cidr.network_address(), Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(cidr.broadcast_address(), Ipv4Addr::new(255, 255, 255, 255));
        assert!(cidr.contains(Ipv4Addr::new(0, 0, 0, 0)));
        assert!(cidr.contains(Ipv4Addr::new(128, 0, 0, 0)));
        assert!(cidr.contains(Ipv4Addr::new(255, 255, 255, 255)));
    }

    #[test]
    fn test_cidr_block_largest_prefix() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 0, 1), 32).unwrap();
        // For prefix 32, only the exact IP is valid
        assert_eq!(cidr.network_address(), Ipv4Addr::new(192, 168, 0, 1));
        assert_eq!(cidr.broadcast_address(), Ipv4Addr::new(192, 168, 0, 1));
        assert!(cidr.contains(Ipv4Addr::new(192, 168, 0, 1)));
        assert!(!cidr.contains(Ipv4Addr::new(192, 168, 0, 0)));
        assert!(!cidr.contains(Ipv4Addr::new(192, 168, 0, 2)));
    }

    #[test]
    fn test_cidr_block_network_and_broadcast_for_various_prefixes() {
        let test_cases = vec![
            (
                Ipv4Addr::new(192, 168, 0, 0),
                16,
                Ipv4Addr::new(192, 168, 0, 0),
                Ipv4Addr::new(192, 168, 255, 255),
            ),
            (
                Ipv4Addr::new(10, 0, 0, 0),
                8,
                Ipv4Addr::new(10, 0, 0, 0),
                Ipv4Addr::new(10, 255, 255, 255),
            ),
            (
                Ipv4Addr::new(172, 16, 0, 0),
                12,
                Ipv4Addr::new(172, 16, 0, 0),
                Ipv4Addr::new(172, 31, 255, 255),
            ),
        ];

        for (addr, prefix, expected_network, expected_broadcast) in test_cases {
            let cidr = Block::new(addr, prefix).unwrap();
            assert_eq!(cidr.network_address(), expected_network);
            assert_eq!(cidr.broadcast_address(), expected_broadcast);
        }
    }

    #[test]
    fn test_cidr_block_contains_for_various_prefixes() {
        let test_cases = vec![
            (
                Ipv4Addr::new(192, 168, 0, 0),
                16,
                Ipv4Addr::new(192, 168, 100, 100),
                true,
            ),
            (
                Ipv4Addr::new(10, 0, 0, 0),
                8,
                Ipv4Addr::new(10, 255, 255, 255),
                true,
            ),
            (
                Ipv4Addr::new(172, 16, 0, 0),
                12,
                Ipv4Addr::new(172, 31, 255, 255),
                true,
            ),
            (
                Ipv4Addr::new(192, 168, 0, 0),
                16,
                Ipv4Addr::new(192, 169, 0, 0),
                false,
            ),
        ];

        for (addr, prefix, test_ip, expected) in test_cases {
            let cidr = Block::new(addr, prefix).unwrap();
            assert_eq!(cidr.contains(test_ip), expected);
        }
    }

    #[test]
    fn test_cidr_block_new_with_different_addresses() {
        let addresses = vec![
            Ipv4Addr::new(0, 0, 0, 0),
            Ipv4Addr::new(127, 0, 0, 1),
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(255, 255, 255, 255),
        ];

        for addr in addresses {
            let cidr = Block::new(addr, 24);
            assert!(cidr.is_ok());
        }
    }

    #[test]
    fn test_cidr_block_display_trait() {
        let cidr = Block::new(Ipv4Addr::new(192, 168, 0, 0), 16).unwrap();
        assert_eq!(format!("{}", cidr), "192.168.0.0/16");
    }
}
