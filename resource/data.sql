-- create first user
insert ignore into `user` (id, account, password, token, token_expire)
values  (1, 'admin', '123456', '', null);