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
  fn _ZNK7QObject8inheritsEPKc(arg0: *const c_char) -> i32;
  fn _ZN7QObject9destroyedEPS_(arg0: *mut c_void) -> i32;
  fn _ZN7QObject12moveToThreadEP7QThread(arg0: *mut c_void) -> i32;
  fn _ZN7QObject17removeEventFilterEPS_(arg0: *mut c_void) -> i32;
  fn _ZN7QObject14dumpObjectTreeEv() -> i32;
  fn _ZN7QObject11eventFilterEPS_P6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  fn _ZN7QObject11setUserDataEjP15QObjectUserData(arg0: c_uint, arg1: *mut c_void) -> i32;
  fn _ZN7QObjectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN7QObject10disconnectEPKS_RK11QMetaMethodS1_S4_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
  fn _ZN7QObject5eventEP6QEvent(arg0: *mut c_void) -> i32;
  fn _ZNK7QObject20dynamicPropertyNamesEv() -> i32;
  fn _ZNK7QObject12isWidgetTypeEv() -> i32;
  fn _ZNK7QObject8propertyEPKc(arg0: *const c_char) -> i32;
  fn _ZNK7QObject6threadEv() -> i32;
  fn _ZNK7QObject10metaObjectEv() -> i32;
  fn _ZN7QObject9setParentEPS_(arg0: *mut c_void) -> i32;
  fn _ZNK7QObject10disconnectEPKS_PKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  fn _ZNK7QObject8childrenEv() -> i32;
  fn _ZNK7QObject12isWindowTypeEv() -> i32;
  fn _ZNK7QObject10disconnectEPKcPKS_S1_(arg0: *const c_char, arg1: *const c_void, arg2: *const c_char) -> i32;
  fn _ZN7QObject11deleteLaterEv() -> i32;
  fn _ZN7QObjectD0Ev() -> i32;
  fn _ZN7QObjectC1EPS_(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK7QObject10objectNameEv() -> i32;
  fn _ZN7QObject11setPropertyEPKcRK8QVariant(arg0: *const c_char, arg1: *const c_void) -> i32;
  fn _ZN7QObject10disconnectEPKS_PKcS1_S3_(arg0: *const c_void, arg1: *const c_char, arg2: *const c_void, arg3: *const c_char) -> i32;
  fn _ZNK7QObject14signalsBlockedEv() -> i32;
  fn _ZN7QObject16registerUserDataEv() -> i32;
  fn _ZNK7QObject8userDataEj(arg0: c_uint) -> i32;
  fn _ZNK7QObject6parentEv() -> i32;
  fn _ZN7QObject18installEventFilterEPS_(arg0: *mut c_void) -> i32;
  fn _ZN7QObject12blockSignalsEb(arg0: int8_t) -> i32;
  fn _ZN7QObject13setObjectNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZN7QObject14dumpObjectInfoEv() -> i32;
  fn _ZN7QObject9killTimerEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QObject)=1
pub struct QObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QObject {
  pub fn inherits<T: QObject_inherits>(&mut self, value: T) -> i32 {
    value.inherits(self);
    return 1;
  }
}

pub trait QObject_inherits {
  fn inherits(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::inherits(const char * classname);
impl<'a> /*trait*/ QObject_inherits for (&'a  String) {
  fn inherits(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8inheritsEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZNK7QObject8inheritsEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn destroyed<T: QObject_destroyed>(&mut self, value: T) -> i32 {
    value.destroyed(self);
    return 1;
  }
}

pub trait QObject_destroyed {
  fn destroyed(self, this: &mut QObject) -> i32;
}

// proto: void QObject::destroyed(QObject * );
impl<'a> /*trait*/ QObject_destroyed for (&'a mut QObject) {
  fn destroyed(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9destroyedEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QObject9destroyedEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn moveToThread<T: QObject_moveToThread>(&mut self, value: T) -> i32 {
    value.moveToThread(self);
    return 1;
  }
}

pub trait QObject_moveToThread {
  fn moveToThread(self, this: &mut QObject) -> i32;
}

// proto: void QObject::moveToThread(QThread * thread);
impl<'a> /*trait*/ QObject_moveToThread for (&'a mut QThread) {
  fn moveToThread(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject12moveToThreadEP7QThread()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QObject12moveToThreadEP7QThread(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn removeEventFilter<T: QObject_removeEventFilter>(&mut self, value: T) -> i32 {
    value.removeEventFilter(self);
    return 1;
  }
}

pub trait QObject_removeEventFilter {
  fn removeEventFilter(self, this: &mut QObject) -> i32;
}

// proto: void QObject::removeEventFilter(QObject * );
impl<'a> /*trait*/ QObject_removeEventFilter for (&'a mut QObject) {
  fn removeEventFilter(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject17removeEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QObject17removeEventFilterEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn dumpObjectTree<T: QObject_dumpObjectTree>(&mut self, value: T) -> i32 {
    value.dumpObjectTree(self);
    return 1;
  }
}

pub trait QObject_dumpObjectTree {
  fn dumpObjectTree(self, this: &mut QObject) -> i32;
}

// proto: void QObject::dumpObjectTree();
impl<'a> /*trait*/ QObject_dumpObjectTree for () {
  fn dumpObjectTree(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject14dumpObjectTreeEv()};
    unsafe {_ZN7QObject14dumpObjectTreeEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn eventFilter<T: QObject_eventFilter>(&mut self, value: T) -> i32 {
    value.eventFilter(self);
    return 1;
  }
}

pub trait QObject_eventFilter {
  fn eventFilter(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::eventFilter(QObject * , QEvent * );
impl<'a> /*trait*/ QObject_eventFilter for (&'a mut QObject, &'a mut QEvent) {
  fn eventFilter(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11eventFilterEPS_P6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QObject11eventFilterEPS_P6QEvent(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn setUserData<T: QObject_setUserData>(&mut self, value: T) -> i32 {
    value.setUserData(self);
    return 1;
  }
}

pub trait QObject_setUserData {
  fn setUserData(self, this: &mut QObject) -> i32;
}

// proto: void QObject::setUserData(uint id, QObjectUserData * data);
impl<'a> /*trait*/ QObject_setUserData for (u32, &'a mut QObjectUserData) {
  fn setUserData(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11setUserDataEjP15QObjectUserData()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QObject11setUserDataEjP15QObjectUserData(arg0, arg1)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QObjectC1ERKS_(qthis, arg0)};
    let rsthis = QObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QObject {
  pub fn disconnect<T: QObject_disconnect>(&mut self, value: T) -> i32 {
    value.disconnect(self);
    return 1;
  }
}

pub trait QObject_disconnect {
  fn disconnect(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::disconnect(const QObject * sender, const QMetaMethod & signal, const QObject * receiver, const QMetaMethod & member);
impl<'a> /*trait*/ QObject_disconnect for (&'a  QObject, &'a  QMetaMethod, &'a  QObject, &'a  QMetaMethod) {
  fn disconnect(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject10disconnectEPKS_RK11QMetaMethodS1_S4_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN7QObject10disconnectEPKS_RK11QMetaMethodS1_S4_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn event<T: QObject_event>(&mut self, value: T) -> i32 {
    value.event(self);
    return 1;
  }
}

pub trait QObject_event {
  fn event(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::event(QEvent * );
impl<'a> /*trait*/ QObject_event for (&'a mut QEvent) {
  fn event(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QObject5eventEP6QEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn dynamicPropertyNames<T: QObject_dynamicPropertyNames>(&mut self, value: T) -> i32 {
    value.dynamicPropertyNames(self);
    return 1;
  }
}

pub trait QObject_dynamicPropertyNames {
  fn dynamicPropertyNames(self, this: &mut QObject) -> i32;
}

// proto: QList<QByteArray> QObject::dynamicPropertyNames();
impl<'a> /*trait*/ QObject_dynamicPropertyNames for () {
  fn dynamicPropertyNames(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject20dynamicPropertyNamesEv()};
    unsafe {_ZNK7QObject20dynamicPropertyNamesEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn isWidgetType<T: QObject_isWidgetType>(&mut self, value: T) -> i32 {
    value.isWidgetType(self);
    return 1;
  }
}

pub trait QObject_isWidgetType {
  fn isWidgetType(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::isWidgetType();
impl<'a> /*trait*/ QObject_isWidgetType for () {
  fn isWidgetType(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject12isWidgetTypeEv()};
    unsafe {_ZNK7QObject12isWidgetTypeEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn property<T: QObject_property>(&mut self, value: T) -> i32 {
    value.property(self);
    return 1;
  }
}

pub trait QObject_property {
  fn property(self, this: &mut QObject) -> i32;
}

// proto: QVariant QObject::property(const char * name);
impl<'a> /*trait*/ QObject_property for (&'a  String) {
  fn property(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8propertyEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZNK7QObject8propertyEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn thread<T: QObject_thread>(&mut self, value: T) -> i32 {
    value.thread(self);
    return 1;
  }
}

pub trait QObject_thread {
  fn thread(self, this: &mut QObject) -> i32;
}

// proto: QThread * QObject::thread();
impl<'a> /*trait*/ QObject_thread for () {
  fn thread(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject6threadEv()};
    unsafe {_ZNK7QObject6threadEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn metaObject<T: QObject_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QObject_metaObject {
  fn metaObject(self, this: &mut QObject) -> i32;
}

// proto: const QMetaObject * QObject::metaObject();
impl<'a> /*trait*/ QObject_metaObject for () {
  fn metaObject(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10metaObjectEv()};
    unsafe {_ZNK7QObject10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn setParent<T: QObject_setParent>(&mut self, value: T) -> i32 {
    value.setParent(self);
    return 1;
  }
}

pub trait QObject_setParent {
  fn setParent(self, this: &mut QObject) -> i32;
}

// proto: void QObject::setParent(QObject * );
impl<'a> /*trait*/ QObject_setParent for (&'a mut QObject) {
  fn setParent(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QObject9setParentEPS_(arg0)};
    return 1;
  }
}

// proto: bool QObject::disconnect(const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect for (&'a  QObject, &'a  String) {
  fn disconnect(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10disconnectEPKS_PKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZNK7QObject10disconnectEPKS_PKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn children<T: QObject_children>(&mut self, value: T) -> i32 {
    value.children(self);
    return 1;
  }
}

pub trait QObject_children {
  fn children(self, this: &mut QObject) -> i32;
}

// proto: const QObjectList & QObject::children();
impl<'a> /*trait*/ QObject_children for () {
  fn children(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8childrenEv()};
    unsafe {_ZNK7QObject8childrenEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn isWindowType<T: QObject_isWindowType>(&mut self, value: T) -> i32 {
    value.isWindowType(self);
    return 1;
  }
}

pub trait QObject_isWindowType {
  fn isWindowType(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::isWindowType();
impl<'a> /*trait*/ QObject_isWindowType for () {
  fn isWindowType(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject12isWindowTypeEv()};
    unsafe {_ZNK7QObject12isWindowTypeEv()};
    return 1;
  }
}

// proto: bool QObject::disconnect(const char * signal, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect for (&'a  String, &'a  QObject, &'a  String) {
  fn disconnect(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10disconnectEPKcPKS_S1_()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZNK7QObject10disconnectEPKcPKS_S1_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn deleteLater<T: QObject_deleteLater>(&mut self, value: T) -> i32 {
    value.deleteLater(self);
    return 1;
  }
}

pub trait QObject_deleteLater {
  fn deleteLater(self, this: &mut QObject) -> i32;
}

// proto: void QObject::deleteLater();
impl<'a> /*trait*/ QObject_deleteLater for () {
  fn deleteLater(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11deleteLaterEv()};
    unsafe {_ZN7QObject11deleteLaterEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn FreeQObject<T: QObject_FreeQObject>(&mut self, value: T) -> i32 {
    value.FreeQObject(self);
    return 1;
  }
}

pub trait QObject_FreeQObject {
  fn FreeQObject(self, this: &mut QObject) -> i32;
}

// proto: void QObject::FreeQObject();
impl<'a> /*trait*/ QObject_FreeQObject for () {
  fn FreeQObject(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObjectD0Ev()};
    unsafe {_ZN7QObjectD0Ev()};
    return 1;
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
  pub fn objectName<T: QObject_objectName>(&mut self, value: T) -> i32 {
    value.objectName(self);
    return 1;
  }
}

pub trait QObject_objectName {
  fn objectName(self, this: &mut QObject) -> i32;
}

// proto: QString QObject::objectName();
impl<'a> /*trait*/ QObject_objectName for () {
  fn objectName(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject10objectNameEv()};
    unsafe {_ZNK7QObject10objectNameEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn setProperty<T: QObject_setProperty>(&mut self, value: T) -> i32 {
    value.setProperty(self);
    return 1;
  }
}

pub trait QObject_setProperty {
  fn setProperty(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::setProperty(const char * name, const QVariant & value);
impl<'a> /*trait*/ QObject_setProperty for (&'a  String, &'a  QVariant) {
  fn setProperty(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject11setPropertyEPKcRK8QVariant()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QObject11setPropertyEPKcRK8QVariant(arg0, arg1)};
    return 1;
  }
}

// proto: bool QObject::disconnect(const QObject * sender, const char * signal, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QObject_disconnect for (&'a  QObject, &'a  String, &'a  QObject, &'a  String) {
  fn disconnect(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject10disconnectEPKS_PKcS1_S3_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.as_ptr()  as *const c_char;
    unsafe {_ZN7QObject10disconnectEPKS_PKcS1_S3_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn signalsBlocked<T: QObject_signalsBlocked>(&mut self, value: T) -> i32 {
    value.signalsBlocked(self);
    return 1;
  }
}

pub trait QObject_signalsBlocked {
  fn signalsBlocked(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::signalsBlocked();
impl<'a> /*trait*/ QObject_signalsBlocked for () {
  fn signalsBlocked(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject14signalsBlockedEv()};
    unsafe {_ZNK7QObject14signalsBlockedEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn registerUserData<T: QObject_registerUserData>(&mut self, value: T) -> i32 {
    value.registerUserData(self);
    return 1;
  }
}

pub trait QObject_registerUserData {
  fn registerUserData(self, this: &mut QObject) -> i32;
}

// proto: unsigned int QObject::registerUserData();
impl<'a> /*trait*/ QObject_registerUserData for () {
  fn registerUserData(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject16registerUserDataEv()};
    unsafe {_ZN7QObject16registerUserDataEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn userData<T: QObject_userData>(&mut self, value: T) -> i32 {
    value.userData(self);
    return 1;
  }
}

pub trait QObject_userData {
  fn userData(self, this: &mut QObject) -> i32;
}

// proto: QObjectUserData * QObject::userData(uint id);
impl<'a> /*trait*/ QObject_userData for (u32) {
  fn userData(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject8userDataEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZNK7QObject8userDataEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn parent<T: QObject_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QObject_parent {
  fn parent(self, this: &mut QObject) -> i32;
}

// proto: QObject * QObject::parent();
impl<'a> /*trait*/ QObject_parent for () {
  fn parent(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QObject6parentEv()};
    unsafe {_ZNK7QObject6parentEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn installEventFilter<T: QObject_installEventFilter>(&mut self, value: T) -> i32 {
    value.installEventFilter(self);
    return 1;
  }
}

pub trait QObject_installEventFilter {
  fn installEventFilter(self, this: &mut QObject) -> i32;
}

// proto: void QObject::installEventFilter(QObject * );
impl<'a> /*trait*/ QObject_installEventFilter for (&'a mut QObject) {
  fn installEventFilter(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject18installEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QObject18installEventFilterEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn blockSignals<T: QObject_blockSignals>(&mut self, value: T) -> i32 {
    value.blockSignals(self);
    return 1;
  }
}

pub trait QObject_blockSignals {
  fn blockSignals(self, this: &mut QObject) -> i32;
}

// proto: bool QObject::blockSignals(bool b);
impl<'a> /*trait*/ QObject_blockSignals for (i8) {
  fn blockSignals(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject12blockSignalsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QObject12blockSignalsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn setObjectName<T: QObject_setObjectName>(&mut self, value: T) -> i32 {
    value.setObjectName(self);
    return 1;
  }
}

pub trait QObject_setObjectName {
  fn setObjectName(self, this: &mut QObject) -> i32;
}

// proto: void QObject::setObjectName(const QString & name);
impl<'a> /*trait*/ QObject_setObjectName for (&'a  QString) {
  fn setObjectName(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject13setObjectNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QObject13setObjectNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn dumpObjectInfo<T: QObject_dumpObjectInfo>(&mut self, value: T) -> i32 {
    value.dumpObjectInfo(self);
    return 1;
  }
}

pub trait QObject_dumpObjectInfo {
  fn dumpObjectInfo(self, this: &mut QObject) -> i32;
}

// proto: void QObject::dumpObjectInfo();
impl<'a> /*trait*/ QObject_dumpObjectInfo for () {
  fn dumpObjectInfo(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject14dumpObjectInfoEv()};
    unsafe {_ZN7QObject14dumpObjectInfoEv()};
    return 1;
  }
}

impl /*struct*/ QObject {
  pub fn killTimer<T: QObject_killTimer>(&mut self, value: T) -> i32 {
    value.killTimer(self);
    return 1;
  }
}

pub trait QObject_killTimer {
  fn killTimer(self, this: &mut QObject) -> i32;
}

// proto: void QObject::killTimer(int id);
impl<'a> /*trait*/ QObject_killTimer for (i32) {
  fn killTimer(self, this: &mut QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QObject9killTimerEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QObject9killTimerEi(arg0)};
    return 1;
  }
}

