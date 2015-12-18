// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QByteArray QItemEditorCreatorBase::valuePropertyName();
  fn _ZNK22QItemEditorCreatorBase17valuePropertyNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
  fn _ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QItemEditorCreatorBase::FreeQItemEditorCreatorBase();
  fn _ZN22QItemEditorCreatorBaseD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QItemEditorCreatorBase)=8
pub struct QItemEditorCreatorBase {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemEditorCreatorBase {
  pub fn valuePropertyName<RetType, T: QItemEditorCreatorBase_valuePropertyName<RetType>>(&mut self, value: T) -> RetType {
    return value.valuePropertyName(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_valuePropertyName<RetType> {
  fn valuePropertyName(self, rsthis: &mut QItemEditorCreatorBase) -> RetType;
}

// proto:  QByteArray QItemEditorCreatorBase::valuePropertyName();
impl<'a> /*trait*/ QItemEditorCreatorBase_valuePropertyName<QByteArray> for () {
  fn valuePropertyName(self, rsthis: &mut QItemEditorCreatorBase) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QItemEditorCreatorBase17valuePropertyNameEv()};
    let mut ret = unsafe {_ZNK22QItemEditorCreatorBase17valuePropertyNameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemEditorCreatorBase {
  pub fn createWidget<RetType, T: QItemEditorCreatorBase_createWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.createWidget(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_createWidget<RetType> {
  fn createWidget(self, rsthis: &mut QItemEditorCreatorBase) -> RetType;
}

// proto:  QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
impl<'a> /*trait*/ QItemEditorCreatorBase_createWidget<QWidget> for (&'a mut QWidget) {
  fn createWidget(self, rsthis: &mut QItemEditorCreatorBase) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemEditorCreatorBase {
  pub fn FreeQItemEditorCreatorBase<RetType, T: QItemEditorCreatorBase_FreeQItemEditorCreatorBase<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQItemEditorCreatorBase(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_FreeQItemEditorCreatorBase<RetType> {
  fn FreeQItemEditorCreatorBase(self, rsthis: &mut QItemEditorCreatorBase) -> RetType;
}

// proto:  void QItemEditorCreatorBase::FreeQItemEditorCreatorBase();
impl<'a> /*trait*/ QItemEditorCreatorBase_FreeQItemEditorCreatorBase<()> for () {
  fn FreeQItemEditorCreatorBase(self, rsthis: &mut QItemEditorCreatorBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QItemEditorCreatorBaseD0Ev()};
     unsafe {_ZN22QItemEditorCreatorBaseD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

