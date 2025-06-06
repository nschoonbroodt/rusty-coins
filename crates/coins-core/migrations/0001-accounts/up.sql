create table if not exists accounts (
    id integer primary key autoincrement,
    name text not null,
    created_at text default current_timestamp,
    updated_at text default current_timestamp
) strict;

create trigger insert_accounts_trigger
after
insert on accounts begin
    update accounts
    set created_at = current_timestamp
    where id = NEW.id;
end;

create trigger update_accounts_trigger
after
update on accounts begin
    update accounts
    set updated_at = current_timestamp
    where id = NEW.id;
end;

insert into accounts (name) values ('Assets'),
                                   ('Liabilities'), 
                                   ('Equity'), 
                                   ('Revenue'), 
                                   ('Expenses');