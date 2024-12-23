-- 如果不存在 "users" 表，则创建该表
CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER primary key AUTOINCREMENT not null,
    name        text                              not null,
    email       char(20) UNIQUE                   not null,
    pass_word   char(65)                          not null,                                        -- 'passwd hash'
    create_time datetime                          not null default (datetime('now', 'localtime')), -- 'create datetime'
    update_time datetime                          not null default (datetime('now', 'localtime')), -- 'update datetime'
    status      char(10)                          not null default 'normal'                        -- comment 'status: normal, blocked, deleted'
);

-- 插入数据
-- INSERT INTO users (name, email, pass_word, status)
-- VALUES ('admin', 'admin@admin.com', 'admin', 'normal')

-- 根据 name 查询 users
-- SELECT *
-- FROM users
-- WHERE name = 'admin'