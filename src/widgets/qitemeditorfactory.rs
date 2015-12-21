// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qitemeditorfactory.h
// dst-file: /src/widgets/qitemeditorfactory.rs
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
use super::super::core::qbytearray::QByteArray; // 771
use super::qwidget::QWidget; // 773
// use super::qitemeditorfactory::QItemEditorCreatorBase; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QByteArray QItemEditorCreatorBase::valuePropertyName();
  fn _ZNK22QItemEditorCreatorBase17valuePropertyNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
  fn _ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QItemEditorCreatorBase::~QItemEditorCreatorBase();
  fn _ZN22QItemEditorCreatorBaseD0Ev(qthis: *mut c_void);
  // proto:  void QItemEditorFactory::QItemEditorFactory();
  fn _ZN18QItemEditorFactoryC1Ev(qthis: *mut c_void);
  // proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
  fn _ZNK18QItemEditorFactory17valuePropertyNameEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
  fn _ZN18QItemEditorFactory14defaultFactoryEv() -> *mut c_void;
  // proto:  void QItemEditorFactory::~QItemEditorFactory();
  fn _ZN18QItemEditorFactoryD0Ev(qthis: *mut c_void);
  // proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
  fn _ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
  fn _ZN18QItemEditorFactory17setDefaultFactoryEPS_(arg0: *mut c_void);
  // proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
  fn _ZNK18QItemEditorFactory12createEditorEiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QItemEditorCreatorBase)=8
pub struct QItemEditorCreatorBase {
  pub qclsinst: *mut c_void,
}

// class sizeof(QItemEditorFactory)=1
pub struct QItemEditorFactory {
  pub qclsinst: *mut c_void,
}

  // proto:  QByteArray QItemEditorCreatorBase::valuePropertyName();
impl /*struct*/ QItemEditorCreatorBase {
  pub fn valuePropertyName<RetType, T: QItemEditorCreatorBase_valuePropertyName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valuePropertyName(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_valuePropertyName<RetType> {
  fn valuePropertyName(self , rsthis: &mut QItemEditorCreatorBase) -> RetType;
}

  // proto:  QByteArray QItemEditorCreatorBase::valuePropertyName();
impl<'a> /*trait*/ QItemEditorCreatorBase_valuePropertyName<QByteArray> for () {
  fn valuePropertyName(self , rsthis: &mut QItemEditorCreatorBase) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QItemEditorCreatorBase17valuePropertyNameEv()};
    let mut ret = unsafe {_ZNK22QItemEditorCreatorBase17valuePropertyNameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
impl /*struct*/ QItemEditorCreatorBase {
  pub fn createWidget<RetType, T: QItemEditorCreatorBase_createWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.createWidget(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_createWidget<RetType> {
  fn createWidget(self , rsthis: &mut QItemEditorCreatorBase) -> RetType;
}

  // proto:  QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
impl<'a> /*trait*/ QItemEditorCreatorBase_createWidget<QWidget> for (QWidget) {
  fn createWidget(self , rsthis: &mut QItemEditorCreatorBase) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemEditorCreatorBase::~QItemEditorCreatorBase();
impl /*struct*/ QItemEditorCreatorBase {
  pub fn FreeQItemEditorCreatorBase<RetType, T: QItemEditorCreatorBase_FreeQItemEditorCreatorBase<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQItemEditorCreatorBase(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_FreeQItemEditorCreatorBase<RetType> {
  fn FreeQItemEditorCreatorBase(self , rsthis: &mut QItemEditorCreatorBase) -> RetType;
}

  // proto:  void QItemEditorCreatorBase::~QItemEditorCreatorBase();
impl<'a> /*trait*/ QItemEditorCreatorBase_FreeQItemEditorCreatorBase<()> for () {
  fn FreeQItemEditorCreatorBase(self , rsthis: &mut QItemEditorCreatorBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QItemEditorCreatorBaseD0Ev()};
     unsafe {_ZN22QItemEditorCreatorBaseD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemEditorFactory::QItemEditorFactory();
impl /*struct*/ QItemEditorFactory {
  pub fn NewQItemEditorFactory<T: QItemEditorFactory_NewQItemEditorFactory>(value: T) -> QItemEditorFactory {
    let rsthis = value.NewQItemEditorFactory();
    return rsthis;
    // return 1;
  }
}

pub trait QItemEditorFactory_NewQItemEditorFactory {
  fn NewQItemEditorFactory(self) -> QItemEditorFactory;
}

  // proto:  void QItemEditorFactory::QItemEditorFactory();
impl<'a> /*trait*/ QItemEditorFactory_NewQItemEditorFactory for () {
  fn NewQItemEditorFactory(self) -> QItemEditorFactory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactoryC1Ev()};
    unsafe {_ZN18QItemEditorFactoryC1Ev(qthis)};
    let rsthis = QItemEditorFactory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
impl /*struct*/ QItemEditorFactory {
  pub fn valuePropertyName<RetType, T: QItemEditorFactory_valuePropertyName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valuePropertyName(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_valuePropertyName<RetType> {
  fn valuePropertyName(self , rsthis: &mut QItemEditorFactory) -> RetType;
}

  // proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
impl<'a> /*trait*/ QItemEditorFactory_valuePropertyName<QByteArray> for (i32) {
  fn valuePropertyName(self , rsthis: &mut QItemEditorFactory) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QItemEditorFactory17valuePropertyNameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK18QItemEditorFactory17valuePropertyNameEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
impl /*struct*/ QItemEditorFactory {
  pub fn defaultFactory_s<RetType, T: QItemEditorFactory_defaultFactory_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultFactory_s();
    // return 1;
  }
}

pub trait QItemEditorFactory_defaultFactory_s<RetType> {
  fn defaultFactory_s(self ) -> RetType;
}

  // proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
impl<'a> /*trait*/ QItemEditorFactory_defaultFactory_s<QItemEditorFactory> for () {
  fn defaultFactory_s(self ) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory14defaultFactoryEv()};
    let mut ret = unsafe {_ZN18QItemEditorFactory14defaultFactoryEv()};
    let mut ret1 = QItemEditorFactory{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemEditorFactory::~QItemEditorFactory();
impl /*struct*/ QItemEditorFactory {
  pub fn FreeQItemEditorFactory<RetType, T: QItemEditorFactory_FreeQItemEditorFactory<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQItemEditorFactory(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_FreeQItemEditorFactory<RetType> {
  fn FreeQItemEditorFactory(self , rsthis: &mut QItemEditorFactory) -> RetType;
}

  // proto:  void QItemEditorFactory::~QItemEditorFactory();
impl<'a> /*trait*/ QItemEditorFactory_FreeQItemEditorFactory<()> for () {
  fn FreeQItemEditorFactory(self , rsthis: &mut QItemEditorFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactoryD0Ev()};
     unsafe {_ZN18QItemEditorFactoryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
impl /*struct*/ QItemEditorFactory {
  pub fn registerEditor<RetType, T: QItemEditorFactory_registerEditor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.registerEditor(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_registerEditor<RetType> {
  fn registerEditor(self , rsthis: &mut QItemEditorFactory) -> RetType;
}

  // proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
impl<'a> /*trait*/ QItemEditorFactory_registerEditor<()> for (i32, QItemEditorCreatorBase) {
  fn registerEditor(self , rsthis: &mut QItemEditorFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
impl /*struct*/ QItemEditorFactory {
  pub fn setDefaultFactory_s<RetType, T: QItemEditorFactory_setDefaultFactory_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultFactory_s();
    // return 1;
  }
}

pub trait QItemEditorFactory_setDefaultFactory_s<RetType> {
  fn setDefaultFactory_s(self ) -> RetType;
}

  // proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QItemEditorFactory_setDefaultFactory_s<()> for (QItemEditorFactory) {
  fn setDefaultFactory_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory17setDefaultFactoryEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QItemEditorFactory17setDefaultFactoryEPS_(arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
impl /*struct*/ QItemEditorFactory {
  pub fn createEditor<RetType, T: QItemEditorFactory_createEditor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.createEditor(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_createEditor<RetType> {
  fn createEditor(self , rsthis: &mut QItemEditorFactory) -> RetType;
}

  // proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
impl<'a> /*trait*/ QItemEditorFactory_createEditor<QWidget> for (i32, QWidget) {
  fn createEditor(self , rsthis: &mut QItemEditorFactory) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QItemEditorFactory12createEditorEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QItemEditorFactory12createEditorEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// <= body block end

