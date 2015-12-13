// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK13QSharedMemory4sizeEv() -> i32;
  fn _ZN13QSharedMemory12setNativeKeyERK7QString(arg0: *const c_void) -> i32;
  fn _ZN13QSharedMemoryC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  fn _ZNK13QSharedMemory11errorStringEv() -> i32;
  fn _ZN13QSharedMemory6setKeyERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK13QSharedMemory3keyEv() -> i32;
  fn _ZNK13QSharedMemory9constDataEv() -> i32;
  fn _ZN13QSharedMemory4dataEv() -> i32;
  fn _ZN13QSharedMemoryC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK13QSharedMemory10isAttachedEv() -> i32;
  fn _ZN13QSharedMemory4lockEv() -> i32;
  fn _ZN13QSharedMemoryD0Ev() -> i32;
  fn _ZN13QSharedMemory6unlockEv() -> i32;
  fn _ZN13QSharedMemory6detachEv() -> i32;
  fn _ZNK13QSharedMemory9nativeKeyEv() -> i32;
  fn _ZNK13QSharedMemory10metaObjectEv() -> i32;
  fn _ZN13QSharedMemoryC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QSharedMemory)=1
pub struct QSharedMemory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSharedMemory {
  pub fn size<T: QSharedMemory_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QSharedMemory_size {
  fn size(self, this: &mut QSharedMemory) -> i32;
}

// proto: int QSharedMemory::size();
impl<'a> /*trait*/ QSharedMemory_size for () {
  fn size(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory4sizeEv()};
    unsafe {_ZNK13QSharedMemory4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn setNativeKey<T: QSharedMemory_setNativeKey>(&mut self, value: T) -> i32 {
    value.setNativeKey(self);
    return 1;
  }
}

pub trait QSharedMemory_setNativeKey {
  fn setNativeKey(self, this: &mut QSharedMemory) -> i32;
}

// proto: void QSharedMemory::setNativeKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setNativeKey for (&'a  QString) {
  fn setNativeKey(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory12setNativeKeyERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QSharedMemory12setNativeKeyERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn NewQSharedMemory<T: QSharedMemory_NewQSharedMemory>(value: T) -> QSharedMemory {
    let rsthis = value.NewQSharedMemory();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedMemory_NewQSharedMemory {
  fn NewQSharedMemory(self) -> QSharedMemory;
}

// proto: void QSharedMemory::NewQSharedMemory(const QString & key, QObject * parent);
impl<'a> /*trait*/ QSharedMemory_NewQSharedMemory for (&'a  QString, &'a mut QObject) {
  fn NewQSharedMemory(self) -> QSharedMemory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QSharedMemoryC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QSharedMemory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn errorString<T: QSharedMemory_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QSharedMemory_errorString {
  fn errorString(self, this: &mut QSharedMemory) -> i32;
}

// proto: QString QSharedMemory::errorString();
impl<'a> /*trait*/ QSharedMemory_errorString for () {
  fn errorString(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory11errorStringEv()};
    unsafe {_ZNK13QSharedMemory11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn setKey<T: QSharedMemory_setKey>(&mut self, value: T) -> i32 {
    value.setKey(self);
    return 1;
  }
}

pub trait QSharedMemory_setKey {
  fn setKey(self, this: &mut QSharedMemory) -> i32;
}

// proto: void QSharedMemory::setKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setKey for (&'a  QString) {
  fn setKey(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6setKeyERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QSharedMemory6setKeyERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn key<T: QSharedMemory_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QSharedMemory_key {
  fn key(self, this: &mut QSharedMemory) -> i32;
}

// proto: QString QSharedMemory::key();
impl<'a> /*trait*/ QSharedMemory_key for () {
  fn key(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory3keyEv()};
    unsafe {_ZNK13QSharedMemory3keyEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn constData<T: QSharedMemory_constData>(&mut self, value: T) -> i32 {
    value.constData(self);
    return 1;
  }
}

pub trait QSharedMemory_constData {
  fn constData(self, this: &mut QSharedMemory) -> i32;
}

// proto: const void * QSharedMemory::constData();
impl<'a> /*trait*/ QSharedMemory_constData for () {
  fn constData(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9constDataEv()};
    unsafe {_ZNK13QSharedMemory9constDataEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn data<T: QSharedMemory_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QSharedMemory_data {
  fn data(self, this: &mut QSharedMemory) -> i32;
}

// proto: void * QSharedMemory::data();
impl<'a> /*trait*/ QSharedMemory_data for () {
  fn data(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4dataEv()};
    unsafe {_ZN13QSharedMemory4dataEv()};
    return 1;
  }
}

// proto: void QSharedMemory::NewQSharedMemory(const QSharedMemory & );
impl<'a> /*trait*/ QSharedMemory_NewQSharedMemory for (&'a  QSharedMemory) {
  fn NewQSharedMemory(self) -> QSharedMemory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QSharedMemoryC1ERKS_(qthis, arg0)};
    let rsthis = QSharedMemory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn isAttached<T: QSharedMemory_isAttached>(&mut self, value: T) -> i32 {
    value.isAttached(self);
    return 1;
  }
}

pub trait QSharedMemory_isAttached {
  fn isAttached(self, this: &mut QSharedMemory) -> i32;
}

// proto: bool QSharedMemory::isAttached();
impl<'a> /*trait*/ QSharedMemory_isAttached for () {
  fn isAttached(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10isAttachedEv()};
    unsafe {_ZNK13QSharedMemory10isAttachedEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn lock<T: QSharedMemory_lock>(&mut self, value: T) -> i32 {
    value.lock(self);
    return 1;
  }
}

pub trait QSharedMemory_lock {
  fn lock(self, this: &mut QSharedMemory) -> i32;
}

// proto: bool QSharedMemory::lock();
impl<'a> /*trait*/ QSharedMemory_lock for () {
  fn lock(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4lockEv()};
    unsafe {_ZN13QSharedMemory4lockEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn FreeQSharedMemory<T: QSharedMemory_FreeQSharedMemory>(&mut self, value: T) -> i32 {
    value.FreeQSharedMemory(self);
    return 1;
  }
}

pub trait QSharedMemory_FreeQSharedMemory {
  fn FreeQSharedMemory(self, this: &mut QSharedMemory) -> i32;
}

// proto: void QSharedMemory::FreeQSharedMemory();
impl<'a> /*trait*/ QSharedMemory_FreeQSharedMemory for () {
  fn FreeQSharedMemory(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryD0Ev()};
    unsafe {_ZN13QSharedMemoryD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn unlock<T: QSharedMemory_unlock>(&mut self, value: T) -> i32 {
    value.unlock(self);
    return 1;
  }
}

pub trait QSharedMemory_unlock {
  fn unlock(self, this: &mut QSharedMemory) -> i32;
}

// proto: bool QSharedMemory::unlock();
impl<'a> /*trait*/ QSharedMemory_unlock for () {
  fn unlock(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6unlockEv()};
    unsafe {_ZN13QSharedMemory6unlockEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn detach<T: QSharedMemory_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QSharedMemory_detach {
  fn detach(self, this: &mut QSharedMemory) -> i32;
}

// proto: bool QSharedMemory::detach();
impl<'a> /*trait*/ QSharedMemory_detach for () {
  fn detach(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6detachEv()};
    unsafe {_ZN13QSharedMemory6detachEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn nativeKey<T: QSharedMemory_nativeKey>(&mut self, value: T) -> i32 {
    value.nativeKey(self);
    return 1;
  }
}

pub trait QSharedMemory_nativeKey {
  fn nativeKey(self, this: &mut QSharedMemory) -> i32;
}

// proto: QString QSharedMemory::nativeKey();
impl<'a> /*trait*/ QSharedMemory_nativeKey for () {
  fn nativeKey(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9nativeKeyEv()};
    unsafe {_ZNK13QSharedMemory9nativeKeyEv()};
    return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn metaObject<T: QSharedMemory_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSharedMemory_metaObject {
  fn metaObject(self, this: &mut QSharedMemory) -> i32;
}

// proto: const QMetaObject * QSharedMemory::metaObject();
impl<'a> /*trait*/ QSharedMemory_metaObject for () {
  fn metaObject(self, this: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10metaObjectEv()};
    unsafe {_ZNK13QSharedMemory10metaObjectEv()};
    return 1;
  }
}

// proto: void QSharedMemory::NewQSharedMemory(QObject * parent);
impl<'a> /*trait*/ QSharedMemory_NewQSharedMemory for (&'a mut QObject) {
  fn NewQSharedMemory(self) -> QSharedMemory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSharedMemoryC1EP7QObject(qthis, arg0)};
    let rsthis = QSharedMemory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

