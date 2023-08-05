// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::{Instant, Duration};

const MAXIMUM_DELAY_BETWEEN_CLEANSING: Duration = Duration::from_millis(30);

/// The dirty state is a mechanism for requesting a certain action (relayout,
/// repaint), without doing it immediately. In a lot of scenario's, two parties
/// can request a repaint in a short amount of time, which don't have to be
/// painted twice. In another scenario, one party can request a repaint, whilst
/// two milliseconds later, another requests a full page layout. Both these
/// scenarios can be optimized by stalling these [`DirtyPhase`]s until
/// appropriate.
#[derive(Debug)]
pub(crate) struct DirtyState {
    phase: DirtyPhase,
    last_update: Option<Instant>,
}

impl DirtyState {
    pub(crate) fn new() -> Self {
        Self {
            phase: DirtyPhase::GenerateLayoutTree,
            last_update: None,
        }
    }

    pub(crate) fn must_act_now(&mut self) -> bool {
        match self.last_update {
            Some(instant) => self.phase != DirtyPhase::Ready && instant.elapsed() > MAXIMUM_DELAY_BETWEEN_CLEANSING,
            None => true,
        }
    }

    pub(crate) const fn phase(&self) -> DirtyPhase {
        self.phase
    }

    pub(crate) fn request(&mut self, phase: DirtyPhase) {
        if phase > self.phase {
            self.phase = phase;
        }
    }

    pub(crate) fn mark_layout_tree_generated(&mut self) {
        self.phase = DirtyPhase::Paint;
    }

    pub(crate) fn mark_layed_out(&mut self) {
        if self.phase == DirtyPhase::Layout {
            self.phase = DirtyPhase::Paint;
        }
    }

    pub(crate) fn mark_painted(&mut self) {
        if self.phase == DirtyPhase::Paint {
            self.phase = DirtyPhase::Ready;
            self.last_update = Some(Instant::now());
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DirtyPhase {
    Ready,

    Paint,

    Layout,

    GenerateLayoutTree,
}
