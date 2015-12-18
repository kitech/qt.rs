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
  pub fn FreeQOpenGLWidget<RetType, T: QOpenGLWidget_FreeQOpenGLWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQOpenGLWidget(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_FreeQOpenGLWidget<RetType> {
  fn FreeQOpenGLWidget(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  void QOpenGLWidget::FreeQOpenGLWidget();
impl<'a> /*trait*/ QOpenGLWidget_FreeQOpenGLWidget<()> for () {
  fn FreeQOpenGLWidget(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidgetD0Ev()};
     unsafe {_ZN13QOpenGLWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn defaultFramebufferObject<RetType, T: QOpenGLWidget_defaultFramebufferObject<RetType>>(&mut self, value: T) -> RetType {
    return value.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_defaultFramebufferObject<RetType> {
  fn defaultFramebufferObject(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  QOpenGLWidget::GLuint QOpenGLWidget::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLWidget_defaultFramebufferObject<()> for () {
  fn defaultFramebufferObject(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget24defaultFramebufferObjectEv()};
     unsafe {_ZNK13QOpenGLWidget24defaultFramebufferObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn resized<RetType, T: QOpenGLWidget_resized<RetType>>(&mut self, value: T) -> RetType {
    return value.resized(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_resized<RetType> {
  fn resized(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  void QOpenGLWidget::resized();
impl<'a> /*trait*/ QOpenGLWidget_resized<()> for () {
  fn resized(self, rsthis: &mut QOpenGLWidget) -> () {
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
  pub fn isValid<RetType, T: QOpenGLWidget_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_isValid<RetType> {
  fn isValid(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  bool QOpenGLWidget::isValid();
impl<'a> /*trait*/ QOpenGLWidget_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QOpenGLWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7isValidEv()};
    let mut ret = unsafe {_ZNK13QOpenGLWidget7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn aboutToResize<RetType, T: QOpenGLWidget_aboutToResize<RetType>>(&mut self, value: T) -> RetType {
    return value.aboutToResize(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_aboutToResize<RetType> {
  fn aboutToResize(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  void QOpenGLWidget::aboutToResize();
impl<'a> /*trait*/ QOpenGLWidget_aboutToResize<()> for () {
  fn aboutToResize(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget13aboutToResizeEv()};
     unsafe {_ZN13QOpenGLWidget13aboutToResizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn context<RetType, T: QOpenGLWidget_context<RetType>>(&mut self, value: T) -> RetType {
    return value.context(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_context<RetType> {
  fn context(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  QOpenGLContext * QOpenGLWidget::context();
impl<'a> /*trait*/ QOpenGLWidget_context<()> for () {
  fn context(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7contextEv()};
     unsafe {_ZNK13QOpenGLWidget7contextEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn doneCurrent<RetType, T: QOpenGLWidget_doneCurrent<RetType>>(&mut self, value: T) -> RetType {
    return value.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_doneCurrent<RetType> {
  fn doneCurrent(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  void QOpenGLWidget::doneCurrent();
impl<'a> /*trait*/ QOpenGLWidget_doneCurrent<()> for () {
  fn doneCurrent(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11doneCurrentEv()};
     unsafe {_ZN13QOpenGLWidget11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn makeCurrent<RetType, T: QOpenGLWidget_makeCurrent<RetType>>(&mut self, value: T) -> RetType {
    return value.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_makeCurrent<RetType> {
  fn makeCurrent(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  void QOpenGLWidget::makeCurrent();
impl<'a> /*trait*/ QOpenGLWidget_makeCurrent<()> for () {
  fn makeCurrent(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11makeCurrentEv()};
     unsafe {_ZN13QOpenGLWidget11makeCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn grabFramebuffer<RetType, T: QOpenGLWidget_grabFramebuffer<RetType>>(&mut self, value: T) -> RetType {
    return value.grabFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_grabFramebuffer<RetType> {
  fn grabFramebuffer(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  QImage QOpenGLWidget::grabFramebuffer();
impl<'a> /*trait*/ QOpenGLWidget_grabFramebuffer<QImage> for () {
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
  pub fn metaObject<RetType, T: QOpenGLWidget_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  const QMetaObject * QOpenGLWidget::metaObject();
impl<'a> /*trait*/ QOpenGLWidget_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget10metaObjectEv()};
     unsafe {_ZNK13QOpenGLWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn frameSwapped<RetType, T: QOpenGLWidget_frameSwapped<RetType>>(&mut self, value: T) -> RetType {
    return value.frameSwapped(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_frameSwapped<RetType> {
  fn frameSwapped(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  void QOpenGLWidget::frameSwapped();
impl<'a> /*trait*/ QOpenGLWidget_frameSwapped<()> for () {
  fn frameSwapped(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget12frameSwappedEv()};
     unsafe {_ZN13QOpenGLWidget12frameSwappedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn aboutToCompose<RetType, T: QOpenGLWidget_aboutToCompose<RetType>>(&mut self, value: T) -> RetType {
    return value.aboutToCompose(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_aboutToCompose<RetType> {
  fn aboutToCompose(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  void QOpenGLWidget::aboutToCompose();
impl<'a> /*trait*/ QOpenGLWidget_aboutToCompose<()> for () {
  fn aboutToCompose(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget14aboutToComposeEv()};
     unsafe {_ZN13QOpenGLWidget14aboutToComposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn setFormat<RetType, T: QOpenGLWidget_setFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.setFormat(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_setFormat<RetType> {
  fn setFormat(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLWidget_setFormat<()> for (&'a  QSurfaceFormat) {
  fn setFormat(self, rsthis: &mut QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLWidget {
  pub fn format<RetType, T: QOpenGLWidget_format<RetType>>(&mut self, value: T) -> RetType {
    return value.format(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_format<RetType> {
  fn format(self, rsthis: &mut QOpenGLWidget) -> RetType;
}

// proto:  QSurfaceFormat QOpenGLWidget::format();
impl<'a> /*trait*/ QOpenGLWidget_format<QSurfaceFormat> for () {
  fn format(self, rsthis: &mut QOpenGLWidget) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget6formatEv()};
    let mut ret = unsafe {_ZNK13QOpenGLWidget6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

