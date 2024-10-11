use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use rocket_db_pools::diesel::{AsyncPgConnection, RunQueryDsl};

pub struct BlogRepo;

impl BlogRepo {
    pub async fn load(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<BlogPosts>> {
        blog_posts::table.limit(limit).load(c).await
    }

    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<BlogPosts> {
        blog_posts::table.find(id).get_result(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_post: NewPost) -> QueryResult<BlogPosts> {
        diesel::insert_into(blog_posts::table)
            .values(new_post)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        id: i32,
        post: BlogPosts,
    ) -> QueryResult<BlogPosts> {
        diesel::update(blog_posts::table.find(id))
            .set((
                blog_posts::post_title.eq(post.post_title),
                blog_posts::post_category.eq(post.post_category),
                blog_posts::post_short_description.eq(post.post_short_description),
                blog_posts::post_description.eq(post.post_description),
                blog_posts::post_photo.eq(post.post_photo),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(blog_posts::table.find(id)).execute(c).await
    }
}
