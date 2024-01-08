use crate::state::AppState;
use crate::dbaccess::course::*;
use crate::errors::MyError;
use crate::models::course::{CreateCourse, UpdateCourse};
use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("Received new course");

    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse,MyError> {
    println!("Searching courses...");

    let teacher_id = path.into_inner();
    get_course_for_teacher_db(&app_state.db, teacher_id)
            .await.map(|courses|HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    println!("Getting course's detail");

    let (teacher_id, course_id) = path.into_inner();
    get_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course_detail|HttpResponse::Ok().json(course_detail))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = path.into_inner();

    delete_course_db(&app_state.db, teacher_id, course_id)
        .await.map(|resp|HttpResponse::Ok().json(resp))
}

pub async fn update_course_detail(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    update_course: web::Json<UpdateCourse>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = path.into_inner();

    update_course_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await.map(|course|HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use sqlx::mysql::MySqlPoolOptions;
    use actix_web::web;
    use crate::AppState;
    use std::sync::Mutex;
    use actix_web::http::StatusCode;
    use super::*;

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let new_course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "post new course from test function".into(),
            description: None,
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("Chinese".to_string()),
            level: Some("Medium".to_string()),
        });

        let resp = post_new_course(new_course, app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id: web::Path<i32>  = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_detail(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = delete_course(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));

        let update_course = web::Json(UpdateCourse {
            name: "Update Course".to_string(),
            description: Some("Update information".to_string()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: None,
            level: None,
        });

        let resp = update_course_detail(app_state, params, update_course).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }
}