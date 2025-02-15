A simple crate that parses geosite.dat file format defined in 
https://github.com/v2fly/v2ray-core/blob/master/app/router/routercommon/common.proto

dat files can be found at
https://github.com/Loyalsoldier/v2ray-rules-dat
and
https://github.com/v2fly/domain-list-community


Also has a rusqlite feature(disabled by default) to convert the format
from and into sqlite format. The database is compatible with the database
defined in the crate "clash_rules", which can be used to do crud with the data.

commented out code in build.rs shows how to generate the protobuf rust code.
