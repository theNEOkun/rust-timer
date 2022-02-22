

pub type CountResult<T> = std::result::Result<T, CountError>;

#[derive(Debug, Clone)]
pub struct CountError;

pub type PlaybackResult = std::result::Result<bool, PlaybackError>;

#[derive(Debug, Clone)]
pub struct PlaybackError;

pub type TimeParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError;

