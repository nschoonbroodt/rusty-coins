create table if not exists transactions (
    id integer primary key autoincrement,
    date text not null,
    description text,
    created_at text default current_timestamp,
    updated_at text default current_timestamp
) strict;

create trigger insert_transactions_trigger
after
insert on transactions begin
    update transactions
    set created_at = current_timestamp
    where id = NEW.id;
end;

create trigger update_transactions_trigger
after
update on transactions begin
    update transactions
    set updated_at = current_timestamp
    where id = NEW.id;
end;