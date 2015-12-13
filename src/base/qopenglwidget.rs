// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qimage::QImage;
use super::qsurfaceformat::QSurfaceFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLWidget::FreeQOpenGLWidget();
  fn _ZN13QOpenGLWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  QOpenGLWidget::GLuint QOpenGLWidget::defaultFramebufferObject();
  fn _ZNK13QOpenGLWidget24defaultFramebufferObjectEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLWidget::resized();
  fn _ZN13QOpenGLWidget7resizedEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLWidget::NewQOpenGLWidget(const QOpenGLWidget & );
  fn _ZN13QOpenGLWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QOpenGLWidget::isValid();
  fn _ZNK13QOpenGLWidget7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLWidget::aboutToResize();
  fn _ZN13QOpenGLWidget13aboutToResizeEv(qthis: *mut c_void) ;
  // proto:  QOpenGLContext * QOpenGLWidget::context();
  fn _ZNK13QOpenGLWidget7contextEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLWidget::doneCurrent();
  fn _ZN13QOpenGLWidget11doneCurrentEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLWidget::makeCurrent();
  fn _ZN13QOpenGLWidget11makeCurrentEv(qthis: *mut c_void) ;
  // proto:  QImage QOpenGLWidget::grabFramebuffer();
  fn _ZN13QOpenGLWidget15grabFramebufferEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QOpenGLWidget::metaObject();
  fn _ZNK13QOpenGLWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLWidget::frameSwapped();
  fn _ZN13QOpenGLWidget12frameSwappedEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLWidget::aboutToCompose();
  fn _ZN13QOpenGLWidget14aboutToComposeEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
  fn _ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSurfaceFormat QOpenGLWidget::format();
  fn _ZNK13QOpenGLWidget6formatEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QOpenGLWidget)=1
pub struct QOpenGLWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLWidget {
  pub fn FreeQOpenGLWidget<T: QOpenGLWidget_FreeQOpenGLWidget>(&mut self, value: T)  {
     value.FreeQOpenGLWidget(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_FreeQOpenGLWidget {
  fn FreeQOpenGLWidget(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  void QOpenGLWidget::FreeQOpenGLWidget();
impl<'a> /*trait*/ QOpenGLWidget_FreeQOpenGLWidget for () {
  fn FreeQOpenGLWidget(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidgetD0Ev()};
     unsafe {_ZN13QOpenGLWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn defaultFramebufferObject<T: QOpenGLWidget_defaultFramebufferObject>(&mut self, value: T)  {
     value.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_defaultFramebufferObject {
  fn defaultFramebufferObject(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  QOpenGLWidget::GLuint QOpenGLWidget::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLWidget_defaultFramebufferObject for () {
  fn defaultFramebufferObject(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget24defaultFramebufferObjectEv()};
     unsafe {_ZNK13QOpenGLWidget24defaultFramebufferObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn resized<T: QOpenGLWidget_resized>(&mut self, value: T)  {
     value.resized(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_resized {
  fn resized(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  void QOpenGLWidget::resized();
impl<'a> /*trait*/ QOpenGLWidget_resized for () {
  fn resized(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget7resizedEv()};
     unsafe {_ZN13QOpenGLWidget7resizedEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QOpenGLWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn isValid<T: QOpenGLWidget_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_isValid {
  fn isValid(self, rsthis: &mut QOpenGLWidget) -> i8;
}

// proto:  bool QOpenGLWidget::isValid();
impl<'a> /*trait*/ QOpenGLWidget_isValid for () {
  fn isValid(self, rsthis: &mut QOpenGLWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7isValidEv()};
    let mut ret = unsafe {_ZNK13QOpenGLWidget7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn aboutToResize<T: QOpenGLWidget_aboutToResize>(&mut self, value: T)  {
     value.aboutToResize(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_aboutToResize {
  fn aboutToResize(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  void QOpenGLWidget::aboutToResize();
impl<'a> /*trait*/ QOpenGLWidget_aboutToResize for () {
  fn aboutToResize(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget13aboutToResizeEv()};
     unsafe {_ZN13QOpenGLWidget13aboutToResizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn context<T: QOpenGLWidget_context>(&mut self, value: T)  {
     value.context(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_context {
  fn context(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  QOpenGLContext * QOpenGLWidget::context();
impl<'a> /*trait*/ QOpenGLWidget_context for () {
  fn context(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7contextEv()};
     unsafe {_ZNK13QOpenGLWidget7contextEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn doneCurrent<T: QOpenGLWidget_doneCurrent>(&mut self, value: T)  {
     value.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_doneCurrent {
  fn doneCurrent(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  void QOpenGLWidget::doneCurrent();
impl<'a> /*trait*/ QOpenGLWidget_doneCurrent for () {
  fn doneCurrent(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11doneCurrentEv()};
     unsafe {_ZN13QOpenGLWidget11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn makeCurrent<T: QOpenGLWidget_makeCurrent>(&mut self, value: T)  {
     value.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_makeCurrent {
  fn makeCurrent(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  void QOpenGLWidget::makeCurrent();
impl<'a> /*trait*/ QOpenGLWidget_makeCurrent for () {
  fn makeCurrent(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11makeCurrentEv()};
     unsafe {_ZN13QOpenGLWidget11makeCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn grabFramebuffer<T: QOpenGLWidget_grabFramebuffer>(&mut self, value: T) -> QImage {
    return value.grabFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_grabFramebuffer {
  fn grabFramebuffer(self, rsthis: &mut QOpenGLWidget) -> QImage;
}

// proto:  QImage QOpenGLWidget::grabFramebuffer();
impl<'a> /*trait*/ QOpenGLWidget_grabFramebuffer for () {
  fn grabFramebuffer(self, rsthis: &mut QOpenGLWidget) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget15grabFramebufferEv()};
    let mut ret = unsafe {_ZN13QOpenGLWidget15grabFramebufferEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn metaObject<T: QOpenGLWidget_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_metaObject {
  fn metaObject(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  const QMetaObject * QOpenGLWidget::metaObject();
impl<'a> /*trait*/ QOpenGLWidget_metaObject for () {
  fn metaObject(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget10metaObjectEv()};
     unsafe {_ZNK13QOpenGLWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn frameSwapped<T: QOpenGLWidget_frameSwapped>(&mut self, value: T)  {
     value.frameSwapped(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_frameSwapped {
  fn frameSwapped(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  void QOpenGLWidget::frameSwapped();
impl<'a> /*trait*/ QOpenGLWidget_frameSwapped for () {
  fn frameSwapped(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget12frameSwappedEv()};
     unsafe {_ZN13QOpenGLWidget12frameSwappedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn aboutToCompose<T: QOpenGLWidget_aboutToCompose>(&mut self, value: T)  {
     value.aboutToCompose(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_aboutToCompose {
  fn aboutToCompose(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  void QOpenGLWidget::aboutToCompose();
impl<'a> /*trait*/ QOpenGLWidget_aboutToCompose for () {
  fn aboutToCompose(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget14aboutToComposeEv()};
     unsafe {_ZN13QOpenGLWidget14aboutToComposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn setFormat<T: QOpenGLWidget_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_setFormat {
  fn setFormat(self, rsthis: &mut QOpenGLWidget) ;
}

// proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLWidget_setFormat for (&'a  QSurfaceFormat) {
  fn setFormat(self, rsthis: &mut QOpenGLWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn format<T: QOpenGLWidget_format>(&mut self, value: T) -> QSurfaceFormat {
    return value.format(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_format {
  fn format(self, rsthis: &mut QOpenGLWidget) -> QSurfaceFormat;
}

// proto:  QSurfaceFormat QOpenGLWidget::format();
impl<'a> /*trait*/ QOpenGLWidget_format for () {
  fn format(self, rsthis: &mut QOpenGLWidget) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget6formatEv()};
    let mut ret = unsafe {_ZNK13QOpenGLWidget6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

