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
    StartGame,
    NextDifficulty,
    PrevDifficulty,
    Quit,
}
