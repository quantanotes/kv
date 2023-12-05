use crate::node::Node;

use bincode::config;

pub const PAGE_SIZE: usize = 4096;

const CONFIG: config::Configuration = config::standard();

#[derive(thiserror::Error, Debug)]
pub enum PageError {
    #[error("encode error: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),

    #[error("decode error: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),

    #[error("overflow error: size: {0}, max: {1}")]
    OverflowError(usize, usize),
}

pub type Page = [u8; PAGE_SIZE];

impl TryFrom<&Node> for Page {
    type Error = PageError;

    fn try_from(value: &Node) -> Result<Self, Self::Error> {
        let mut page = [0u8; PAGE_SIZE];

        bincode::encode_into_slice(value, page.as_mut_slice(), CONFIG)?;

        Ok(page)
    }
}

impl TryInto<Node> for Page {
    type Error = PageError;

    fn try_into(self) -> Result<Node, Self::Error> {
        bincode::decode_from_slice::<Node, config::Configuration>(&self, CONFIG)
            .map(|(node, _)| node)
            .map_err(Into::into)
    }
}
