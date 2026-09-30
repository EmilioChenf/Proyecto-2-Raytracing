#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainWorld,
    CharacterSelection,
    Transition,
    PolarWorld,
    PardoWorld,
    PandaWorld,
}

impl GameState {
    #[must_use]
    pub fn back_target(self) -> Option<Self> {
        match self {
            Self::PolarWorld | Self::PardoWorld | Self::PandaWorld => {
                Some(Self::CharacterSelection)
            }
            Self::CharacterSelection => Some(Self::MainWorld),
            Self::MainWorld | Self::Transition => None,
        }
    }

    #[must_use]
    pub fn is_character_world(self) -> bool {
        matches!(self, Self::PolarWorld | Self::PardoWorld | Self::PandaWorld)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StateMachine {
    current: GameState,
    source: GameState,
    target: Option<GameState>,
    transition_remaining: f32,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            current: GameState::MainWorld,
            source: GameState::MainWorld,
            target: None,
            transition_remaining: 0.0,
        }
    }
}

impl StateMachine {
    #[must_use]
    pub fn current(&self) -> GameState {
        self.current
    }

    /// Durante una transición se sigue mostrando la escena de origen.
    #[must_use]
    pub fn visible(&self) -> GameState {
        if self.current == GameState::Transition {
            self.source
        } else {
            self.current
        }
    }

    pub fn begin_transition(&mut self, target: GameState) {
        if self.current == GameState::Transition || target == self.current {
            return;
        }
        self.source = self.current;
        self.target = Some(target);
        self.transition_remaining = 0.16;
        self.current = GameState::Transition;
    }

    /// Devuelve el nuevo estado cuando termina la transición.
    pub fn update(&mut self, delta_seconds: f32) -> Option<GameState> {
        if self.current != GameState::Transition {
            return None;
        }
        self.transition_remaining -= delta_seconds.max(0.0);
        if self.transition_remaining > 0.0 {
            return None;
        }

        let target = self.target.take()?;
        self.current = target;
        self.source = target;
        Some(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transitions_are_debounced_and_keep_source_visible() {
        let mut machine = StateMachine::default();
        machine.begin_transition(GameState::CharacterSelection);
        machine.begin_transition(GameState::PandaWorld);

        assert_eq!(machine.current(), GameState::Transition);
        assert_eq!(machine.visible(), GameState::MainWorld);
        assert_eq!(machine.update(0.2), Some(GameState::CharacterSelection));
    }

    #[test]
    fn escape_targets_follow_scene_hierarchy() {
        assert_eq!(
            GameState::PolarWorld.back_target(),
            Some(GameState::CharacterSelection)
        );
        assert_eq!(
            GameState::CharacterSelection.back_target(),
            Some(GameState::MainWorld)
        );
        assert_eq!(GameState::MainWorld.back_target(), None);
    }
}
