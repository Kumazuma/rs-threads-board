# rs-threads-board
## table schema
### tb_users
```sql
CREATE TABLE tb_users (
	uid SERIAL PRIMARY KEY,
	email VARCHAR(32) UNIQUE NOT NULL DEFAULT '0',
	nickname VARCHAR(32) NOT NULL DEFAULT '0',
	password VARCHAR(128) NOT NULL DEFAULT '0'
);
```
### tb_threads
```sql
CREATE TABLE tb_threads (
	uid SERIAL NOT NULL,
	opener_uid INT NOT NULL,
	opener_nickname VARCHAR(32) NOT NULL,
	subject VARCHAR(64) NOT NULL,
	created_datetime timestamp NOT NULL,
	PRIMARY KEY (uid),
	CONSTRAINT FK__tb_users FOREIGN KEY (opener_uid) REFERENCES tb_users (uid)
);
```
### tb_tags
```sql
CREATE TABLE tb_tags (
	tag_name VARCHAR(64) NOT NULL,
	thread_uid INT NOT NULL,
	PRIMARY KEY (tag_name, thread_uid),
	CONSTRAINT t_tags_fk_1 FOREIGN KEY (thread_uid) REFERENCES tb_threads (uid)
);
```
### tb_comments
```sql
CREATE TABLE tb_comments (
	uid SERIAL NOT NULL,
	thread_uid INT NOT NULL,
	writer_uid INT NOT NULL,
	write_datetime timestamp NOT NULL,
	comment TEXT NOT NULL,
	PRIMARY KEY (uid),
	CONSTRAINT FK5_tb_comments_tb_users FOREIGN KEY (writer_uid) REFERENCES tb_users (uid),
	CONSTRAINT FK__tb_threads FOREIGN KEY (thread_uid) REFERENCES tb_threads (uid)
);
```

### v_thread_list
```sql
SELECT tb_threads.uid, tb_threads.subject, tb_threads.created_datetime, tb_threads.opener_uid, tb_threads.opener_nickname, tb_users.email as opener_email, recent_update
FROM tb_threads
INNER JOIN tb_users ON tb_threads.opener_uid = tb_users.uid
INNER JOIN (SELECT tb_comments.thread_uid, MAX(tb_comments.write_datetime) as recent_update FROM tb_comments GROUP BY thread_uid) as comments_max ON tb_threads.uid = comments_max.thread_uid 
ORDER BY recent_update DESC 
```