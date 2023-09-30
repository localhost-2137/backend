use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Univertsity {
    #[serde(rename = "rank_order")]
    pub rank_order: String,
    pub rank: String,
    pub name: String,
    #[serde(rename = "scores_overall")]
    pub scores_overall: String,
    #[serde(rename = "scores_overall_rank")]
    pub scores_overall_rank: String,
    #[serde(rename = "scores_teaching")]
    pub scores_teaching: String,
    #[serde(rename = "scores_teaching_rank")]
    pub scores_teaching_rank: String,
    #[serde(rename = "scores_research")]
    pub scores_research: String,
    #[serde(rename = "scores_research_rank")]
    pub scores_research_rank: String,
    #[serde(rename = "scores_citations")]
    pub scores_citations: String,
    #[serde(rename = "scores_citations_rank")]
    pub scores_citations_rank: String,
    #[serde(rename = "scores_industry_income")]
    pub scores_industry_income: String,
    #[serde(rename = "scores_industry_income_rank")]
    pub scores_industry_income_rank: String,
    #[serde(rename = "scores_international_outlook")]
    pub scores_international_outlook: String,
    #[serde(rename = "record_type")]
    pub record_type: String,
    #[serde(rename = "member_level")]
    pub member_level: String,
    pub url: String,
    pub nid: i64,
    pub location: String,
    #[serde(rename = "stats_number_students")]
    pub stats_number_students: String,
    #[serde(rename = "stats_student_staff_ratio")]
    pub stats_student_staff_ratio: String,
    #[serde(rename = "stats_pc_intl_students")]
    pub stats_pc_intl_students: String,
    #[serde(rename = "stats_female_male_ratio")]
    pub stats_female_male_ratio: Option<String>,
    pub aliases: String,
    #[serde(rename = "subjects_offered")]
    pub subjects_offered: String,
    pub closed: bool,
    pub unaccredited: bool,
    pub disabled: bool,
    #[serde(rename = "apply_link")]
    pub apply_link: Option<String>,
    #[serde(rename = "cta_button")]
    pub cta_button: Option<CtaButton>,
    #[serde(rename = "scores_international_outlook_rank")]
    pub scores_international_outlook_rank: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CtaButton {
    pub link: String,
    pub text: String,
}
