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
  // proto:  int QSharedMemory::size();
  fn _ZNK13QSharedMemory4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSharedMemory::setNativeKey(const QString & key);
  fn _ZN13QSharedMemory12setNativeKeyERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSharedMemory::NewQSharedMemory(const QString & key, QObject * parent);
  fn _ZN13QSharedMemoryC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QSharedMemory::errorString();
  fn _ZNK13QSharedMemory11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSharedMemory::setKey(const QString & key);
  fn _ZN13QSharedMemory6setKeyERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QSharedMemory::key();
  fn _ZNK13QSharedMemory3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const void * QSharedMemory::constData();
  fn _ZNK13QSharedMemory9constDataEv(qthis: *mut c_void) ;
  // proto:  void * QSharedMemory::data();
  fn _ZN13QSharedMemory4dataEv(qthis: *mut c_void) ;
  // proto:  void QSharedMemory::NewQSharedMemory(const QSharedMemory & );
  fn _ZN13QSharedMemoryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSharedMemory::isAttached();
  fn _ZNK13QSharedMemory10isAttachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QSharedMemory::lock();
  fn _ZN13QSharedMemory4lockEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSharedMemory::FreeQSharedMemory();
  fn _ZN13QSharedMemoryD0Ev(qthis: *mut c_void) ;
  // proto:  bool QSharedMemory::unlock();
  fn _ZN13QSharedMemory6unlockEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QSharedMemory::detach();
  fn _ZN13QSharedMemory6detachEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QSharedMemory::nativeKey();
  fn _ZNK13QSharedMemory9nativeKeyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSharedMemory::metaObject();
  fn _ZNK13QSharedMemory10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QSharedMemory::NewQSharedMemory(QObject * parent);
  fn _ZN13QSharedMemoryC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QSharedMemory)=1
pub struct QSharedMemory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSharedMemory {
  pub fn size<T: QSharedMemory_size>(&mut self, value: T) -> i32 {
    return value.size(self);
    // return 1;
  }
}

pub trait QSharedMemory_size {
  fn size(self, rsthis: &mut QSharedMemory) -> i32;
}

// proto:  int QSharedMemory::size();
impl<'a> /*trait*/ QSharedMemory_size for () {
  fn size(self, rsthis: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory4sizeEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn setNativeKey<T: QSharedMemory_setNativeKey>(&mut self, value: T)  {
     value.setNativeKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_setNativeKey {
  fn setNativeKey(self, rsthis: &mut QSharedMemory) ;
}

// proto:  void QSharedMemory::setNativeKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setNativeKey for (&'a  QString) {
  fn setNativeKey(self, rsthis: &mut QSharedMemory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory12setNativeKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSharedMemory12setNativeKeyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QSharedMemoryC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QSharedMemory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn errorString<T: QSharedMemory_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QSharedMemory_errorString {
  fn errorString(self, rsthis: &mut QSharedMemory) -> QString;
}

// proto:  QString QSharedMemory::errorString();
impl<'a> /*trait*/ QSharedMemory_errorString for () {
  fn errorString(self, rsthis: &mut QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory11errorStringEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn setKey<T: QSharedMemory_setKey>(&mut self, value: T)  {
     value.setKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_setKey {
  fn setKey(self, rsthis: &mut QSharedMemory) ;
}

// proto:  void QSharedMemory::setKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setKey for (&'a  QString) {
  fn setKey(self, rsthis: &mut QSharedMemory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6setKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSharedMemory6setKeyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn key<T: QSharedMemory_key>(&mut self, value: T) -> QString {
    return value.key(self);
    // return 1;
  }
}

pub trait QSharedMemory_key {
  fn key(self, rsthis: &mut QSharedMemory) -> QString;
}

// proto:  QString QSharedMemory::key();
impl<'a> /*trait*/ QSharedMemory_key for () {
  fn key(self, rsthis: &mut QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory3keyEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn constData<T: QSharedMemory_constData>(&mut self, value: T)  {
     value.constData(self);
    // return 1;
  }
}

pub trait QSharedMemory_constData {
  fn constData(self, rsthis: &mut QSharedMemory) ;
}

// proto:  const void * QSharedMemory::constData();
impl<'a> /*trait*/ QSharedMemory_constData for () {
  fn constData(self, rsthis: &mut QSharedMemory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9constDataEv()};
     unsafe {_ZNK13QSharedMemory9constDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn data<T: QSharedMemory_data>(&mut self, value: T)  {
     value.data(self);
    // return 1;
  }
}

pub trait QSharedMemory_data {
  fn data(self, rsthis: &mut QSharedMemory) ;
}

// proto:  void * QSharedMemory::data();
impl<'a> /*trait*/ QSharedMemory_data for () {
  fn data(self, rsthis: &mut QSharedMemory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4dataEv()};
     unsafe {_ZN13QSharedMemory4dataEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QSharedMemory::NewQSharedMemory(const QSharedMemory & );
impl<'a> /*trait*/ QSharedMemory_NewQSharedMemory for (&'a  QSharedMemory) {
  fn NewQSharedMemory(self) -> QSharedMemory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSharedMemoryC1ERKS_(qthis, arg0)};
    let rsthis = QSharedMemory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn isAttached<T: QSharedMemory_isAttached>(&mut self, value: T) -> i8 {
    return value.isAttached(self);
    // return 1;
  }
}

pub trait QSharedMemory_isAttached {
  fn isAttached(self, rsthis: &mut QSharedMemory) -> i8;
}

// proto:  bool QSharedMemory::isAttached();
impl<'a> /*trait*/ QSharedMemory_isAttached for () {
  fn isAttached(self, rsthis: &mut QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10isAttachedEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory10isAttachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn lock<T: QSharedMemory_lock>(&mut self, value: T) -> i8 {
    return value.lock(self);
    // return 1;
  }
}

pub trait QSharedMemory_lock {
  fn lock(self, rsthis: &mut QSharedMemory) -> i8;
}

// proto:  bool QSharedMemory::lock();
impl<'a> /*trait*/ QSharedMemory_lock for () {
  fn lock(self, rsthis: &mut QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4lockEv()};
    let mut ret = unsafe {_ZN13QSharedMemory4lockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn FreeQSharedMemory<T: QSharedMemory_FreeQSharedMemory>(&mut self, value: T)  {
     value.FreeQSharedMemory(self);
    // return 1;
  }
}

pub trait QSharedMemory_FreeQSharedMemory {
  fn FreeQSharedMemory(self, rsthis: &mut QSharedMemory) ;
}

// proto:  void QSharedMemory::FreeQSharedMemory();
impl<'a> /*trait*/ QSharedMemory_FreeQSharedMemory for () {
  fn FreeQSharedMemory(self, rsthis: &mut QSharedMemory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryD0Ev()};
     unsafe {_ZN13QSharedMemoryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn unlock<T: QSharedMemory_unlock>(&mut self, value: T) -> i8 {
    return value.unlock(self);
    // return 1;
  }
}

pub trait QSharedMemory_unlock {
  fn unlock(self, rsthis: &mut QSharedMemory) -> i8;
}

// proto:  bool QSharedMemory::unlock();
impl<'a> /*trait*/ QSharedMemory_unlock for () {
  fn unlock(self, rsthis: &mut QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6unlockEv()};
    let mut ret = unsafe {_ZN13QSharedMemory6unlockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn detach<T: QSharedMemory_detach>(&mut self, value: T) -> i8 {
    return value.detach(self);
    // return 1;
  }
}

pub trait QSharedMemory_detach {
  fn detach(self, rsthis: &mut QSharedMemory) -> i8;
}

// proto:  bool QSharedMemory::detach();
impl<'a> /*trait*/ QSharedMemory_detach for () {
  fn detach(self, rsthis: &mut QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6detachEv()};
    let mut ret = unsafe {_ZN13QSharedMemory6detachEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn nativeKey<T: QSharedMemory_nativeKey>(&mut self, value: T) -> QString {
    return value.nativeKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_nativeKey {
  fn nativeKey(self, rsthis: &mut QSharedMemory) -> QString;
}

// proto:  QString QSharedMemory::nativeKey();
impl<'a> /*trait*/ QSharedMemory_nativeKey for () {
  fn nativeKey(self, rsthis: &mut QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9nativeKeyEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory9nativeKeyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSharedMemory {
  pub fn metaObject<T: QSharedMemory_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSharedMemory_metaObject {
  fn metaObject(self, rsthis: &mut QSharedMemory) ;
}

// proto:  const QMetaObject * QSharedMemory::metaObject();
impl<'a> /*trait*/ QSharedMemory_metaObject for () {
  fn metaObject(self, rsthis: &mut QSharedMemory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10metaObjectEv()};
     unsafe {_ZNK13QSharedMemory10metaObjectEv(rsthis.qclsinst)};
    // return 1;
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

