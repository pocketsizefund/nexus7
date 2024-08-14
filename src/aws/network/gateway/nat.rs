use crate::aws::network::subnet::Subnet;
use crate::aws::network::vpc::ElasticIp;
use crate::aws::network::vpc::Vpc;
use hcl::{Block, Expression, ObjectKey};
use std::collections::HashMap;

/// Represents an AWS NAT Gateway resource.
#[derive(Debug, Clone)]
pub struct NAT<'a> {
    /// The ID of the NAT Gateway.
    pub id: Option<String>,

    /// The VPC in which the NAT Gateway is located.
    pub vpc: &'a Vpc,

    /// The Subnet in which the NAT Gateway is located.
    pub subnet: &'a Subnet<'a>,

    /// The Elastic IP associated with the NAT Gateway.
    pub elastic_ip: &'a ElasticIp,

    /// The connectivity type for the NAT Gateway. Valid values are private and public.
    pub connectivity_type: Option<String>,

    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,

    /// A state of the NAT Gateway (pending | failed | available | deleting | deleted).
    pub state: Option<String>,
}

impl<'a> From<NAT<'a>> for Block {
    fn from(nat: NAT<'a>) -> Self {
        let mut block = Block::builder("resource")
            .add_label("aws_nat_gateway")
            .add_label(nat.id.as_deref().unwrap_or("nat"))
            .add_attribute((
                "subnet_id",
                Expression::from(format!("${{aws_subnet.{}.id}}", nat.subnet.name)),
            ))
            .add_attribute((
                "allocation_id",
                Expression::from(format!("${{aws_eip.{}.id}}", nat.elastic_ip.name)),
            ));

        match nat.connectivity_type {
            Some(connectivity_type) => {
                block = block
                    .add_attribute(("connectivity_type", Expression::String(connectivity_type)))
            }
            None => (),
        }

        match nat.state {
            Some(state) => block = block.add_attribute(("state", Expression::String(state))),
            None => (),
        }

        match nat.tags {
            Some(tags) => {
                let tags_expr = Expression::Object(
                    tags.into_iter()
                        .map(|(k, v)| (ObjectKey::from(k), Expression::String(v)))
                        .collect(),
                );
                block = block.add_attribute(("tags", tags_expr));
            }
            None => (),
        }

        block.build()
    }
}

/// Represents a data source for an AWS NAT Gateway.
#[derive(Debug, Clone)]
pub struct NATDataSource {
    /// One or more name-value pairs to filter by.
    pub filter: Option<Vec<Filter>>,

    /// The ID of the NAT Gateway.
    pub id: Option<String>,

    /// The state of the NAT Gateway (pending | failed | available | deleting | deleted).
    pub state: Option<String>,

    /// The ID of the subnet in which the NAT Gateway is placed.
    pub subnet_id: Option<String>,

    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,

    /// The ID of the VPC in which the NAT Gateway is located.
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub values: Vec<String>,
}

impl From<NATDataSource> for Block {
    fn from(data_source: NATDataSource) -> Self {
        let mut block = Block::builder("data")
            .add_label("aws_nat_gateway")
            .add_label(data_source.id.as_deref().unwrap_or("nat"));

        if let Some(id) = data_source.id {
            block = block.add_attribute(("id", Expression::String(id)));
        }

        if let Some(vpc_id) = data_source.vpc_id {
            block = block.add_attribute(("vpc_id", Expression::String(vpc_id)));
        }

        if let Some(state) = data_source.state {
            block = block.add_attribute(("state", Expression::String(state)));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_gateway_to_hcl() {
        let vpc = Vpc {
            name: "test-vpc".to_string(),
            cidr_block: "10.0.0.0/16".to_string(),
            instance_tenancy: None,
            enable_dns_hostnames: None,
            enable_dns_support: None,
            enable_classiclink: None,
            enable_classiclink_dns_support: None,
            assign_generated_ipv6_cidr_block: None,
            tags: None,
        };

        let nat_gateway = Gateway {
            id: Some("ngw-12345".to_string()),
            subnet_id: "public_subnet".to_string(),
            vpc: &vpc,
            allocation_id: Some("eipalloc-12345".to_string()),
            connectivity_type: Some("public".to_string()),
            tags: Some(HashMap::from([
                ("Name".to_string(), "Main NAT Gateway".to_string()),
                ("Environment".to_string(), "Production".to_string()),
            ])),
            state: None,
        };

        let block: Block = nat_gateway.into();
        let hcl = hcl::to_string(&block).unwrap();

        assert!(hcl.contains(r#"resource "aws_nat_gateway" "ngw-12345""#));
        assert!(hcl.contains(r#"subnet_id = aws_subnet.public_subnet.id"#));
        assert!(hcl.contains(r#"allocation_id = "eipalloc-12345""#));
        assert!(hcl.contains(r#"connectivity_type = "public""#));
        assert!(hcl.contains(r#"tags = {"#));
        assert!(hcl.contains(r#""Name" = "Main NAT Gateway""#));
        assert!(hcl.contains(r#""Environment" = "Production""#));
    }

    #[test]
    fn test_nat_gateway_data_source_to_hcl() {
        let data_source = NatGatewayDataSource {
            id: Some("ngw-12345".to_string()),
            subnet_id: Some("subnet-12345".to_string()),
            vpc_id: Some("vpc-12345".to_string()),
            state: Some("available".to_string()),
            tags: Some(HashMap::from([(
                "Name".to_string(),
                "Main NAT Gateway".to_string(),
            )])),
            filter: Some(vec![Filter {
                name: "vpc-id".to_string(),
                values: vec!["vpc-12345".to_string()],
            }]),
        };

        let block: Block = data_source.into();
        let hcl = hcl::to_string(&block).unwrap();

        assert!(hcl.contains(r#"data "aws_nat_gateway" "ngw-12345""#));
        assert!(hcl.contains(r#"id = "ngw-12345""#));
        assert!(hcl.contains(r#"subnet_id = "subnet-12345""#));
        assert!(hcl.contains(r#"vpc_id = "vpc-12345""#));
        assert!(hcl.contains(r#"state = "available""#));
        assert!(hcl.contains(r#"tags = {"#));
        assert!(hcl.contains(r#""Name" = "Main NAT Gateway""#));
        assert!(hcl.contains(r#"filter {"#));
        assert!(hcl.contains(r#"name = "vpc-id""#));
        assert!(hcl.contains(
            r#"values = [
      "vpc-12345"
    ]"#
        ));
    }
}
