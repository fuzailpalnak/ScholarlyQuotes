use crate::entities::quotes::{Column, Entity as QuoteEntity};
use crate::models::data::ResponseQuote;
use crate::models::errors::AppError;
use log::info;
use rand::Rng;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

use crate::entities::quote_of_the_day::{self, Column as QOTDColumn, Entity as QOTDEntity};

use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;

pub async fn fetch_ids_by_language(
    db: &DatabaseConnection,
    language: &str,
) -> Result<Vec<i32>, AppError> {
    let quote_ids: Vec<i32> = QuoteEntity::find()
        .filter(Column::Language.eq(language))
        .column(Column::Id)
        .all(db)
        .await?
        .into_iter()
        .map(|quote| quote.id)
        .collect();

    Ok(quote_ids)
}

pub async fn fetch_random_quote_by_language(
    db: &DatabaseConnection,
    language: &str,
) -> Result<ResponseQuote, AppError> {
    let quote_ids = fetch_ids_by_language(db, language).await?;

    match quote_ids.is_empty() {
        true => Err(AppError::NotFound(
            "No quotes found in the database.".to_string(),
        )),
        false => {
            let random_id =
                rand::thread_rng().gen_range(quote_ids[0]..=quote_ids[quote_ids.len() - 1]);

            let random_quote = QuoteEntity::find_by_id(random_id)
                .one(db)
                .await?
                .ok_or_else(|| AppError::NotFound("Quote Not Found in DB".to_string()))?;
            info!("{:?}", random_quote);
            Ok(ResponseQuote {
                id: random_quote.id,
                content: random_quote.quote,
                author: random_quote.author,
                reference: random_quote.reference.expect("Category should not be None"),
                language: random_quote.language,
                ..Default::default()
            })
        }
    }
}

#[warn(dead_code)]
pub async fn insert_qotd_into_db(
    db_conn: &DatabaseConnection,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let conflict = OnConflict::columns([QOTDColumn::Language])
        .do_nothing()
        .clone();
    let _ = QOTDEntity::insert(quote_of_the_day::ActiveModel {
        language: Set(quote.language.to_string()),
        quote_id: Set(quote.id),
        ..Default::default()
    })
    .on_conflict(conflict)
    .exec(db_conn)
    .await?;

    Ok(())
}

pub async fn update_qotd_in_db(
    db_conn: &DatabaseConnection,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let _ = QOTDEntity::update_many()
        .set(quote_of_the_day::ActiveModel {
            language: Set(quote.language.to_string()),
            quote_id: Set(quote.id),
            ..Default::default()
        })
        .filter(quote_of_the_day::Column::Language.eq(quote.language.to_string()))
        .exec(db_conn)
        .await?;

    Ok(())
}

pub async fn get_qotd_from_db(
    db_conn: &DatabaseConnection,
    language: &str,
) -> Result<ResponseQuote, AppError> {
    let qotd = QOTDEntity::find()
        .filter(QOTDColumn::Language.eq(language))
        .find_also_related(QuoteEntity)
        .one(db_conn)
        .await?;

    match qotd {
        Some((_, Some(quote))) => {
            let response_quote = ResponseQuote {
                id: quote.id,
                content: quote.quote,
                author: quote.author,
                reference: quote.reference.unwrap_or_else(|| "Unknown".to_string()),
                language: quote.language,
                ..Default::default()
            };

            Ok(response_quote)
        }
        Some((_, None)) => Err(AppError::NotFound("No quote content available".to_string())),
        None => Err(AppError::NotFound(
            "No quote found for this language".to_string(),
        )),
    }
}
