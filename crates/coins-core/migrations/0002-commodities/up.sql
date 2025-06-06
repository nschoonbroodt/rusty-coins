create table if not exists commodities (
    id integer primary key autoincrement,
    name text not null,
    symbol text not null unique,
    created_at text default current_timestamp,
    updated_at text default current_timestamp
) strict;

create trigger insert_commodities_trigger
after
insert on commodities begin
    update commodities
    set created_at = current_timestamp
    where id = NEW.id;
end;

create trigger update_commodities_trigger
after
update on commodities begin
    update commodities
    set updated_at = current_timestamp
    where id = NEW.id;
end;