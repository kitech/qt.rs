// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qthread::QThread;
use super::qevent::QEvent;
use super::qobjectuserdata::QObjectUserData;
use super::qmetamethod::QMetaMethod;
use super::qvariant::QVariant;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QObject::inherits(const char * classname);
  fn _ZNK7QObject8inheritsEPKc(qthis: *mut c_void, arg0: *const c_char) -> int8_t;
  // proto:  void QObject::destroyed(QObject * );
  fn _ZN7QObject9destroyedEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QObject::moveToThread(QThread * thread);
  fn _ZN7QObject12moveToThreadEP7QThread(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QObject::removeEventFilter(QObject * );
  fn _ZN7QObject17removeEventFilterEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QObject::dumpObjectTree();
  fn _ZN7QObject14dumpObjectTreeEv(qthis: *mut c_void) ;
  // proto:  bool QObject::eventFilter(QObject * , QEvent * );
  fn _ZN7QObject11eventFilterEPS_P6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  void QObject::setUserData(uint id, QObjectUserData * data);
  fn _ZN7QObject11setUserDataEjP15QObjectUserData(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_void) ;
  // proto:  void QObject::NewQObject(const QObject & );
  fn _ZN7QObjectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QObject::disconnect(const QObject * sender, const QMetaMethod & signal, const QObject * receiver, const QMetaMethod & member);
  fn _ZN7QObject10disconnectEPKS_RK11QMetaMethodS1_S4_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> int8_t;
  // proto:  bool QObject::event(QEvent * );
  fn _ZN7QObject5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QList<QByteArray> QObject::dynamicPropertyNames();
  fn _ZNK7QObject20dynamicPropertyNamesEv(qthis: *mut c_void) ;
  // proto:  bool QObject::isWidgetType();
  fn _ZNK7QObject12isWidgetTypeEv(qthis: *mut c_void) -> int8_t;
  // proto:  QVariant QObject::property(const char * name);
  fn _ZNK7QObject8propertyEPKc(qthis: *mut c_void, arg0: *const c_char) -> *mut c_void;
  // proto:  QThread * QObject::thread();
  fn _ZNK7QObject6threadEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QObject::metaObject();
  fn _ZNK7QObject10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QObject::setParent(QObject * );
  fn _ZN7QObject9setParentEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QObject::disconnect(const QObject * receiver, const char * member);
  fn _ZNK7QObject10disconnectEPKS_PKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> int8_t;
  // proto:  const QObjectList & QObject::children();
  fn _ZNK7QObject8childrenEv(qthis: *mut c_void) ;
  // proto:  bool QObject::isWindowType();
  fn _ZNK7QObject12isWindowTypeEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QObject::disconnect(const char * signal, const QObject * receiver, const char * member);
  fn _ZNK7QObject10disconnectEPKcPKS_S1_(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void, arg2: *const c_char) -> int8_t;
  // proto:  void QObject::deleteLater();
  fn _ZN7QObject11deleteLaterEv(qthis: *mut c_void) ;
  // proto:  void QObject::FreeQObject();
  fn _ZN7QObjectD0Ev(qthis: *mut c_void) ;
  // proto:  void QObject::NewQObject(QObject * parent);
  fn _ZN7QObjectC1EPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QObject::objectName();
  fn _ZNK7QObject10objectNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QObject::setProperty(const char * name, const QVariant & value);
  fn _ZN7QObject11setPropertyEPKcRK8QVariant(qthis: *mut c_void, arg0: *const c_char, arg1: *mut c_void) -> int8_t;
  // proto: static bool QObject::disconnect(const QObject * sender, const char * signal, const QObject * receiver, const char * member);
  fn _ZN7QObject10disconnectEPKS_PKcS1_S3_(arg0: *mut c_void, arg1: *const c_char, arg2: *mut c_void, arg3: *const c_char) -> int8_t;
  // proto:  bool QObject::signalsBlocked();
  fn _ZNK7QObject14signalsBlockedEv(qthis: *mut c_void) -> int8_t;
  // proto: static unsigned int QObject::registerUserData();
  fn _ZN7QObject16registerUserDataEv() -> c_uint;
  // proto:  QObjectUserData * QObject::userData(uint id);
  fn _ZNK7QObject8userDataEj(qthis: *mut c_void, arg0: c_uint) -> *mut c_void;
  // proto:  QObject * QObject::parent();
  fn _ZNK7QObject6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QObject::installEventFilter(QObject * );
  fn _ZN7QObject18installEventFilterEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QObject::blockSignals(bool b);
  fn _ZN7QObject12blockSignalsEb(qthis: *mut c_void, arg0: int8_t) -> int8_t;
  // proto:  void QObject::setObjectName(const QString & name);
  fn _ZN7QObject13setObjectNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QObject::dumpObjectInfo();
  fn _ZN7QObject14dumpObjectInfoEv(qthis: *mut c_void) ;
  // proto:  void QObject::killTimer(int id);
  fn _ZN7QObject9killTimerEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QObject)=1
pub struct QObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QObject {
  pub fn inherits<T: QObject_inherits>(&mut self, value: T) -> i8 {
    return value.inherits(self);
    // return 1;
  }
}

pub trait QObject_inherits {
  fn inherits(self, rsthis: &mut QObject) -> i8;
}

// proto:  bool QObject::inherits(const char * classname);
impl<'a> /*trait*/ QObject_inherits for (&'a  String) {
  fn inherits(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8inheritsEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZNK7QObject8inheritsEPKc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn destroyed<T: QObject_destroyed>(&mut self, value: T)  {
     value.destroyed(self);
    // return 1;
  }
}

pub trait QObject_destroyed {
  fn destroyed(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::destroyed(QObject * );
impl<'a> /*trait*/ QObject_destroyed for (&'a mut QObject) {
  fn destroyed(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9destroyedEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject9destroyedEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn moveToThread<T: QObject_moveToThread>(&mut self, value: T)  {
     value.moveToThread(self);
    // return 1;
  }
}

pub trait QObject_moveToThread {
  fn moveToThread(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::moveToThread(QThread * thread);
impl<'a> /*trait*/ QObject_moveToThread for (&'a mut QThread) {
  fn moveToThread(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject12moveToThreadEP7QThread()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject12moveToThreadEP7QThread(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn removeEventFilter<T: QObject_removeEventFilter>(&mut self, value: T)  {
     value.removeEventFilter(self);
    // return 1;
  }
}

pub trait QObject_removeEventFilter {
  fn removeEventFilter(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::removeEventFilter(QObject * );
impl<'a> /*trait*/ QObject_removeEventFilter for (&'a mut QObject) {
  fn removeEventFilter(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject17removeEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject17removeEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn dumpObjectTree<T: QObject_dumpObjectTree>(&mut self, value: T)  {
     value.dumpObjectTree(self);
    // return 1;
  }
}

pub trait QObject_dumpObjectTree {
  fn dumpObjectTree(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::dumpObjectTree();
impl<'a> /*trait*/ QObject_dumpObjectTree for () {
  fn dumpObjectTree(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject14dumpObjectTreeEv()};
     unsafe {_ZN7QObject14dumpObjectTreeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn eventFilter<T: QObject_eventFilter>(&mut self, value: T) -> i8 {
    return value.eventFilter(self);
    // return 1;
  }
}

pub trait QObject_eventFilter {
  fn eventFilter(self, rsthis: &mut QObject) -> i8;
}

// proto:  bool QObject::eventFilter(QObject * , QEvent * );
impl<'a> /*trait*/ QObject_eventFilter for (&'a mut QObject, &'a mut QEvent) {
  fn eventFilter(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11eventFilterEPS_P6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QObject11eventFilterEPS_P6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn setUserData<T: QObject_setUserData>(&mut self, value: T)  {
     value.setUserData(self);
    // return 1;
  }
}

pub trait QObject_setUserData {
  fn setUserData(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::setUserData(uint id, QObjectUserData * data);
impl<'a> /*trait*/ QObject_setUserData for (u32, &'a mut QObjectUserData) {
  fn setUserData(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11setUserDataEjP15QObjectUserData()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject11setUserDataEjP15QObjectUserData(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn NewQObject<T: QObject_NewQObject>(value: T) -> QObject {
    let rsthis = value.NewQObject();
    return rsthis;
    // return 1;
  }
}

pub trait QObject_NewQObject {
  fn NewQObject(self) -> QObject;
}

// proto: void QObject::NewQObject(const QObject & );
impl<'a> /*trait*/ QObject_NewQObject for (&'a  QObject) {
  fn NewQObject(self) -> QObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QObjectC1ERKS_(qthis, arg0)};
    let rsthis = QObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn disconnect<T: QObject_disconnect>(&mut self, value: T) -> i8 {
    return value.disconnect(self);
    // return 1;
  }
}

pub trait QObject_disconnect {
  fn disconnect(self, rsthis: &mut QObject) -> i8;
}

// proto: static bool QObject::disconnect(const QObject * sender, const QMetaMethod & signal, const QObject * receiver, const QMetaMethod & member);
impl<'a> /*trait*/ QObject_disconnect for (&'a  QObject, &'a  QMetaMethod, &'a  QObject, &'a  QMetaMethod) {
  fn disconnect(self, rsthis: &mut QObject) -> i8 {
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

impl /*struct*/ QObject {
  pub fn event<T: QObject_event>(&mut self, value: T) -> i8 {
    return value.event(self);
    // return 1;
  }
}

pub trait QObject_event {
  fn event(self, rsthis: &mut QObject) -> i8;
}

// proto:  bool QObject::event(QEvent * );
impl<'a> /*trait*/ QObject_event for (&'a mut QEvent) {
  fn event(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QObject5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn dynamicPropertyNames<T: QObject_dynamicPropertyNames>(&mut self, value: T)  {
     value.dynamicPropertyNames(self);
    // return 1;
  }
}

pub trait QObject_dynamicPropertyNames {
  fn dynamicPropertyNames(self, rsthis: &mut QObject) ;
}

// proto:  QList<QByteArray> QObject::dynamicPropertyNames();
impl<'a> /*trait*/ QObject_dynamicPropertyNames for () {
  fn dynamicPropertyNames(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject20dynamicPropertyNamesEv()};
     unsafe {_ZNK7QObject20dynamicPropertyNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn isWidgetType<T: QObject_isWidgetType>(&mut self, value: T) -> i8 {
    return value.isWidgetType(self);
    // return 1;
  }
}

pub trait QObject_isWidgetType {
  fn isWidgetType(self, rsthis: &mut QObject) -> i8;
}

// proto:  bool QObject::isWidgetType();
impl<'a> /*trait*/ QObject_isWidgetType for () {
  fn isWidgetType(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject12isWidgetTypeEv()};
    let mut ret = unsafe {_ZNK7QObject12isWidgetTypeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn property<T: QObject_property>(&mut self, value: T) -> QVariant {
    return value.property(self);
    // return 1;
  }
}

pub trait QObject_property {
  fn property(self, rsthis: &mut QObject) -> QVariant;
}

// proto:  QVariant QObject::property(const char * name);
impl<'a> /*trait*/ QObject_property for (&'a  String) {
  fn property(self, rsthis: &mut QObject) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8propertyEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZNK7QObject8propertyEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn thread<T: QObject_thread>(&mut self, value: T) -> QThread {
    return value.thread(self);
    // return 1;
  }
}

pub trait QObject_thread {
  fn thread(self, rsthis: &mut QObject) -> QThread;
}

// proto:  QThread * QObject::thread();
impl<'a> /*trait*/ QObject_thread for () {
  fn thread(self, rsthis: &mut QObject) -> QThread {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject6threadEv()};
    let mut ret = unsafe {_ZNK7QObject6threadEv(rsthis.qclsinst)};
    let mut ret1 = QThread{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn metaObject<T: QObject_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QObject_metaObject {
  fn metaObject(self, rsthis: &mut QObject) ;
}

// proto:  const QMetaObject * QObject::metaObject();
impl<'a> /*trait*/ QObject_metaObject for () {
  fn metaObject(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10metaObjectEv()};
     unsafe {_ZNK7QObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn setParent<T: QObject_setParent>(&mut self, value: T)  {
     value.setParent(self);
    // return 1;
  }
}

pub trait QObject_setParent {
  fn setParent(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::setParent(QObject * );
impl<'a> /*trait*/ QObject_setParent for (&'a mut QObject) {
  fn setParent(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject9setParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QObject::disconnect(const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect for (&'a  QObject, &'a  String) {
  fn disconnect(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10disconnectEPKS_PKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZNK7QObject10disconnectEPKS_PKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn children<T: QObject_children>(&mut self, value: T)  {
     value.children(self);
    // return 1;
  }
}

pub trait QObject_children {
  fn children(self, rsthis: &mut QObject) ;
}

// proto:  const QObjectList & QObject::children();
impl<'a> /*trait*/ QObject_children for () {
  fn children(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8childrenEv()};
     unsafe {_ZNK7QObject8childrenEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn isWindowType<T: QObject_isWindowType>(&mut self, value: T) -> i8 {
    return value.isWindowType(self);
    // return 1;
  }
}

pub trait QObject_isWindowType {
  fn isWindowType(self, rsthis: &mut QObject) -> i8;
}

// proto:  bool QObject::isWindowType();
impl<'a> /*trait*/ QObject_isWindowType for () {
  fn isWindowType(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject12isWindowTypeEv()};
    let mut ret = unsafe {_ZNK7QObject12isWindowTypeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QObject::disconnect(const char * signal, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect for (&'a  String, &'a  QObject, &'a  String) {
  fn disconnect(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10disconnectEPKcPKS_S1_()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZNK7QObject10disconnectEPKcPKS_S1_(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn deleteLater<T: QObject_deleteLater>(&mut self, value: T)  {
     value.deleteLater(self);
    // return 1;
  }
}

pub trait QObject_deleteLater {
  fn deleteLater(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::deleteLater();
impl<'a> /*trait*/ QObject_deleteLater for () {
  fn deleteLater(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11deleteLaterEv()};
     unsafe {_ZN7QObject11deleteLaterEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn FreeQObject<T: QObject_FreeQObject>(&mut self, value: T)  {
     value.FreeQObject(self);
    // return 1;
  }
}

pub trait QObject_FreeQObject {
  fn FreeQObject(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::FreeQObject();
impl<'a> /*trait*/ QObject_FreeQObject for () {
  fn FreeQObject(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObjectD0Ev()};
     unsafe {_ZN7QObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QObject::NewQObject(QObject * parent);
impl<'a> /*trait*/ QObject_NewQObject for (&'a mut QObject) {
  fn NewQObject(self) -> QObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObjectC1EPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QObjectC1EPS_(qthis, arg0)};
    let rsthis = QObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn objectName<T: QObject_objectName>(&mut self, value: T) -> QString {
    return value.objectName(self);
    // return 1;
  }
}

pub trait QObject_objectName {
  fn objectName(self, rsthis: &mut QObject) -> QString;
}

// proto:  QString QObject::objectName();
impl<'a> /*trait*/ QObject_objectName for () {
  fn objectName(self, rsthis: &mut QObject) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10objectNameEv()};
    let mut ret = unsafe {_ZNK7QObject10objectNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn setProperty<T: QObject_setProperty>(&mut self, value: T) -> i8 {
    return value.setProperty(self);
    // return 1;
  }
}

pub trait QObject_setProperty {
  fn setProperty(self, rsthis: &mut QObject) -> i8;
}

// proto:  bool QObject::setProperty(const char * name, const QVariant & value);
impl<'a> /*trait*/ QObject_setProperty for (&'a  String, &'a  QVariant) {
  fn setProperty(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11setPropertyEPKcRK8QVariant()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QObject11setPropertyEPKcRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto: static bool QObject::disconnect(const QObject * sender, const char * signal, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect for (&'a  QObject, &'a  String, &'a  QObject, &'a  String) {
  fn disconnect(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject10disconnectEPKS_PKcS1_S3_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN7QObject10disconnectEPKS_PKcS1_S3_(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn signalsBlocked<T: QObject_signalsBlocked>(&mut self, value: T) -> i8 {
    return value.signalsBlocked(self);
    // return 1;
  }
}

pub trait QObject_signalsBlocked {
  fn signalsBlocked(self, rsthis: &mut QObject) -> i8;
}

// proto:  bool QObject::signalsBlocked();
impl<'a> /*trait*/ QObject_signalsBlocked for () {
  fn signalsBlocked(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject14signalsBlockedEv()};
    let mut ret = unsafe {_ZNK7QObject14signalsBlockedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn registerUserData<T: QObject_registerUserData>(&mut self, value: T) -> u32 {
    return value.registerUserData(self);
    // return 1;
  }
}

pub trait QObject_registerUserData {
  fn registerUserData(self, rsthis: &mut QObject) -> u32;
}

// proto: static unsigned int QObject::registerUserData();
impl<'a> /*trait*/ QObject_registerUserData for () {
  fn registerUserData(self, rsthis: &mut QObject) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject16registerUserDataEv()};
    let mut ret = unsafe {_ZN7QObject16registerUserDataEv()};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn userData<T: QObject_userData>(&mut self, value: T) -> QObjectUserData {
    return value.userData(self);
    // return 1;
  }
}

pub trait QObject_userData {
  fn userData(self, rsthis: &mut QObject) -> QObjectUserData;
}

// proto:  QObjectUserData * QObject::userData(uint id);
impl<'a> /*trait*/ QObject_userData for (u32) {
  fn userData(self, rsthis: &mut QObject) -> QObjectUserData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8userDataEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK7QObject8userDataEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QObjectUserData{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn parent<T: QObject_parent>(&mut self, value: T) -> QObject {
    return value.parent(self);
    // return 1;
  }
}

pub trait QObject_parent {
  fn parent(self, rsthis: &mut QObject) -> QObject;
}

// proto:  QObject * QObject::parent();
impl<'a> /*trait*/ QObject_parent for () {
  fn parent(self, rsthis: &mut QObject) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject6parentEv()};
    let mut ret = unsafe {_ZNK7QObject6parentEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn installEventFilter<T: QObject_installEventFilter>(&mut self, value: T)  {
     value.installEventFilter(self);
    // return 1;
  }
}

pub trait QObject_installEventFilter {
  fn installEventFilter(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::installEventFilter(QObject * );
impl<'a> /*trait*/ QObject_installEventFilter for (&'a mut QObject) {
  fn installEventFilter(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject18installEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject18installEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn blockSignals<T: QObject_blockSignals>(&mut self, value: T) -> i8 {
    return value.blockSignals(self);
    // return 1;
  }
}

pub trait QObject_blockSignals {
  fn blockSignals(self, rsthis: &mut QObject) -> i8;
}

// proto:  bool QObject::blockSignals(bool b);
impl<'a> /*trait*/ QObject_blockSignals for (i8) {
  fn blockSignals(self, rsthis: &mut QObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject12blockSignalsEb()};
    let arg0 = self  as int8_t;
    let mut ret = unsafe {_ZN7QObject12blockSignalsEb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn setObjectName<T: QObject_setObjectName>(&mut self, value: T)  {
     value.setObjectName(self);
    // return 1;
  }
}

pub trait QObject_setObjectName {
  fn setObjectName(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::setObjectName(const QString & name);
impl<'a> /*trait*/ QObject_setObjectName for (&'a  QString) {
  fn setObjectName(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject13setObjectNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QObject13setObjectNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn dumpObjectInfo<T: QObject_dumpObjectInfo>(&mut self, value: T)  {
     value.dumpObjectInfo(self);
    // return 1;
  }
}

pub trait QObject_dumpObjectInfo {
  fn dumpObjectInfo(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::dumpObjectInfo();
impl<'a> /*trait*/ QObject_dumpObjectInfo for () {
  fn dumpObjectInfo(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject14dumpObjectInfoEv()};
     unsafe {_ZN7QObject14dumpObjectInfoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn killTimer<T: QObject_killTimer>(&mut self, value: T)  {
     value.killTimer(self);
    // return 1;
  }
}

pub trait QObject_killTimer {
  fn killTimer(self, rsthis: &mut QObject) ;
}

// proto:  void QObject::killTimer(int id);
impl<'a> /*trait*/ QObject_killTimer for (i32) {
  fn killTimer(self, rsthis: &mut QObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9killTimerEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QObject9killTimerEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

