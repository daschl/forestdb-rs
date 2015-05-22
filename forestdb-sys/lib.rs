/**
you can make it a #[repr(u8)] enum but then rustc should complain in a lint that you're using non-repr C types in ffi
12:28 but the correct type here is uint8_t for sure
Dtgr
12:29 I agree
bluss
12:29 fwiw, the C code is great in defining explicit size struct members instead of relying on the implementation defined size of enum
12:30 it makes it easier for everyone
*/

extern crate libc;

use libc::{c_void, size_t, uint8_t, uint16_t, uint32_t, uint64_t};

#[repr(u8)]
#[derive(Debug,PartialEq)]
#[allow(non_snake_case)]
pub enum fdb_seqtree_opt_t {
	FDB_SEQTREE_NOT_USE = 0,
	FDB_SEQTREE_USE = 1
}

#[repr(C)]
pub struct fdb_config {
	chunksize: uint16_t,
	blocksize: uint32_t,
	buffercache_size: uint64_t,
	wal_threshold: uint64_t,
	wal_flush_before_commit: bool,
	auto_commit: bool,
	purging_interval: uint32_t,
	seqtree_opt: fdb_seqtree_opt_t,
	durability_opt: uint8_t,
	flags: uint32_t,
	compaction_buf_maxsize: uint32_t,
	cleanup_cache_onclose: bool,
	compress_document_body: bool,
	compaction_mode: uint8_t,
	compaction_threshold: uint8_t,
	compaction_minimum_filesize: uint64_t,
	compactor_sleep_duration: uint64_t,
	multi_kv_instances: bool,
	prefetch_duration: uint64_t,
	num_wal_partitions: uint16_t,
	num_bcache_partitions: uint16_t,
	compaction_cb: size_t, /* TODO: IS THIS RIGHT? .. fix compaction callback */
	compaction_cb_mask: uint32_t,
	compaction_cb_ctx: c_void, /* TODO: IS THIS RIGHT? */
	max_writer_lock_prob: size_t
}

extern {
	pub fn fdb_get_default_config() -> fdb_config;
}

#[test]
fn should_access_default_config() {
	unsafe {
		let cfg = fdb_get_default_config();
		assert_eq!(8, cfg.chunksize);
		assert_eq!(4096, cfg.blocksize);
		assert_eq!(134217728, cfg.buffercache_size);
		assert_eq!(4096, cfg.wal_threshold);
		assert_eq!(true, cfg.wal_flush_before_commit);
		assert_eq!(false, cfg.auto_commit);
		assert_eq!(0, cfg.purging_interval);
		assert_eq!(fdb_seqtree_opt_t::FDB_SEQTREE_USE, cfg.seqtree_opt);
		assert_eq!(0, cfg.durability_opt);
		assert_eq!(1, cfg.flags);
		assert_eq!(4194304, cfg.compaction_buf_maxsize);
		assert_eq!(true, cfg.cleanup_cache_onclose);
		assert_eq!(false, cfg.compress_document_body);
		assert_eq!(0, cfg.compaction_mode);
		assert_eq!(30, cfg.compaction_threshold);
		assert_eq!(1048576, cfg.compaction_minimum_filesize);
		assert_eq!(15, cfg.compactor_sleep_duration);
		assert_eq!(true, cfg.multi_kv_instances);

		/* TODO: check defaults for all */
	}
}