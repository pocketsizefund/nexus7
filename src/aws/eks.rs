use crate::aws::iam;
use crate::aws::network::subnet::Subnet;
use crate::aws::network::vpc::Vpc;
use hcl::{Block, Expression, ObjectKey};
use std::collections::HashMap;

/// Represents an AWS EKS Cluster resource.
#[derive(Debug, Clone)]
pub struct Cluster<'a> {
    /// Name of the cluster.
    pub name: String,
    /// The VPC associated with your cluster.
    pub vpc: &'a Vpc,
    /// List of subnet IDs where the EKS cluster will be created.
    pub subnet_ids: Vec<&'a Subnet<'a>>,
    /// Desired Kubernetes master version.
    pub version: Option<String>,
    /// Role to use to access other AWS services.
    pub role: &'a iam::Role,
    /// Desired Kubernetes version for the cluster.
    pub kubernetes_version: Option<String>,
    /// Indicates whether or not the Amazon EKS private API server endpoint is enabled.
    pub enabled_cluster_log_types: Option<Vec<String>>,
    /// List of the desired control plane logging to enable.
    pub endpoint_private_access: Option<bool>,
    /// Indicates whether or not the Amazon EKS public API server endpoint is enabled.
    pub endpoint_public_access: Option<bool>,
    /// Configuration block with encryption configuration for the cluster.
    pub encryption_config: Option<EncryptionConfig>,
    /// A map of tags to assign to the resource.
    pub tags: Option<HashMap<String, String>>,
}

/// Represents the encryption configuration for an EKS cluster.
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// KMS key ARN to use for envelope encryption of Kubernetes secrets.
    pub kms_key_arn: String,
}

impl<'a> From<Cluster<'a>> for Block {
    fn from(cluster: Cluster<'a>) -> Self {
        let mut block = Block::builder("resource")
            .add_label("aws_eks_cluster")
            .add_label(&cluster.name)
            .add_attribute(("name", Expression::String(cluster.name)))
            .add_attribute(("role_arn", Expression::String(cluster.role.arn.to_string())));

        let vpc_config = Block::builder("vpc_config")
            .add_attribute((
                "subnet_ids",
                Expression::Array(
                    cluster
                        .subnet_ids
                        .iter()
                        .map(|s| Expression::from(format!("${{aws_subnet.{}.id}}", s.name)))
                        .collect(),
                ),
            ))
            .build();

        block = block.add_block(vpc_config);

        if let Some(version) = cluster.version {
            block = block.add_attribute(("version", Expression::String(version)));
        }

        if let Some(kubernetes_version) = cluster.kubernetes_version {
            block =
                block.add_attribute(("kubernetes_version", Expression::String(kubernetes_version)));
        }

        if let Some(enabled_cluster_log_types) = cluster.enabled_cluster_log_types {
            block = block.add_attribute((
                "enabled_cluster_log_types",
                Expression::Array(
                    enabled_cluster_log_types
                        .into_iter()
                        .map(Expression::String)
                        .collect(),
                ),
            ));
        }

        if let Some(endpoint_private_access) = cluster.endpoint_private_access {
            block = block.add_attribute((
                "endpoint_private_access",
                Expression::Bool(endpoint_private_access),
            ));
        }

        if let Some(endpoint_public_access) = cluster.endpoint_public_access {
            block = block.add_attribute((
                "endpoint_public_access",
                Expression::Bool(endpoint_public_access),
            ));
        }

        if let Some(encryption_config) = cluster.encryption_config {
            let encryption_block = Block::builder("encryption_config")
                .add_block(
                    Block::builder("provider")
                        .add_attribute((
                            "key_arn",
                            Expression::String(encryption_config.kms_key_arn),
                        ))
                        .build(),
                )
                .build();
            block = block.add_block(encryption_block);
        }

        if let Some(tags) = cluster.tags {
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
    fn test_eks_cluster_to_hcl() {
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

        let subnet1 = Subnet {
            name: "subnet1".to_string(),
            vpc: &vpc,
            cidr_block: "10.0.1.0/24".to_string(),
            availability_zone: None,
            map_public_ip_on_launch: None,
            tags: None,
        };

        let subnet2 = Subnet {
            name: "subnet2".to_string(),
            vpc: &vpc,
            cidr_block: "10.0.2.0/24".to_string(),
            availability_zone: None,
            map_public_ip_on_launch: None,
            tags: None,
        };

        let cluster = Cluster {
            name: "test-cluster".to_string(),
            vpc: &vpc,
            subnet_ids: vec![&subnet1, &subnet2],
            version: Some("1.21".to_string()),
            role_arn: "arn:aws:iam::123456789012:role/eks-cluster-role".to_string(),
            kubernetes_version: None,
            enabled_cluster_log_types: Some(vec!["api".to_string(), "audit".to_string()]),
            endpoint_private_access: Some(true),
            endpoint_public_access: Some(false),
            encryption_config: Some(EncryptionConfig {
                kms_key_arn:
                    "arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"
                        .to_string(),
            }),
            tags: Some(HashMap::from([
                ("Environment".to_string(), "Production".to_string()),
                ("Project".to_string(), "EKS".to_string()),
            ])),
        };

        let block: Block = cluster.into();
        let hcl = hcl::to_string(&block).unwrap();

        assert!(hcl.contains(r#"resource "aws_eks_cluster" "test-cluster" {"#));
        assert!(hcl.contains(r#"name = "test-cluster""#));
        assert!(hcl.contains(r#"role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role""#));
        assert!(hcl.contains(r#"vpc_config {"#));
        assert!(hcl.contains(r#"subnet_ids = [aws_subnet.subnet1.id, aws_subnet.subnet2.id]"#));
        assert!(hcl.contains(r#"version = "1.21""#));
        assert!(hcl.contains(r#"enabled_cluster_log_types = ["api", "audit"]"#));
        assert!(hcl.contains(r#"endpoint_private_access = true"#));
        assert!(hcl.contains(r#"endpoint_public_access = false"#));
        assert!(hcl.contains(r#"encryption_config {"#));
        assert!(hcl.contains(r#"provider {"#));
        assert!(hcl.contains(r#"key_arn = "arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab""#));
        assert!(hcl.contains(r#"tags = {"#));
        assert!(hcl.contains(r#""Environment" = "Production""#));
        assert!(hcl.contains(r#""Project" = "EKS""#));
    }
}
