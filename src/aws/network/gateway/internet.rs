use crate::aws::network::vpc::Vpc;
use hcl::{Block, Expression, ObjectKey};
use std::collections::HashMap;

/// Represents an AWS Internet Gateway resource.
#[derive(Debug, Clone)]
pub struct Internet<'a> {
    /// The name of the Internet Gateway.
    pub name: String,
    /// The VPC to which the Internet Gateway is attached.
    pub vpc: &'a Vpc,
    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,
}

impl<'a> From<Internet<'a>> for Block {
    fn from(internet: Internet<'a>) -> Self {
        let mut block = Block::builder("resource")
            .add_label("aws_internet_gateway")
            .add_label(&internet.name)
            .add_attribute((
                "vpc_id",
                Expression::from(format!("${{aws_vpc.{}.id}}", internet.vpc.name)),
            ));

        match internet.tags {
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

/// Represents a data source for an AWS Internet Gateway.
#[derive(Debug, Clone)]
pub struct InternetDataSource {
    /// The name of the Internet Gateway data source.
    pub name: String,

    /// The ID of the specific Internet Gateway to retrieve.
    pub internet_gateway_id: Option<String>,

    /// One or more name-value pairs to filter by.
    pub filter: Option<Vec<Filter>>,

    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub values: Vec<String>,
}

impl From<InternetDataSource> for Block {
    fn from(data_source: InternetDataSource) -> Self {
        let mut block = Block::builder("data")
            .add_label("aws_internet_gateway")
            .add_label(&data_source.name);

        if let Some(id) = data_source.internet_gateway_id {
            block = block.add_attribute(("internet_gateway_id", Expression::String(id)));
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
    fn test_internet_gateway_to_hcl() {
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

        let internet_gateway = Gateway {
            name: "main-igw".to_string(),
            vpc: &vpc,
            tags: Some(HashMap::from([
                ("Name".to_string(), "Main Internet Gateway".to_string()),
                ("Environment".to_string(), "Production".to_string()),
            ])),
            vpc_ipv6_cidr_block: Some(true),
        };

        let block: Block = internet_gateway.into();
        let hcl = hcl::to_string(&block).unwrap();

        assert!(hcl.contains(r#"resource "aws_internet_gateway" "main-igw""#));
        assert!(hcl.contains(r#"vpc_id = ${aws_vpc.test-vpc.id}"#));
        assert!(hcl.contains(r#"tags = {"#));
        assert!(hcl.contains(r#""Name" = "Main Internet Gateway""#));
        assert!(hcl.contains(r#""Environment" = "Production""#));
        assert!(hcl.contains(r#"vpc_ipv6_cidr_block = true"#));
    }

    #[test]
    fn test_internet_gateway_data_source_to_hcl() {
        let data_source = InternetGatewayDataSource {
            name: "main-igw".to_string(),
            internet_gateway_id: Some("igw-12345".to_string()),
            tags: Some(HashMap::from([(
                "Name".to_string(),
                "Main Internet Gateway".to_string(),
            )])),
            filter: Some(vec![Filter {
                name: "attachment.vpc-id".to_string(),
                values: vec!["vpc-12345".to_string()],
            }]),
        };

        let block: Block = data_source.into();
        let hcl = hcl::to_string(&block).unwrap();

        assert!(hcl.contains(r#"data "aws_internet_gateway" "main-igw""#));
        assert!(hcl.contains(r#"internet_gateway_id = "igw-12345""#));
        assert!(hcl.contains(r#"tags = {"#));
        assert!(hcl.contains(r#""Name" = "Main Internet Gateway""#));
        assert!(hcl.contains(r#"filter {"#));
        assert!(hcl.contains(r#"name = "attachment.vpc-id""#));
        assert!(hcl.contains(
            r#"values = [
      "vpc-12345"
    ]"#
        ));
    }
}
