// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qcolor::QColor;
use super::qpixmap::QPixmap;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QMetaObject * QSplashScreen::metaObject();
  fn _ZNK13QSplashScreen10metaObjectEv() -> i32;
  // proto: void QSplashScreen::FreeQSplashScreen();
  fn _ZN13QSplashScreenD0Ev() -> i32;
  // proto: void QSplashScreen::clearMessage();
  fn _ZN13QSplashScreen12clearMessageEv() -> i32;
  // proto: void QSplashScreen::NewQSplashScreen(const QSplashScreen & );
  fn _ZN13QSplashScreenC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QPixmap QSplashScreen::pixmap();
  fn _ZNK13QSplashScreen6pixmapEv() -> i32;
  // proto: void QSplashScreen::messageChanged(const QString & message);
  fn _ZN13QSplashScreen14messageChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QSplashScreen::showMessage(const QString & message, int alignment, const QColor & color);
  fn _ZN13QSplashScreen11showMessageERK7QStringiRK6QColor(arg0: *const c_void, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QSplashScreen::setPixmap(const QPixmap & pixmap);
  fn _ZN13QSplashScreen9setPixmapERK7QPixmap(arg0: *const c_void) -> i32;
  // proto: QString QSplashScreen::message();
  fn _ZNK13QSplashScreen7messageEv() -> i32;
  // proto: void QSplashScreen::repaint();
  fn _ZN13QSplashScreen7repaintEv() -> i32;
  // proto: void QSplashScreen::finish(QWidget * w);
  fn _ZN13QSplashScreen6finishEP7QWidget(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QSplashScreen)=1
pub struct QSplashScreen {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSplashScreen {
  pub fn metaObject<T: QSplashScreen_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSplashScreen_metaObject {
  fn metaObject(self, this: &mut QSplashScreen) -> i32;
}

// proto: const QMetaObject * QSplashScreen::metaObject();
impl<'a> /*trait*/ QSplashScreen_metaObject for () {
  fn metaObject(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSplashScreen10metaObjectEv()};
    unsafe {_ZNK13QSplashScreen10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn FreeQSplashScreen<T: QSplashScreen_FreeQSplashScreen>(&mut self, value: T) -> i32 {
    value.FreeQSplashScreen(self);
    return 1;
  }
}

pub trait QSplashScreen_FreeQSplashScreen {
  fn FreeQSplashScreen(self, this: &mut QSplashScreen) -> i32;
}

// proto: void QSplashScreen::FreeQSplashScreen();
impl<'a> /*trait*/ QSplashScreen_FreeQSplashScreen for () {
  fn FreeQSplashScreen(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreenD0Ev()};
    unsafe {_ZN13QSplashScreenD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn clearMessage<T: QSplashScreen_clearMessage>(&mut self, value: T) -> i32 {
    value.clearMessage(self);
    return 1;
  }
}

pub trait QSplashScreen_clearMessage {
  fn clearMessage(self, this: &mut QSplashScreen) -> i32;
}

// proto: void QSplashScreen::clearMessage();
impl<'a> /*trait*/ QSplashScreen_clearMessage for () {
  fn clearMessage(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen12clearMessageEv()};
    unsafe {_ZN13QSplashScreen12clearMessageEv()};
    return 1;
  }
}

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

// proto: void QSplashScreen::NewQSplashScreen(const QSplashScreen & );
impl<'a> /*trait*/ QSplashScreen_NewQSplashScreen for (&'a  QSplashScreen) {
  fn NewQSplashScreen(self) -> QSplashScreen {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreenC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QSplashScreenC1ERKS_(qthis, arg0)};
    let rsthis = QSplashScreen{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn pixmap<T: QSplashScreen_pixmap>(&mut self, value: T) -> i32 {
    value.pixmap(self);
    return 1;
  }
}

pub trait QSplashScreen_pixmap {
  fn pixmap(self, this: &mut QSplashScreen) -> i32;
}

// proto: const QPixmap QSplashScreen::pixmap();
impl<'a> /*trait*/ QSplashScreen_pixmap for () {
  fn pixmap(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSplashScreen6pixmapEv()};
    unsafe {_ZNK13QSplashScreen6pixmapEv()};
    return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn messageChanged<T: QSplashScreen_messageChanged>(&mut self, value: T) -> i32 {
    value.messageChanged(self);
    return 1;
  }
}

pub trait QSplashScreen_messageChanged {
  fn messageChanged(self, this: &mut QSplashScreen) -> i32;
}

// proto: void QSplashScreen::messageChanged(const QString & message);
impl<'a> /*trait*/ QSplashScreen_messageChanged for (&'a  QString) {
  fn messageChanged(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen14messageChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QSplashScreen14messageChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn showMessage<T: QSplashScreen_showMessage>(&mut self, value: T) -> i32 {
    value.showMessage(self);
    return 1;
  }
}

pub trait QSplashScreen_showMessage {
  fn showMessage(self, this: &mut QSplashScreen) -> i32;
}

// proto: void QSplashScreen::showMessage(const QString & message, int alignment, const QColor & color);
impl<'a> /*trait*/ QSplashScreen_showMessage for (&'a  QString, i32, &'a  QColor) {
  fn showMessage(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen11showMessageERK7QStringiRK6QColor()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN13QSplashScreen11showMessageERK7QStringiRK6QColor(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn setPixmap<T: QSplashScreen_setPixmap>(&mut self, value: T) -> i32 {
    value.setPixmap(self);
    return 1;
  }
}

pub trait QSplashScreen_setPixmap {
  fn setPixmap(self, this: &mut QSplashScreen) -> i32;
}

// proto: void QSplashScreen::setPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QSplashScreen_setPixmap for (&'a  QPixmap) {
  fn setPixmap(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QSplashScreen9setPixmapERK7QPixmap(arg0)};
    return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn message<T: QSplashScreen_message>(&mut self, value: T) -> i32 {
    value.message(self);
    return 1;
  }
}

pub trait QSplashScreen_message {
  fn message(self, this: &mut QSplashScreen) -> i32;
}

// proto: QString QSplashScreen::message();
impl<'a> /*trait*/ QSplashScreen_message for () {
  fn message(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSplashScreen7messageEv()};
    unsafe {_ZNK13QSplashScreen7messageEv()};
    return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn repaint<T: QSplashScreen_repaint>(&mut self, value: T) -> i32 {
    value.repaint(self);
    return 1;
  }
}

pub trait QSplashScreen_repaint {
  fn repaint(self, this: &mut QSplashScreen) -> i32;
}

// proto: void QSplashScreen::repaint();
impl<'a> /*trait*/ QSplashScreen_repaint for () {
  fn repaint(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen7repaintEv()};
    unsafe {_ZN13QSplashScreen7repaintEv()};
    return 1;
  }
}

impl /*struct*/ QSplashScreen {
  pub fn finish<T: QSplashScreen_finish>(&mut self, value: T) -> i32 {
    value.finish(self);
    return 1;
  }
}

pub trait QSplashScreen_finish {
  fn finish(self, this: &mut QSplashScreen) -> i32;
}

// proto: void QSplashScreen::finish(QWidget * w);
impl<'a> /*trait*/ QSplashScreen_finish for (&'a mut QWidget) {
  fn finish(self, this: &mut QSplashScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSplashScreen6finishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSplashScreen6finishEP7QWidget(arg0)};
    return 1;
  }
}

