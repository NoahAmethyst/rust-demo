-- create table user
create table if not exists `user`
(
    id           int auto_increment primary key,
    account      varchar(20) default ''                null,
    password     varchar(20) default ''                not null,
    token        varchar(36) default ''                not null,
    token_expire timestamp                             null,
    create_time  timestamp   default CURRENT_TIMESTAMP not null,
    constraint account_index
    unique (account),
    constraint token_index
    unique (token)
    );

-- create table logs
create table if not exists logs
(
    id          int auto_increment,
    hash_code   bigint         default 0     not null,
    app         varchar(20) default ''    not null,
    pod         varchar(36) default ''    not null,
    content     text                      not null,
    create_time timestamp   default now() not null,
    constraint `primary`
        primary key (id),
    constraint unique_index
        unique (hash_code, pod)
);



