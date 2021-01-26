mod create_file;
mod detach_all_files_from_namespace;
mod detach_all_trashed_files_from_namespace;
mod find_all_trashed_files;
mod find_children;
mod find_children_recursively;
mod find_explicitly_trashed_files;
mod find_file_by_id;
mod find_file_by_parent_and_name;
mod find_files_by_ids;
mod find_root_file_for_namespace;
mod get_file_path;
mod update_file;

#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
