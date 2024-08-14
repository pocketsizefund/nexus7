use hcl::{Block, Expression, ObjectKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ACL {
    #[serde(rename = "acl")]
    Acl,
    #[serde(rename = "access_control_policy")]
    AccessControlPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessControlPolicy {
    #[serde(rename = "access_control_policy")]
    AccessControlPolicy,
    #[serde(rename = "acl")]
    ACL,
}

#[derive(Debug, Clone)]
pub enum ACLOptions {
    Private,
    PublicRead,
    PublicReadWrite,
    AuthenticatedRead,
    LogDeliveryWrite,
    BucketOwnerRead,
    BucketOwnerFullControl,
}

impl fmt::Display for ACLOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ACLOptions::Private => write!(f, "private"),
            ACLOptions::PublicRead => write!(f, "public-read"),
            ACLOptions::PublicReadWrite => write!(f, "public-read-write"),
            ACLOptions::AuthenticatedRead => write!(f, "authenticated-read"),
            ACLOptions::LogDeliveryWrite => write!(f, "log-delivery-write"),
            ACLOptions::BucketOwnerRead => write!(f, "bucket-owner-read"),
            ACLOptions::BucketOwnerFullControl => write!(f, "bucket-owner-full-control"),
        }
    }
}

/// Represents an AWS S3 bucket ACL resource.
#[derive(Debug, Clone)]
pub struct BucketACL {
    /// Canned ACL to apply to the bucket.
    /// Optional, but one of `acl` or `access_control_policy` is required.
    pub acl: Option<ACLOptions>,

    /// Configuration block that sets the ACL permissions for an object per grantee.
    /// Optional, but one of `access_control_policy` or `acl` is required.
    pub access_control_policy: Option<AccessControlPolicy>,

    /// Bucket to which to apply the ACL.
    pub bucket: String,

    /// Account ID of the expected bucket owner.
    pub expected_bucket_owner: Option<String>,
}

impl From<BucketACL> for Block {
    fn from(bucket_acl: BucketACL) -> Self {
        let mut block = Block::builder("resource")
            .add_label("aws_s3_bucket_acl")
            .add_label(&bucket_acl.bucket)
            .add_attribute(("bucket", Expression::String(bucket_acl.bucket)));

        if let Some(acl) = bucket_acl.acl {
            block = block.add_attribute(("acl", Expression::String(acl.to_string())));
        }

        if let Some(access_control_policy) = bucket_acl.access_control_policy {
            let mut acp_block = Block::builder("access_control_policy");

            match access_control_policy {
                AccessControlPolicy::AccessControlPolicy => {
                    acp_block = acp_block.add_attribute((
                        "type",
                        Expression::String("access_control_policy".to_string()),
                    ));
                }
                AccessControlPolicy::ACL => {
                    acp_block =
                        acp_block.add_attribute(("type", Expression::String("acl".to_string())));
                }
            }

            block = block.add_block(acp_block.build());
        }

        if let Some(expected_bucket_owner) = bucket_acl.expected_bucket_owner {
            block = block.add_attribute((
                "expected_bucket_owner",
                Expression::String(expected_bucket_owner),
            ));
        }

        block.build()
    }
}

/// Represents an AWS S3 bucket resource.
#[derive(Debug, Clone)]
pub struct Bucket {
    /// Name of the bucket. If omitted, Terraform will assign a random, unique name.
    /// Must be lowercase and less than or equal to 63 characters in length.
    /// The name must not be in the format [bucket_name]--[azid]--x-s3.
    /// Use the aws_s3_directory_bucket resource to manage S3 Express buckets.
    pub name: Option<String>,

    /// Access Control List (ACL) for the bucket.
    /// Defaults to "private".
    pub acl: Option<ACLOptions>,

    /// Creates a unique bucket name beginning with the specified prefix.
    /// Conflicts with `bucket`.
    /// Must be lowercase and less than or equal to 37 characters in length.
    pub prefix: Option<String>,

    /// Boolean that indicates all objects (including any locked objects) should be deleted
    /// from the bucket when the bucket is destroyed so that the bucket can be destroyed without error.
    /// These objects are not recoverable. This only deletes objects when the bucket is destroyed,
    /// not when setting this parameter to true.
    pub force_destroy: Option<bool>,

    /// Indicates whether this bucket has an Object Lock configuration enabled.
    /// Valid values are true or false.
    /// This argument is not supported in all regions or partitions.
    pub object_lock_enabled: Option<bool>,

    /// Map of tags to assign to the bucket.
    /// If configured with a provider default_tags configuration block present,
    /// tags with matching keys will overwrite those defined at the provider-level.
    pub tags: Option<HashMap<String, String>>,
}

impl From<Bucket> for Block {
    fn from(bucket: Bucket) -> Self {
        let mut block = Block::builder("resource").add_label("aws_s3_bucket");

        if let Some(name) = &bucket.name {
            block = block
                .add_label(name)
                .add_attribute(("bucket", Expression::String(name.clone())));
        }

        match bucket.acl {
            Some(acl) => block = block.add_attribute(("acl", Expression::String(acl.to_string()))),
            None => {
                block = block
                    .add_attribute(("acl", Expression::String(ACLOptions::Private.to_string())))
            }
        }

        if let Some(prefix) = bucket.prefix {
            block = block.add_attribute(("bucket_prefix", Expression::String(prefix)));
        }

        if let Some(force_destroy) = bucket.force_destroy {
            block = block.add_attribute(("force_destroy", Expression::Bool(force_destroy)));
        }

        if let Some(object_lock_enabled) = bucket.object_lock_enabled {
            block =
                block.add_attribute(("object_lock_enabled", Expression::Bool(object_lock_enabled)));
        }

        if let Some(tags) = bucket.tags {
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

    use hcl;

    #[test]
    fn test_bucket_with_name() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"resource "aws_s3_bucket" "my-bucket" {"#));
        assert!(hcl.contains(r#"bucket = "my-bucket""#));
    }

    #[test]
    fn test_bucket_with_prefix() {
        let bucket = Bucket {
            name: None,
            acl: None,
            prefix: Some("my-prefix-".to_string()),
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"bucket_prefix = "my-prefix-""#));
    }

    #[test]
    fn test_bucket_with_force_destroy() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: Some(true),
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"force_destroy = true"#));
    }

    #[test]
    fn test_bucket_with_object_lock_enabled() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: Some(true),
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"object_lock_enabled = true"#));
    }

    #[test]
    fn test_bucket_with_tags() {
        let mut tags = HashMap::new();
        tags.insert("Environment".to_string(), "Production".to_string());
        tags.insert("Project".to_string(), "MyProject".to_string());
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: Some(tags),
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"tags = {"#));
        assert!(hcl.contains(r#""Environment" = "Production""#));
        assert!(hcl.contains(r#""Project" = "MyProject""#));
    }

    #[test]
    fn test_bucket_with_all_fields() {
        let mut tags = HashMap::new();
        tags.insert("Environment".to_string(), "Production".to_string());
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: Some("prefix-".to_string()),
            acl: None,
            force_destroy: Some(true),
            object_lock_enabled: Some(false),
            tags: Some(tags),
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"resource "aws_s3_bucket" "my-bucket" {"#));
        assert!(hcl.contains(r#"bucket = "my-bucket""#));
        assert!(hcl.contains(r#"bucket_prefix = "prefix-""#));
        assert!(hcl.contains(r#"force_destroy = true"#));
        assert!(hcl.contains(r#"object_lock_enabled = false"#));
        assert!(hcl.contains(r#"tags = {"#));
        assert!(hcl.contains(r#""Environment" = "Production""#));
    }

    #[test]
    fn test_bucket_without_name_or_prefix() {
        let bucket = Bucket {
            name: None,
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"resource "aws_s3_bucket" {"#));
    }

    #[test]
    fn test_bucket_with_empty_tags() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: Some(HashMap::new()),
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"tags = {}"#));
    }

    #[test]
    fn test_bucket_with_multiple_tags() {
        let mut tags = HashMap::new();
        tags.insert("Environment".to_string(), "Production".to_string());
        tags.insert("Project".to_string(), "MyProject".to_string());
        tags.insert("Owner".to_string(), "TeamA".to_string());
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: Some(tags),
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"tags = {"#));
        assert!(hcl.contains(r#""Environment" = "Production""#));
        assert!(hcl.contains(r#""Project" = "MyProject""#));
        assert!(hcl.contains(r#""Owner" = "TeamA""#));
    }

    #[test]
    fn test_bucket_with_long_name() {
        let bucket = Bucket {
            name: Some("my-very-long-bucket-name-that-is-exactly-63-characters-long".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"resource "aws_s3_bucket" "my-very-long-bucket-name-that-is-exactly-63-characters-long" {"#));
        assert!(hcl
            .contains(r#"bucket = "my-very-long-bucket-name-that-is-exactly-63-characters-long""#));
    }

    #[test]
    fn test_bucket_with_special_characters_in_name() {
        let bucket = Bucket {
            name: Some("my-bucket-with-special-chars-_.-".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"resource "aws_s3_bucket" "my-bucket-with-special-chars-_.-" {"#));
        assert!(hcl.contains(r#"bucket = "my-bucket-with-special-chars-_.-""#));
    }

    #[test]
    fn test_bucket_with_force_destroy_false() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: Some(false),
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"force_destroy = false"#));
    }

    #[test]
    fn test_bucket_with_object_lock_enabled_false() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: Some(false),
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"object_lock_enabled = false"#));
    }

    #[test]
    fn test_bucket_with_prefix_and_name() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: Some("prefix-".to_string()),
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"bucket = "my-bucket""#));
        assert!(hcl.contains(r#"bucket_prefix = "prefix-""#));
    }

    #[test]
    fn test_bucket_with_tags_containing_quotes() {
        let mut tags = HashMap::new();
        tags.insert(
            "Description".to_string(),
            "This is a \"quoted\" value".to_string(),
        );
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: Some(tags),
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#""Description" = "This is a \"quoted\" value""#));
    }

    #[test]
    fn test_bucket_with_all_fields_none() {
        let bucket = Bucket {
            name: None,
            acl: None,
            prefix: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"resource "aws_s3_bucket" {"#));
        assert!(!hcl.contains("bucket ="));
        assert!(!hcl.contains("bucket_prefix ="));
        assert!(!hcl.contains("force_destroy ="));
        assert!(!hcl.contains("object_lock_enabled ="));
        assert!(!hcl.contains("tags ="));
    }

    #[test]
    fn test_bucket_with_empty_name() {
        let bucket = Bucket {
            name: Some("".to_string()),
            prefix: None,
            acl: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"resource "aws_s3_bucket" "" {"#));
        assert!(hcl.contains(r#"bucket = """#));
    }

    #[test]
    fn test_bucket_with_empty_prefix() {
        let bucket = Bucket {
            name: None,
            acl: None,
            prefix: Some("".to_string()),
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"bucket_prefix = """#));
    }

    #[test]
    fn test_bucket_with_acl() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: Some(ACLOptions::PublicRead),
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"acl = "public-read""#));
    }

    #[test]
    fn test_bucket_with_private_acl() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            prefix: None,
            acl: Some(ACLOptions::Private),
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"acl = "private""#));
    }

    #[test]
    fn test_no_acl_defaults_to_private() {
        let bucket = Bucket {
            name: Some("my-bucket".to_string()),
            acl: None,
            prefix: None,
            force_destroy: None,
            object_lock_enabled: None,
            tags: None,
        };
        let block: Block = bucket.into();
        let hcl = hcl::to_string(&block).unwrap();
        assert!(hcl.contains(r#"acl = "private""#));
    }
}
