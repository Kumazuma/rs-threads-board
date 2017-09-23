# rs-threads-board
## table schema
### tb_users
```sql
CREATE TABLE `tb_users` (
	`uid` INT(11) NOT NULL AUTO_INCREMENT,
	`email` VARCHAR(32) NOT NULL DEFAULT '0',
	`nickname` VARCHAR(32) NOT NULL DEFAULT '0',
	`password` VARCHAR(128) NOT NULL DEFAULT '0',
	PRIMARY KEY (`uid`),
	UNIQUE INDEX `email` (`email`)
);
```
### tb_threads
```sql
CREATE TABLE `tb_threads` (
	`uid` INT(11) NOT NULL AUTO_INCREMENT,
	`opener_uid` INT(11) NOT NULL DEFAULT '0',
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
	CONSTRAINT `t_tags_fk_1` FOREIGN KEY (`thread_uid`) REFERENCES `tb_threads` (`uid`)
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
	CONSTRAINT `FK__tb_threads` FOREIGN KEY (`thread_uid`) REFERENCES `tb_threads` (`uid`)
);
```