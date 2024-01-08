use actix_web::{HttpResponse, web};
use actix_web::web::Path;
use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;

pub async fn get_all_teachers(
    app_state: web::Data<AppState>
) ->Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>, path: Path<i32>
) -> Result<HttpResponse, MyError> {
    get_teacher_details_db(&app_state.db, path.into_inner())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    app_state: web::Data<AppState>, teacher: web::Json<CreateTeacher>
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, CreateTeacher::from(teacher))
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    path: Path<i32>,
    update_teacher: web::Json<UpdateTeacher>,
) -> Result<HttpResponse, MyError> {
    update_teacher_details_db(&app_state.db, path.into_inner(), UpdateTeacher::from(update_teacher))
        .await
        .map(|teacher|HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>, path: Path<i32>
) -> Result<HttpResponse, MyError> {
    delete_teacher_db(&app_state.db, path.into_inner())
        .await.
        map(|result| HttpResponse::Ok().json(result))
}

#[cfg(test)]
mod  tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use std::env;
    use std::sync::Mutex;
    use sqlx::mysql::MySqlPoolOptions;

    #[ignore]
    #[actix_rt::test]
    async fn post_new_teacher_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher = web::Json(CreateTeacher{
            name: "Han Siyuan".to_string(),
            picture_url: "https://onederive.com/Haydn.Kong".to_string(),
            profile: "rich".to_string(),
        });

        let resp = post_new_teacher(app_state, teacher).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_all_teachers_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let resp = get_all_teachers(app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_teacher_detail_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id = web::Path::from(3);

        let resp = get_teacher_details(app_state, teacher_id).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_teacher_details_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher = web::Json(UpdateTeacher{
            name: Some("Haydn Kong".to_string()),
            picture_url: Some("https://onederive.com/Haydn.Kong".to_string()),
            profile: Some("rich".to_string()),
        });
        let teacher_id = web::Path::from(2);

        let resp = update_teacher_details(app_state, teacher_id, teacher).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn delete_teacher_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id = web::Path::from(6);

        let resp = delete_teacher(app_state, teacher_id).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK)
    }
}