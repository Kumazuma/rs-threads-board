# rs-threads-board
## table schema
### tb_users
```sql
CREATE TABLE `tb_users` (
	`uid` INT(11) NOT NULL AUTO_INCREMENT,
	`email` VARCHAR(32) NOT NULL,
	`nickname` VARCHAR(32) NOT NULL,
	`password` VARCHAR(128) NOT NULL ,
	PRIMARY KEY (`uid`),
	UNIQUE INDEX `email` (`email`)
);
```
### tb_threads
```sql
CREATE TABLE `tb_threads` (
	`uid` INT(11) NOT NULL AUTO_INCREMENT,
	`opener_uid` INT(11) NOT NULL,
	`opener_nickname` VARCHAR(32) NOT NULL,
	`subject` VARCHAR(64) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`created_datetime` DATETIME NOT NULL,
	PRIMARY KEY (`uid`),
	INDEX `FK__tb_users` (`opener_uid`),
	CONSTRAINT `FK__tb_users` FOREIGN KEY (`opener_uid`) REFERENCES `tb_users` (`uid`)
);
```
### tb_tags
```sql
CREATE TABLE `tb_tags` (
	`tag_name` VARCHAR(64) NOT NULL,
	`thread_uid` INT(11) NOT NULL,
	PRIMARY KEY (`tag_name`, `thread_uid`),
	INDEX `t_tags_fk_1` (`thread_uid`, `tag_name`),
	CONSTRAINT `t_tags_fk_1` FOREIGN KEY (`thread_uid`) REFERENCES `tb_threads` (`uid`)  ON DELETE CASCADE
);
```
### tb_comments
```sql
CREATE TABLE `tb_comments` (
	`uid` INT(11) NOT NULL AUTO_INCREMENT,
	`thread_uid` INT(11) NOT NULL,
	`writer_uid` INT(11) NOT NULL,
	`write_datetime` DATETIME NOT NULL,
	`comment` TEXT NOT NULL COLLATE 'utf8mb4_unicode_ci',
	PRIMARY KEY (`uid`),
	INDEX `FK_tb_comments_tb_users` (`thread_uid`),
	INDEX `FK5_tb_comments_tb_users` (`writer_uid`),
	CONSTRAINT `FK5_tb_comments_tb_users` FOREIGN KEY (`writer_uid`) REFERENCES `tb_users` (`uid`),
	CONSTRAINT `FK__tb_threads` FOREIGN KEY (`thread_uid`) REFERENCES `tb_threads` (`uid`)  ON DELETE CASCADE
);
```
### v_thread_last_update
```sql
select `tb_comments`.`thread_uid` AS `thread_uid`,max(`tb_comments`.`write_datetime`) AS `recent_update` from `tb_comments` group by `tb_comments`.`thread_uid`
```
### v_thread_list
```sql
SELECT tb_threads.uid, tb_threads.subject, tb_threads.created_datetime, tb_threads.opener_uid, tb_threads.opener_nickname, tb_users.email as opener_email, recent_update
FROM tb_threads
INNER JOIN tb_users ON tb_threads.opener_uid = tb_users.uid
INNER JOIN (SELECT tb_comments.thread_uid, MAX(tb_comments.write_datetime) as `recent_update` FROM tb_comments GROUP BY thread_uid) as `comments_max` ON tb_threads.uid = comments_max.thread_uid 
ORDER BY recent_update DESC 
```
### v_comments
```sql
SELECT tb_comments.thread_uid as thread_uid, tb_comments.uid, tb_users.uid as user_uid, tb_users.nickname as user_nickname, tb_users.email as user_email, tb_comments.write_datetime, tb_comments.`comment` FROM tb_comments
JOIN tb_users ON tb_comments.writer_uid = tb_users.uid ORDER BY write_datetime ASC
```

### v_tag_threads_count_list
```sql
SELECT tag_name, COUNT(*) FROM tb_tags GROUP BY tag_name 
```

### v_tags
```SQL
SELECT tb_tags.tag_name, tb_tags.thread_uid, v_thread_list.recent_update FROM tb_tags JOIN v_thread_list ON tb_tags.thread_uid = v_thread_list.uid ORDER BY recent_update DESC 
```