﻿#[derive(Debug)]
pub struct StaffFocus {
    pub technical_focus: Vec<StaffSkillFocusType>,
    pub mental_focus: Vec<StaffSkillFocusType>,
    pub physical_focus: Vec<StaffSkillFocusType>,
}

#[derive(Debug)]
pub enum StaffSkillFocusType {
    Corners,
    Crossing,
    Dribbling,
    Finishing,
    FirstTouch,
    FreeKickTaking,
    Heading,
    LongShots,
    LongThrows,
    Marking,
    Passing,
    PenaltyTaking,
    Tackling,
    Technique,
    Aggression,
    Anticipation,
    Bravery,
    Composure,
    Concentration,
    Decisions,
    Determination,
    Flair,
    Leadership,
    OffTheBall,
    Positioning,
    Teamwork,
    Vision,
    WorkRate,
    Acceleration,
    Agility,
    Balance,
    JumpingReach,
    NaturalFitness,
    Pace,
    Stamina,
    Strength,
    MatchReadiness,
}
