extern crate ceph_rust;
use self::ceph_rust::ceph::RadosError;
use cephfs_sys::*;

use libc::{dev_t, ERANGE, mode_t, statvfs, strerror, utimbuf};
use std::ffi::CString;

struct CephFSVersion {
    major: i32,
    minor: i32,
    patch: i32,
}

fn get_error(n: i32) -> Result<String, RadosError> {
    unsafe {
        let error_cstring = CString::from_raw(strerror(n));
        let message = try!(error_cstring.into_string());
        Ok(message)
    }
}

pub fn version() -> Result<CephFSVersion, RadosError> {
    let mut major: i32 = 0;
    let mut minor: i32 = 0;
    let mut patch: i32 = 0;
    unsafe {
        let ret_code = ceph_version(&mut major, &mut minor, &mut patch);
        Ok(CephFSVersion {
            major: major,
            minor: minor,
            patch: patch,
        })
    }
}

pub fn create(cmount: &mut &mut ceph_mount_info, id: &str) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_create(cmount, id);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn create_from_rados(cmount: &mut &mut ceph_mount_info,

                         cluster: rados_t)
                         -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_create_from_rados(cmount, cluster);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn init(cmount: &mut ceph_mount_info) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_init(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn mount(cmount: &mut ceph_mount_info, root: *const &str) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_mount(cmount, root);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn mds_command(cmount: &mut ceph_mount_info,

                   mds_spec: *const &str,

                   cmd: &mut *const &str,

                   cmdlen: usize,

                   inbuf: *const &str,

                   inbuflen: usize,

                   outbuf: &mut &mut &str,

                   outbuflen: &mut usize,

                   outs: &mut &mut &str,

                   outslen: &mut usize)
                   -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_mds_command(cmount,
                                        mds_spec,
                                        cmd,
                                        cmdlen,
                                        inbuf,
                                        inbuflen,
                                        outbuf,
                                        outbuflen,
                                        outs,
                                        outslen);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn unmount(cmount: &mut ceph_mount_info) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_unmount(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn release(cmount: &mut ceph_mount_info) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_release(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_mount_context(cmount: &mut ceph_mount_info) -> Result<&mut CephContext, RadosError> {
    unsafe {
        let ret_code = ceph_get_mount_context(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn is_mounted(cmount: &mut ceph_mount_info) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_is_mounted(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn conf_read_file(cmount: &mut ceph_mount_info, path_list: &str) -> Result<(), RadosError> {
    let path_list = try!(CString::new(path_list));
    unsafe {
        let ret_code = ceph_conf_read_file(cmount, path_list.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn conf_parse_argv(cmount: &mut ceph_mount_info,
                       argc: i32,
                       argv: &str)
                       -> Result<(), RadosError> {
    let argv = try!(CString::new(argv));
    unsafe {
        let ret_code = ceph_conf_parse_argv(cmount, argc, argv.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn conf_parse_env(cmount: &mut ceph_mount_info, var: &str) -> Result<(), RadosError> {
    let var = try!(CString::new(var));
    unsafe {
        let ret_code = ceph_conf_parse_env(cmount, var.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn conf_set(cmount: &mut ceph_mount_info, option: &str, value: &str) -> Result<(), RadosError> {
    let option = try!(CString::new(option));
    let value = try!(CString::new(value));
    unsafe {
        let ret_code = ceph_conf_set(cmount, option.as_ptr(), value.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn conf_get(cmount: &mut ceph_mount_info,

                option: &str,

                buf: &mut &str,

                len: usize)
                -> Result<(), RadosError> {
    let option = try!(CString::new(option));
    unsafe {
        let ret_code = ceph_conf_get(cmount, option.as_ptr(), buf, len);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn statfs(cmount: &mut ceph_mount_info,
              path: &str,
              stbuf: &mut statvfs)
              -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_statfs(cmount, path.as_ptr(), stbuf);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn sync_fs(cmount: &mut ceph_mount_info) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_sync_fs(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn getcwd(cmount: &mut ceph_mount_info) -> Result<*const &str, RadosError> {
    unsafe {
        let ret_code = ceph_getcwd(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn chdir(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_chdir(cmount, path.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn opendir(cmount: &mut ceph_mount_info,

               name: &str,

               dirpp: &mut &mut ceph_dir_result)
               -> Result<(), RadosError> {
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_opendir(cmount, name.as_ptr(), dirpp);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn closedir(cmount: &mut ceph_mount_info,

                dirp: &mut ceph_dir_result)
                -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_closedir(cmount, dirp);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn readdir(cmount: &mut ceph_mount_info,
               dirp: &mut ceph_dir_result)
               -> Result<dirent, RadosError> {
    unsafe {
        let ret_code = ceph_readdir(cmount, dirp);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn readdir_r(cmount: &mut ceph_mount_info,

                 dirp: &mut ceph_dir_result,

                 de: &mut dirent)
                 -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_readdir_r(cmount, dirp, de);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn readdirplus_r(cmount: &mut ceph_mount_info,
                     dirp: &mut ceph_dir_result,
                     de: &mut dirent,
                     st: &mut ::std::os::linux::raw::stat,
                     stmask: i32)
                     -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_readdirplus_r(cmount, dirp, de, st, stmask);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn getdents(cmount: &mut ceph_mount_info,

                dirp: &mut ceph_dir_result,

                name: &mut &str,

                buflen: i32)
                -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_getdents(cmount, dirp, name, buflen);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn getdnames(cmount: &mut ceph_mount_info,

                 dirp: &mut ceph_dir_result,

                 name: &mut &str,

                 buflen: i32)
                 -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_getdnames(cmount, dirp, name, buflen);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn telldir(cmount: &mut ceph_mount_info,
               dirp: &mut ceph_dir_result)
               -> Result<i64, RadosError> {
    unsafe {
        let ret_code = ceph_telldir(cmount, dirp);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn mkdir(cmount: &mut ceph_mount_info, path: &str, mode: mode_t) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_mkdir(cmount, path.as_ptr(), mode);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn mkdirs(cmount: &mut ceph_mount_info, path: &str, mode: mode_t) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_mkdirs(cmount, path.as_ptr(), mode);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn rmdir(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_rmdir(cmount, path.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn link(cmount: &mut ceph_mount_info,

            existing: &str,

            new_name: &str)
            -> Result<(), RadosError> {
    let existing = try!(CString::new(existing));
    let new_name = try!(CString::new(new_name));
    unsafe {
        let ret_code = ceph_link(cmount, existing.as_ptr(), new_name.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn symlink(cmount: &mut ceph_mount_info,

               existing: &str,

               new_name: &str)
               -> Result<(), RadosError> {
    let existing = try!(CString::new(existing));
    let new_name = try!(CString::new(new_name));
    unsafe {
        let ret_code = ceph_symlink(cmount, existing.as_ptr(), new_name.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn unlink(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_unlink(cmount, path.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn rename(cmount: &mut ceph_mount_info, from: &str, to: &str) -> Result<(), RadosError> {
    let from = try!(CString::new(from));
    let to = try!(CString::new(to));
    unsafe {
        let ret_code = ceph_rename(cmount, from.as_ptr(), to.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn stat(cmount: &mut ceph_mount_info,

            path: &str,

            stbuf: &mut ::std::os::linux::raw::stat)
            -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_stat(cmount, path.as_ptr(), stbuf);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn lstat(cmount: &mut ceph_mount_info,
             path: &str,
             stbuf: &mut ::std::os::linux::raw::stat)
             -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_lstat(cmount, path.as_ptr(), stbuf);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn setattr(cmount: &mut ceph_mount_info,
               relpath: &str,
               attr: &mut ::std::os::linux::raw::stat,
               mask: i32)
               -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_setattr(cmount, relpath, attr, mask);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn chmod(cmount: &mut ceph_mount_info, path: &str, mode: mode_t) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_chmod(cmount, path.as_ptr(), mode);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn fchmod(cmount: &mut ceph_mount_info, fd: i32, mode: mode_t) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_fchmod(cmount, fd, mode);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn chown(cmount: &mut ceph_mount_info,

             path: &str,

             uid: i32,

             gid: i32)
             -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_chown(cmount, path.as_ptr(), uid, gid);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn fchown(cmount: &mut ceph_mount_info, fd: i32, uid: i32, gid: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_fchown(cmount, fd, uid, gid);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn lchown(cmount: &mut ceph_mount_info,

              path: &str,

              uid: i32,

              gid: i32)
              -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_lchown(cmount, path.as_ptr(), uid, gid);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn utime(cmount: &mut ceph_mount_info,

             path: &str,

             buf: &mut utimbuf)
             -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_utime(cmount, path.as_ptr(), buf);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn mknod(cmount: &mut ceph_mount_info,

             path: &str,

             mode: mode_t,

             rdev: dev_t)
             -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_mknod(cmount, path.as_ptr(), mode, rdev);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn open(cmount: &mut ceph_mount_info,

            path: &str,

            flags: i32,

            mode: mode_t)
            -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_open(cmount, path.as_ptr(), flags, mode);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn open_layout(cmount: &mut ceph_mount_info,
                   path: &str,
                   flags: i32,
                   mode: mode_t,
                   stripe_unit: i32,
                   stripe_count: i32,
                   object_size: i32,
                   data_pool: &str)
                   -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    let data_pool = try!(CString::new(data_pool));
    unsafe {
        let ret_code = ceph_open_layout(cmount,
                                        path.as_ptr(),
                                        flags,
                                        mode,
                                        stripe_unit,
                                        stripe_count,
                                        object_size,
                                        data_pool.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn close(cmount: &mut ceph_mount_info, fd: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_close(cmount, fd);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn fsync(cmount: &mut ceph_mount_info, fd: i32, syncdataonly: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_fsync(cmount, fd, syncdataonly);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn fstat(cmount: &mut ceph_mount_info,
             fd: i32,
             stbuf: &mut ::std::os::linux::raw::stat)
             -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_fstat(cmount, fd, stbuf);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn getxattr(cmount: &mut ceph_mount_info,

                path: &str,

                name: &str,

                value: &mut ::std::os::raw::c_void,

                size: usize)
                -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_getxattr(cmount, path.as_ptr(), name.as_ptr(), value, size);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn fgetxattr(cmount: &mut ceph_mount_info,

                 fd: i32,

                 name: &str,

                 value: &mut ::std::os::raw::c_void,

                 size: usize)
                 -> Result<(), RadosError> {
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_fgetxattr(cmount, fd, name.as_ptr(), value, size);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn lgetxattr(cmount: &mut ceph_mount_info,

                 path: &str,

                 name: &str,

                 value: &mut ::std::os::raw::c_void,

                 size: usize)
                 -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    let name = try!(CString::new(name));
    let mut value: ::std::os::raw::c_void = ptr::null_mut();
    unsafe {
        let ret_code = ceph_lgetxattr(cmount, path.as_ptr(), name.as_ptr(), value, size);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn listxattr(cmount: &mut ceph_mount_info,

                 path: &str,

                 list: &mut &str,

                 size: usize)
                 -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_listxattr(cmount, path.as_ptr(), list, size);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn flistxattr(cmount: &mut ceph_mount_info,

                  fd: i32,

                  list: &mut &str,

                  size: usize)
                  -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_flistxattr(cmount, fd, list, size);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn llistxattr(cmount: &mut ceph_mount_info,

                  path: &str,

                  list: &mut &str,

                  size: usize)
                  -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_llistxattr(cmount, path.as_ptr(), list, size);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn removexattr(cmount: &mut ceph_mount_info, path: &str, name: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_removexattr(cmount, path.as_ptr(), name.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn fremovexattr(cmount: &mut ceph_mount_info, fd: i32, name: &str) -> Result<(), RadosError> {
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_fremovexattr(cmount, fd, name.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn lremovexattr(cmount: &mut ceph_mount_info,

                    path: &str,

                    name: &str)
                    -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_lremovexattr(cmount, path.as_ptr(), name.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn setxattr(cmount: &mut ceph_mount_info,

                path: &str,

                name: &str,

                value: *const ::std::os::raw::c_void,

                size: usize,

                flags: i32)
                -> Result<(), RadosError> {

    let path = try!(CString::new(path));
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_setxattr(cmount, path.as_ptr(), name.as_ptr(), value, size, flags);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn fsetxattr(cmount: &mut ceph_mount_info,

                 fd: i32,

                 name: &str,

                 value: *const ::std::os::raw::c_void,

                 size: usize,

                 flags: i32)
                 -> Result<(), RadosError> {
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_fsetxattr(cmount, fd, name.as_ptr(), value, size, flags);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn lsetxattr(cmount: &mut ceph_mount_info,

                 path: &str,

                 name: &str,

                 value: *const ::std::os::raw::c_void,

                 size: usize,

                 flags: i32)
                 -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    let name = try!(CString::new(name));
    unsafe {
        let ret_code = ceph_lsetxattr(cmount, path.as_ptr(), name.as_ptr(), value, size, flags);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_file_stripe_unit(cmount: &mut ceph_mount_info, fh: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_get_file_stripe_unit(cmount, fh);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_path_stripe_unit(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_get_path_stripe_unit(cmount, path.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_file_stripe_count(cmount: &mut ceph_mount_info, fh: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_get_file_stripe_count(cmount, fh);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_path_stripe_count(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_get_path_stripe_count(cmount, path.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_file_object_size(cmount: &mut ceph_mount_info, fh: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_get_file_object_size(cmount, fh);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_path_object_size(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_get_path_object_size(cmount, path.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_file_pool(cmount: &mut ceph_mount_info, fh: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_get_file_pool(cmount, fh);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_path_pool(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_get_path_pool(cmount, path.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_file_pool_name(cmount: &mut ceph_mount_info,

                          fh: i32,

                          buf: &mut &str,

                          buflen: usize)
                          -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_get_file_pool_name(cmount, fh, buf, buflen);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_path_pool_name(cmount: &mut ceph_mount_info,

                          path: &str,
                          buflen: usize)
                          -> Result<String, RadosError> {
    let path = try!(CString::new(path));
    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    unsafe {
        // Try to get the name with 1 call.  Otherwise ask for the correct size
        let ret_code =
            ceph_get_path_pool_name(cmount, path.as_ptr(), buf.as_ptr() as *mut i8, 1024);
        if ret_code < -ERANGE {
            // buf was too small
            let suggested_size =
                ceph_get_path_pool_name(cmount, path.as_ptr(), buf.as_ptr() as *mut i8, 0);
            buf = Vec::with_capacity(suggested_size as usize);
            let ret_code =
                ceph_get_path_pool_name(cmount, path.as_ptr(), buf.as_ptr() as *mut i8, 1024);
            if ret_code < 0 {
                return Err(RadosError::new(try!(get_error(ret_code))));
            }
            // Tell Vec how much we wrote into it
            buf.set_len(ret_code as usize);
        }
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
        // Tell Vec how much we wrote into it
        buf.set_len(ret_code as usize);
    }
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

pub fn get_path_layout(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    let mut stripe_unit: i32 = 0;
    let mut stripe_count: i32 = 0;
    let mut object_size: i32 = 0;
    let mut pg_pool: i32 = 0;
    unsafe {
        let ret_code = ceph_get_path_layout(cmount,
                                            path.as_ptr(),
                                            &mut stripe_unit,
                                            &mut stripe_count,
                                            &mut object_size,
                                            &mut pg_pool);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_path_replication(cmount: &mut ceph_mount_info, path: &str) -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_get_path_replication(cmount, path.as_ptr());
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_pool_replication(cmount: &mut ceph_mount_info, pool_id: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_get_pool_replication(cmount, pool_id);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_osd_crush_location(cmount: &mut ceph_mount_info,

                              osd: i32,

                              path: &str,

                              len: usize)
                              -> Result<(), RadosError> {
    let path = try!(CString::new(path));
    unsafe {
        let ret_code = ceph_get_osd_crush_location(cmount, osd, path.as_ptr() as *mut i8, len);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn get_stripe_unit_granularity(cmount: &mut ceph_mount_info) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_get_stripe_unit_granularity(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn set_default_file_stripe_count(cmount: &mut ceph_mount_info,

                                     count: i32)
                                     -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_set_default_file_stripe_count(cmount, count);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn set_default_preferred_pg(cmount: &mut ceph_mount_info, osd: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_set_default_preferred_pg(cmount, osd);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn localize_reads(cmount: &mut ceph_mount_info, val: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_localize_reads(cmount, val);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn debug_get_fd_caps(cmount: &mut ceph_mount_info, fd: i32) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_debug_get_fd_caps(cmount, fd);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn ll_get_inode(cmount: &mut ceph_mount_info, vino: vinodeno_t) -> Result<Inode, RadosError> {
    unsafe {
        let inode = ceph_ll_get_inode(cmount, vino);
        Ok(inode)
    }
}

pub fn ll_releasedir(cmount: &mut ceph_mount_info,

                     dir: &mut ceph_dir_result)
                     -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_ll_releasedir(cmount, dir);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}

pub fn ll_num_osds(cmount: &mut ceph_mount_info) -> Result<(), RadosError> {
    unsafe {
        let ret_code = ceph_ll_num_osds(cmount);
        if ret_code < 0 {
            return Err(RadosError::new(try!(get_error(ret_code))));
        }
    }
    Ok(())
}
