use anchor_lang::error_code;

#[error_code]
pub enum ComicErrors {
  #[msg("User is not authorized to perform this action")]
  NotAuthorized,
  #[msg("User is not the comic creator")]
  NotComicCreator,
  #[msg("User is not a creator")]
  NotCreator,
}
