create table if not exists prices (
    id integer primary key autoincrement,
    commodity_id integer not null,
    base_commodity_id integer not null,
    price real not null,
    date text not null,
    created_at text default current_timestamp,
    updated_at text default current_timestamp,
    foreign key (commodity_id) references commodities(id),
    foreign key (base_commodity_id) references commodities(id),
    unique(commodity_id, base_commodity_id, date)
) strict;

create trigger insert_prices_trigger
after
insert on prices begin
    update prices
    set created_at = current_timestamp
    where id = NEW.id;
end;

create trigger update_prices_trigger
after
update on prices begin
    update prices
    set updated_at = current_timestamp
    where id = NEW.id;
end;