// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qaccessibleobject.h
// dst-file: /src/gui/qaccessibleobject.rs
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
// use super::qaccessibleobject::QAccessibleObject; // 773
use std::ops::Deref;
use super::qwindow::QWindow; // 773
use super::qaccessible::QAccessibleInterface; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QAccessibleApplication::QAccessibleApplication();
  fn _ZN22QAccessibleApplicationC1Ev(qthis: *mut c_void);
  // proto:  QWindow * QAccessibleApplication::window();
  fn _ZNK22QAccessibleApplication6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleApplication::child(int index);
  fn _ZNK22QAccessibleApplication5childEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QAccessibleApplication::childCount();
  fn _ZNK22QAccessibleApplication10childCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QAccessibleInterface * QAccessibleApplication::parent();
  fn _ZNK22QAccessibleApplication6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleApplication::focusChild();
  fn _ZNK22QAccessibleApplication10focusChildEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleApplication::indexOfChild(const QAccessibleInterface * );
  fn _ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QAccessibleObject::QAccessibleObject(QObject * object);
  fn _ZN17QAccessibleObjectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QObject * QAccessibleObject::object();
  fn _ZNK17QAccessibleObject6objectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QAccessibleObject::rect();
  fn _ZNK17QAccessibleObject4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleObject::childAt(int x, int y);
  fn _ZNK17QAccessibleObject7childAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QAccessibleObject::QAccessibleObject(const QAccessibleObject & );
  fn _ZN17QAccessibleObjectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QAccessibleObject::isValid();
  fn _ZNK17QAccessibleObject7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAccessibleObject::~QAccessibleObject();
  fn _ZN17QAccessibleObjectD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAccessibleApplication)=16
pub struct QAccessibleApplication {
  qbase: QAccessibleObject,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleObject)=16
pub struct QAccessibleObject {
  qbase: QAccessibleInterface,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleApplication {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleApplication {
    return QAccessibleApplication{qbase: QAccessibleObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleApplication {
  type Target = QAccessibleObject;

  fn deref(&self) -> &QAccessibleObject {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleObject> for QAccessibleApplication {
  fn as_ref(&self) -> &QAccessibleObject {
    return &self.qbase;
  }
}
  // proto:  void QAccessibleApplication::QAccessibleApplication();
impl /*struct*/ QAccessibleApplication {
  pub fn NewQAccessibleApplication<T: QAccessibleApplication_NewQAccessibleApplication>(value: T) -> QAccessibleApplication {
    let rsthis = value.NewQAccessibleApplication();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleApplication_NewQAccessibleApplication {
  fn NewQAccessibleApplication(self) -> QAccessibleApplication;
}

  // proto:  void QAccessibleApplication::QAccessibleApplication();
impl<'a> /*trait*/ QAccessibleApplication_NewQAccessibleApplication for () {
  fn NewQAccessibleApplication(self) -> QAccessibleApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QAccessibleApplicationC1Ev()};
    unsafe {_ZN22QAccessibleApplicationC1Ev(qthis)};
    let rsthis = QAccessibleApplication{/**/qbase: QAccessibleObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWindow * QAccessibleApplication::window();
impl /*struct*/ QAccessibleApplication {
  pub fn window<RetType, T: QAccessibleApplication_window<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_window<RetType> {
  fn window(self , rsthis: &mut QAccessibleApplication) -> RetType;
}

  // proto:  QWindow * QAccessibleApplication::window();
impl<'a> /*trait*/ QAccessibleApplication_window<QWindow> for () {
  fn window(self , rsthis: &mut QAccessibleApplication) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication6windowEv()};
    let mut ret = unsafe {_ZNK22QAccessibleApplication6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleApplication::child(int index);
impl /*struct*/ QAccessibleApplication {
  pub fn child<RetType, T: QAccessibleApplication_child<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_child<RetType> {
  fn child(self , rsthis: &mut QAccessibleApplication) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleApplication::child(int index);
impl<'a> /*trait*/ QAccessibleApplication_child<QAccessibleInterface> for (i32) {
  fn child(self , rsthis: &mut QAccessibleApplication) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK22QAccessibleApplication5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleApplication::childCount();
impl /*struct*/ QAccessibleApplication {
  pub fn childCount<RetType, T: QAccessibleApplication_childCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.childCount(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_childCount<RetType> {
  fn childCount(self , rsthis: &mut QAccessibleApplication) -> RetType;
}

  // proto:  int QAccessibleApplication::childCount();
impl<'a> /*trait*/ QAccessibleApplication_childCount<i32> for () {
  fn childCount(self , rsthis: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication10childCountEv()};
    let mut ret = unsafe {_ZNK22QAccessibleApplication10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleApplication::parent();
impl /*struct*/ QAccessibleApplication {
  pub fn parent<RetType, T: QAccessibleApplication_parent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_parent<RetType> {
  fn parent(self , rsthis: &mut QAccessibleApplication) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleApplication::parent();
impl<'a> /*trait*/ QAccessibleApplication_parent<QAccessibleInterface> for () {
  fn parent(self , rsthis: &mut QAccessibleApplication) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication6parentEv()};
    let mut ret = unsafe {_ZNK22QAccessibleApplication6parentEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleApplication::focusChild();
impl /*struct*/ QAccessibleApplication {
  pub fn focusChild<RetType, T: QAccessibleApplication_focusChild<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focusChild(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_focusChild<RetType> {
  fn focusChild(self , rsthis: &mut QAccessibleApplication) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleApplication::focusChild();
impl<'a> /*trait*/ QAccessibleApplication_focusChild<QAccessibleInterface> for () {
  fn focusChild(self , rsthis: &mut QAccessibleApplication) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication10focusChildEv()};
    let mut ret = unsafe {_ZNK22QAccessibleApplication10focusChildEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleApplication::indexOfChild(const QAccessibleInterface * );
impl /*struct*/ QAccessibleApplication {
  pub fn indexOfChild<RetType, T: QAccessibleApplication_indexOfChild<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexOfChild(self);
    // return 1;
  }
}

pub trait QAccessibleApplication_indexOfChild<RetType> {
  fn indexOfChild(self , rsthis: &mut QAccessibleApplication) -> RetType;
}

  // proto:  int QAccessibleApplication::indexOfChild(const QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleApplication_indexOfChild<i32> for (QAccessibleInterface) {
  fn indexOfChild(self , rsthis: &mut QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleObject {
    return QAccessibleObject{qbase: QAccessibleInterface::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleObject {
  type Target = QAccessibleInterface;

  fn deref(&self) -> &QAccessibleInterface {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleInterface> for QAccessibleObject {
  fn as_ref(&self) -> &QAccessibleInterface {
    return &self.qbase;
  }
}
  // proto:  void QAccessibleObject::QAccessibleObject(QObject * object);
impl /*struct*/ QAccessibleObject {
  pub fn NewQAccessibleObject<T: QAccessibleObject_NewQAccessibleObject>(value: T) -> QAccessibleObject {
    let rsthis = value.NewQAccessibleObject();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleObject_NewQAccessibleObject {
  fn NewQAccessibleObject(self) -> QAccessibleObject;
}

  // proto:  void QAccessibleObject::QAccessibleObject(QObject * object);
impl<'a> /*trait*/ QAccessibleObject_NewQAccessibleObject for (QObject) {
  fn NewQAccessibleObject(self) -> QAccessibleObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessibleObjectC1EP7QObject(qthis, arg0)};
    let rsthis = QAccessibleObject{/**/qbase: QAccessibleInterface::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QObject * QAccessibleObject::object();
impl /*struct*/ QAccessibleObject {
  pub fn object<RetType, T: QAccessibleObject_object<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QAccessibleObject_object<RetType> {
  fn object(self , rsthis: &mut QAccessibleObject) -> RetType;
}

  // proto:  QObject * QAccessibleObject::object();
impl<'a> /*trait*/ QAccessibleObject_object<QObject> for () {
  fn object(self , rsthis: &mut QAccessibleObject) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject6objectEv()};
    let mut ret = unsafe {_ZNK17QAccessibleObject6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QAccessibleObject::rect();
impl /*struct*/ QAccessibleObject {
  pub fn rect<RetType, T: QAccessibleObject_rect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QAccessibleObject_rect<RetType> {
  fn rect(self , rsthis: &mut QAccessibleObject) -> RetType;
}

  // proto:  QRect QAccessibleObject::rect();
impl<'a> /*trait*/ QAccessibleObject_rect<QRect> for () {
  fn rect(self , rsthis: &mut QAccessibleObject) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject4rectEv()};
    let mut ret = unsafe {_ZNK17QAccessibleObject4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleObject::childAt(int x, int y);
impl /*struct*/ QAccessibleObject {
  pub fn childAt<RetType, T: QAccessibleObject_childAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.childAt(self);
    // return 1;
  }
}

pub trait QAccessibleObject_childAt<RetType> {
  fn childAt(self , rsthis: &mut QAccessibleObject) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleObject::childAt(int x, int y);
impl<'a> /*trait*/ QAccessibleObject_childAt<QAccessibleInterface> for (i32, i32) {
  fn childAt(self , rsthis: &mut QAccessibleObject) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK17QAccessibleObject7childAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleObject::QAccessibleObject(const QAccessibleObject & );
impl<'a> /*trait*/ QAccessibleObject_NewQAccessibleObject for (QAccessibleObject) {
  fn NewQAccessibleObject(self) -> QAccessibleObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessibleObjectC1ERKS_(qthis, arg0)};
    let rsthis = QAccessibleObject{/**/qbase: QAccessibleInterface::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QAccessibleObject::isValid();
impl /*struct*/ QAccessibleObject {
  pub fn isValid<RetType, T: QAccessibleObject_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QAccessibleObject_isValid<RetType> {
  fn isValid(self , rsthis: &mut QAccessibleObject) -> RetType;
}

  // proto:  bool QAccessibleObject::isValid();
impl<'a> /*trait*/ QAccessibleObject_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QAccessibleObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject7isValidEv()};
    let mut ret = unsafe {_ZNK17QAccessibleObject7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAccessibleObject::~QAccessibleObject();
impl /*struct*/ QAccessibleObject {
  pub fn FreeQAccessibleObject<RetType, T: QAccessibleObject_FreeQAccessibleObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleObject(self);
    // return 1;
  }
}

pub trait QAccessibleObject_FreeQAccessibleObject<RetType> {
  fn FreeQAccessibleObject(self , rsthis: &mut QAccessibleObject) -> RetType;
}

  // proto:  void QAccessibleObject::~QAccessibleObject();
impl<'a> /*trait*/ QAccessibleObject_FreeQAccessibleObject<()> for () {
  fn FreeQAccessibleObject(self , rsthis: &mut QAccessibleObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectD0Ev()};
     unsafe {_ZN17QAccessibleObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

