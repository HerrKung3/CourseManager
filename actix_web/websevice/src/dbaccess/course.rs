use crate::models::course::{Course, CreateCourse, UpdateCourse};
use crate::errors::MyError;
use sqlx::mysql::MySqlPool;

pub async fn get_course_for_teacher_db(
    pool: &MySqlPool, teacher_id: i32
) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query_as!(
        Course,
        r#"SELECT *
        FROM course
        WHERE teacher_id = ? "#,
        teacher_id
    ).fetch_all(pool).await?;

    Ok(rows)
}

pub async fn get_course_detail_db(
    pool: &MySqlPool, teacher_id: i32, course_id: i32
) -> Result<Course, MyError> {
    //fetch_optional: can find it or can't
    let row = sqlx::query_as!(
        Course,
        r#"SELECT *
        FROM course
        WHERE teacher_id = ? and id = ? "#,
        teacher_id,
        course_id,
    ).fetch_optional(pool).await?;

    if let Some(course) = row {
        Ok(course)
    }else {
        Err(MyError::NotFound("Course ID not found".to_string()))
    }
}

pub async fn post_new_course_db(
    pool: &MySqlPool, new_course: CreateCourse
) -> Result<Course, MyError> {
    let _ = sqlx::query!(
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUE (?, ?, ?, ?, ?, ?, ?, ?, ?) "#,
        new_course.teacher_id,
        new_course.name.clone(),
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,
    ).execute(pool).await?;

    //just fetch one, do NOT consider duplicate course name
    let course_row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE name = ? "#,
        new_course.name
    ).fetch_one(pool).await?;

    //println!("row = {:?}", row);
    Ok(course_row)
}

pub async fn delete_course_db(
    pool: &MySqlPool, teacher_id: i32, id: i32
) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        "DELETE FROM course WHERE teacher_id = ? and id = ?",
        teacher_id,
        id,
    ).execute(pool).await?;

    Ok(format!("Deleted {:?} record", course_row))
}

pub async fn update_course_db(
    pool: &MySqlPool, teacher_id: i32, id: i32, update_course: UpdateCourse
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM course WHERE teacher_id = ? and id = ?",
        teacher_id,
        id,
    )
        .fetch_one(pool)
        .await
        .map_err(|_| MyError::NotFound("Course ID not found".into()))?;

    let name = update_course.name;

    let description = if let Some(description) = update_course.description {
        Some(description)
    }else {
        current_course_row.description
    };

    let format = if let Some(format) = update_course.format {
        Some(format)
    }else {
        current_course_row.format
    };

    let structure = if let Some(structure) = update_course.structure {
        Some(structure)
    }else {
        current_course_row.structure
    };

    let duration = if let Some(duration) = update_course.duration {
        Some(duration)
    }else {
        current_course_row.duration
    };

    let price = if let Some(price) = update_course.price {
        Some(price)
    }else {
        current_course_row.price
    };

    let language = if let Some(language) = update_course.language {
        Some(language)
    }else {
        current_course_row.language
    };

    let level = if let Some(level) = update_course.level {
        Some(level)
    }else {
        current_course_row.level
    };

    let _ = sqlx::query_as!(
        Course,
        "UPDATE course SET name = ?, description = ?, format = ?, structure = ?, duration = ?, \
        price = ?, language = ?, level = ?",
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
    ).execute(pool).await?;

    Ok(Course{
        teacher_id,
        id,
        name,
        time: current_course_row.time,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
    })
}