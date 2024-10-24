use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "work_experience_level", rename_all = "snake_case")]
pub enum ExperienceLevel {
    Intern,
    EntryLevel,
    MidLevel,
    SeniorLevel,
    Executive,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "work_contract_type", rename_all = "snake_case")]
pub enum ContractType {
    FullTime,
    PartTime,
    FreelanceContract,
    FixedTermContract,
    ZeroHourContract,
    Internship,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[repr(i16)]
pub enum JobStatus {
    Open = 1,
    Closed = 0,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct DetailedJob {
    pub id: String,
    pub title: String,
    pub description: String,
    pub country_id: i32,
    pub country_name: String,
    pub organization_id: i64,
    pub organization_name: String,
    pub organization_description: Option<String>,
    pub organization_image_url: Option<String>,
    pub organization_website_url: Option<String>,
    pub work_experience_level: ExperienceLevel,
    pub work_contract_type: ContractType,
    pub is_remote: bool,
    pub status: JobStatus,
}

impl DetailedJob {
    pub async fn get_with_id(pool: &PgPool, id: &str) -> sqlx::Result<DetailedJob> {
        let query = r#"
            SELECT
                j.id,
                j.title,
                j.description,
                j.country_id,
                c.name AS country_name,
                j.organization_id,
                o.name AS organization_name,
                o.description AS organization_description,
                o.image_url AS organization_image_url,
                o.website_url AS organization_website_url,
                j.work_experience_level,
                j.work_contract_type,
                j.is_remote,
                j.status
            FROM jobs AS j
            INNER JOIN countries AS c ON j.country_id = c.id
            INNER JOIN organizations AS o on j.organization_id = o.id
            WHERE j.id = $1
        "#;

        sqlx::query_as::<_, DetailedJob>(&query)
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub fn is_open(&self) -> bool {
        self.status == JobStatus::Open
    }

    pub fn is_remote(&self) -> bool {
        self.is_remote
    }

    pub fn is_remote_text(&self) -> String {
        if self.is_remote {
            "Remote".to_string()
        } else {
            self.country_name.clone()
        }
    }

    pub fn title(&self) -> String {
        super::title_case(&self.title)
    }

    pub fn contract_type(&self) -> String {
        match self.work_contract_type {
            ContractType::FullTime => "Full-time".to_string(),
            ContractType::PartTime => "Part-time".to_string(),
            ContractType::FreelanceContract => "Freelance".to_string(),
            ContractType::FixedTermContract => "Fixed-term".to_string(),
            ContractType::ZeroHourContract => "Zero-hour".to_string(),
            ContractType::Internship => "Internship".to_string(),
        }
    }

    pub fn experience_level(&self) -> String {
        match self.work_experience_level {
            ExperienceLevel::Intern => "Intern".to_string(),
            ExperienceLevel::EntryLevel => "Entry-level".to_string(),
            ExperienceLevel::MidLevel => "Mid-level".to_string(),
            ExperienceLevel::SeniorLevel => "Senior-level".to_string(),
            ExperienceLevel::Executive => "Executive".to_string(),
        }
    }

    pub fn organization_image_url(&self) -> String {
        self.organization_image_url
            .clone()
            .unwrap_or_else(|| "https://placehold.co/1280x566.webp".to_string())
    }

    pub fn organization_description(&self) -> String {
        self.organization_description
            .clone()
            .unwrap_or_else(|| "No description provided.".to_string())
    }

    pub fn organization_website_url(&self) -> String {
        self.organization_website_url
            .clone()
            .unwrap_or_else(|| "https://t.me/jobsesame".to_string())
    }
}
