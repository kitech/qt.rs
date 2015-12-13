// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsurfaceformat::QSurfaceFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QOpenGLWidget::FreeQOpenGLWidget();
  fn _ZN13QOpenGLWidgetD0Ev() -> i32;
  // proto: QOpenGLWidget::GLuint QOpenGLWidget::defaultFramebufferObject();
  fn _ZNK13QOpenGLWidget24defaultFramebufferObjectEv() -> i32;
  // proto: void QOpenGLWidget::resized();
  fn _ZN13QOpenGLWidget7resizedEv() -> i32;
  // proto: void QOpenGLWidget::NewQOpenGLWidget(const QOpenGLWidget & );
  fn _ZN13QOpenGLWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QOpenGLWidget::isValid();
  fn _ZNK13QOpenGLWidget7isValidEv() -> i32;
  // proto: void QOpenGLWidget::aboutToResize();
  fn _ZN13QOpenGLWidget13aboutToResizeEv() -> i32;
  // proto: QOpenGLContext * QOpenGLWidget::context();
  fn _ZNK13QOpenGLWidget7contextEv() -> i32;
  // proto: void QOpenGLWidget::doneCurrent();
  fn _ZN13QOpenGLWidget11doneCurrentEv() -> i32;
  // proto: void QOpenGLWidget::makeCurrent();
  fn _ZN13QOpenGLWidget11makeCurrentEv() -> i32;
  // proto: QImage QOpenGLWidget::grabFramebuffer();
  fn _ZN13QOpenGLWidget15grabFramebufferEv() -> i32;
  // proto: const QMetaObject * QOpenGLWidget::metaObject();
  fn _ZNK13QOpenGLWidget10metaObjectEv() -> i32;
  // proto: void QOpenGLWidget::frameSwapped();
  fn _ZN13QOpenGLWidget12frameSwappedEv() -> i32;
  // proto: void QOpenGLWidget::aboutToCompose();
  fn _ZN13QOpenGLWidget14aboutToComposeEv() -> i32;
  // proto: void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
  fn _ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(arg0: *const c_void) -> i32;
  // proto: QSurfaceFormat QOpenGLWidget::format();
  fn _ZNK13QOpenGLWidget6formatEv() -> i32;
}

// body block begin
// class sizeof(QOpenGLWidget)=1
pub struct QOpenGLWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLWidget {
  pub fn FreeQOpenGLWidget<T: QOpenGLWidget_FreeQOpenGLWidget>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLWidget(self);
    return 1;
  }
}

pub trait QOpenGLWidget_FreeQOpenGLWidget {
  fn FreeQOpenGLWidget(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: void QOpenGLWidget::FreeQOpenGLWidget();
impl<'a> /*trait*/ QOpenGLWidget_FreeQOpenGLWidget for () {
  fn FreeQOpenGLWidget(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidgetD0Ev()};
    unsafe {_ZN13QOpenGLWidgetD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn defaultFramebufferObject<T: QOpenGLWidget_defaultFramebufferObject>(&mut self, value: T) -> i32 {
    value.defaultFramebufferObject(self);
    return 1;
  }
}

pub trait QOpenGLWidget_defaultFramebufferObject {
  fn defaultFramebufferObject(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: QOpenGLWidget::GLuint QOpenGLWidget::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLWidget_defaultFramebufferObject for () {
  fn defaultFramebufferObject(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget24defaultFramebufferObjectEv()};
    unsafe {_ZNK13QOpenGLWidget24defaultFramebufferObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn resized<T: QOpenGLWidget_resized>(&mut self, value: T) -> i32 {
    value.resized(self);
    return 1;
  }
}

pub trait QOpenGLWidget_resized {
  fn resized(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: void QOpenGLWidget::resized();
impl<'a> /*trait*/ QOpenGLWidget_resized for () {
  fn resized(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget7resizedEv()};
    unsafe {_ZN13QOpenGLWidget7resizedEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn NewQOpenGLWidget<T: QOpenGLWidget_NewQOpenGLWidget>(value: T) -> QOpenGLWidget {
    let rsthis = value.NewQOpenGLWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLWidget_NewQOpenGLWidget {
  fn NewQOpenGLWidget(self) -> QOpenGLWidget;
}

// proto: void QOpenGLWidget::NewQOpenGLWidget(const QOpenGLWidget & );
impl<'a> /*trait*/ QOpenGLWidget_NewQOpenGLWidget for (&'a  QOpenGLWidget) {
  fn NewQOpenGLWidget(self) -> QOpenGLWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QOpenGLWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn isValid<T: QOpenGLWidget_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QOpenGLWidget_isValid {
  fn isValid(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: bool QOpenGLWidget::isValid();
impl<'a> /*trait*/ QOpenGLWidget_isValid for () {
  fn isValid(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7isValidEv()};
    unsafe {_ZNK13QOpenGLWidget7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn aboutToResize<T: QOpenGLWidget_aboutToResize>(&mut self, value: T) -> i32 {
    value.aboutToResize(self);
    return 1;
  }
}

pub trait QOpenGLWidget_aboutToResize {
  fn aboutToResize(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: void QOpenGLWidget::aboutToResize();
impl<'a> /*trait*/ QOpenGLWidget_aboutToResize for () {
  fn aboutToResize(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget13aboutToResizeEv()};
    unsafe {_ZN13QOpenGLWidget13aboutToResizeEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn context<T: QOpenGLWidget_context>(&mut self, value: T) -> i32 {
    value.context(self);
    return 1;
  }
}

pub trait QOpenGLWidget_context {
  fn context(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: QOpenGLContext * QOpenGLWidget::context();
impl<'a> /*trait*/ QOpenGLWidget_context for () {
  fn context(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7contextEv()};
    unsafe {_ZNK13QOpenGLWidget7contextEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn doneCurrent<T: QOpenGLWidget_doneCurrent>(&mut self, value: T) -> i32 {
    value.doneCurrent(self);
    return 1;
  }
}

pub trait QOpenGLWidget_doneCurrent {
  fn doneCurrent(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: void QOpenGLWidget::doneCurrent();
impl<'a> /*trait*/ QOpenGLWidget_doneCurrent for () {
  fn doneCurrent(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11doneCurrentEv()};
    unsafe {_ZN13QOpenGLWidget11doneCurrentEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn makeCurrent<T: QOpenGLWidget_makeCurrent>(&mut self, value: T) -> i32 {
    value.makeCurrent(self);
    return 1;
  }
}

pub trait QOpenGLWidget_makeCurrent {
  fn makeCurrent(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: void QOpenGLWidget::makeCurrent();
impl<'a> /*trait*/ QOpenGLWidget_makeCurrent for () {
  fn makeCurrent(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11makeCurrentEv()};
    unsafe {_ZN13QOpenGLWidget11makeCurrentEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn grabFramebuffer<T: QOpenGLWidget_grabFramebuffer>(&mut self, value: T) -> i32 {
    value.grabFramebuffer(self);
    return 1;
  }
}

pub trait QOpenGLWidget_grabFramebuffer {
  fn grabFramebuffer(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: QImage QOpenGLWidget::grabFramebuffer();
impl<'a> /*trait*/ QOpenGLWidget_grabFramebuffer for () {
  fn grabFramebuffer(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget15grabFramebufferEv()};
    unsafe {_ZN13QOpenGLWidget15grabFramebufferEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn metaObject<T: QOpenGLWidget_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QOpenGLWidget_metaObject {
  fn metaObject(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: const QMetaObject * QOpenGLWidget::metaObject();
impl<'a> /*trait*/ QOpenGLWidget_metaObject for () {
  fn metaObject(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget10metaObjectEv()};
    unsafe {_ZNK13QOpenGLWidget10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn frameSwapped<T: QOpenGLWidget_frameSwapped>(&mut self, value: T) -> i32 {
    value.frameSwapped(self);
    return 1;
  }
}

pub trait QOpenGLWidget_frameSwapped {
  fn frameSwapped(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: void QOpenGLWidget::frameSwapped();
impl<'a> /*trait*/ QOpenGLWidget_frameSwapped for () {
  fn frameSwapped(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget12frameSwappedEv()};
    unsafe {_ZN13QOpenGLWidget12frameSwappedEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn aboutToCompose<T: QOpenGLWidget_aboutToCompose>(&mut self, value: T) -> i32 {
    value.aboutToCompose(self);
    return 1;
  }
}

pub trait QOpenGLWidget_aboutToCompose {
  fn aboutToCompose(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: void QOpenGLWidget::aboutToCompose();
impl<'a> /*trait*/ QOpenGLWidget_aboutToCompose for () {
  fn aboutToCompose(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget14aboutToComposeEv()};
    unsafe {_ZN13QOpenGLWidget14aboutToComposeEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn setFormat<T: QOpenGLWidget_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QOpenGLWidget_setFormat {
  fn setFormat(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLWidget_setFormat for (&'a  QSurfaceFormat) {
  fn setFormat(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn format<T: QOpenGLWidget_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QOpenGLWidget_format {
  fn format(self, this: &mut QOpenGLWidget) -> i32;
}

// proto: QSurfaceFormat QOpenGLWidget::format();
impl<'a> /*trait*/ QOpenGLWidget_format for () {
  fn format(self, this: &mut QOpenGLWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget6formatEv()};
    unsafe {_ZNK13QOpenGLWidget6formatEv()};
    return 1;
  }
}

