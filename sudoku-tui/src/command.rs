#[derive(Debug, Clone, Copy)]
pub enum Command {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    PlaceNumber(u8),
    Erase,
    Undo,
    Pause,
    TogglePencilMode,
    ToggleHintMode,
    PlaceHint,
    ToggleShowCandidates,
    StartGame,
    NextDifficulty,
    PrevDifficulty,
    Quit,
}
