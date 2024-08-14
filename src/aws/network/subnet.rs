use crate::aws::availability_zone::AvailabilityZone;
use crate::aws::network::{cidr, vpc::Vpc};
use hcl::{Block, Expression, ObjectKey};
use std::collections::HashMap;

/// Represents an AWS Subnet resource.
#[derive(Debug, Clone)]
pub struct Subnet<'a> {
    /// The name of the subnet.
    pub name: String,

    /// The VPC in which the subnet is located.
    pub vpc: &'a Vpc,

    /// The IPv4 CIDR block for the subnet.
    pub cidr_block: cidr::Block,

    /// The AZ for the subnet.
    pub availability_zone: Option<AvailabilityZone>,

    /// Specify true to indicate that network interfaces created in the specified subnet should be assigned an IPv6 address.
    pub assign_ipv6_address_on_creation: Option<bool>,

    /// The IPv6 network range for the subnet, in CIDR notation.
    pub ipv6_cidr_block: Option<String>,

    /// Specify true to indicate that instances launched into the subnet should be assigned a public IP address.
    pub map_public_ip_on_launch: Option<bool>,

    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,
}

impl<'a> From<Subnet<'a>> for Block {
    fn from(subnet: Subnet<'a>) -> Self {
        let mut block = Block::builder("resource")
            .add_label("aws_subnet")
            .add_label(&subnet.name)
            .add_attribute((
                "vpc_id",
                Expression::from(format!("${{{}.id}}", subnet.vpc.resource_name())),
            ))
            .add_attribute((
                "cidr_block",
                Expression::String(subnet.cidr_block.to_string()),
            ));

        if let Some(az) = subnet.availability_zone {
            block = block.add_attribute(("availability_zone", Expression::String(az.to_string())));
        }

        if let Some(assign_ipv6) = subnet.assign_ipv6_address_on_creation {
            block = block.add_attribute((
                "assign_ipv6_address_on_creation",
                Expression::Bool(assign_ipv6),
            ));
        }

        if let Some(ipv6_cidr) = subnet.ipv6_cidr_block {
            block = block.add_attribute(("ipv6_cidr_block", Expression::String(ipv6_cidr)));
        }

        if let Some(map_public_ip) = subnet.map_public_ip_on_launch {
            block =
                block.add_attribute(("map_public_ip_on_launch", Expression::Bool(map_public_ip)));
        }

        if let Some(tags) = subnet.tags {
            let tags_expr = Expression::Object(
                tags.into_iter()
                    .map(|(k, v)| (ObjectKey::from(k), Expression::String(v)))
                    .collect(),
            );
            block = block.add_attribute(("tags", tags_expr));
        }

        block.build()
    }
}
