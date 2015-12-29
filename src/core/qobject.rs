// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qobject.h
// dst-file: /src/core/qobject.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
// use super::qobject::QObject; // 773
use super::qthread::QThread; // 773
use super::qcoreevent::QEvent; // 773
// use super::qobject::QObjectUserData; // 773
use super::qmetaobject::QMetaMethod; // 773
use super::qvariant::QVariant; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSignalBlocker_Class_Size() -> c_int;
  // proto:  void QSignalBlocker::unblock();
  fn demth_ZN14QSignalBlocker7unblockEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSignalBlocker::QSignalBlocker(QObject & o);
  fn dector_ZN14QSignalBlockerC1ER7QObject(arg0: *mut c_void) -> *mut c_void;
  fn demth_ZN14QSignalBlockerC1ER7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSignalBlocker::QSignalBlocker(QObject * o);
  fn dector_ZN14QSignalBlockerC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn demth_ZN14QSignalBlockerC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSignalBlocker::QSignalBlocker(const QSignalBlocker & );
  fn dector_ZN14QSignalBlockerC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QSignalBlockerC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSignalBlocker::reblock();
  fn demth_ZN14QSignalBlocker7reblockEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSignalBlocker::~QSignalBlocker();
  fn demth_ZN14QSignalBlockerD0Ev(qthis: u64 /* *mut c_void*/);
  fn QObjectData_Class_Size() -> c_int;
  // proto:  QMetaObject * QObjectData::dynamicMetaObject();
  fn _ZNK11QObjectData17dynamicMetaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QObjectData::~QObjectData();
  fn _ZN11QObjectDataD0Ev(qthis: u64 /* *mut c_void*/);
  fn QObjectUserData_Class_Size() -> c_int;
  // proto:  void QObjectUserData::~QObjectUserData();
  fn _ZN15QObjectUserDataD0Ev(qthis: u64 /* *mut c_void*/);
  fn QObject_Class_Size() -> c_int;
  // proto:  bool QObject::inherits(const char * classname);
  fn demth_ZNK7QObject8inheritsEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_char;
  // proto:  void QObject::destroyed(QObject * );
  fn _ZN7QObject9destroyedEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QObject::moveToThread(QThread * thread);
  fn _ZN7QObject12moveToThreadEP7QThread(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QObject::removeEventFilter(QObject * );
  fn _ZN7QObject17removeEventFilterEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QObject::dumpObjectTree();
  fn _ZN7QObject14dumpObjectTreeEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QObject::eventFilter(QObject * , QEvent * );
  fn _ZN7QObject11eventFilterEPS_P6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QObject::setUserData(uint id, QObjectUserData * data);
  fn _ZN7QObject11setUserDataEjP15QObjectUserData(qthis: u64 /* *mut c_void*/, arg0: c_uint, arg1: *mut c_void);
  // proto:  void QObject::QObject(const QObject & );
  fn dector_ZN7QObjectC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QObjectC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static bool QObject::disconnect(const QObject * sender, const QMetaMethod & signal, const QObject * receiver, const QMetaMethod & member);
  fn _ZN7QObject10disconnectEPKS_RK11QMetaMethodS1_S4_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_char;
  // proto:  bool QObject::event(QEvent * );
  fn _ZN7QObject5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QList<QByteArray> QObject::dynamicPropertyNames();
  fn _ZNK7QObject20dynamicPropertyNamesEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QObject::isWidgetType();
  fn demth_ZNK7QObject12isWidgetTypeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QVariant QObject::property(const char * name);
  fn _ZNK7QObject8propertyEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> *mut c_void;
  // proto:  QThread * QObject::thread();
  fn _ZNK7QObject6threadEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QObject::metaObject();
  fn _ZNK7QObject10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QObject::setParent(QObject * );
  fn _ZN7QObject9setParentEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QObject::disconnect(const QObject * receiver, const char * member);
  fn demth_ZNK7QObject10disconnectEPKS_PKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  const QObjectList & QObject::children();
  fn demth_ZNK7QObject8childrenEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QObject::isWindowType();
  fn demth_ZNK7QObject12isWindowTypeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QObject::disconnect(const char * signal, const QObject * receiver, const char * member);
  fn demth_ZNK7QObject10disconnectEPKcPKS_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void, arg2: *mut c_char) -> c_char;
  // proto:  void QObject::deleteLater();
  fn _ZN7QObject11deleteLaterEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QObject::~QObject();
  fn _ZN7QObjectD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QObject::QObject(QObject * parent);
  fn dector_ZN7QObjectC1EPS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QObjectC1EPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QObject::objectName();
  fn _ZNK7QObject10objectNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QObject::setProperty(const char * name, const QVariant & value);
  fn _ZN7QObject11setPropertyEPKcRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_void) -> c_char;
  // proto: static bool QObject::disconnect(const QObject * sender, const char * signal, const QObject * receiver, const char * member);
  fn _ZN7QObject10disconnectEPKS_PKcS1_S3_(arg0: *mut c_void, arg1: *mut c_char, arg2: *mut c_void, arg3: *mut c_char) -> c_char;
  // proto:  bool QObject::signalsBlocked();
  fn demth_ZNK7QObject14signalsBlockedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static uint QObject::registerUserData();
  fn _ZN7QObject16registerUserDataEv() -> c_uint;
  // proto:  QObjectUserData * QObject::userData(uint id);
  fn _ZNK7QObject8userDataEj(qthis: u64 /* *mut c_void*/, arg0: c_uint) -> *mut c_void;
  // proto:  QObject * QObject::parent();
  fn demth_ZNK7QObject6parentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QObject::installEventFilter(QObject * );
  fn _ZN7QObject18installEventFilterEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QObject::blockSignals(bool b);
  fn _ZN7QObject12blockSignalsEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> c_char;
  // proto:  void QObject::setObjectName(const QString & name);
  fn _ZN7QObject13setObjectNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QObject::dumpObjectInfo();
  fn _ZN7QObject14dumpObjectInfoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QObject::killTimer(int id);
  fn _ZN7QObject9killTimerEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QObject_SlotProxy_connect__ZN7QObject9destroyedEPS_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QObject_SlotProxy_connect_box__ZN7QObject9destroyedEPS_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSignalBlocker)=16
#[derive(Default)]
pub struct QSignalBlocker {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QObjectData)=1
#[derive(Default)]
pub struct QObjectData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QObjectUserData)=8
#[derive(Default)]
pub struct QObjectUserData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QObject)=1
#[derive(Default)]
pub struct QObject {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _destroyed_1: QObject_destroyed_signal,
  pub _objectNameChanged_1: QObject_objectNameChanged_signal,
}

impl /*struct*/ QSignalBlocker {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSignalBlocker {
    return QSignalBlocker{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSignalBlocker::unblock();
impl /*struct*/ QSignalBlocker {
  pub fn unblock<RetType, T: QSignalBlocker_unblock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unblock(self);
    // return 1;
  }
}

pub trait QSignalBlocker_unblock<RetType> {
  fn unblock(self , rsthis: & QSignalBlocker) -> RetType;
}

  // proto:  void QSignalBlocker::unblock();
impl<'a> /*trait*/ QSignalBlocker_unblock<()> for () {
  fn unblock(self , rsthis: & QSignalBlocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlocker7unblockEv()};
     unsafe {demth_ZN14QSignalBlocker7unblockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalBlocker::QSignalBlocker(QObject & o);
impl /*struct*/ QSignalBlocker {
  pub fn New<T: QSignalBlocker_New>(value: T) -> QSignalBlocker {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalBlocker_New {
  fn New(self) -> QSignalBlocker;
}

  // proto:  void QSignalBlocker::QSignalBlocker(QObject & o);
impl<'a> /*trait*/ QSignalBlocker_New for (&'a QObject) {
  fn New(self) -> QSignalBlocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerC1ER7QObject()};
    let ctysz: c_int = unsafe{QSignalBlocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QSignalBlockerC1ER7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QSignalBlockerC1ER7QObject(arg0)} as u64;
    let rsthis = QSignalBlocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalBlocker::QSignalBlocker(const QSignalBlocker & );
impl<'a> /*trait*/ QSignalBlocker_New for (&'a QSignalBlocker) {
  fn New(self) -> QSignalBlocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerC1ERKS_()};
    let ctysz: c_int = unsafe{QSignalBlocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QSignalBlockerC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QSignalBlockerC1ERKS_(arg0)} as u64;
    let rsthis = QSignalBlocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalBlocker::reblock();
impl /*struct*/ QSignalBlocker {
  pub fn reblock<RetType, T: QSignalBlocker_reblock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reblock(self);
    // return 1;
  }
}

pub trait QSignalBlocker_reblock<RetType> {
  fn reblock(self , rsthis: & QSignalBlocker) -> RetType;
}

  // proto:  void QSignalBlocker::reblock();
impl<'a> /*trait*/ QSignalBlocker_reblock<()> for () {
  fn reblock(self , rsthis: & QSignalBlocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlocker7reblockEv()};
     unsafe {demth_ZN14QSignalBlocker7reblockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalBlocker::~QSignalBlocker();
impl /*struct*/ QSignalBlocker {
  pub fn Free<RetType, T: QSignalBlocker_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSignalBlocker_Free<RetType> {
  fn Free(self , rsthis: & QSignalBlocker) -> RetType;
}

  // proto:  void QSignalBlocker::~QSignalBlocker();
impl<'a> /*trait*/ QSignalBlocker_Free<()> for () {
  fn Free(self , rsthis: & QSignalBlocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerD0Ev()};
     unsafe {demth_ZN14QSignalBlockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObjectData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QObjectData {
    return QObjectData{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QMetaObject * QObjectData::dynamicMetaObject();
impl /*struct*/ QObjectData {
  pub fn dynamicMetaObject<RetType, T: QObjectData_dynamicMetaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dynamicMetaObject(self);
    // return 1;
  }
}

pub trait QObjectData_dynamicMetaObject<RetType> {
  fn dynamicMetaObject(self , rsthis: & QObjectData) -> RetType;
}

  // proto:  QMetaObject * QObjectData::dynamicMetaObject();
impl<'a> /*trait*/ QObjectData_dynamicMetaObject<()> for () {
  fn dynamicMetaObject(self , rsthis: & QObjectData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QObjectData17dynamicMetaObjectEv()};
     unsafe {_ZNK11QObjectData17dynamicMetaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QObjectData::~QObjectData();
impl /*struct*/ QObjectData {
  pub fn Free<RetType, T: QObjectData_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QObjectData_Free<RetType> {
  fn Free(self , rsthis: & QObjectData) -> RetType;
}

  // proto:  void QObjectData::~QObjectData();
impl<'a> /*trait*/ QObjectData_Free<()> for () {
  fn Free(self , rsthis: & QObjectData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QObjectDataD0Ev()};
     unsafe {_ZN11QObjectDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObjectUserData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QObjectUserData {
    return QObjectUserData{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QObjectUserData::~QObjectUserData();
impl /*struct*/ QObjectUserData {
  pub fn Free<RetType, T: QObjectUserData_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QObjectUserData_Free<RetType> {
  fn Free(self , rsthis: & QObjectUserData) -> RetType;
}

  // proto:  void QObjectUserData::~QObjectUserData();
impl<'a> /*trait*/ QObjectUserData_Free<()> for () {
  fn Free(self , rsthis: & QObjectUserData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QObjectUserDataD0Ev()};
     unsafe {_ZN15QObjectUserDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QObject {
    return QObject{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QObject::inherits(const char * classname);
impl /*struct*/ QObject {
  pub fn inherits<RetType, T: QObject_inherits<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.inherits(self);
    // return 1;
  }
}

pub trait QObject_inherits<RetType> {
  fn inherits(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::inherits(const char * classname);
impl<'a> /*trait*/ QObject_inherits<i8> for (&'a  String) {
  fn inherits(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8inheritsEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {demth_ZNK7QObject8inheritsEPKc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QObject::destroyed(QObject * );
impl /*struct*/ QObject {
  pub fn destroyed<RetType, T: QObject_destroyed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroyed(self);
    // return 1;
  }
}

pub trait QObject_destroyed<RetType> {
  fn destroyed(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::destroyed(QObject * );
impl<'a> /*trait*/ QObject_destroyed<()> for (&'a QObject) {
  fn destroyed(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9destroyedEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject9destroyedEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QObject::moveToThread(QThread * thread);
impl /*struct*/ QObject {
  pub fn moveToThread<RetType, T: QObject_moveToThread<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveToThread(self);
    // return 1;
  }
}

pub trait QObject_moveToThread<RetType> {
  fn moveToThread(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::moveToThread(QThread * thread);
impl<'a> /*trait*/ QObject_moveToThread<()> for (&'a QThread) {
  fn moveToThread(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject12moveToThreadEP7QThread()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject12moveToThreadEP7QThread(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QObject::removeEventFilter(QObject * );
impl /*struct*/ QObject {
  pub fn removeEventFilter<RetType, T: QObject_removeEventFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeEventFilter(self);
    // return 1;
  }
}

pub trait QObject_removeEventFilter<RetType> {
  fn removeEventFilter(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::removeEventFilter(QObject * );
impl<'a> /*trait*/ QObject_removeEventFilter<()> for (&'a QObject) {
  fn removeEventFilter(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject17removeEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject17removeEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QObject::dumpObjectTree();
impl /*struct*/ QObject {
  pub fn dumpObjectTree<RetType, T: QObject_dumpObjectTree<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dumpObjectTree(self);
    // return 1;
  }
}

pub trait QObject_dumpObjectTree<RetType> {
  fn dumpObjectTree(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::dumpObjectTree();
impl<'a> /*trait*/ QObject_dumpObjectTree<()> for () {
  fn dumpObjectTree(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject14dumpObjectTreeEv()};
     unsafe {_ZN7QObject14dumpObjectTreeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QObject::eventFilter(QObject * , QEvent * );
impl /*struct*/ QObject {
  pub fn eventFilter<RetType, T: QObject_eventFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.eventFilter(self);
    // return 1;
  }
}

pub trait QObject_eventFilter<RetType> {
  fn eventFilter(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::eventFilter(QObject * , QEvent * );
impl<'a> /*trait*/ QObject_eventFilter<i8> for (&'a QObject, &'a QEvent) {
  fn eventFilter(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11eventFilterEPS_P6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QObject11eventFilterEPS_P6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QObject::setUserData(uint id, QObjectUserData * data);
impl /*struct*/ QObject {
  pub fn setUserData<RetType, T: QObject_setUserData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUserData(self);
    // return 1;
  }
}

pub trait QObject_setUserData<RetType> {
  fn setUserData(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::setUserData(uint id, QObjectUserData * data);
impl<'a> /*trait*/ QObject_setUserData<()> for (u32, &'a QObjectUserData) {
  fn setUserData(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11setUserDataEjP15QObjectUserData()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject11setUserDataEjP15QObjectUserData(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QObject::QObject(const QObject & );
impl /*struct*/ QObject {
  pub fn New<T: QObject_New>(value: T) -> QObject {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QObject_New {
  fn New(self) -> QObject;
}

  // proto:  void QObject::QObject(const QObject & );
impl<'a> /*trait*/ QObject_New for (&'a QObject) {
  fn New(self) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObjectC1ERKS_()};
    let ctysz: c_int = unsafe{QObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QObjectC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QObjectC1ERKS_(arg0)} as u64;
    let rsthis = QObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static bool QObject::disconnect(const QObject * sender, const QMetaMethod & signal, const QObject * receiver, const QMetaMethod & member);
impl /*struct*/ QObject {
  pub fn disconnect_s<RetType, T: QObject_disconnect_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.disconnect_s();
    // return 1;
  }
}

pub trait QObject_disconnect_s<RetType> {
  fn disconnect_s(self ) -> RetType;
}

  // proto: static bool QObject::disconnect(const QObject * sender, const QMetaMethod & signal, const QObject * receiver, const QMetaMethod & member);
impl<'a> /*trait*/ QObject_disconnect_s<i8> for (&'a QObject, &'a QMetaMethod, &'a QObject, &'a QMetaMethod) {
  fn disconnect_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject10disconnectEPKS_RK11QMetaMethodS1_S4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QObject10disconnectEPKS_RK11QMetaMethodS1_S4_(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QObject::event(QEvent * );
impl /*struct*/ QObject {
  pub fn event<RetType, T: QObject_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QObject_event<RetType> {
  fn event(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::event(QEvent * );
impl<'a> /*trait*/ QObject_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QObject5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QObject::dynamicPropertyNames();
impl /*struct*/ QObject {
  pub fn dynamicPropertyNames<RetType, T: QObject_dynamicPropertyNames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dynamicPropertyNames(self);
    // return 1;
  }
}

pub trait QObject_dynamicPropertyNames<RetType> {
  fn dynamicPropertyNames(self , rsthis: & QObject) -> RetType;
}

  // proto:  QList<QByteArray> QObject::dynamicPropertyNames();
impl<'a> /*trait*/ QObject_dynamicPropertyNames<()> for () {
  fn dynamicPropertyNames(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject20dynamicPropertyNamesEv()};
     unsafe {_ZNK7QObject20dynamicPropertyNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QObject::isWidgetType();
impl /*struct*/ QObject {
  pub fn isWidgetType<RetType, T: QObject_isWidgetType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWidgetType(self);
    // return 1;
  }
}

pub trait QObject_isWidgetType<RetType> {
  fn isWidgetType(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::isWidgetType();
impl<'a> /*trait*/ QObject_isWidgetType<i8> for () {
  fn isWidgetType(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject12isWidgetTypeEv()};
    let mut ret = unsafe {demth_ZNK7QObject12isWidgetTypeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVariant QObject::property(const char * name);
impl /*struct*/ QObject {
  pub fn property<RetType, T: QObject_property<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.property(self);
    // return 1;
  }
}

pub trait QObject_property<RetType> {
  fn property(self , rsthis: & QObject) -> RetType;
}

  // proto:  QVariant QObject::property(const char * name);
impl<'a> /*trait*/ QObject_property<QVariant> for (&'a  String) {
  fn property(self , rsthis: & QObject) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8propertyEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QObject8propertyEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QThread * QObject::thread();
impl /*struct*/ QObject {
  pub fn thread<RetType, T: QObject_thread<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.thread(self);
    // return 1;
  }
}

pub trait QObject_thread<RetType> {
  fn thread(self , rsthis: & QObject) -> RetType;
}

  // proto:  QThread * QObject::thread();
impl<'a> /*trait*/ QObject_thread<QThread> for () {
  fn thread(self , rsthis: & QObject) -> QThread {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject6threadEv()};
    let mut ret = unsafe {_ZNK7QObject6threadEv(rsthis.qclsinst)};
    let mut ret1 = QThread::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QObject::metaObject();
impl /*struct*/ QObject {
  pub fn metaObject<RetType, T: QObject_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QObject_metaObject<RetType> {
  fn metaObject(self , rsthis: & QObject) -> RetType;
}

  // proto:  const QMetaObject * QObject::metaObject();
impl<'a> /*trait*/ QObject_metaObject<()> for () {
  fn metaObject(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10metaObjectEv()};
     unsafe {_ZNK7QObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QObject::setParent(QObject * );
impl /*struct*/ QObject {
  pub fn setParent<RetType, T: QObject_setParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setParent(self);
    // return 1;
  }
}

pub trait QObject_setParent<RetType> {
  fn setParent(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::setParent(QObject * );
impl<'a> /*trait*/ QObject_setParent<()> for (&'a QObject) {
  fn setParent(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject9setParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QObject::disconnect(const QObject * receiver, const char * member);
impl /*struct*/ QObject {
  pub fn disconnect<RetType, T: QObject_disconnect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.disconnect(self);
    // return 1;
  }
}

pub trait QObject_disconnect<RetType> {
  fn disconnect(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::disconnect(const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect<i8> for (&'a QObject, &'a  String) {
  fn disconnect(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10disconnectEPKS_PKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {demth_ZNK7QObject10disconnectEPKS_PKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QObjectList & QObject::children();
impl /*struct*/ QObject {
  pub fn children<RetType, T: QObject_children<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.children(self);
    // return 1;
  }
}

pub trait QObject_children<RetType> {
  fn children(self , rsthis: & QObject) -> RetType;
}

  // proto:  const QObjectList & QObject::children();
impl<'a> /*trait*/ QObject_children<()> for () {
  fn children(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8childrenEv()};
     unsafe {demth_ZNK7QObject8childrenEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QObject::isWindowType();
impl /*struct*/ QObject {
  pub fn isWindowType<RetType, T: QObject_isWindowType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWindowType(self);
    // return 1;
  }
}

pub trait QObject_isWindowType<RetType> {
  fn isWindowType(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::isWindowType();
impl<'a> /*trait*/ QObject_isWindowType<i8> for () {
  fn isWindowType(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject12isWindowTypeEv()};
    let mut ret = unsafe {demth_ZNK7QObject12isWindowTypeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QObject::disconnect(const char * signal, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect<i8> for (&'a  String, &'a QObject, &'a  String) {
  fn disconnect(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10disconnectEPKcPKS_S1_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let mut ret = unsafe {demth_ZNK7QObject10disconnectEPKcPKS_S1_(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QObject::deleteLater();
impl /*struct*/ QObject {
  pub fn deleteLater<RetType, T: QObject_deleteLater<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deleteLater(self);
    // return 1;
  }
}

pub trait QObject_deleteLater<RetType> {
  fn deleteLater(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::deleteLater();
impl<'a> /*trait*/ QObject_deleteLater<()> for () {
  fn deleteLater(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11deleteLaterEv()};
     unsafe {_ZN7QObject11deleteLaterEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QObject::~QObject();
impl /*struct*/ QObject {
  pub fn Free<RetType, T: QObject_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QObject_Free<RetType> {
  fn Free(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::~QObject();
impl<'a> /*trait*/ QObject_Free<()> for () {
  fn Free(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObjectD0Ev()};
     unsafe {_ZN7QObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QObject::objectName();
impl /*struct*/ QObject {
  pub fn objectName<RetType, T: QObject_objectName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.objectName(self);
    // return 1;
  }
}

pub trait QObject_objectName<RetType> {
  fn objectName(self , rsthis: & QObject) -> RetType;
}

  // proto:  QString QObject::objectName();
impl<'a> /*trait*/ QObject_objectName<QString> for () {
  fn objectName(self , rsthis: & QObject) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10objectNameEv()};
    let mut ret = unsafe {_ZNK7QObject10objectNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QObject::setProperty(const char * name, const QVariant & value);
impl /*struct*/ QObject {
  pub fn setProperty<RetType, T: QObject_setProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProperty(self);
    // return 1;
  }
}

pub trait QObject_setProperty<RetType> {
  fn setProperty(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::setProperty(const char * name, const QVariant & value);
impl<'a> /*trait*/ QObject_setProperty<i8> for (&'a  String, &'a QVariant) {
  fn setProperty(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11setPropertyEPKcRK8QVariant()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QObject11setPropertyEPKcRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QObject::disconnect(const QObject * sender, const char * signal, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect_s<i8> for (&'a QObject, &'a  String, &'a QObject, &'a  String) {
  fn disconnect_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject10disconnectEPKS_PKcS1_S3_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN7QObject10disconnectEPKS_PKcS1_S3_(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QObject::signalsBlocked();
impl /*struct*/ QObject {
  pub fn signalsBlocked<RetType, T: QObject_signalsBlocked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.signalsBlocked(self);
    // return 1;
  }
}

pub trait QObject_signalsBlocked<RetType> {
  fn signalsBlocked(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::signalsBlocked();
impl<'a> /*trait*/ QObject_signalsBlocked<i8> for () {
  fn signalsBlocked(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject14signalsBlockedEv()};
    let mut ret = unsafe {demth_ZNK7QObject14signalsBlockedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static uint QObject::registerUserData();
impl /*struct*/ QObject {
  pub fn registerUserData_s<RetType, T: QObject_registerUserData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerUserData_s();
    // return 1;
  }
}

pub trait QObject_registerUserData_s<RetType> {
  fn registerUserData_s(self ) -> RetType;
}

  // proto: static uint QObject::registerUserData();
impl<'a> /*trait*/ QObject_registerUserData_s<u32> for () {
  fn registerUserData_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject16registerUserDataEv()};
    let mut ret = unsafe {_ZN7QObject16registerUserDataEv()};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QObjectUserData * QObject::userData(uint id);
impl /*struct*/ QObject {
  pub fn userData<RetType, T: QObject_userData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.userData(self);
    // return 1;
  }
}

pub trait QObject_userData<RetType> {
  fn userData(self , rsthis: & QObject) -> RetType;
}

  // proto:  QObjectUserData * QObject::userData(uint id);
impl<'a> /*trait*/ QObject_userData<QObjectUserData> for (u32) {
  fn userData(self , rsthis: & QObject) -> QObjectUserData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8userDataEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK7QObject8userDataEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QObjectUserData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QObject::parent();
impl /*struct*/ QObject {
  pub fn parent<RetType, T: QObject_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QObject_parent<RetType> {
  fn parent(self , rsthis: & QObject) -> RetType;
}

  // proto:  QObject * QObject::parent();
impl<'a> /*trait*/ QObject_parent<QObject> for () {
  fn parent(self , rsthis: & QObject) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject6parentEv()};
    let mut ret = unsafe {demth_ZNK7QObject6parentEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QObject::installEventFilter(QObject * );
impl /*struct*/ QObject {
  pub fn installEventFilter<RetType, T: QObject_installEventFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.installEventFilter(self);
    // return 1;
  }
}

pub trait QObject_installEventFilter<RetType> {
  fn installEventFilter(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::installEventFilter(QObject * );
impl<'a> /*trait*/ QObject_installEventFilter<()> for (&'a QObject) {
  fn installEventFilter(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject18installEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject18installEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QObject::blockSignals(bool b);
impl /*struct*/ QObject {
  pub fn blockSignals<RetType, T: QObject_blockSignals<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockSignals(self);
    // return 1;
  }
}

pub trait QObject_blockSignals<RetType> {
  fn blockSignals(self , rsthis: & QObject) -> RetType;
}

  // proto:  bool QObject::blockSignals(bool b);
impl<'a> /*trait*/ QObject_blockSignals<i8> for (i8) {
  fn blockSignals(self , rsthis: & QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject12blockSignalsEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN7QObject12blockSignalsEb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QObject::setObjectName(const QString & name);
impl /*struct*/ QObject {
  pub fn setObjectName<RetType, T: QObject_setObjectName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setObjectName(self);
    // return 1;
  }
}

pub trait QObject_setObjectName<RetType> {
  fn setObjectName(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::setObjectName(const QString & name);
impl<'a> /*trait*/ QObject_setObjectName<()> for (&'a QString) {
  fn setObjectName(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject13setObjectNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject13setObjectNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QObject::dumpObjectInfo();
impl /*struct*/ QObject {
  pub fn dumpObjectInfo<RetType, T: QObject_dumpObjectInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dumpObjectInfo(self);
    // return 1;
  }
}

pub trait QObject_dumpObjectInfo<RetType> {
  fn dumpObjectInfo(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::dumpObjectInfo();
impl<'a> /*trait*/ QObject_dumpObjectInfo<()> for () {
  fn dumpObjectInfo(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject14dumpObjectInfoEv()};
     unsafe {_ZN7QObject14dumpObjectInfoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QObject::killTimer(int id);
impl /*struct*/ QObject {
  pub fn killTimer<RetType, T: QObject_killTimer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.killTimer(self);
    // return 1;
  }
}

pub trait QObject_killTimer<RetType> {
  fn killTimer(self , rsthis: & QObject) -> RetType;
}

  // proto:  void QObject::killTimer(int id);
impl<'a> /*trait*/ QObject_killTimer<()> for (i32) {
  fn killTimer(self , rsthis: & QObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9killTimerEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QObject9killTimerEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QObject_destroyed
pub struct QObject_destroyed_signal{poi:u64}
impl /* struct */ QObject {
  pub fn destroyed_1(&self) -> QObject_destroyed_signal {
     return QObject_destroyed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QObject_destroyed_signal {
  pub fn connect<T: QObject_destroyed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QObject_destroyed_signal_connect {
  fn connect(self, sigthis: QObject_destroyed_signal);
}

#[derive(Default)] // for QObject_objectNameChanged
pub struct QObject_objectNameChanged_signal{poi:u64}
impl /* struct */ QObject {
  pub fn objectNameChanged_1(&self) -> QObject_objectNameChanged_signal {
     return QObject_objectNameChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QObject_objectNameChanged_signal {
  pub fn connect<T: QObject_objectNameChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QObject_objectNameChanged_signal_connect {
  fn connect(self, sigthis: QObject_objectNameChanged_signal);
}

// destroyed(class QObject *)
extern fn QObject_destroyed_signal_connect_cb_0(rsfptr:fn(QObject), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QObject_destroyed_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QObject_destroyed_signal_connect for fn(QObject) {
  fn connect(self, sigthis: QObject_destroyed_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QObject_destroyed_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QObject_SlotProxy_connect__ZN7QObject9destroyedEPS_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QObject_destroyed_signal_connect for Box<fn(QObject)> {
  fn connect(self, sigthis: QObject_destroyed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QObject_destroyed_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QObject_SlotProxy_connect__ZN7QObject9destroyedEPS_(arg0, arg1, arg2)};
  }
}
// <= body block end

