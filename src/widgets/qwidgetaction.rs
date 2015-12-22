// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qwidgetaction.h
// dst-file: /src/widgets/qwidgetaction.rs
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
use super::qaction::QAction; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  const QMetaObject * QWidgetAction::metaObject();
  fn _ZNK13QWidgetAction10metaObjectEv(qthis: *mut c_void);
  // proto:  void QWidgetAction::~QWidgetAction();
  fn _ZN13QWidgetActionD0Ev(qthis: *mut c_void);
  // proto:  void QWidgetAction::setDefaultWidget(QWidget * w);
  fn _ZN13QWidgetAction16setDefaultWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWidgetAction::releaseWidget(QWidget * widget);
  fn _ZN13QWidgetAction13releaseWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWidgetAction::QWidgetAction(const QWidgetAction & );
  fn _ZN13QWidgetActionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWidgetAction::QWidgetAction(QObject * parent);
  fn _ZN13QWidgetActionC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QWidgetAction::requestWidget(QWidget * parent);
  fn _ZN13QWidgetAction13requestWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QWidgetAction::defaultWidget();
  fn _ZNK13QWidgetAction13defaultWidgetEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QWidgetAction)=1
pub struct QWidgetAction {
  qbase: QAction,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWidgetAction {
  pub fn inheritFrom(qthis: *mut c_void) -> QWidgetAction {
    return QWidgetAction{qbase: QAction::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QWidgetAction {
  type Target = QAction;

  fn deref(&self) -> &QAction {
    return &self.qbase;
  }
}
impl AsRef<QAction> for QWidgetAction {
  fn as_ref(&self) -> &QAction {
    return &self.qbase;
  }
}
  // proto:  const QMetaObject * QWidgetAction::metaObject();
impl /*struct*/ QWidgetAction {
  pub fn metaObject<RetType, T: QWidgetAction_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QWidgetAction_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QWidgetAction) -> RetType;
}

  // proto:  const QMetaObject * QWidgetAction::metaObject();
impl<'a> /*trait*/ QWidgetAction_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QWidgetAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetAction10metaObjectEv()};
     unsafe {_ZNK13QWidgetAction10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidgetAction::~QWidgetAction();
impl /*struct*/ QWidgetAction {
  pub fn FreeQWidgetAction<RetType, T: QWidgetAction_FreeQWidgetAction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWidgetAction(self);
    // return 1;
  }
}

pub trait QWidgetAction_FreeQWidgetAction<RetType> {
  fn FreeQWidgetAction(self , rsthis: &mut QWidgetAction) -> RetType;
}

  // proto:  void QWidgetAction::~QWidgetAction();
impl<'a> /*trait*/ QWidgetAction_FreeQWidgetAction<()> for () {
  fn FreeQWidgetAction(self , rsthis: &mut QWidgetAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetActionD0Ev()};
     unsafe {_ZN13QWidgetActionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidgetAction::setDefaultWidget(QWidget * w);
impl /*struct*/ QWidgetAction {
  pub fn setDefaultWidget<RetType, T: QWidgetAction_setDefaultWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDefaultWidget(self);
    // return 1;
  }
}

pub trait QWidgetAction_setDefaultWidget<RetType> {
  fn setDefaultWidget(self , rsthis: &mut QWidgetAction) -> RetType;
}

  // proto:  void QWidgetAction::setDefaultWidget(QWidget * w);
impl<'a> /*trait*/ QWidgetAction_setDefaultWidget<()> for (QWidget) {
  fn setDefaultWidget(self , rsthis: &mut QWidgetAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetAction16setDefaultWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QWidgetAction16setDefaultWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidgetAction::releaseWidget(QWidget * widget);
impl /*struct*/ QWidgetAction {
  pub fn releaseWidget<RetType, T: QWidgetAction_releaseWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.releaseWidget(self);
    // return 1;
  }
}

pub trait QWidgetAction_releaseWidget<RetType> {
  fn releaseWidget(self , rsthis: &mut QWidgetAction) -> RetType;
}

  // proto:  void QWidgetAction::releaseWidget(QWidget * widget);
impl<'a> /*trait*/ QWidgetAction_releaseWidget<()> for (QWidget) {
  fn releaseWidget(self , rsthis: &mut QWidgetAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetAction13releaseWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QWidgetAction13releaseWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidgetAction::QWidgetAction(const QWidgetAction & );
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

  // proto:  void QWidgetAction::QWidgetAction(const QWidgetAction & );
impl<'a> /*trait*/ QWidgetAction_NewQWidgetAction for (QWidgetAction) {
  fn NewQWidgetAction(self) -> QWidgetAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetActionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetActionC1ERKS_(qthis, arg0)};
    let rsthis = QWidgetAction{/**/qbase: QAction::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWidgetAction::QWidgetAction(QObject * parent);
impl<'a> /*trait*/ QWidgetAction_NewQWidgetAction for (QObject) {
  fn NewQWidgetAction(self) -> QWidgetAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetActionC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetActionC1EP7QObject(qthis, arg0)};
    let rsthis = QWidgetAction{/**/qbase: QAction::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWidget * QWidgetAction::requestWidget(QWidget * parent);
impl /*struct*/ QWidgetAction {
  pub fn requestWidget<RetType, T: QWidgetAction_requestWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.requestWidget(self);
    // return 1;
  }
}

pub trait QWidgetAction_requestWidget<RetType> {
  fn requestWidget(self , rsthis: &mut QWidgetAction) -> RetType;
}

  // proto:  QWidget * QWidgetAction::requestWidget(QWidget * parent);
impl<'a> /*trait*/ QWidgetAction_requestWidget<QWidget> for (QWidget) {
  fn requestWidget(self , rsthis: &mut QWidgetAction) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetAction13requestWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QWidgetAction13requestWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QWidgetAction::defaultWidget();
impl /*struct*/ QWidgetAction {
  pub fn defaultWidget<RetType, T: QWidgetAction_defaultWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.defaultWidget(self);
    // return 1;
  }
}

pub trait QWidgetAction_defaultWidget<RetType> {
  fn defaultWidget(self , rsthis: &mut QWidgetAction) -> RetType;
}

  // proto:  QWidget * QWidgetAction::defaultWidget();
impl<'a> /*trait*/ QWidgetAction_defaultWidget<QWidget> for () {
  fn defaultWidget(self , rsthis: &mut QWidgetAction) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetAction13defaultWidgetEv()};
    let mut ret = unsafe {_ZNK13QWidgetAction13defaultWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

