extern crate libc;

pub use libc::{c_void,c_char,size_t,c_int};

#[link(name = "unarr")]


pub type p_ar_stream = *const c_void;

pub type p_ar_archive = *const c_void;

pub type off64_t = i64;

pub type time64_t = i64;

extern "C"{
    pub fn ar_open_file(path:*const c_char ) -> p_ar_stream;


    #[cfg(windows)]
    pub fn ar_open_file_w(path:*const c_wchar ) -> p_ar_stream;

    pub fn ar_open_memory(data : *const c_void, datalen :size_t)->p_ar_stream;

    pub fn ar_close(ar_stream: p_ar_stream);

    pub fn ar_read(ar_stream: p_ar_stream, buffer: *mut c_void, count : size_t)->size_t;

    pub fn ar_seek(ar_stream: p_ar_stream, offset: off64_t, origin: c_int)->bool;

    pub fn ar_skip(ar_stream: p_ar_stream, count: i64)->bool ;

    pub fn ar_tell(ar_stream :p_ar_stream)->i64;

    pub fn ar_close_archive(ar: p_ar_archive );

    pub fn ar_parse_entry(ar:p_ar_archive)->bool;

    pub fn ar_parse_entry_at( ar:p_ar_archive, offset:off64_t)->bool;

    pub fn ar_parse_entry_for(ar: p_ar_archive, entry_name:*const c_char )->bool;

    pub fn ar_at_eof(ar:p_ar_archive )->bool;

    pub fn ar_entry_get_name(ar:p_ar_archive )-> *const c_char ;

    pub fn ar_entry_get_offset(ar:p_ar_archive)->off64_t;

    pub fn ar_entry_get_size(ar :p_ar_archive)->size_t ;

    pub fn ar_entry_get_filetime(ar:p_ar_archive)->time64_t;

    pub fn ar_entry_uncompress(ar : p_ar_archive, buffer : *mut c_void, count:size_t )->bool ;

    pub fn ar_get_global_comment(ar :p_ar_archive, buffer: *mut c_void , count:size_t )->size_t ;

    pub fn ar_open_rar_archive(stream:p_ar_stream )->p_ar_archive;

    pub fn ar_open_tar_archive(stream:p_ar_stream )->p_ar_archive;

    pub fn ar_open_zip_archive(stream:p_ar_stream,deflatedonly : bool )->p_ar_archive;

    pub fn ar_open_7z_archive(stream:p_ar_stream)->p_ar_archive;


}
