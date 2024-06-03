use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE<'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16,'h17,'h18,'h19,'h20,'h21,'h22,'h23,'h24> {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: &'h7 (libc::c_char),
    pub _IO_read_end: &'h8 (libc::c_char),
    pub _IO_read_base: &'h9 (libc::c_char),
    pub _IO_write_base: &'h10 (libc::c_char),
    pub _IO_write_ptr: &'h11 (libc::c_char),
    pub _IO_write_end: &'h12 (libc::c_char),
    pub _IO_buf_base: &'h13 (libc::c_char),
    pub _IO_buf_end: &'h14 (libc::c_char),
    pub _IO_save_base: &'h15 (libc::c_char),
    pub _IO_backup_base: &'h16 (libc::c_char),
    pub _IO_save_end: &'h17 (libc::c_char),
    pub _markers: &'h18 (_IO_marker),
    pub _chain: &'h19 (_IO_FILE<'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16,'h17,'h18,'h19,'h20,'h21,'h22,'h23,'h24>),
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: &'h20 (libc::c_void),
    pub _offset: __off64_t,
    pub _codecvt: &'h21 (_IO_codecvt),
    pub _wide_data: &'h22 (_IO_wide_data),
    pub _freeres_list: &'h23 (_IO_FILE<'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16,'h17,'h18,'h19,'h20,'h21,'h22,'h23,'h24>),
    pub _freeres_buf: &'h24 (libc::c_void),
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE<'a> = _IO_FILE<'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>;
#[no_mangle]
pub unsafe extern "C" fn fseterr<'h0,'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16,'h17,'h18>(mut fp: &'h0 mut (_IO_FILE)) {
    (*fp)._flags |= 0x20 as libc::c_int;
}
