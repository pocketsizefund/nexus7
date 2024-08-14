struct EncryptionConfig {
    kms_key_arn: String,
}

#[derive(Debug)]
pub struct Role {
    pub arn: String,
}
