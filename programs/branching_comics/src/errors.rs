use anchor_lang::error_code;

#[error_code]
pub enum ComicErrors {
  
  // ==========
  // User
  // ==========

  #[msg("User is not authorized to perform this action")]
  NotAuthorized,
  #[msg("User is not the comic creator")]
  NotComicCreator,
  #[msg("User is not the chapter owner")]
  NotChapterOwner,
  #[msg("User is not a creator")]
  NotCreator,
  
  // ==========
  // Comic
  // ==========

  #[msg("Comic is not published")]
  NoPublishedComic,

  // ==========
  // Chapter
  // ==========

  #[msg("Chapter has no choices")]
  NoChoicesChapter,

  #[msg("Chapter is not the end of the branch/path")]
  NoEndChapter,

  // ==========
  // Choice
  // ==========
    
  #[msg("Choice already selected in this chapter")]
  ChoiceSelected,
  #[msg("Choice not found in this chapter")]
  ChoiceNotFound,
}
