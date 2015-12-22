// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qsplashscreen.h
// dst-file: /src/widgets/qsplashscreen.rs
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::gui::qpixmap::QPixmap; // 771
use super::super::core::qstring::QString; // 771
use super::super::gui::qcolor::QColor; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  const QMetaObject * QSplashScreen::metaObject();
  fn _ZNK13QSplashScreen10metaObjectEv(qthis: *mut c_void);
  // proto:  void QSplashScreen::~QSplashScreen();
  fn _ZN13QSplashScreenD0Ev(qthis: *mut c_void);
  // proto:  void QSplashScreen::clearMessage();
  fn _ZN13QSplashScreen12clearMessageEv(qthis: *mut c_void);
  // proto:  void QSplashScreen::QSplashScreen(const QSplashScreen & );
  fn _ZN13QSplashScreenC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QPixmap QSplashScreen::pixmap();
  fn _ZNK13QSplashScreen6pixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSplashScreen::messageChanged(const QString & message);
  fn _ZN13QSplashScreen14messageChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSplashScreen::showMessage(const QString & message, int alignment, const QColor & color);
  fn _ZN13QSplashScreen11showMessageERK7QStringiRK6QColor(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  void QSplashScreen::setPixmap(const QPixmap & pixmap);
  fn _ZN13QSplashScreen9setPixmapERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QSplashScreen::message();
  fn _ZNK13QSplashScreen7messageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSplashScreen::repaint();
  fn _ZN13QSplashScreen7repaintEv(qthis: *mut c_void);
  // proto:  void QSplashScreen::finish(QWidget * w);
  fn _ZN13QSplashScreen6finishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSplashScreen)=1
pub struct QSplashScreen {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSplashScreen {
  pub fn inheritFrom(qthis: *mut c_void) -> QSplashScreen {
    return QSplashScreen{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QSplashScreen {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return &self.qbase;
  }
}
impl AsRef<QWidget> for QSplashScreen {
  fn as_ref(&self) -> &QWidget {
    return &self.qbase;
  }
}
  // proto:  const QMetaObject * QSplashScreen::metaObject();
impl /*struct*/ QSplashScreen {
  pub fn metaObject<RetType, T: QSplashScreen_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSplashScreen_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  const QMetaObject * QSplashScreen::metaObject();
impl<'a> /*trait*/ QSplashScreen_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSplashScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSplashScreen10metaObjectEv()};
     unsafe {_ZNK13QSplashScreen10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSplashScreen::~QSplashScreen();
impl /*struct*/ QSplashScreen {
  pub fn FreeQSplashScreen<RetType, T: QSplashScreen_FreeQSplashScreen<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSplashScreen(self);
    // return 1;
  }
}

pub trait QSplashScreen_FreeQSplashScreen<RetType> {
  fn FreeQSplashScreen(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  void QSplashScreen::~QSplashScreen();
impl<'a> /*trait*/ QSplashScreen_FreeQSplashScreen<()> for () {
  fn FreeQSplashScreen(self , rsthis: &mut QSplashScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreenD0Ev()};
     unsafe {_ZN13QSplashScreenD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSplashScreen::clearMessage();
impl /*struct*/ QSplashScreen {
  pub fn clearMessage<RetType, T: QSplashScreen_clearMessage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearMessage(self);
    // return 1;
  }
}

pub trait QSplashScreen_clearMessage<RetType> {
  fn clearMessage(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  void QSplashScreen::clearMessage();
impl<'a> /*trait*/ QSplashScreen_clearMessage<()> for () {
  fn clearMessage(self , rsthis: &mut QSplashScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen12clearMessageEv()};
     unsafe {_ZN13QSplashScreen12clearMessageEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSplashScreen::QSplashScreen(const QSplashScreen & );
impl /*struct*/ QSplashScreen {
  pub fn NewQSplashScreen<T: QSplashScreen_NewQSplashScreen>(value: T) -> QSplashScreen {
    let rsthis = value.NewQSplashScreen();
    return rsthis;
    // return 1;
  }
}

pub trait QSplashScreen_NewQSplashScreen {
  fn NewQSplashScreen(self) -> QSplashScreen;
}

  // proto:  void QSplashScreen::QSplashScreen(const QSplashScreen & );
impl<'a> /*trait*/ QSplashScreen_NewQSplashScreen for (QSplashScreen) {
  fn NewQSplashScreen(self) -> QSplashScreen {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreenC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSplashScreenC1ERKS_(qthis, arg0)};
    let rsthis = QSplashScreen{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QPixmap QSplashScreen::pixmap();
impl /*struct*/ QSplashScreen {
  pub fn pixmap<RetType, T: QSplashScreen_pixmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pixmap(self);
    // return 1;
  }
}

pub trait QSplashScreen_pixmap<RetType> {
  fn pixmap(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  const QPixmap QSplashScreen::pixmap();
impl<'a> /*trait*/ QSplashScreen_pixmap<QPixmap> for () {
  fn pixmap(self , rsthis: &mut QSplashScreen) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSplashScreen6pixmapEv()};
    let mut ret = unsafe {_ZNK13QSplashScreen6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSplashScreen::messageChanged(const QString & message);
impl /*struct*/ QSplashScreen {
  pub fn messageChanged<RetType, T: QSplashScreen_messageChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.messageChanged(self);
    // return 1;
  }
}

pub trait QSplashScreen_messageChanged<RetType> {
  fn messageChanged(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  void QSplashScreen::messageChanged(const QString & message);
impl<'a> /*trait*/ QSplashScreen_messageChanged<()> for (QString) {
  fn messageChanged(self , rsthis: &mut QSplashScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen14messageChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSplashScreen14messageChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSplashScreen::showMessage(const QString & message, int alignment, const QColor & color);
impl /*struct*/ QSplashScreen {
  pub fn showMessage<RetType, T: QSplashScreen_showMessage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showMessage(self);
    // return 1;
  }
}

pub trait QSplashScreen_showMessage<RetType> {
  fn showMessage(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  void QSplashScreen::showMessage(const QString & message, int alignment, const QColor & color);
impl<'a> /*trait*/ QSplashScreen_showMessage<()> for (QString, i32, QColor) {
  fn showMessage(self , rsthis: &mut QSplashScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen11showMessageERK7QStringiRK6QColor()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QSplashScreen11showMessageERK7QStringiRK6QColor(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QSplashScreen::setPixmap(const QPixmap & pixmap);
impl /*struct*/ QSplashScreen {
  pub fn setPixmap<RetType, T: QSplashScreen_setPixmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPixmap(self);
    // return 1;
  }
}

pub trait QSplashScreen_setPixmap<RetType> {
  fn setPixmap(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  void QSplashScreen::setPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QSplashScreen_setPixmap<()> for (QPixmap) {
  fn setPixmap(self , rsthis: &mut QSplashScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSplashScreen9setPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSplashScreen::message();
impl /*struct*/ QSplashScreen {
  pub fn message<RetType, T: QSplashScreen_message<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.message(self);
    // return 1;
  }
}

pub trait QSplashScreen_message<RetType> {
  fn message(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  QString QSplashScreen::message();
impl<'a> /*trait*/ QSplashScreen_message<QString> for () {
  fn message(self , rsthis: &mut QSplashScreen) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSplashScreen7messageEv()};
    let mut ret = unsafe {_ZNK13QSplashScreen7messageEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSplashScreen::repaint();
impl /*struct*/ QSplashScreen {
  pub fn repaint<RetType, T: QSplashScreen_repaint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.repaint(self);
    // return 1;
  }
}

pub trait QSplashScreen_repaint<RetType> {
  fn repaint(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  void QSplashScreen::repaint();
impl<'a> /*trait*/ QSplashScreen_repaint<()> for () {
  fn repaint(self , rsthis: &mut QSplashScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen7repaintEv()};
     unsafe {_ZN13QSplashScreen7repaintEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSplashScreen::finish(QWidget * w);
impl /*struct*/ QSplashScreen {
  pub fn finish<RetType, T: QSplashScreen_finish<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.finish(self);
    // return 1;
  }
}

pub trait QSplashScreen_finish<RetType> {
  fn finish(self , rsthis: &mut QSplashScreen) -> RetType;
}

  // proto:  void QSplashScreen::finish(QWidget * w);
impl<'a> /*trait*/ QSplashScreen_finish<()> for (QWidget) {
  fn finish(self , rsthis: &mut QSplashScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen6finishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSplashScreen6finishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

