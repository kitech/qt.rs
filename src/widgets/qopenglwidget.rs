// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtWidgets/qopenglwidget.h
// dst-file: /src/widgets/qopenglwidget.rs
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
use super::super::gui::qimage::QImage; // 771
use super::super::gui::qsurfaceformat::QSurfaceFormat; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLWidget_Class_Size() -> c_int;
  // proto:  void QOpenGLWidget::~QOpenGLWidget();
  fn _ZN13QOpenGLWidgetD0Ev(qthis: *mut c_void);
  // proto:  GLuint QOpenGLWidget::defaultFramebufferObject();
  fn _ZNK13QOpenGLWidget24defaultFramebufferObjectEv(qthis: *mut c_void) -> c_uint;
  // proto:  void QOpenGLWidget::resized();
  fn _ZN13QOpenGLWidget7resizedEv(qthis: *mut c_void);
  // proto:  void QOpenGLWidget::QOpenGLWidget(const QOpenGLWidget & );
  fn dector_ZN13QOpenGLWidgetC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QOpenGLWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QOpenGLWidget::isValid();
  fn _ZNK13QOpenGLWidget7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLWidget::aboutToResize();
  fn _ZN13QOpenGLWidget13aboutToResizeEv(qthis: *mut c_void);
  // proto:  QOpenGLContext * QOpenGLWidget::context();
  fn _ZNK13QOpenGLWidget7contextEv(qthis: *mut c_void);
  // proto:  void QOpenGLWidget::doneCurrent();
  fn _ZN13QOpenGLWidget11doneCurrentEv(qthis: *mut c_void);
  // proto:  void QOpenGLWidget::makeCurrent();
  fn _ZN13QOpenGLWidget11makeCurrentEv(qthis: *mut c_void);
  // proto:  QImage QOpenGLWidget::grabFramebuffer();
  fn _ZN13QOpenGLWidget15grabFramebufferEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QOpenGLWidget::metaObject();
  fn _ZNK13QOpenGLWidget10metaObjectEv(qthis: *mut c_void);
  // proto:  void QOpenGLWidget::frameSwapped();
  fn _ZN13QOpenGLWidget12frameSwappedEv(qthis: *mut c_void);
  // proto:  void QOpenGLWidget::aboutToCompose();
  fn _ZN13QOpenGLWidget14aboutToComposeEv(qthis: *mut c_void);
  // proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
  fn _ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSurfaceFormat QOpenGLWidget::format();
  fn _ZNK13QOpenGLWidget6formatEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLWidget)=1
pub struct QOpenGLWidget {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLWidget {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLWidget {
    return QOpenGLWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLWidget {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QOpenGLWidget {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLWidget::~QOpenGLWidget();
impl /*struct*/ QOpenGLWidget {
  pub fn Free<RetType, T: QOpenGLWidget_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::~QOpenGLWidget();
impl<'a> /*trait*/ QOpenGLWidget_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidgetD0Ev()};
     unsafe {_ZN13QOpenGLWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLWidget::defaultFramebufferObject();
impl /*struct*/ QOpenGLWidget {
  pub fn defaultFramebufferObject<RetType, T: QOpenGLWidget_defaultFramebufferObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_defaultFramebufferObject<RetType> {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  GLuint QOpenGLWidget::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLWidget_defaultFramebufferObject<u32> for () {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLWidget) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget24defaultFramebufferObjectEv()};
    let mut ret = unsafe {_ZNK13QOpenGLWidget24defaultFramebufferObjectEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::resized();
impl /*struct*/ QOpenGLWidget {
  pub fn resized<RetType, T: QOpenGLWidget_resized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resized(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_resized<RetType> {
  fn resized(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::resized();
impl<'a> /*trait*/ QOpenGLWidget_resized<()> for () {
  fn resized(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget7resizedEv()};
     unsafe {_ZN13QOpenGLWidget7resizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::QOpenGLWidget(const QOpenGLWidget & );
impl /*struct*/ QOpenGLWidget {
  pub fn New<T: QOpenGLWidget_New>(value: T) -> QOpenGLWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLWidget_New {
  fn New(self) -> QOpenGLWidget;
}

  // proto:  void QOpenGLWidget::QOpenGLWidget(const QOpenGLWidget & );
impl<'a> /*trait*/ QOpenGLWidget_New for (&'a QOpenGLWidget) {
  fn New(self) -> QOpenGLWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidgetC1ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLWidget_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QOpenGLWidgetC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QOpenGLWidgetC1ERKS_(arg0)};
    let rsthis = QOpenGLWidget{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLWidget::isValid();
impl /*struct*/ QOpenGLWidget {
  pub fn isValid<RetType, T: QOpenGLWidget_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_isValid<RetType> {
  fn isValid(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  bool QOpenGLWidget::isValid();
impl<'a> /*trait*/ QOpenGLWidget_isValid<i8> for () {
  fn isValid(self , rsthis: & QOpenGLWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7isValidEv()};
    let mut ret = unsafe {_ZNK13QOpenGLWidget7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::aboutToResize();
impl /*struct*/ QOpenGLWidget {
  pub fn aboutToResize<RetType, T: QOpenGLWidget_aboutToResize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.aboutToResize(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_aboutToResize<RetType> {
  fn aboutToResize(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::aboutToResize();
impl<'a> /*trait*/ QOpenGLWidget_aboutToResize<()> for () {
  fn aboutToResize(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget13aboutToResizeEv()};
     unsafe {_ZN13QOpenGLWidget13aboutToResizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLWidget::context();
impl /*struct*/ QOpenGLWidget {
  pub fn context<RetType, T: QOpenGLWidget_context<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.context(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_context<RetType> {
  fn context(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLWidget::context();
impl<'a> /*trait*/ QOpenGLWidget_context<()> for () {
  fn context(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7contextEv()};
     unsafe {_ZNK13QOpenGLWidget7contextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::doneCurrent();
impl /*struct*/ QOpenGLWidget {
  pub fn doneCurrent<RetType, T: QOpenGLWidget_doneCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_doneCurrent<RetType> {
  fn doneCurrent(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::doneCurrent();
impl<'a> /*trait*/ QOpenGLWidget_doneCurrent<()> for () {
  fn doneCurrent(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11doneCurrentEv()};
     unsafe {_ZN13QOpenGLWidget11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::makeCurrent();
impl /*struct*/ QOpenGLWidget {
  pub fn makeCurrent<RetType, T: QOpenGLWidget_makeCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_makeCurrent<RetType> {
  fn makeCurrent(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::makeCurrent();
impl<'a> /*trait*/ QOpenGLWidget_makeCurrent<()> for () {
  fn makeCurrent(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11makeCurrentEv()};
     unsafe {_ZN13QOpenGLWidget11makeCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QImage QOpenGLWidget::grabFramebuffer();
impl /*struct*/ QOpenGLWidget {
  pub fn grabFramebuffer<RetType, T: QOpenGLWidget_grabFramebuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_grabFramebuffer<RetType> {
  fn grabFramebuffer(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  QImage QOpenGLWidget::grabFramebuffer();
impl<'a> /*trait*/ QOpenGLWidget_grabFramebuffer<QImage> for () {
  fn grabFramebuffer(self , rsthis: & QOpenGLWidget) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget15grabFramebufferEv()};
    let mut ret = unsafe {_ZN13QOpenGLWidget15grabFramebufferEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLWidget::metaObject();
impl /*struct*/ QOpenGLWidget {
  pub fn metaObject<RetType, T: QOpenGLWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLWidget::metaObject();
impl<'a> /*trait*/ QOpenGLWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget10metaObjectEv()};
     unsafe {_ZNK13QOpenGLWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::frameSwapped();
impl /*struct*/ QOpenGLWidget {
  pub fn frameSwapped<RetType, T: QOpenGLWidget_frameSwapped<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameSwapped(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_frameSwapped<RetType> {
  fn frameSwapped(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::frameSwapped();
impl<'a> /*trait*/ QOpenGLWidget_frameSwapped<()> for () {
  fn frameSwapped(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget12frameSwappedEv()};
     unsafe {_ZN13QOpenGLWidget12frameSwappedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::aboutToCompose();
impl /*struct*/ QOpenGLWidget {
  pub fn aboutToCompose<RetType, T: QOpenGLWidget_aboutToCompose<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.aboutToCompose(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_aboutToCompose<RetType> {
  fn aboutToCompose(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::aboutToCompose();
impl<'a> /*trait*/ QOpenGLWidget_aboutToCompose<()> for () {
  fn aboutToCompose(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget14aboutToComposeEv()};
     unsafe {_ZN13QOpenGLWidget14aboutToComposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
impl /*struct*/ QOpenGLWidget {
  pub fn setFormat<RetType, T: QOpenGLWidget_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_setFormat<RetType> {
  fn setFormat(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLWidget_setFormat<()> for (&'a QSurfaceFormat) {
  fn setFormat(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSurfaceFormat QOpenGLWidget::format();
impl /*struct*/ QOpenGLWidget {
  pub fn format<RetType, T: QOpenGLWidget_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_format<RetType> {
  fn format(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  QSurfaceFormat QOpenGLWidget::format();
impl<'a> /*trait*/ QOpenGLWidget_format<QSurfaceFormat> for () {
  fn format(self , rsthis: & QOpenGLWidget) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget6formatEv()};
    let mut ret = unsafe {_ZNK13QOpenGLWidget6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

