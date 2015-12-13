// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK11QFileDevice4sizeEv() -> i32;
  fn _ZN11QFileDevice4seekEx(arg0: c_longlong) -> i32;
  fn _ZN11QFileDevice5unmapEPh(arg0: *mut c_uchar) -> i32;
  fn _ZN11QFileDevice5closeEv() -> i32;
  fn _ZNK11QFileDevice3posEv() -> i32;
  fn _ZNK11QFileDevice6handleEv() -> i32;
  fn _ZNK11QFileDevice8fileNameEv() -> i32;
  fn _ZN11QFileDeviceC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN11QFileDeviceD0Ev() -> i32;
  fn _ZNK11QFileDevice5atEndEv() -> i32;
  fn _ZNK11QFileDevice12isSequentialEv() -> i32;
  fn _ZN11QFileDevice5flushEv() -> i32;
  fn _ZN11QFileDeviceC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN11QFileDevice10unsetErrorEv() -> i32;
  fn _ZN11QFileDeviceC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK11QFileDevice10metaObjectEv() -> i32;
  fn _ZN11QFileDevice14setPermissionsE6QFlagsINS_10PermissionEE(arg0: c_int) -> i32;
  fn _ZN11QFileDevice6resizeEx(arg0: c_longlong) -> i32;
}

// body block begin
// class sizeof(QFileDevice)=1
pub struct QFileDevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileDevice {
  pub fn size<T: QFileDevice_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QFileDevice_size {
  fn size(self, this: &mut QFileDevice) -> i32;
}

// proto: long long QFileDevice::size();
impl<'a> /*trait*/ QFileDevice_size for () {
  fn size(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice4sizeEv()};
    unsafe {_ZNK11QFileDevice4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn seek<T: QFileDevice_seek>(&mut self, value: T) -> i32 {
    value.seek(self);
    return 1;
  }
}

pub trait QFileDevice_seek {
  fn seek(self, this: &mut QFileDevice) -> i32;
}

// proto: bool QFileDevice::seek(qint64 offset);
impl<'a> /*trait*/ QFileDevice_seek for (i64) {
  fn seek(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice4seekEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN11QFileDevice4seekEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn unmap<T: QFileDevice_unmap>(&mut self, value: T) -> i32 {
    value.unmap(self);
    return 1;
  }
}

pub trait QFileDevice_unmap {
  fn unmap(self, this: &mut QFileDevice) -> i32;
}

// proto: bool QFileDevice::unmap(uchar * address);
impl<'a> /*trait*/ QFileDevice_unmap for (&'a mut String) {
  fn unmap(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5unmapEPh()};
    let arg0 = self.as_ptr()  as *mut c_uchar;
    unsafe {_ZN11QFileDevice5unmapEPh(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn close<T: QFileDevice_close>(&mut self, value: T) -> i32 {
    value.close(self);
    return 1;
  }
}

pub trait QFileDevice_close {
  fn close(self, this: &mut QFileDevice) -> i32;
}

// proto: void QFileDevice::close();
impl<'a> /*trait*/ QFileDevice_close for () {
  fn close(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5closeEv()};
    unsafe {_ZN11QFileDevice5closeEv()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn pos<T: QFileDevice_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QFileDevice_pos {
  fn pos(self, this: &mut QFileDevice) -> i32;
}

// proto: long long QFileDevice::pos();
impl<'a> /*trait*/ QFileDevice_pos for () {
  fn pos(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice3posEv()};
    unsafe {_ZNK11QFileDevice3posEv()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn handle<T: QFileDevice_handle>(&mut self, value: T) -> i32 {
    value.handle(self);
    return 1;
  }
}

pub trait QFileDevice_handle {
  fn handle(self, this: &mut QFileDevice) -> i32;
}

// proto: int QFileDevice::handle();
impl<'a> /*trait*/ QFileDevice_handle for () {
  fn handle(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice6handleEv()};
    unsafe {_ZNK11QFileDevice6handleEv()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn fileName<T: QFileDevice_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QFileDevice_fileName {
  fn fileName(self, this: &mut QFileDevice) -> i32;
}

// proto: QString QFileDevice::fileName();
impl<'a> /*trait*/ QFileDevice_fileName for () {
  fn fileName(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice8fileNameEv()};
    unsafe {_ZNK11QFileDevice8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn NewQFileDevice<T: QFileDevice_NewQFileDevice>(value: T) -> QFileDevice {
    let rsthis = value.NewQFileDevice();
    return rsthis;
    // return 1;
  }
}

pub trait QFileDevice_NewQFileDevice {
  fn NewQFileDevice(self) -> QFileDevice;
}

// proto: void QFileDevice::NewQFileDevice(QObject * parent);
impl<'a> /*trait*/ QFileDevice_NewQFileDevice for (&'a mut QObject) {
  fn NewQFileDevice(self) -> QFileDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFileDeviceC1EP7QObject(qthis, arg0)};
    let rsthis = QFileDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn FreeQFileDevice<T: QFileDevice_FreeQFileDevice>(&mut self, value: T) -> i32 {
    value.FreeQFileDevice(self);
    return 1;
  }
}

pub trait QFileDevice_FreeQFileDevice {
  fn FreeQFileDevice(self, this: &mut QFileDevice) -> i32;
}

// proto: void QFileDevice::FreeQFileDevice();
impl<'a> /*trait*/ QFileDevice_FreeQFileDevice for () {
  fn FreeQFileDevice(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceD0Ev()};
    unsafe {_ZN11QFileDeviceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn atEnd<T: QFileDevice_atEnd>(&mut self, value: T) -> i32 {
    value.atEnd(self);
    return 1;
  }
}

pub trait QFileDevice_atEnd {
  fn atEnd(self, this: &mut QFileDevice) -> i32;
}

// proto: bool QFileDevice::atEnd();
impl<'a> /*trait*/ QFileDevice_atEnd for () {
  fn atEnd(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice5atEndEv()};
    unsafe {_ZNK11QFileDevice5atEndEv()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn isSequential<T: QFileDevice_isSequential>(&mut self, value: T) -> i32 {
    value.isSequential(self);
    return 1;
  }
}

pub trait QFileDevice_isSequential {
  fn isSequential(self, this: &mut QFileDevice) -> i32;
}

// proto: bool QFileDevice::isSequential();
impl<'a> /*trait*/ QFileDevice_isSequential for () {
  fn isSequential(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice12isSequentialEv()};
    unsafe {_ZNK11QFileDevice12isSequentialEv()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn flush<T: QFileDevice_flush>(&mut self, value: T) -> i32 {
    value.flush(self);
    return 1;
  }
}

pub trait QFileDevice_flush {
  fn flush(self, this: &mut QFileDevice) -> i32;
}

// proto: bool QFileDevice::flush();
impl<'a> /*trait*/ QFileDevice_flush for () {
  fn flush(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5flushEv()};
    unsafe {_ZN11QFileDevice5flushEv()};
    return 1;
  }
}

// proto: void QFileDevice::NewQFileDevice();
impl<'a> /*trait*/ QFileDevice_NewQFileDevice for () {
  fn NewQFileDevice(self) -> QFileDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1Ev()};
    unsafe {_ZN11QFileDeviceC1Ev(qthis)};
    let rsthis = QFileDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn unsetError<T: QFileDevice_unsetError>(&mut self, value: T) -> i32 {
    value.unsetError(self);
    return 1;
  }
}

pub trait QFileDevice_unsetError {
  fn unsetError(self, this: &mut QFileDevice) -> i32;
}

// proto: void QFileDevice::unsetError();
impl<'a> /*trait*/ QFileDevice_unsetError for () {
  fn unsetError(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice10unsetErrorEv()};
    unsafe {_ZN11QFileDevice10unsetErrorEv()};
    return 1;
  }
}

// proto: void QFileDevice::NewQFileDevice(const QFileDevice & );
impl<'a> /*trait*/ QFileDevice_NewQFileDevice for (&'a  QFileDevice) {
  fn NewQFileDevice(self) -> QFileDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDeviceC1ERKS_(qthis, arg0)};
    let rsthis = QFileDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn metaObject<T: QFileDevice_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFileDevice_metaObject {
  fn metaObject(self, this: &mut QFileDevice) -> i32;
}

// proto: const QMetaObject * QFileDevice::metaObject();
impl<'a> /*trait*/ QFileDevice_metaObject for () {
  fn metaObject(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice10metaObjectEv()};
    unsafe {_ZNK11QFileDevice10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn setPermissions<T: QFileDevice_setPermissions>(&mut self, value: T) -> i32 {
    value.setPermissions(self);
    return 1;
  }
}

pub trait QFileDevice_setPermissions {
  fn setPermissions(self, this: &mut QFileDevice) -> i32;
}

// proto: bool QFileDevice::setPermissions(Permissions permissionSpec);
impl<'a> /*trait*/ QFileDevice_setPermissions for (i32) {
  fn setPermissions(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice14setPermissionsE6QFlagsINS_10PermissionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QFileDevice14setPermissionsE6QFlagsINS_10PermissionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn resize<T: QFileDevice_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QFileDevice_resize {
  fn resize(self, this: &mut QFileDevice) -> i32;
}

// proto: bool QFileDevice::resize(qint64 sz);
impl<'a> /*trait*/ QFileDevice_resize for (i64) {
  fn resize(self, this: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice6resizeEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN11QFileDevice6resizeEx(arg0)};
    return 1;
  }
}

