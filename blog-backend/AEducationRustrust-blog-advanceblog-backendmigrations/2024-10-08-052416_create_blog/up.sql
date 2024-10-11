CREATE TABLE blog_posts (
    id SERIAL PRIMARY KEY,
    post_title TEXT NOT NULL UNIQUE,
    post_category TEXT NOT NULL UNIQUE,
    post_short_description TEXT NOT NULL UNIQUE,
    post_description TEXT NOT NULL,
    post_photo TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);