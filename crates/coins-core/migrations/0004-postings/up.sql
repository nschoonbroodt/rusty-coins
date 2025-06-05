create table if not exists postings (
    id integer primary key autoincrement,
    account_id integer not null,
    commodity_id integer not null,
    transaction_id integer not null,
    amount real not null,
    description text,
    created_at text default current_timestamp,
    updated_at text default current_timestamp,
    foreign key (account_id) references accounts(id),
    foreign key (commodity_id) references commodities(id),
    foreign key (transaction_id) references transactions(id)
) strict;

create trigger insert_postings_trigger
after
insert on postings begin
    update postings
    set created_at = current_timestamp
    where id = NEW.id;
end;

create trigger update_postings_trigger
after
update on postings begin
    update postings
    set updated_at = current_timestamp
    where id = NEW.id;
end;