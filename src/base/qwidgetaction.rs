// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QMetaObject * QWidgetAction::metaObject();
  fn _ZNK13QWidgetAction10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QWidgetAction::FreeQWidgetAction();
  fn _ZN13QWidgetActionD0Ev(qthis: *mut c_void) ;
  // proto:  void QWidgetAction::setDefaultWidget(QWidget * w);
  fn _ZN13QWidgetAction16setDefaultWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidgetAction::releaseWidget(QWidget * widget);
  fn _ZN13QWidgetAction13releaseWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidgetAction::NewQWidgetAction(const QWidgetAction & );
  fn _ZN13QWidgetActionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidgetAction::NewQWidgetAction(QObject * parent);
  fn _ZN13QWidgetActionC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QWidgetAction::requestWidget(QWidget * parent);
  fn _ZN13QWidgetAction13requestWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QWidgetAction::defaultWidget();
  fn _ZNK13QWidgetAction13defaultWidgetEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QWidgetAction)=1
pub struct QWidgetAction {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWidgetAction {
  pub fn metaObject<T: QWidgetAction_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QWidgetAction_metaObject {
  fn metaObject(self, rsthis: &mut QWidgetAction) ;
}

// proto:  const QMetaObject * QWidgetAction::metaObject();
impl<'a> /*trait*/ QWidgetAction_metaObject for () {
  fn metaObject(self, rsthis: &mut QWidgetAction)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetAction10metaObjectEv()};
     unsafe {_ZNK13QWidgetAction10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidgetAction {
  pub fn FreeQWidgetAction<T: QWidgetAction_FreeQWidgetAction>(&mut self, value: T)  {
     value.FreeQWidgetAction(self);
    // return 1;
  }
}

pub trait QWidgetAction_FreeQWidgetAction {
  fn FreeQWidgetAction(self, rsthis: &mut QWidgetAction) ;
}

// proto:  void QWidgetAction::FreeQWidgetAction();
impl<'a> /*trait*/ QWidgetAction_FreeQWidgetAction for () {
  fn FreeQWidgetAction(self, rsthis: &mut QWidgetAction)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetActionD0Ev()};
     unsafe {_ZN13QWidgetActionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidgetAction {
  pub fn setDefaultWidget<T: QWidgetAction_setDefaultWidget>(&mut self, value: T)  {
     value.setDefaultWidget(self);
    // return 1;
  }
}

pub trait QWidgetAction_setDefaultWidget {
  fn setDefaultWidget(self, rsthis: &mut QWidgetAction) ;
}

// proto:  void QWidgetAction::setDefaultWidget(QWidget * w);
impl<'a> /*trait*/ QWidgetAction_setDefaultWidget for (&'a mut QWidget) {
  fn setDefaultWidget(self, rsthis: &mut QWidgetAction)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetAction16setDefaultWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QWidgetAction16setDefaultWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidgetAction {
  pub fn releaseWidget<T: QWidgetAction_releaseWidget>(&mut self, value: T)  {
     value.releaseWidget(self);
    // return 1;
  }
}

pub trait QWidgetAction_releaseWidget {
  fn releaseWidget(self, rsthis: &mut QWidgetAction) ;
}

// proto:  void QWidgetAction::releaseWidget(QWidget * widget);
impl<'a> /*trait*/ QWidgetAction_releaseWidget for (&'a mut QWidget) {
  fn releaseWidget(self, rsthis: &mut QWidgetAction)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetAction13releaseWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QWidgetAction13releaseWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidgetAction {
  pub fn NewQWidgetAction<T: QWidgetAction_NewQWidgetAction>(value: T) -> QWidgetAction {
    let rsthis = value.NewQWidgetAction();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetAction_NewQWidgetAction {
  fn NewQWidgetAction(self) -> QWidgetAction;
}

// proto: void QWidgetAction::NewQWidgetAction(const QWidgetAction & );
impl<'a> /*trait*/ QWidgetAction_NewQWidgetAction for (&'a  QWidgetAction) {
  fn NewQWidgetAction(self) -> QWidgetAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetActionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetActionC1ERKS_(qthis, arg0)};
    let rsthis = QWidgetAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QWidgetAction::NewQWidgetAction(QObject * parent);
impl<'a> /*trait*/ QWidgetAction_NewQWidgetAction for (&'a mut QObject) {
  fn NewQWidgetAction(self) -> QWidgetAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetActionC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetActionC1EP7QObject(qthis, arg0)};
    let rsthis = QWidgetAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWidgetAction {
  pub fn requestWidget<T: QWidgetAction_requestWidget>(&mut self, value: T) -> QWidget {
    return value.requestWidget(self);
    // return 1;
  }
}

pub trait QWidgetAction_requestWidget {
  fn requestWidget(self, rsthis: &mut QWidgetAction) -> QWidget;
}

// proto:  QWidget * QWidgetAction::requestWidget(QWidget * parent);
impl<'a> /*trait*/ QWidgetAction_requestWidget for (&'a mut QWidget) {
  fn requestWidget(self, rsthis: &mut QWidgetAction) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetAction13requestWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QWidgetAction13requestWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetAction {
  pub fn defaultWidget<T: QWidgetAction_defaultWidget>(&mut self, value: T) -> QWidget {
    return value.defaultWidget(self);
    // return 1;
  }
}

pub trait QWidgetAction_defaultWidget {
  fn defaultWidget(self, rsthis: &mut QWidgetAction) -> QWidget;
}

// proto:  QWidget * QWidgetAction::defaultWidget();
impl<'a> /*trait*/ QWidgetAction_defaultWidget for () {
  fn defaultWidget(self, rsthis: &mut QWidgetAction) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetAction13defaultWidgetEv()};
    let mut ret = unsafe {_ZNK13QWidgetAction13defaultWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

