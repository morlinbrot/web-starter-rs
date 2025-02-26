drop collation if exists case_insensitive;

drop function if exists trigger_updated_at(regclass);

drop function if exists set_updated_at();

drop extension if exists "uuid-ossp";
