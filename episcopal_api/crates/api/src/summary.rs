use calendar::{Date, Feast, LiturgicalDay, LiturgicalDayId};
use lectionary::Reading;
use liturgy::{Document, Psalm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct EucharisticLectionarySummary {
    pub day: LiturgicalDay,
    pub observed: EucharisticObservanceSummary,
    pub alternate: Option<EucharisticObservanceSummary>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct EucharisticObservanceSummary {
    pub observance: LiturgicalDayId,
    pub localized_name: String,
    pub collects: Option<Document>,
    pub vigil_readings: Vec<Document>,
    pub tracked_readings: TrackedReadings,
    pub epistle: Option<Document>,
    pub gospel: Option<Document>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum TrackedReadings {
    Any(Box<FirstLessonAndPsalm>),
    Tracked {
        track_one: Box<FirstLessonAndPsalm>,
        track_two: Box<FirstLessonAndPsalm>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct FirstLessonAndPsalm {
    pub first_lesson: Option<Document>,
    pub psalm: Option<Document>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DailySummary {
    pub date: Date,
    pub morning: PartialDailySummary,
    pub evening: PartialDailySummary,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PartialDailySummary {
    pub day: LiturgicalDay,
    pub observed: ObservanceSummary,
    pub alternate: Option<ObservanceSummary>,
    pub thirty_day_psalms: Vec<Psalm>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ObservanceSummary {
    pub observance: LiturgicalDayId,
    pub localized_name: String,
    pub bcp_black_letter_days: Vec<(Feast, String)>,
    pub lff_black_letter_days: Vec<(Feast, String)>,
    pub collects: Option<Document>,
    pub daily_office_readings: Vec<Reading>,
    pub daily_office_psalms: Vec<Psalm>,
}
