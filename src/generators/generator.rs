use crate::club::{Club, ClubBoard, ClubMood, TrainingSchedule, ClubFinances, ClubSponsorshipContract};
use crate::country::Country;
use crate::league::{League, LeagueSettings};
use crate::shared::fullname::FullName;
use crate::simulator::SimulatorData;
use crate::utils::{IntegerUtils, StringUtils};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::continent::Continent;
use crate::people::{
    Mental, Physical, Player, PlayerAttributes, PlayerClubContract, PlayerCollection,
    PlayerPosition, PlayerPositionType, PlayerSkills, Staff, StaffClubContract, StaffCollection,
    StaffPosition, StaffStatus, Technical,
};
use crate::transfers::TransferPool;

impl SimulatorData {
    pub fn generate() -> SimulatorData {
        let date = NaiveDate::from_ymd(2020, 11, 15);
        let time = NaiveTime::from_hms(0, 0, 0);

        SimulatorData {
            id: SimulatorData::generate_id(),
            continents: vec![
                Continent {
                    id: 0,
                    name: "Africa".to_string(),
                    countries: (0..20)
                        .map(|_| Country::generate())
                        .collect(),
                },
                Continent {
                    id: 1,
                    name: "Eurasia".to_string(),
                    countries: (0..50)
                        .map(|_| Country::generate())
                        .collect(),
                },
                Continent {
                    id: 2,
                    name: "North America".to_string(),
                    countries: (0..2)
                        .map(|_| Country::generate())
                        .collect(),
                },
                Continent {
                    id: 3,
                    name: "Sourth America".to_string(),
                    countries: (0..10)
                        .map(|_| Country::generate())
                        .collect(),
                },
                Continent {
                    id: 4,
                    name: "Australia".to_string(),
                    countries: vec![
                        Country {
                            id: 7,
                            name: "Australia".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        }],
                },
            ],
            date: NaiveDateTime::new(date, time),
            transfer_pool: TransferPool::new(),
        }
    }
}

impl Country {
    fn generate() -> Country {
        Country {
            id: IntegerUtils::random(1, 10_000_000) as u32,
            name: StringUtils::random_string(10),
            leagues: (0..6).map(|_| League::generate()).collect(),
            reputation: 5000,
        }
    }
}

impl League {
    fn generate() -> League {
        let clubs = (0..30).map(|_| Club::generate()).collect();

        League {
            id: IntegerUtils::random(1, 10_000_000) as u32,
            name: StringUtils::random_string(30),
            clubs,
            schedule: None,
            settings: LeagueSettings {
                season_starting: (1, 1),
                season_ending: (1, 12),
            },
            reputation: 5000,
        }
    }
}

impl Club {
    fn generate() -> Club {
        let training_schedule = TrainingSchedule::new(
            NaiveTime::from_hms(10, 0, 0),
            NaiveTime::from_hms(17, 0, 0),
        );

        let sponsortship_contracts = vec![
            ClubSponsorshipContract::new(String::from("Sponsor 1"),
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2023, 1, 1)),
            ClubSponsorshipContract::new(String::from("Sponsor 2"),
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2025, 1, 1)),
            ClubSponsorshipContract::new(String::from("Sponsor 3"),
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2020, 1, 1))
        ];

        Club {
            id: IntegerUtils::random(1, 10_000_000) as u32,
            name: StringUtils::random_string(15),
            finance: ClubFinances::new(IntegerUtils::random(-10000, 10000000) as i32, sponsortship_contracts),
            mood: ClubMood::default(),
            board: ClubBoard::new(),
            players: PlayerCollection::new((0..50).map(|_| Player::generate()).collect()),
            staffs: StaffCollection::new((0..10).map(|_| Staff::generate()).collect()),
            tactics: None,
            training_schedule,
            transfer_list: Vec::new(),
            match_history: Vec::new(),
        }
    }
}

impl Player {
    fn generate() -> Player {
        let year = IntegerUtils::random(1980, 2010) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        return Player::new(
            IntegerUtils::random(1, 1_000_000) as u32,
            FullName {
                first_name: StringUtils::random_string(5),
                last_name: StringUtils::random_string(10),
                middle_name: StringUtils::random_string(15),
            },
            NaiveDate::from_ymd(year as i32, month, day),
            generate_skills(),
            generate_attributes(),
            Some(PlayerClubContract::new(NaiveDate::from_ymd(2020, 3, 14))),
            generate_positions(),
        );

        fn generate_skills() -> PlayerSkills {
            PlayerSkills {
                technical: Technical {
                    corners: IntegerUtils::random(1, 20) as u8,
                    crossing: IntegerUtils::random(1, 20) as u8,
                    dribbling: IntegerUtils::random(1, 20) as u8,
                    finishing: IntegerUtils::random(1, 20) as u8,
                    first_touch: IntegerUtils::random(1, 20) as u8,
                    free_kick_taking: IntegerUtils::random(1, 20) as u8,
                    heading: IntegerUtils::random(1, 20) as u8,
                    long_shots: IntegerUtils::random(1, 20) as u8,
                    long_throws: IntegerUtils::random(1, 20) as u8,
                    marking: IntegerUtils::random(1, 20) as u8,
                    passing: IntegerUtils::random(1, 20) as u8,
                    penalty_taking: IntegerUtils::random(1, 20) as u8,
                    tackling: IntegerUtils::random(1, 20) as u8,
                    technique: IntegerUtils::random(1, 20) as u8,
                },
                mental: Mental {
                    aggression: IntegerUtils::random(1, 20) as u8,
                    anticipation: IntegerUtils::random(1, 20) as u8,
                    bravery: IntegerUtils::random(1, 20) as u8,
                    composure: IntegerUtils::random(1, 20) as u8,
                    concentration: IntegerUtils::random(1, 20) as u8,
                    decisions: IntegerUtils::random(1, 20) as u8,
                    determination: IntegerUtils::random(1, 20) as u8,
                    flair: IntegerUtils::random(1, 20) as u8,
                    leadership: IntegerUtils::random(1, 20) as u8,
                    off_the_ball: IntegerUtils::random(1, 20) as u8,
                    positioning: IntegerUtils::random(1, 20) as u8,
                    teamwork: IntegerUtils::random(1, 20) as u8,
                    vision: IntegerUtils::random(1, 20) as u8,
                    work_rate: IntegerUtils::random(1, 20) as u8,
                },
                physical: Physical {
                    acceleration: IntegerUtils::random(1, 20) as u8,
                    agility: IntegerUtils::random(1, 20) as u8,
                    balance: IntegerUtils::random(1, 20) as u8,
                    jumping_reach: IntegerUtils::random(1, 20) as u8,
                    natural_fitness: IntegerUtils::random(1, 20) as u8,
                    pace: IntegerUtils::random(1, 20) as u8,
                    stamina: IntegerUtils::random(1, 20) as u8,
                    strength: IntegerUtils::random(1, 20) as u8,
                    match_readiness: IntegerUtils::random(1, 20) as u8,
                },
            }
        }

        fn generate_positions() -> Vec<PlayerPosition> {
            let positions_to_generate = IntegerUtils::random(1, 4) as u32;

            let mut positions = Vec::with_capacity(positions_to_generate as usize);

            for pos in 0..positions_to_generate {
                positions.push(PlayerPosition {
                    position: PlayerPositionGenerator::generate(),
                    level: IntegerUtils::random(0, 20) as u8,
                })
            }

            positions
        }

        fn generate_attributes() -> PlayerAttributes {
            PlayerAttributes::new(
                IntegerUtils::random(0, 20) as u8,
                IntegerUtils::random(-20, 20) as i8,
            )
        }
    }
}

impl Staff {
    fn generate() -> Staff {
        let year = IntegerUtils::random(1980, 2010) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        Staff::new(
            IntegerUtils::random(1, 10_000_000) as u32,
            FullName {
                first_name: StringUtils::random_string(5),
                last_name: StringUtils::random_string(10),
                middle_name: StringUtils::random_string(15),
            },
            NaiveDate::from_ymd(year as i32, month, day),
            Some(StaffClubContract::new(
                NaiveDate::from_ymd(2020, 3, 14),
                StaffPosition::MainCoach,
                StaffStatus::Active,
            )),
        )
    }
}

pub struct PlayerPositionGenerator;

impl PlayerPositionGenerator {
    pub fn generate() -> PlayerPositionType {
        match IntegerUtils::random(0, 3) {
            0 => PlayerPositionType::Goalkeeper,
            1 => PlayerPositionType::Defender,
            2 => PlayerPositionType::Midfielder,
            3 => PlayerPositionType::Forward,
            _ => PlayerPositionType::Goalkeeper,
        }
    }
}
