#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type re_dfa_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut flags: [libc::c_uint; 4];
    static mut openfile: *mut openfilestruct;
    static mut search_history: *mut linestruct;
    static mut replace_history: *mut linestruct;
    static mut execute_history: *mut linestruct;
    static mut searchtop: *mut linestruct;
    static mut searchbot: *mut linestruct;
    static mut replacetop: *mut linestruct;
    static mut replacebot: *mut linestruct;
    static mut executetop: *mut linestruct;
    static mut executebot: *mut linestruct;
    static mut homedir: *mut libc::c_char;
    static mut statedir: *mut libc::c_char;
    fn revstrstr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
        pointer: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn get_full_path(origpath: *const libc::c_char) -> *mut libc::c_char;
    fn make_new_node(prevnode: *mut linestruct) -> *mut linestruct;
    fn splice_node(afterthis: *mut linestruct, newnode: *mut linestruct);
    fn unlink_node(line: *mut linestruct);
    fn renumber_from(line: *mut linestruct);
    fn jot_error(msg: *const libc::c_char, _: ...);
    fn goto_line_and_column(line: ssize_t, column: ssize_t, hugfloor: bool);
    fn get_homedir();
    fn concatenate(
        path: *const libc::c_char,
        name: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn recode_NUL_to_LF(string: *mut libc::c_char, length: size_t);
    fn recode_LF_to_NUL(string: *mut libc::c_char) -> size_t;
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn nrealloc(ptr: *mut libc::c_void, howmuch: size_t) -> *mut libc::c_void;
    fn measured_copy(string: *const libc::c_char, count: size_t) -> *mut libc::c_char;
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn xplustabs() -> size_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type format_type = libc::c_uint;
pub const MAC_FILE: format_type = 3;
pub const DOS_FILE: format_type = 2;
pub const NIX_FILE: format_type = 1;
pub const UNSPECIFIED: format_type = 0;
pub type undo_type = libc::c_uint;
pub const OTHER: undo_type = 21;
pub const COUPLE_END: undo_type = 20;
pub const COUPLE_BEGIN: undo_type = 19;
pub const INSERT: undo_type = 18;
pub const PASTE: undo_type = 17;
pub const COPY: undo_type = 16;
pub const CUT_TO_EOF: undo_type = 15;
pub const CUT: undo_type = 14;
pub const ZAP: undo_type = 13;
pub const PREFLIGHT: undo_type = 12;
pub const UNCOMMENT: undo_type = 11;
pub const COMMENT: undo_type = 10;
pub const UNINDENT: undo_type = 9;
pub const INDENT: undo_type = 8;
pub const SPLIT_END: undo_type = 7;
pub const SPLIT_BEGIN: undo_type = 6;
pub const REPLACE: undo_type = 5;
pub const JOIN: undo_type = 4;
pub const DEL: undo_type = 3;
pub const BACK: undo_type = 2;
pub const ENTER: undo_type = 1;
pub const ADD: undo_type = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const MODERN_BINDINGS: C2RustUnnamed = 50;
pub const ZERO: C2RustUnnamed = 49;
pub const MINIBAR: C2RustUnnamed = 48;
pub const USE_MAGIC: C2RustUnnamed = 47;
pub const STATEFLAGS: C2RustUnnamed = 46;
pub const COLON_PARSING: C2RustUnnamed = 45;
pub const BOOKSTYLE: C2RustUnnamed = 44;
pub const INDICATOR: C2RustUnnamed = 43;
pub const EMPTY_LINE: C2RustUnnamed = 42;
pub const JUMPY_SCROLLING: C2RustUnnamed = 41;
pub const BREAK_LONG_LINES: C2RustUnnamed = 40;
pub const LET_THEM_ZAP: C2RustUnnamed = 39;
pub const AFTER_ENDS: C2RustUnnamed = 38;
pub const AT_BLANKS: C2RustUnnamed = 37;
pub const LINE_NUMBERS: C2RustUnnamed = 36;
pub const SHOW_CURSOR: C2RustUnnamed = 35;
pub const TRIM_BLANKS: C2RustUnnamed = 34;
pub const MAKE_IT_UNIX: C2RustUnnamed = 33;
pub const NOREAD_MODE: C2RustUnnamed = 32;
pub const LOCKING: C2RustUnnamed = 31;
pub const POSITIONLOG: C2RustUnnamed = 30;
pub const SOFTWRAP: C2RustUnnamed = 29;
pub const BOLD_TEXT: C2RustUnnamed = 28;
pub const NO_NEWLINES: C2RustUnnamed = 27;
pub const WORD_BOUNDS: C2RustUnnamed = 26;
pub const QUICK_BLANK: C2RustUnnamed = 25;
pub const TABS_TO_SPACES: C2RustUnnamed = 24;
pub const WHITESPACE_DISPLAY: C2RustUnnamed = 23;
pub const SMART_HOME: C2RustUnnamed = 22;
pub const RESTRICTED: C2RustUnnamed = 21;
pub const HISTORYLOG: C2RustUnnamed = 20;
pub const PRESERVE: C2RustUnnamed = 19;
pub const NO_SYNTAX: C2RustUnnamed = 18;
pub const INSECURE_BACKUP: C2RustUnnamed = 17;
pub const MAKE_BACKUP: C2RustUnnamed = 16;
pub const NO_CONVERT: C2RustUnnamed = 15;
pub const RAW_SEQUENCES: C2RustUnnamed = 14;
pub const REBIND_DELETE: C2RustUnnamed = 13;
pub const MULTIBUFFER: C2RustUnnamed = 12;
pub const BACKWARDS_SEARCH: C2RustUnnamed = 11;
pub const CUT_FROM_CURSOR: C2RustUnnamed = 10;
pub const SAVE_ON_EXIT: C2RustUnnamed = 9;
pub const USE_REGEXP: C2RustUnnamed = 8;
pub const USE_MOUSE: C2RustUnnamed = 7;
pub const VIEW_MODE: C2RustUnnamed = 6;
pub const AUTOINDENT: C2RustUnnamed = 5;
pub const NO_WRAP: C2RustUnnamed = 4;
pub const NO_HELP: C2RustUnnamed = 3;
pub const CONSTANT_SHOW: C2RustUnnamed = 2;
pub const CASE_SENSITIVE: C2RustUnnamed = 1;
pub const DONTUSE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colortype {
    pub id: libc::c_short,
    pub fg: libc::c_short,
    pub bg: libc::c_short,
    pub pairnum: libc::c_short,
    pub attributes: libc::c_int,
    pub start: *mut regex_t,
    pub end: *mut regex_t,
    pub next: *mut colortype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regexlisttype {
    pub one_rgx: *mut regex_t,
    pub next: *mut regexlisttype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct augmentstruct {
    pub filename: *mut libc::c_char,
    pub lineno: ssize_t,
    pub data: *mut libc::c_char,
    pub next: *mut augmentstruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct syntaxtype {
    pub name: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub lineno: size_t,
    pub augmentations: *mut augmentstruct,
    pub extensions: *mut regexlisttype,
    pub headers: *mut regexlisttype,
    pub magics: *mut regexlisttype,
    pub linter: *mut libc::c_char,
    pub formatter: *mut libc::c_char,
    pub tabstring: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub color: *mut colortype,
    pub multiscore: libc::c_short,
    pub next: *mut syntaxtype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linestruct {
    pub data: *mut libc::c_char,
    pub lineno: ssize_t,
    pub next: *mut linestruct,
    pub prev: *mut linestruct,
    pub multidata: *mut libc::c_short,
    pub has_anchor: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct groupstruct {
    pub top_line: ssize_t,
    pub bottom_line: ssize_t,
    pub indentations: *mut *mut libc::c_char,
    pub next: *mut groupstruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct undostruct {
    pub type_0: undo_type,
    pub xflags: libc::c_int,
    pub head_lineno: ssize_t,
    pub head_x: size_t,
    pub strdata: *mut libc::c_char,
    pub wassize: size_t,
    pub newsize: size_t,
    pub grouping: *mut groupstruct,
    pub cutbuffer: *mut linestruct,
    pub tail_lineno: ssize_t,
    pub tail_x: size_t,
    pub next: *mut undostruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poshiststruct {
    pub filename: *mut libc::c_char,
    pub linenumber: ssize_t,
    pub columnnumber: ssize_t,
    pub anchors: *mut libc::c_char,
    pub next: *mut poshiststruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct openfilestruct {
    pub filename: *mut libc::c_char,
    pub filetop: *mut linestruct,
    pub filebot: *mut linestruct,
    pub edittop: *mut linestruct,
    pub current: *mut linestruct,
    pub totsize: size_t,
    pub firstcolumn: size_t,
    pub current_x: size_t,
    pub placewewant: size_t,
    pub cursor_row: ssize_t,
    pub statinfo: *mut stat,
    pub spillage_line: *mut linestruct,
    pub mark: *mut linestruct,
    pub mark_x: size_t,
    pub softmark: bool,
    pub fmt: format_type,
    pub lock_filename: *mut libc::c_char,
    pub undotop: *mut undostruct,
    pub current_undo: *mut undostruct,
    pub last_saved: *mut undostruct,
    pub last_action: undo_type,
    pub modified: bool,
    pub syntax: *mut syntaxtype,
    pub errormessage: *mut libc::c_char,
    pub next: *mut openfilestruct,
    pub prev: *mut openfilestruct,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
static mut history_changed: bool = 0 as libc::c_int != 0;
static mut poshistname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut latest_timestamp: time_t = 942927132 as libc::c_int as time_t;
static mut position_history: *mut poshiststruct = 0 as *const poshiststruct
    as *mut poshiststruct;
#[no_mangle]
pub unsafe extern "C" fn history_init() {
    search_history = make_new_node(0 as *mut linestruct);
    (*search_history).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    searchtop = search_history;
    searchbot = search_history;
    replace_history = make_new_node(0 as *mut linestruct);
    (*replace_history).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    replacetop = replace_history;
    replacebot = replace_history;
    execute_history = make_new_node(0 as *mut linestruct);
    (*execute_history).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    executetop = execute_history;
    executebot = execute_history;
}
#[no_mangle]
pub unsafe extern "C" fn reset_history_pointer_for(mut item: *const linestruct) {
    if item == search_history as *const linestruct {
        search_history = searchbot;
    } else if item == replace_history as *const linestruct {
        replace_history = replacebot;
    } else if item == execute_history as *const linestruct {
        execute_history = executebot;
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_in_history(
    mut start: *const linestruct,
    mut end: *const linestruct,
    mut text: *const libc::c_char,
    mut len: size_t,
) -> *mut linestruct {
    let mut item: *const linestruct = 0 as *const linestruct;
    item = start;
    while item != (*end).prev as *const linestruct && !item.is_null() {
        if strncmp((*item).data, text, len) == 0 as libc::c_int {
            return item as *mut linestruct;
        }
        item = (*item).prev;
    }
    return 0 as *mut linestruct;
}
#[no_mangle]
pub unsafe extern "C" fn update_history(
    mut item: *mut *mut linestruct,
    mut text: *const libc::c_char,
    mut avoid_duplicates: bool,
) {
    let mut htop: *mut *mut linestruct = 0 as *mut *mut linestruct;
    let mut hbot: *mut *mut linestruct = 0 as *mut *mut linestruct;
    let mut thesame: *mut linestruct = 0 as *mut linestruct;
    if *item == search_history {
        htop = &mut searchtop;
        hbot = &mut searchbot;
    } else if *item == replace_history {
        htop = &mut replacetop;
        hbot = &mut replacebot;
    } else if *item == execute_history {
        htop = &mut executetop;
        hbot = &mut executebot;
    }
    if avoid_duplicates {
        thesame = find_in_history(
            *hbot,
            *htop,
            text,
            !(0 as libc::c_int as size_t) >> 1 as libc::c_int,
        );
    }
    if !thesame.is_null() {
        let mut after: *mut linestruct = (*thesame).next;
        if thesame == *htop {
            *htop = after;
        }
        unlink_node(thesame);
        renumber_from(after);
    }
    if (**hbot).lineno == (100 as libc::c_int + 1 as libc::c_int) as ssize_t {
        let mut oldest: *mut linestruct = *htop;
        *htop = (**htop).next;
        unlink_node(oldest);
        renumber_from(*htop);
    }
    (**hbot).data = mallocstrcpy((**hbot).data, text);
    splice_node(*hbot, make_new_node(*hbot));
    *hbot = (**hbot).next;
    (**hbot).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    history_changed = 1 as libc::c_int != 0;
    *item = *hbot;
}
#[no_mangle]
pub unsafe extern "C" fn get_history_completion(
    mut here: *mut *mut linestruct,
    mut string: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut htop: *mut linestruct = 0 as *mut linestruct;
    let mut hbot: *mut linestruct = 0 as *mut linestruct;
    let mut item: *mut linestruct = 0 as *mut linestruct;
    if *here == search_history {
        htop = searchtop;
        hbot = searchbot;
    } else if *here == replace_history {
        htop = replacetop;
        hbot = replacebot;
    } else if *here == execute_history {
        htop = executetop;
        hbot = executebot;
    }
    item = find_in_history((**here).prev, htop, string, len);
    while !item.is_null() && strcmp((*item).data, string) == 0 as libc::c_int {
        item = find_in_history((*item).prev, htop, string, len);
    }
    if !item.is_null() {
        *here = item;
        return mallocstrcpy(string, (*item).data);
    }
    item = find_in_history(hbot, *here, string, len);
    while !item.is_null() && strcmp((*item).data, string) == 0 as libc::c_int {
        item = find_in_history((*item).prev, *here, string, len);
    }
    if !item.is_null() {
        *here = item;
        return mallocstrcpy(string, (*item).data);
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn have_statedir() -> bool {
    let mut xdgdatadir: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirinfo: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    get_homedir();
    if !homedir.is_null() {
        statedir = concatenate(
            homedir,
            b"/.nano/\0" as *const u8 as *const libc::c_char,
        );
        if stat(statedir, &mut dirinfo) == 0 as libc::c_int
            && dirinfo.st_mode & 0o170000 as libc::c_int as __mode_t
                == 0o40000 as libc::c_int as __mode_t
        {
            poshistname = concatenate(
                statedir,
                b"filepos_history\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int != 0;
        }
    }
    free(statedir as *mut libc::c_void);
    xdgdatadir = getenv(b"XDG_DATA_HOME\0" as *const u8 as *const libc::c_char);
    if homedir.is_null() && xdgdatadir.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !xdgdatadir.is_null() {
        statedir = concatenate(
            xdgdatadir,
            b"/nano/\0" as *const u8 as *const libc::c_char,
        );
    } else {
        statedir = concatenate(
            homedir,
            b"/.local/share/nano/\0" as *const u8 as *const libc::c_char,
        );
    }
    if stat(statedir, &mut dirinfo) == -(1 as libc::c_int) {
        if xdgdatadir.is_null() {
            let mut statepath: *mut libc::c_char = concatenate(
                homedir,
                b"/.local\0" as *const u8 as *const libc::c_char,
            );
            mkdir(
                statepath,
                (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                    as __mode_t,
            );
            free(statepath as *mut libc::c_void);
            statepath = concatenate(
                homedir,
                b"/.local/share\0" as *const u8 as *const libc::c_char,
            );
            mkdir(
                statepath,
                (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    as __mode_t,
            );
            free(statepath as *mut libc::c_void);
        }
        if mkdir(
            statedir,
            (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                as __mode_t,
        ) == -(1 as libc::c_int)
        {
            jot_error(
                b"Unable to create directory %s: %s\nIt is required for saving/loading search history or cursor positions.\n\0"
                    as *const u8 as *const libc::c_char,
                statedir,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int != 0;
        }
    } else if !(dirinfo.st_mode & 0o170000 as libc::c_int as __mode_t
        == 0o40000 as libc::c_int as __mode_t)
    {
        jot_error(
            b"Path %s is not a directory and needs to be.\nNano will be unable to load or save search history or cursor positions.\n\0"
                as *const u8 as *const libc::c_char,
            statedir,
        );
        return 0 as libc::c_int != 0;
    }
    poshistname = concatenate(
        statedir,
        b"filepos_history\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn load_history() {
    let mut histname: *mut libc::c_char = concatenate(
        statedir,
        b"search_history\0" as *const u8 as *const libc::c_char,
    );
    let mut histfile: *mut FILE = fopen(
        histname,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if histfile.is_null() && *__errno_location() != 2 as libc::c_int {
        jot_error(
            b"Error reading %s: %s\0" as *const u8 as *const libc::c_char,
            histname,
            strerror(*__errno_location()),
        );
        flags[(HISTORYLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (HISTORYLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
    }
    if histfile.is_null() {
        free(histname as *mut libc::c_void);
        return;
    }
    let mut history: *mut *mut linestruct = &mut search_history;
    let mut stanza: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dummy: size_t = 0 as libc::c_int as size_t;
    let mut read: ssize_t = 0;
    loop {
        read = getline(&mut stanza, &mut dummy, histfile);
        if !(read > 0 as libc::c_int as ssize_t) {
            break;
        }
        read -= 1;
        *stanza.offset(read as isize) = '\0' as i32 as libc::c_char;
        if read > 0 as libc::c_int as ssize_t {
            recode_NUL_to_LF(stanza, read as size_t);
            update_history(history, stanza, 0 as libc::c_int != 0);
        } else if history == &mut search_history as *mut *mut linestruct {
            history = &mut replace_history;
        } else {
            history = &mut execute_history;
        }
    }
    if fclose(histfile) == -(1 as libc::c_int) {
        jot_error(
            b"Error reading %s: %s\0" as *const u8 as *const libc::c_char,
            histname,
            strerror(*__errno_location()),
        );
    }
    free(histname as *mut libc::c_void);
    free(stanza as *mut libc::c_void);
    history_changed = 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn write_list(
    mut head: *const linestruct,
    mut histfile: *mut FILE,
) -> bool {
    let mut item: *const linestruct = 0 as *const linestruct;
    item = head;
    while !item.is_null() {
        let mut length: size_t = recode_LF_to_NUL((*item).data);
        if fwrite(
            (*item).data as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            length,
            histfile,
        ) < length
        {
            return 0 as libc::c_int != 0;
        }
        if putc('\n' as i32, histfile) == -(1 as libc::c_int) {
            return 0 as libc::c_int != 0;
        }
        item = (*item).next;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn save_history() {
    let mut histname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut histfile: *mut FILE = 0 as *mut FILE;
    if !history_changed {
        return;
    }
    histname = concatenate(
        statedir,
        b"search_history\0" as *const u8 as *const libc::c_char,
    );
    histfile = fopen(histname, b"wb\0" as *const u8 as *const libc::c_char);
    if histfile.is_null() {
        jot_error(
            b"Error writing %s: %s\0" as *const u8 as *const libc::c_char,
            histname,
            strerror(*__errno_location()),
        );
        free(histname as *mut libc::c_void);
        return;
    }
    if chmod(histname, (0o400 as libc::c_int | 0o200 as libc::c_int) as __mode_t)
        < 0 as libc::c_int
    {
        jot_error(
            b"Cannot limit permissions on %s: %s\0" as *const u8 as *const libc::c_char,
            histname,
            strerror(*__errno_location()),
        );
    }
    if !write_list(searchtop, histfile) || !write_list(replacetop, histfile)
        || !write_list(executetop, histfile)
    {
        jot_error(
            b"Error writing %s: %s\0" as *const u8 as *const libc::c_char,
            histname,
            strerror(*__errno_location()),
        );
    }
    if fclose(histfile) == -(1 as libc::c_int) {
        jot_error(
            b"Error writing %s: %s\0" as *const u8 as *const libc::c_char,
            histname,
            strerror(*__errno_location()),
        );
    }
    free(histname as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn stringify_anchors() -> *mut libc::c_char {
    let mut string: *mut libc::c_char = copy_of(
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut number: [libc::c_char; 24] = [0; 24];
    let mut line: *mut linestruct = (*openfile).filetop;
    while !line.is_null() {
        if (*line).has_anchor {
            sprintf(
                number.as_mut_ptr(),
                b"%zi \0" as *const u8 as *const libc::c_char,
                (*line).lineno,
            );
            string = nrealloc(
                string as *mut libc::c_void,
                (strlen(string))
                    .wrapping_add(strlen(number.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcat(string, number.as_mut_ptr());
        }
        line = (*line).next;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn restore_anchors(mut string: *mut libc::c_char) {
    let mut line: *mut linestruct = (*openfile).filetop;
    let mut number: ssize_t = 0;
    let mut space: *mut libc::c_char = 0 as *mut libc::c_char;
    while *string != 0 {
        space = strchr(string, ' ' as i32);
        if space.is_null() {
            return;
        }
        *space = '\0' as i32 as libc::c_char;
        number = atoi(string) as ssize_t;
        string = space.offset(1 as libc::c_int as isize);
        while (*line).lineno < number {
            line = (*line).next;
            if line.is_null() {
                return;
            }
        }
        (*line).has_anchor = 1 as libc::c_int != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn load_poshistory() {
    let mut histfile: *mut FILE = fopen(
        poshistname,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if histfile.is_null() && *__errno_location() != 2 as libc::c_int {
        jot_error(
            b"Error reading %s: %s\0" as *const u8 as *const libc::c_char,
            poshistname,
            strerror(*__errno_location()),
        );
        flags[(POSITIONLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (POSITIONLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
    }
    if histfile.is_null() {
        return;
    }
    let mut lastitem: *mut poshiststruct = 0 as *mut poshiststruct;
    let mut newitem: *mut poshiststruct = 0 as *mut poshiststruct;
    let mut stanza: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lineptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut columnptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut phrase: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileinfo: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut dummy: size_t = 0 as libc::c_int as size_t;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut length: ssize_t = 0;
    loop {
        let fresh0 = count;
        count = count + 1;
        if !(fresh0 < 200 as libc::c_int
            && {
                length = getline(&mut phrase, &mut dummy, histfile);
                length > 1 as libc::c_int as ssize_t
            })
        {
            break;
        }
        stanza = strchr(phrase, '/' as i32);
        length
            -= if !stanza.is_null() {
                stanza.offset_from(phrase) as libc::c_long
            } else {
                0 as libc::c_int as libc::c_long
            };
        recode_NUL_to_LF(stanza, length as size_t);
        columnptr = revstrstr(
            stanza,
            b" \0" as *const u8 as *const libc::c_char,
            stanza.offset(length as isize).offset(-(3 as libc::c_int as isize)),
        );
        if columnptr.is_null() {
            continue;
        }
        lineptr = revstrstr(
            stanza,
            b" \0" as *const u8 as *const libc::c_char,
            columnptr.offset(-(2 as libc::c_int as isize)),
        );
        if lineptr.is_null() {
            continue;
        }
        let fresh1 = columnptr;
        columnptr = columnptr.offset(1);
        *fresh1 = '\0' as i32 as libc::c_char;
        let fresh2 = lineptr;
        lineptr = lineptr.offset(1);
        *fresh2 = '\0' as i32 as libc::c_char;
        newitem = nmalloc(::core::mem::size_of::<poshiststruct>() as libc::c_ulong)
            as *mut poshiststruct;
        (*newitem).filename = copy_of(stanza);
        (*newitem).linenumber = atoi(lineptr) as ssize_t;
        (*newitem).columnnumber = atoi(columnptr) as ssize_t;
        (*newitem)
            .anchors = if phrase == stanza {
            0 as *mut libc::c_char
        } else {
            measured_copy(phrase, stanza.offset_from(phrase) as libc::c_long as size_t)
        };
        (*newitem).next = 0 as *mut poshiststruct;
        if position_history.is_null() {
            position_history = newitem;
        } else {
            (*lastitem).next = newitem;
        }
        lastitem = newitem;
    }
    if fclose(histfile) == -(1 as libc::c_int) {
        jot_error(
            b"Error reading %s: %s\0" as *const u8 as *const libc::c_char,
            poshistname,
            strerror(*__errno_location()),
        );
    }
    free(phrase as *mut libc::c_void);
    if stat(poshistname, &mut fileinfo) == 0 as libc::c_int {
        latest_timestamp = fileinfo.st_mtim.tv_sec;
    }
}
#[no_mangle]
pub unsafe extern "C" fn save_poshistory() {
    let mut histfile: *mut FILE = fopen(
        poshistname,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    let mut fileinfo: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut item: *mut poshiststruct = 0 as *mut poshiststruct;
    let mut count: libc::c_int = 0 as libc::c_int;
    if histfile.is_null() {
        jot_error(
            b"Error writing %s: %s\0" as *const u8 as *const libc::c_char,
            poshistname,
            strerror(*__errno_location()),
        );
        return;
    }
    if chmod(poshistname, (0o400 as libc::c_int | 0o200 as libc::c_int) as __mode_t)
        < 0 as libc::c_int
    {
        jot_error(
            b"Cannot limit permissions on %s: %s\0" as *const u8 as *const libc::c_char,
            poshistname,
            strerror(*__errno_location()),
        );
    }
    item = position_history;
    while !item.is_null()
        && {
            let fresh3 = count;
            count = count + 1;
            fresh3 < 200 as libc::c_int
        }
    {
        let mut path_and_place: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut length: size_t = if ((*item).anchors).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen((*item).anchors)
        };
        if length != 0
            && fwrite(
                (*item).anchors as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                length,
                histfile,
            ) < length
        {
            jot_error(
                b"Error writing %s: %s\0" as *const u8 as *const libc::c_char,
                poshistname,
                strerror(*__errno_location()),
            );
        }
        path_and_place = nmalloc(
            (strlen((*item).filename)).wrapping_add(44 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            path_and_place,
            b"%s %zd %zd\n\0" as *const u8 as *const libc::c_char,
            (*item).filename,
            (*item).linenumber,
            (*item).columnnumber,
        );
        length = recode_LF_to_NUL(path_and_place);
        *path_and_place
            .offset(
                length.wrapping_sub(1 as libc::c_int as size_t) as isize,
            ) = '\n' as i32 as libc::c_char;
        if fwrite(
            path_and_place as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            length,
            histfile,
        ) < length
        {
            jot_error(
                b"Error writing %s: %s\0" as *const u8 as *const libc::c_char,
                poshistname,
                strerror(*__errno_location()),
            );
        }
        free(path_and_place as *mut libc::c_void);
        item = (*item).next;
    }
    if fclose(histfile) == -(1 as libc::c_int) {
        jot_error(
            b"Error writing %s: %s\0" as *const u8 as *const libc::c_char,
            poshistname,
            strerror(*__errno_location()),
        );
    }
    if stat(poshistname, &mut fileinfo) == 0 as libc::c_int {
        latest_timestamp = fileinfo.st_mtim.tv_sec;
    }
}
#[no_mangle]
pub unsafe extern "C" fn reload_positions_if_needed() {
    let mut item: *mut poshiststruct = 0 as *mut poshiststruct;
    let mut nextone: *mut poshiststruct = 0 as *mut poshiststruct;
    let mut fileinfo: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(poshistname, &mut fileinfo) != 0 as libc::c_int
        || fileinfo.st_mtim.tv_sec == latest_timestamp
    {
        return;
    }
    item = position_history;
    while !item.is_null() {
        nextone = (*item).next;
        free((*item).filename as *mut libc::c_void);
        free((*item).anchors as *mut libc::c_void);
        free(item as *mut libc::c_void);
        item = nextone;
    }
    position_history = 0 as *mut poshiststruct;
    load_poshistory();
}
#[no_mangle]
pub unsafe extern "C" fn update_poshistory() {
    let mut fullpath: *mut libc::c_char = get_full_path((*openfile).filename);
    let mut previous: *mut poshiststruct = 0 as *mut poshiststruct;
    let mut item: *mut poshiststruct = 0 as *mut poshiststruct;
    if fullpath.is_null() {
        return;
    }
    reload_positions_if_needed();
    item = position_history;
    while !item.is_null() {
        if strcmp((*item).filename, fullpath) == 0 {
            break;
        }
        previous = item;
        item = (*item).next;
    }
    if item.is_null() {
        item = nmalloc(::core::mem::size_of::<poshiststruct>() as libc::c_ulong)
            as *mut poshiststruct;
        (*item).filename = copy_of(fullpath);
        (*item).anchors = 0 as *mut libc::c_char;
    } else if !previous.is_null() {
        (*previous).next = (*item).next;
    }
    if item != position_history {
        (*item).next = position_history;
        position_history = item;
    }
    (*item).linenumber = (*(*openfile).current).lineno;
    (*item)
        .columnnumber = (xplustabs()).wrapping_add(1 as libc::c_int as size_t)
        as ssize_t;
    free((*item).anchors as *mut libc::c_void);
    (*item).anchors = stringify_anchors();
    free(fullpath as *mut libc::c_void);
    save_poshistory();
}
#[no_mangle]
pub unsafe extern "C" fn restore_cursor_position_if_any() {
    let mut fullpath: *mut libc::c_char = get_full_path((*openfile).filename);
    let mut item: *mut poshiststruct = 0 as *mut poshiststruct;
    if fullpath.is_null() {
        return;
    }
    reload_positions_if_needed();
    item = position_history;
    while !item.is_null() && strcmp((*item).filename, fullpath) != 0 as libc::c_int {
        item = (*item).next;
    }
    free(fullpath as *mut libc::c_void);
    if !item.is_null() && !((*item).anchors).is_null() {
        restore_anchors((*item).anchors);
    }
    if !item.is_null() {
        goto_line_and_column(
            (*item).linenumber,
            (*item).columnnumber,
            1 as libc::c_int != 0,
        );
    }
}
