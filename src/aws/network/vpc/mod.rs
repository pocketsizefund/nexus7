use crate::aws::network::cidr;
use hcl::{Block, Expression, ObjectKey};
use std::collections::HashMap;

/// Represents an AWS VPC resource.
#[derive(Debug, Clone)]
pub struct Vpc {
    /// The name of the VPC.
    pub name: String,

    /// The IPv4 CIDR block for the VPC.
    pub cidr_block: cidr::Block,

    /// A tenancy option for instances launched into the VPC.
    pub instance_tenancy: Option<String>,

    /// A boolean flag to enable/disable DNS hostnames in the VPC.
    pub enable_dns_hostnames: Option<bool>,

    /// A boolean flag to enable/disable DNS support in the VPC.
    pub enable_dns_support: Option<bool>,

    /// A boolean flag to enable/disable ClassicLink for the VPC.
    pub enable_classiclink: Option<bool>,

    /// A boolean flag to enable/disable ClassicLink DNS Support for the VPC.
    pub enable_classiclink_dns_support: Option<bool>,

    /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC.
    pub assign_generated_ipv6_cidr_block: Option<bool>,

    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,
}

impl Vpc {
    /// Method to get the HCL resource name
    pub fn resource_name(&self) -> String {
        format!("aws_vpc.{}", self.name)
    }
}

impl From<Vpc> for Block {
    fn from(vpc: Vpc) -> Self {
        let mut block = Block::builder("resource")
            .add_label("aws_vpc")
            .add_label(&vpc.name)
            .add_attribute(("cidr_block", Expression::String(vpc.cidr_block.to_string())));

        if let Some(instance_tenancy) = vpc.instance_tenancy {
            block = block.add_attribute(("instance_tenancy", Expression::String(instance_tenancy)));
        }

        if let Some(enable_dns_hostnames) = vpc.enable_dns_hostnames {
            block = block.add_attribute((
                "enable_dns_hostnames",
                Expression::Bool(enable_dns_hostnames),
            ));
        }

        if let Some(enable_dns_support) = vpc.enable_dns_support {
            block =
                block.add_attribute(("enable_dns_support", Expression::Bool(enable_dns_support)));
        }

        if let Some(enable_classiclink) = vpc.enable_classiclink {
            block =
                block.add_attribute(("enable_classiclink", Expression::Bool(enable_classiclink)));
        }

        if let Some(enable_classiclink_dns_support) = vpc.enable_classiclink_dns_support {
            block = block.add_attribute((
                "enable_classiclink_dns_support",
                Expression::Bool(enable_classiclink_dns_support),
            ));
        }

        if let Some(assign_generated_ipv6_cidr_block) = vpc.assign_generated_ipv6_cidr_block {
            block = block.add_attribute((
                "assign_generated_ipv6_cidr_block",
                Expression::Bool(assign_generated_ipv6_cidr_block),
            ));
        }

        if let Some(tags) = vpc.tags {
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

/// Represents a data source for an AWS VPC.
#[derive(Debug, Clone)]
pub struct VpcDataSource {
    /// The ID of the VPC.
    pub id: Option<String>,

    /// The IPv4 CIDR block for the VPC.
    pub cidr_block: Option<String>,

    /// The ID of the AWS account that owns the VPC.
    pub owner_id: Option<String>,

    /// A boolean flag to enable/disable DNS hostnames in the VPC.
    pub enable_dns_hostnames: Option<bool>,

    /// A boolean flag to enable/disable DNS support in the VPC.
    pub enable_dns_support: Option<bool>,

    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,

    /// One or more name-value pairs to filter by.
    pub filter: Option<Vec<Filter>>,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub values: Vec<String>,
}

impl From<VpcDataSource> for Block {
    fn from(data_source: VpcDataSource) -> Self {
        let mut block = Block::builder("data")
            .add_label("aws_vpc")
            .add_label(data_source.id.as_deref().unwrap_or("vpc"));

        if let Some(id) = data_source.id {
            block = block.add_attribute(("id", Expression::String(id)));
        }

        if let Some(cidr_block) = data_source.cidr_block {
            block = block.add_attribute(("cidr_block", Expression::String(cidr_block)));
        }

        if let Some(owner_id) = data_source.owner_id {
            block = block.add_attribute(("owner_id", Expression::String(owner_id)));
        }

        if let Some(enable_dns_hostnames) = data_source.enable_dns_hostnames {
            block = block.add_attribute((
                "enable_dns_hostnames",
                Expression::Bool(enable_dns_hostnames),
            ));
        }

        if let Some(enable_dns_support) = data_source.enable_dns_support {
            block =
                block.add_attribute(("enable_dns_support", Expression::Bool(enable_dns_support)));
        }

        if let Some(tags) = data_source.tags {
            let tags_expr = Expression::Object(
                tags.into_iter()
                    .map(|(k, v)| (ObjectKey::from(k), Expression::String(v)))
                    .collect(),
            );
            block = block.add_attribute(("tags", tags_expr));
        }

        if let Some(filters) = data_source.filter {
            let filter_blocks: Vec<Block> = filters
                .into_iter()
                .map(|f| {
                    Block::builder("filter")
                        .add_attribute(("name", Expression::String(f.name)))
                        .add_attribute((
                            "values",
                            Expression::Array(
                                f.values.into_iter().map(Expression::String).collect(),
                            ),
                        ))
                        .build()
                })
                .collect();
            block = block.add_blocks(filter_blocks);
        }

        block.build()
    }
}

/// Represents an AWS Elastic IP resource.
#[derive(Debug, Clone)]
pub struct ElasticIp {
    /// The name of the Elastic IP.
    pub name: String,
    /// EC2-VPC or EC2-Classic.
    pub domain: Option<String>,
    /// Instance ID to associate with.
    pub instance: Option<String>,
    /// Network interface ID to associate with.
    pub network_interface: Option<String>,
    /// IP address from an EC2 BYOIP pool.
    pub public_ipv4_pool: Option<String>,
    /// Customer owned IPv4 address pool.
    pub customer_owned_ipv4_pool: Option<String>,
    /// Private IP address to associate with the Elastic IP address.
    pub associate_with_private_ip: Option<String>,
    /// Address of a VPC endpoint or NAT gateway.
    pub address: Option<String>,
    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,
}

impl From<ElasticIp> for Block {
    fn from(eip: ElasticIp) -> Self {
        let mut block = Block::builder("resource")
            .add_label("aws_eip")
            .add_label(&eip.name);

        if let Some(domain) = eip.domain {
            block = block.add_attribute(("domain", Expression::String(domain)));
        }

        if let Some(instance) = eip.instance {
            block = block.add_attribute(("instance", Expression::String(instance)));
        }

        if let Some(network_interface) = eip.network_interface {
            block =
                block.add_attribute(("network_interface", Expression::String(network_interface)));
        }

        if let Some(public_ipv4_pool) = eip.public_ipv4_pool {
            block = block.add_attribute(("public_ipv4_pool", Expression::String(public_ipv4_pool)));
        }

        if let Some(customer_owned_ipv4_pool) = eip.customer_owned_ipv4_pool {
            block = block.add_attribute((
                "customer_owned_ipv4_pool",
                Expression::String(customer_owned_ipv4_pool),
            ));
        }

        if let Some(associate_with_private_ip) = eip.associate_with_private_ip {
            block = block.add_attribute((
                "associate_with_private_ip",
                Expression::String(associate_with_private_ip),
            ));
        }

        if let Some(address) = eip.address {
            block = block.add_attribute(("address", Expression::String(address)));
        }

        if let Some(tags) = eip.tags {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vpc_to_hcl() {
        let vpc = Vpc {
            name: "main".to_string(),
            cidr_block: "10.0.0.0/16".to_string(),
            instance_tenancy: Some("default".to_string()),
            enable_dns_hostnames: Some(true),
            enable_dns_support: Some(true),
            enable_classiclink: None,
            enable_classiclink_dns_support: None,
            assign_generated_ipv6_cidr_block: Some(false),
            tags: Some(HashMap::from([
                ("Name".to_string(), "Main VPC".to_string()),
                ("Environment".to_string(), "Production".to_string()),
            ])),
        };

        let block: Block = vpc.into();
        let hcl = hcl::to_string(&block).unwrap();

        assert!(hcl.contains("resource \"aws_vpc\" \"main\""));
        assert!(hcl.contains("cidr_block = \"10.0.0.0/16\""));
        assert!(hcl.contains("instance_tenancy = \"default\""));
        assert!(hcl.contains("enable_dns_hostnames = true"));
        assert!(hcl.contains("enable_dns_support = true"));
        assert!(hcl.contains("assign_generated_ipv6_cidr_block = false"));
        assert!(hcl.contains("tags = {"));
        assert!(hcl.contains("Name = \"Main VPC\""));
        assert!(hcl.contains("Environment = \"Production\""));
        assert!(hcl.contains("output \"id\" = aws_vpc.main.id"));
    }

    #[test]
    fn test_vpc_data_source_to_hcl() {
        let data_source = VpcDataSource {
            id: Some("vpc-12345".to_string()),
            cidr_block: Some("10.0.0.0/16".to_string()),
            owner_id: Some("123456789012".to_string()),
            enable_dns_hostnames: Some(true),
            enable_dns_support: Some(true),
            tags: Some(HashMap::from([(
                "Name".to_string(),
                "Main VPC".to_string(),
            )])),
            filter: Some(vec![Filter {
                name: "tag:Environment".to_string(),
                values: vec!["Production".to_string()],
            }]),
        };

        let block: Block = data_source.into();
        let hcl = hcl::to_string(&block).unwrap();

        assert!(hcl.contains("data \"aws_vpc\" \"vpc-12345\""));
        assert!(hcl.contains("id = \"vpc-12345\""));
        assert!(hcl.contains("cidr_block = \"10.0.0.0/16\""));
        assert!(hcl.contains("owner_id = \"123456789012\""));
        assert!(hcl.contains("enable_dns_hostnames = true"));
        assert!(hcl.contains("enable_dns_support = true"));
        assert!(hcl.contains("tags = {"));
        assert!(hcl.contains("Name = \"Main VPC\""));
        assert!(hcl.contains("filter {"));
        assert!(hcl.contains("name = \"tag:Environment\""));
        assert!(hcl.contains("values = [\"Production\"]"));
    }
}
