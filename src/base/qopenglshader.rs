// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QOpenGLShader::NewQOpenGLShader(const QOpenGLShader & );
  fn _ZN13QOpenGLShaderC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QOpenGLShader::isCompiled();
  fn _ZNK13QOpenGLShader10isCompiledEv() -> i32;
  // proto: const QMetaObject * QOpenGLShader::metaObject();
  fn _ZNK13QOpenGLShader10metaObjectEv() -> i32;
  // proto: QString QOpenGLShader::log();
  fn _ZNK13QOpenGLShader3logEv() -> i32;
  // proto: bool QOpenGLShader::compileSourceCode(const QString & source);
  fn _ZN13QOpenGLShader17compileSourceCodeERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QOpenGLShader::compileSourceFile(const QString & fileName);
  fn _ZN13QOpenGLShader17compileSourceFileERK7QString(arg0: *const c_void) -> i32;
  // proto: QByteArray QOpenGLShader::sourceCode();
  fn _ZNK13QOpenGLShader10sourceCodeEv() -> i32;
  // proto: void QOpenGLShader::FreeQOpenGLShader();
  fn _ZN13QOpenGLShaderD0Ev() -> i32;
  // proto: bool QOpenGLShader::compileSourceCode(const QByteArray & source);
  fn _ZN13QOpenGLShader17compileSourceCodeERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: QOpenGLShader::GLuint QOpenGLShader::shaderId();
  fn _ZNK13QOpenGLShader8shaderIdEv() -> i32;
  // proto: bool QOpenGLShader::compileSourceCode(const char * source);
  fn _ZN13QOpenGLShader17compileSourceCodeEPKc(arg0: *const c_char) -> i32;
}

// body block begin
// class sizeof(QOpenGLShader)=1
pub struct QOpenGLShader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLShader {
  pub fn NewQOpenGLShader<T: QOpenGLShader_NewQOpenGLShader>(value: T) -> QOpenGLShader {
    let rsthis = value.NewQOpenGLShader();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLShader_NewQOpenGLShader {
  fn NewQOpenGLShader(self) -> QOpenGLShader;
}

// proto: void QOpenGLShader::NewQOpenGLShader(const QOpenGLShader & );
impl<'a> /*trait*/ QOpenGLShader_NewQOpenGLShader for (&'a  QOpenGLShader) {
  fn NewQOpenGLShader(self) -> QOpenGLShader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShaderC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QOpenGLShaderC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLShader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn isCompiled<T: QOpenGLShader_isCompiled>(&mut self, value: T) -> i32 {
    value.isCompiled(self);
    return 1;
  }
}

pub trait QOpenGLShader_isCompiled {
  fn isCompiled(self, this: &mut QOpenGLShader) -> i32;
}

// proto: bool QOpenGLShader::isCompiled();
impl<'a> /*trait*/ QOpenGLShader_isCompiled for () {
  fn isCompiled(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10isCompiledEv()};
    unsafe {_ZNK13QOpenGLShader10isCompiledEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn metaObject<T: QOpenGLShader_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QOpenGLShader_metaObject {
  fn metaObject(self, this: &mut QOpenGLShader) -> i32;
}

// proto: const QMetaObject * QOpenGLShader::metaObject();
impl<'a> /*trait*/ QOpenGLShader_metaObject for () {
  fn metaObject(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10metaObjectEv()};
    unsafe {_ZNK13QOpenGLShader10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn log<T: QOpenGLShader_log>(&mut self, value: T) -> i32 {
    value.log(self);
    return 1;
  }
}

pub trait QOpenGLShader_log {
  fn log(self, this: &mut QOpenGLShader) -> i32;
}

// proto: QString QOpenGLShader::log();
impl<'a> /*trait*/ QOpenGLShader_log for () {
  fn log(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader3logEv()};
    unsafe {_ZNK13QOpenGLShader3logEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn compileSourceCode<T: QOpenGLShader_compileSourceCode>(&mut self, value: T) -> i32 {
    value.compileSourceCode(self);
    return 1;
  }
}

pub trait QOpenGLShader_compileSourceCode {
  fn compileSourceCode(self, this: &mut QOpenGLShader) -> i32;
}

// proto: bool QOpenGLShader::compileSourceCode(const QString & source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode for (&'a  QString) {
  fn compileSourceCode(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QOpenGLShader17compileSourceCodeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn compileSourceFile<T: QOpenGLShader_compileSourceFile>(&mut self, value: T) -> i32 {
    value.compileSourceFile(self);
    return 1;
  }
}

pub trait QOpenGLShader_compileSourceFile {
  fn compileSourceFile(self, this: &mut QOpenGLShader) -> i32;
}

// proto: bool QOpenGLShader::compileSourceFile(const QString & fileName);
impl<'a> /*trait*/ QOpenGLShader_compileSourceFile for (&'a  QString) {
  fn compileSourceFile(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceFileERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QOpenGLShader17compileSourceFileERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn sourceCode<T: QOpenGLShader_sourceCode>(&mut self, value: T) -> i32 {
    value.sourceCode(self);
    return 1;
  }
}

pub trait QOpenGLShader_sourceCode {
  fn sourceCode(self, this: &mut QOpenGLShader) -> i32;
}

// proto: QByteArray QOpenGLShader::sourceCode();
impl<'a> /*trait*/ QOpenGLShader_sourceCode for () {
  fn sourceCode(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10sourceCodeEv()};
    unsafe {_ZNK13QOpenGLShader10sourceCodeEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn FreeQOpenGLShader<T: QOpenGLShader_FreeQOpenGLShader>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLShader(self);
    return 1;
  }
}

pub trait QOpenGLShader_FreeQOpenGLShader {
  fn FreeQOpenGLShader(self, this: &mut QOpenGLShader) -> i32;
}

// proto: void QOpenGLShader::FreeQOpenGLShader();
impl<'a> /*trait*/ QOpenGLShader_FreeQOpenGLShader for () {
  fn FreeQOpenGLShader(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShaderD0Ev()};
    unsafe {_ZN13QOpenGLShaderD0Ev()};
    return 1;
  }
}

// proto: bool QOpenGLShader::compileSourceCode(const QByteArray & source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode for (&'a  QByteArray) {
  fn compileSourceCode(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QOpenGLShader17compileSourceCodeERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLShader {
  pub fn shaderId<T: QOpenGLShader_shaderId>(&mut self, value: T) -> i32 {
    value.shaderId(self);
    return 1;
  }
}

pub trait QOpenGLShader_shaderId {
  fn shaderId(self, this: &mut QOpenGLShader) -> i32;
}

// proto: QOpenGLShader::GLuint QOpenGLShader::shaderId();
impl<'a> /*trait*/ QOpenGLShader_shaderId for () {
  fn shaderId(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader8shaderIdEv()};
    unsafe {_ZNK13QOpenGLShader8shaderIdEv()};
    return 1;
  }
}

// proto: bool QOpenGLShader::compileSourceCode(const char * source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode for (&'a  String) {
  fn compileSourceCode(self, this: &mut QOpenGLShader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN13QOpenGLShader17compileSourceCodeEPKc(arg0)};
    return 1;
  }
}

