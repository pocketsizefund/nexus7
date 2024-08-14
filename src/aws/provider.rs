use crate::aws::region::Region;
use hcl::{Block, Expression};

pub struct AwsProvider {
    pub region: Region,
}

impl From<AwsProvider> for Block {
    fn from(provider: AwsProvider) -> Self {
        Block::builder("provider")
            .add_label("aws")
            .add_attribute(("region", Expression::from(provider.region)))
            .build()
    }
}
