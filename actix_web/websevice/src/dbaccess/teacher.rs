use sqlx::MySqlPool;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};

pub async fn get_all_teachers_db(pool: &MySqlPool) ->Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT * FROM teacher")
        .fetch_all(pool).await?;

    let teachers: Vec<Teacher> = rows.iter().map(|r| Teacher {
        id: r.id,
        name: r.name.clone(),
        picture_url: r.picture_url.clone(),
        profile: r.profile.clone(),
    }).collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_details_db(pool: &MySqlPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!("SELECT * FROM teacher WHERE id = ?", teacher_id)
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone(),
        })
        .map_err(|_| MyError::NotFound("Teacher id not found".into()))?;

    Ok(row)
}

pub async fn post_new_teacher_db(pool: &MySqlPool, new_teacher: CreateTeacher) -> Result<Teacher, MyError> {
    let post_row = sqlx::query!(
        "INSERT INTO teacher (name, picture_url, profile)\
        VALUE (?, ?, ?)",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile,
    ).execute(pool).await?;

    let row = sqlx::query!(
        "SELECT * FROM teacher WHERE id = ?", post_row.last_insert_id()
    ).fetch_one(pool).await?;

    Ok(Teacher{
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile,
    })
}

pub async fn update_teacher_details_db(
    pool: &MySqlPool, teacher_id: i32, update_teacher: UpdateTeacher
) -> Result<Teacher, MyError> {
    let row = sqlx::query_as!(Teacher, "SELECT * FROM teacher WHERE id = ?", teacher_id)
        .fetch_one(pool).await.map_err(|_|MyError::NotFound("Teacher id not found".into()))?;

    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = update_teacher.name {
            name
        }else {
            row.name.clone()
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        }else {
            row.picture_url.clone()
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        }else {
            row.profile.clone()
        },
    };

    let _update_row = sqlx::query!(
        "UPDATE teacher SET name = ?, picture_url = ?, profile = ? WHERE id = ?",
        temp.name, temp.picture_url, temp.profile, temp.id)
        .execute(pool).await.map_err(|_| MyError::DBError("Update teacher failed".into()))?;

    let teacher_row = sqlx::query!("SELECT * FROM teacher where id = ?", row.id)
        .fetch_one(pool).await.map_err(|_|MyError::NotFound("Updated teacher not found".into()))?;

    Ok(Teacher{
        id: teacher_row.id,
        name: teacher_row.name.clone(),
        picture_url: teacher_row.picture_url.clone(),
        profile: teacher_row.profile.clone(),
    })
}

pub async fn delete_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query!("DELETE FROM teacher WHERE id = ?", teacher_id)
        .execute(pool).await.map_err(|_|MyError::DBError("Unable to delete teacher".into()))?;

    Ok(format!("Delete {:?} record", row))
}