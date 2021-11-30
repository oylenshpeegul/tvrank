#![warn(clippy::all)]

use super::error::Err;
use crate::imdb::genre::Genres;
use deepsize::DeepSizeOf;
use derive_more::{Display, Into};
use enum_utils::FromStr;
use std::{cmp::Ordering, error::Error, fmt, time::Duration};

#[derive(Debug, Display, FromStr, PartialEq, Eq, Hash, Clone, Copy, DeepSizeOf)]
#[enumeration(rename_all = "camelCase")]
#[display(fmt = "{}")]
pub enum TitleType {
  // Games
  VideoGame,

  // Movies
  Short,
  Video,
  Movie,
  TvShort,
  TvMovie,
  TvSpecial,

  // Episodes
  TvEpisode,
  TvPilot,
  RadioEpisode,

  // Series
  TvSeries,
  TvMiniSeries,
  RadioSeries,
}

impl TitleType {
  pub(crate) fn is_movie(&self) -> bool {
    match self {
      // Games
      TitleType::VideoGame => false,

      // Movies
      TitleType::Short
      | TitleType::Video
      | TitleType::Movie
      | TitleType::TvShort
      | TitleType::TvMovie
      | TitleType::TvSpecial => true,

      // Episodes
      TitleType::TvEpisode | TitleType::TvPilot | TitleType::RadioEpisode => false,

      // Series
      TitleType::TvSeries | TitleType::TvMiniSeries | TitleType::RadioSeries => false,
    }
  }

  pub(crate) fn is_series(&self) -> bool {
    match self {
      // Games
      TitleType::VideoGame => false,

      // Movies
      TitleType::Short
      | TitleType::Video
      | TitleType::Movie
      | TitleType::TvShort
      | TitleType::TvMovie
      | TitleType::TvSpecial => false,

      // Episodes
      TitleType::TvEpisode | TitleType::TvPilot | TitleType::RadioEpisode => false,

      // Series
      TitleType::TvSeries | TitleType::TvMiniSeries | TitleType::RadioSeries => true,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Into, DeepSizeOf)]
pub struct TitleId(&'static [u8]);

impl TryFrom<&'static [u8]> for TitleId {
  type Error = Box<dyn Error>;

  fn try_from(id: &'static [u8]) -> Result<Self, Self::Error> {
    if &id[0..=1] != super::parsing::TT {
      return Err::id(id);
    }

    Ok(TitleId(id))
  }
}

impl fmt::Display for TitleId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for &d in self.0.iter() {
      write!(f, "{}", char::from(d))?;
    }

    Ok(())
  }
}

impl PartialOrd for TitleId {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(other.0)
  }
}

impl Ord for TitleId {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(other.0)
  }
}

#[derive(Debug, PartialEq, Eq, Clone, DeepSizeOf)]
pub(crate) struct TitleBasics {
  pub(crate) title_id: TitleId,
  pub(crate) title_type: TitleType,
  pub(crate) primary_title: &'static str,
  pub(crate) original_title: &'static str,
  pub(crate) is_adult: bool,
  pub(crate) start_year: Option<u16>,
  pub(crate) end_year: Option<u16>,
  pub(crate) runtime_minutes: Option<u16>,
  pub(crate) genres: Genres,
}

#[derive(Debug, PartialEq, Eq, Clone, DeepSizeOf)]
pub struct Title<'basics, 'ratings> {
  basics: &'basics TitleBasics,
  rating: Option<&'ratings (u8, u64)>,
}

impl<'basics, 'ratings> Title<'basics, 'ratings> {
  pub(crate) fn new(basics: &'basics TitleBasics, rating: Option<&'ratings (u8, u64)>) -> Self {
    Self { basics, rating }
  }

  pub fn title_id(&self) -> TitleId {
    self.basics.title_id
  }

  pub fn title_type(&self) -> TitleType {
    self.basics.title_type
  }

  pub fn primary_title(&self) -> &str {
    self.basics.primary_title
  }

  pub fn original_title(&self) -> &str {
    self.basics.original_title
  }

  pub fn is_adult(&self) -> bool {
    self.basics.is_adult
  }

  pub fn start_year(&self) -> Option<u16> {
    self.basics.start_year
  }

  pub fn end_year(&self) -> Option<u16> {
    self.basics.end_year
  }

  pub fn runtime(&self) -> Option<Duration> {
    self
      .basics
      .runtime_minutes
      .map(|runtime| Duration::from_secs(u64::from(runtime) * 60))
  }

  pub fn genres(&self) -> Genres {
    self.basics.genres
  }

  pub fn rating(&self) -> Option<&(u8, u64)> {
    self.rating
  }
}
