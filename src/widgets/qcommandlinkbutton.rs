// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, const QString & description, QWidget * parent);
  fn _ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  const QMetaObject * QCommandLinkButton::metaObject();
  fn _ZNK18QCommandLinkButton10metaObjectEv(qthis: *mut c_void);
  // proto:  void QCommandLinkButton::~QCommandLinkButton();
  fn _ZN18QCommandLinkButtonD0Ev(qthis: *mut c_void);
  // proto:  void QCommandLinkButton::QCommandLinkButton(const QCommandLinkButton & );
  fn _ZN18QCommandLinkButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommandLinkButton::QCommandLinkButton(QWidget * parent);
  fn _ZN18QCommandLinkButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QCommandLinkButton::description();
  fn _ZNK18QCommandLinkButton11descriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, QWidget * parent);
  fn _ZN18QCommandLinkButtonC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QCommandLinkButton::setDescription(const QString & description);
  fn _ZN18QCommandLinkButton14setDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QCommandLinkButton)=1
pub struct QCommandLinkButton {
  pub qclsinst: *mut c_void,
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, const QString & description, QWidget * parent);
impl /*struct*/ QCommandLinkButton {
  pub fn NewQCommandLinkButton<T: QCommandLinkButton_NewQCommandLinkButton>(value: T) -> QCommandLinkButton {
    let rsthis = value.NewQCommandLinkButton();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLinkButton_NewQCommandLinkButton {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton;
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, const QString & description, QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_NewQCommandLinkButton for (QString, QString, QWidget) {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget(qthis, arg0, arg1, arg2)};
    let rsthis = QCommandLinkButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QCommandLinkButton::metaObject();
impl /*struct*/ QCommandLinkButton {
  pub fn metaObject<RetType, T: QCommandLinkButton_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCommandLinkButton_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QCommandLinkButton) -> RetType;
}

  // proto:  const QMetaObject * QCommandLinkButton::metaObject();
impl<'a> /*trait*/ QCommandLinkButton_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QCommandLinkButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLinkButton10metaObjectEv()};
     unsafe {_ZNK18QCommandLinkButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::~QCommandLinkButton();
impl /*struct*/ QCommandLinkButton {
  pub fn FreeQCommandLinkButton<RetType, T: QCommandLinkButton_FreeQCommandLinkButton<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQCommandLinkButton(self);
    // return 1;
  }
}

pub trait QCommandLinkButton_FreeQCommandLinkButton<RetType> {
  fn FreeQCommandLinkButton(self , rsthis: &mut QCommandLinkButton) -> RetType;
}

  // proto:  void QCommandLinkButton::~QCommandLinkButton();
impl<'a> /*trait*/ QCommandLinkButton_FreeQCommandLinkButton<()> for () {
  fn FreeQCommandLinkButton(self , rsthis: &mut QCommandLinkButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonD0Ev()};
     unsafe {_ZN18QCommandLinkButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(const QCommandLinkButton & );
impl<'a> /*trait*/ QCommandLinkButton_NewQCommandLinkButton for (QCommandLinkButton) {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLinkButtonC1ERKS_(qthis, arg0)};
    let rsthis = QCommandLinkButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_NewQCommandLinkButton for (QWidget) {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLinkButtonC1EP7QWidget(qthis, arg0)};
    let rsthis = QCommandLinkButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QCommandLinkButton::description();
impl /*struct*/ QCommandLinkButton {
  pub fn description<RetType, T: QCommandLinkButton_description<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.description(self);
    // return 1;
  }
}

pub trait QCommandLinkButton_description<RetType> {
  fn description(self , rsthis: &mut QCommandLinkButton) -> RetType;
}

  // proto:  QString QCommandLinkButton::description();
impl<'a> /*trait*/ QCommandLinkButton_description<QString> for () {
  fn description(self , rsthis: &mut QCommandLinkButton) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLinkButton11descriptionEv()};
    let mut ret = unsafe {_ZNK18QCommandLinkButton11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_NewQCommandLinkButton for (QString, QWidget) {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLinkButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QCommandLinkButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::setDescription(const QString & description);
impl /*struct*/ QCommandLinkButton {
  pub fn setDescription<RetType, T: QCommandLinkButton_setDescription<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDescription(self);
    // return 1;
  }
}

pub trait QCommandLinkButton_setDescription<RetType> {
  fn setDescription(self , rsthis: &mut QCommandLinkButton) -> RetType;
}

  // proto:  void QCommandLinkButton::setDescription(const QString & description);
impl<'a> /*trait*/ QCommandLinkButton_setDescription<()> for (QString) {
  fn setDescription(self , rsthis: &mut QCommandLinkButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButton14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLinkButton14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

