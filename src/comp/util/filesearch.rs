// A module for searching for libraries

import std::option;
import std::fs;
import std::vec;
import std::str;
import back::link;

export filesearch;
export mk_filesearch;
export pick;
export search;

type pick<@T> = block(path: fs::path) -> option::t<T>;

type filesearch = obj {
    fn sysroot() -> fs::path;
    fn lib_search_paths() -> [fs::path];
    fn get_target_lib_path() -> fs::path;
    fn get_target_lib_file_path(file: fs::path) -> fs::path;
};

fn mk_filesearch(binary_name: fs::path,
                 maybe_sysroot: option::t<fs::path>,
                 target_triple: str,
                 addl_lib_search_paths: [fs::path]) -> filesearch {
    obj filesearch_impl(sysroot: fs::path,
                        addl_lib_search_paths: [fs::path],
                        target_triple: str) {
        fn sysroot() -> fs::path { sysroot }
        fn lib_search_paths() -> [fs::path] {
            addl_lib_search_paths
                + [make_target_lib_path(sysroot, target_triple)]
        }

        fn get_target_lib_path() -> fs::path {
            make_target_lib_path(sysroot, target_triple)
        }

        fn get_target_lib_file_path(file: fs::path) -> fs::path {
            fs::connect(self.get_target_lib_path(), file)
        }
    }

    let sysroot = get_sysroot(maybe_sysroot, binary_name);
    log #fmt("using sysroot = %s", sysroot);
    ret filesearch_impl(sysroot, addl_lib_search_paths, target_triple);
}

// FIXME #1001: This can't be an obj method
fn search<@T>(filesearch: filesearch, pick: pick<T>) -> option::t<T> {
    for lib_search_path in filesearch.lib_search_paths() {
        log #fmt["searching %s", lib_search_path];
        for path in fs::list_dir(lib_search_path) {
            log #fmt["testing %s", path];
            let maybe_picked = pick(path);
            if option::is_some(maybe_picked) {
                log #fmt("picked %s", path);
                ret maybe_picked;
            } else {
                log #fmt("rejected %s", path);
            }
        }
    }
    ret option::none;
}

fn make_target_lib_path(sysroot: fs::path,
                        target_triple: str) -> fs::path {
    let path = [sysroot, "lib/rustc", target_triple, "lib"];
    check vec::is_not_empty(path);
    let path = fs::connect_many(path);
    ret path;
}

fn get_default_sysroot(binary: fs::path) -> fs::path {
    let dirname = fs::dirname(binary);
    if str::eq(dirname, binary) { ret "../"; }
    ret fs::connect(dirname, "../");
}

fn get_sysroot(maybe_sysroot: option::t<fs::path>,
               binary: fs::path) -> fs::path {
    alt maybe_sysroot {
      option::some(sr) { sr }
      option::none. { get_default_sysroot(binary) }
    }
}