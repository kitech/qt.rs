// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QOpenGLWindow::isValid();
  fn _ZNK13QOpenGLWindow7isValidEv() -> i32;
  // proto: QImage QOpenGLWindow::grabFramebuffer();
  fn _ZN13QOpenGLWindow15grabFramebufferEv() -> i32;
  // proto: void QOpenGLWindow::NewQOpenGLWindow(const QOpenGLWindow & );
  fn _ZN13QOpenGLWindowC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QOpenGLWindow::frameSwapped();
  fn _ZN13QOpenGLWindow12frameSwappedEv() -> i32;
  // proto: QOpenGLContext * QOpenGLWindow::shareContext();
  fn _ZNK13QOpenGLWindow12shareContextEv() -> i32;
  // proto: void QOpenGLWindow::makeCurrent();
  fn _ZN13QOpenGLWindow11makeCurrentEv() -> i32;
  // proto: QOpenGLContext * QOpenGLWindow::context();
  fn _ZNK13QOpenGLWindow7contextEv() -> i32;
  // proto: void QOpenGLWindow::doneCurrent();
  fn _ZN13QOpenGLWindow11doneCurrentEv() -> i32;
  // proto: QOpenGLWindow::GLuint QOpenGLWindow::defaultFramebufferObject();
  fn _ZNK13QOpenGLWindow24defaultFramebufferObjectEv() -> i32;
  // proto: void QOpenGLWindow::FreeQOpenGLWindow();
  fn _ZN13QOpenGLWindowD0Ev() -> i32;
  // proto: const QMetaObject * QOpenGLWindow::metaObject();
  fn _ZNK13QOpenGLWindow10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QOpenGLWindow)=1
pub struct QOpenGLWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLWindow {
  pub fn isValid<T: QOpenGLWindow_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QOpenGLWindow_isValid {
  fn isValid(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: bool QOpenGLWindow::isValid();
impl<'a> /*trait*/ QOpenGLWindow_isValid for () {
  fn isValid(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow7isValidEv()};
    unsafe {_ZNK13QOpenGLWindow7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn grabFramebuffer<T: QOpenGLWindow_grabFramebuffer>(&mut self, value: T) -> i32 {
    value.grabFramebuffer(self);
    return 1;
  }
}

pub trait QOpenGLWindow_grabFramebuffer {
  fn grabFramebuffer(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: QImage QOpenGLWindow::grabFramebuffer();
impl<'a> /*trait*/ QOpenGLWindow_grabFramebuffer for () {
  fn grabFramebuffer(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow15grabFramebufferEv()};
    unsafe {_ZN13QOpenGLWindow15grabFramebufferEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn NewQOpenGLWindow<T: QOpenGLWindow_NewQOpenGLWindow>(value: T) -> QOpenGLWindow {
    let rsthis = value.NewQOpenGLWindow();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLWindow_NewQOpenGLWindow {
  fn NewQOpenGLWindow(self) -> QOpenGLWindow;
}

// proto: void QOpenGLWindow::NewQOpenGLWindow(const QOpenGLWindow & );
impl<'a> /*trait*/ QOpenGLWindow_NewQOpenGLWindow for (&'a  QOpenGLWindow) {
  fn NewQOpenGLWindow(self) -> QOpenGLWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QOpenGLWindowC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn frameSwapped<T: QOpenGLWindow_frameSwapped>(&mut self, value: T) -> i32 {
    value.frameSwapped(self);
    return 1;
  }
}

pub trait QOpenGLWindow_frameSwapped {
  fn frameSwapped(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: void QOpenGLWindow::frameSwapped();
impl<'a> /*trait*/ QOpenGLWindow_frameSwapped for () {
  fn frameSwapped(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow12frameSwappedEv()};
    unsafe {_ZN13QOpenGLWindow12frameSwappedEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn shareContext<T: QOpenGLWindow_shareContext>(&mut self, value: T) -> i32 {
    value.shareContext(self);
    return 1;
  }
}

pub trait QOpenGLWindow_shareContext {
  fn shareContext(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: QOpenGLContext * QOpenGLWindow::shareContext();
impl<'a> /*trait*/ QOpenGLWindow_shareContext for () {
  fn shareContext(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow12shareContextEv()};
    unsafe {_ZNK13QOpenGLWindow12shareContextEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn makeCurrent<T: QOpenGLWindow_makeCurrent>(&mut self, value: T) -> i32 {
    value.makeCurrent(self);
    return 1;
  }
}

pub trait QOpenGLWindow_makeCurrent {
  fn makeCurrent(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: void QOpenGLWindow::makeCurrent();
impl<'a> /*trait*/ QOpenGLWindow_makeCurrent for () {
  fn makeCurrent(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow11makeCurrentEv()};
    unsafe {_ZN13QOpenGLWindow11makeCurrentEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn context<T: QOpenGLWindow_context>(&mut self, value: T) -> i32 {
    value.context(self);
    return 1;
  }
}

pub trait QOpenGLWindow_context {
  fn context(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: QOpenGLContext * QOpenGLWindow::context();
impl<'a> /*trait*/ QOpenGLWindow_context for () {
  fn context(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow7contextEv()};
    unsafe {_ZNK13QOpenGLWindow7contextEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn doneCurrent<T: QOpenGLWindow_doneCurrent>(&mut self, value: T) -> i32 {
    value.doneCurrent(self);
    return 1;
  }
}

pub trait QOpenGLWindow_doneCurrent {
  fn doneCurrent(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: void QOpenGLWindow::doneCurrent();
impl<'a> /*trait*/ QOpenGLWindow_doneCurrent for () {
  fn doneCurrent(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow11doneCurrentEv()};
    unsafe {_ZN13QOpenGLWindow11doneCurrentEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn defaultFramebufferObject<T: QOpenGLWindow_defaultFramebufferObject>(&mut self, value: T) -> i32 {
    value.defaultFramebufferObject(self);
    return 1;
  }
}

pub trait QOpenGLWindow_defaultFramebufferObject {
  fn defaultFramebufferObject(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: QOpenGLWindow::GLuint QOpenGLWindow::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLWindow_defaultFramebufferObject for () {
  fn defaultFramebufferObject(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow24defaultFramebufferObjectEv()};
    unsafe {_ZNK13QOpenGLWindow24defaultFramebufferObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn FreeQOpenGLWindow<T: QOpenGLWindow_FreeQOpenGLWindow>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLWindow(self);
    return 1;
  }
}

pub trait QOpenGLWindow_FreeQOpenGLWindow {
  fn FreeQOpenGLWindow(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: void QOpenGLWindow::FreeQOpenGLWindow();
impl<'a> /*trait*/ QOpenGLWindow_FreeQOpenGLWindow for () {
  fn FreeQOpenGLWindow(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindowD0Ev()};
    unsafe {_ZN13QOpenGLWindowD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWindow {
  pub fn metaObject<T: QOpenGLWindow_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QOpenGLWindow_metaObject {
  fn metaObject(self, this: &mut QOpenGLWindow) -> i32;
}

// proto: const QMetaObject * QOpenGLWindow::metaObject();
impl<'a> /*trait*/ QOpenGLWindow_metaObject for () {
  fn metaObject(self, this: &mut QOpenGLWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow10metaObjectEv()};
    unsafe {_ZNK13QOpenGLWindow10metaObjectEv()};
    return 1;
  }
}

