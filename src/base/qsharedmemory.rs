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
  fn _ZN13QSharedMemory12setNativeKeyERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
  fn _ZN13QSharedMemoryC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QSharedMemory::errorString();
  fn _ZNK13QSharedMemory11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSharedMemory::setKey(const QString & key);
  fn _ZN13QSharedMemory6setKeyERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QSharedMemory::key();
  fn _ZNK13QSharedMemory3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const void * QSharedMemory::constData();
  fn _ZNK13QSharedMemory9constDataEv(qthis: *mut c_void);
  // proto:  void * QSharedMemory::data();
  fn _ZN13QSharedMemory4dataEv(qthis: *mut c_void);
  // proto:  void QSharedMemory::QSharedMemory(const QSharedMemory & );
  fn _ZN13QSharedMemoryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSharedMemory::isAttached();
  fn _ZNK13QSharedMemory10isAttachedEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QSharedMemory::lock();
  fn _ZN13QSharedMemory4lockEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSharedMemory::~QSharedMemory();
  fn _ZN13QSharedMemoryD0Ev(qthis: *mut c_void);
  // proto:  bool QSharedMemory::unlock();
  fn _ZN13QSharedMemory6unlockEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QSharedMemory::detach();
  fn _ZN13QSharedMemory6detachEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QSharedMemory::nativeKey();
  fn _ZNK13QSharedMemory9nativeKeyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSharedMemory::metaObject();
  fn _ZNK13QSharedMemory10metaObjectEv(qthis: *mut c_void);
  // proto:  void QSharedMemory::QSharedMemory(QObject * parent);
  fn _ZN13QSharedMemoryC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QSharedMemory)=1
pub struct QSharedMemory {
  pub qclsinst: *mut c_void,
}

  // proto:  int QSharedMemory::size();
impl /*struct*/ QSharedMemory {
  pub fn size<RetType, T: QSharedMemory_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QSharedMemory_size<RetType> {
  fn size(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  int QSharedMemory::size();
impl<'a> /*trait*/ QSharedMemory_size<i32> for () {
  fn size(self , rsthis: &mut QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory4sizeEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSharedMemory::setNativeKey(const QString & key);
impl /*struct*/ QSharedMemory {
  pub fn setNativeKey<RetType, T: QSharedMemory_setNativeKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNativeKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_setNativeKey<RetType> {
  fn setNativeKey(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::setNativeKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setNativeKey<()> for (QString) {
  fn setNativeKey(self , rsthis: &mut QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory12setNativeKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSharedMemory12setNativeKeyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
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

  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
impl<'a> /*trait*/ QSharedMemory_NewQSharedMemory for (QString, QObject) {
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

  // proto:  QString QSharedMemory::errorString();
impl /*struct*/ QSharedMemory {
  pub fn errorString<RetType, T: QSharedMemory_errorString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QSharedMemory_errorString<RetType> {
  fn errorString(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::errorString();
impl<'a> /*trait*/ QSharedMemory_errorString<QString> for () {
  fn errorString(self , rsthis: &mut QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory11errorStringEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSharedMemory::setKey(const QString & key);
impl /*struct*/ QSharedMemory {
  pub fn setKey<RetType, T: QSharedMemory_setKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_setKey<RetType> {
  fn setKey(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::setKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setKey<()> for (QString) {
  fn setKey(self , rsthis: &mut QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6setKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSharedMemory6setKeyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSharedMemory::key();
impl /*struct*/ QSharedMemory {
  pub fn key<RetType, T: QSharedMemory_key<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QSharedMemory_key<RetType> {
  fn key(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::key();
impl<'a> /*trait*/ QSharedMemory_key<QString> for () {
  fn key(self , rsthis: &mut QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory3keyEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const void * QSharedMemory::constData();
impl /*struct*/ QSharedMemory {
  pub fn constData<RetType, T: QSharedMemory_constData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QSharedMemory_constData<RetType> {
  fn constData(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  const void * QSharedMemory::constData();
impl<'a> /*trait*/ QSharedMemory_constData<()> for () {
  fn constData(self , rsthis: &mut QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9constDataEv()};
     unsafe {_ZNK13QSharedMemory9constDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void * QSharedMemory::data();
impl /*struct*/ QSharedMemory {
  pub fn data<RetType, T: QSharedMemory_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QSharedMemory_data<RetType> {
  fn data(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  void * QSharedMemory::data();
impl<'a> /*trait*/ QSharedMemory_data<()> for () {
  fn data(self , rsthis: &mut QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4dataEv()};
     unsafe {_ZN13QSharedMemory4dataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSharedMemory::QSharedMemory(const QSharedMemory & );
impl<'a> /*trait*/ QSharedMemory_NewQSharedMemory for (QSharedMemory) {
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

  // proto:  bool QSharedMemory::isAttached();
impl /*struct*/ QSharedMemory {
  pub fn isAttached<RetType, T: QSharedMemory_isAttached<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isAttached(self);
    // return 1;
  }
}

pub trait QSharedMemory_isAttached<RetType> {
  fn isAttached(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::isAttached();
impl<'a> /*trait*/ QSharedMemory_isAttached<i8> for () {
  fn isAttached(self , rsthis: &mut QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10isAttachedEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory10isAttachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSharedMemory::lock();
impl /*struct*/ QSharedMemory {
  pub fn lock<RetType, T: QSharedMemory_lock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QSharedMemory_lock<RetType> {
  fn lock(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::lock();
impl<'a> /*trait*/ QSharedMemory_lock<i8> for () {
  fn lock(self , rsthis: &mut QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4lockEv()};
    let mut ret = unsafe {_ZN13QSharedMemory4lockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSharedMemory::~QSharedMemory();
impl /*struct*/ QSharedMemory {
  pub fn FreeQSharedMemory<RetType, T: QSharedMemory_FreeQSharedMemory<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSharedMemory(self);
    // return 1;
  }
}

pub trait QSharedMemory_FreeQSharedMemory<RetType> {
  fn FreeQSharedMemory(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::~QSharedMemory();
impl<'a> /*trait*/ QSharedMemory_FreeQSharedMemory<()> for () {
  fn FreeQSharedMemory(self , rsthis: &mut QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryD0Ev()};
     unsafe {_ZN13QSharedMemoryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSharedMemory::unlock();
impl /*struct*/ QSharedMemory {
  pub fn unlock<RetType, T: QSharedMemory_unlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QSharedMemory_unlock<RetType> {
  fn unlock(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::unlock();
impl<'a> /*trait*/ QSharedMemory_unlock<i8> for () {
  fn unlock(self , rsthis: &mut QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6unlockEv()};
    let mut ret = unsafe {_ZN13QSharedMemory6unlockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSharedMemory::detach();
impl /*struct*/ QSharedMemory {
  pub fn detach<RetType, T: QSharedMemory_detach<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QSharedMemory_detach<RetType> {
  fn detach(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::detach();
impl<'a> /*trait*/ QSharedMemory_detach<i8> for () {
  fn detach(self , rsthis: &mut QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6detachEv()};
    let mut ret = unsafe {_ZN13QSharedMemory6detachEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QSharedMemory::nativeKey();
impl /*struct*/ QSharedMemory {
  pub fn nativeKey<RetType, T: QSharedMemory_nativeKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nativeKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_nativeKey<RetType> {
  fn nativeKey(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::nativeKey();
impl<'a> /*trait*/ QSharedMemory_nativeKey<QString> for () {
  fn nativeKey(self , rsthis: &mut QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9nativeKeyEv()};
    let mut ret = unsafe {_ZNK13QSharedMemory9nativeKeyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSharedMemory::metaObject();
impl /*struct*/ QSharedMemory {
  pub fn metaObject<RetType, T: QSharedMemory_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSharedMemory_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSharedMemory) -> RetType;
}

  // proto:  const QMetaObject * QSharedMemory::metaObject();
impl<'a> /*trait*/ QSharedMemory_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10metaObjectEv()};
     unsafe {_ZNK13QSharedMemory10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSharedMemory::QSharedMemory(QObject * parent);
impl<'a> /*trait*/ QSharedMemory_NewQSharedMemory for (QObject) {
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

