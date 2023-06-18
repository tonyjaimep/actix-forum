use super::models::Forum;
use crate::build_create_one;
use crate::build_find_one;
use crate::build_get_all;
use crate::build_update;

build_get_all!(get_all_forums, forums, Forum);
build_create_one!(create_forum, forums, Forum, Forum);
build_find_one!(find_one_forum, forums, Forum, String);
build_update!(update_forum, forums, Forum, String);
