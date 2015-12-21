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
use super::qobject::QObject;
use super::qopenglcontext::QOpenGLContext;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLShader::QOpenGLShader(const QOpenGLShader & );
  fn _ZN13QOpenGLShaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QOpenGLShader::isCompiled();
  fn _ZNK13QOpenGLShader10isCompiledEv(qthis: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QOpenGLShader::metaObject();
  fn _ZNK13QOpenGLShader10metaObjectEv(qthis: *mut c_void);
  // proto:  QString QOpenGLShader::log();
  fn _ZNK13QOpenGLShader3logEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QOpenGLShader::compileSourceCode(const QString & source);
  fn _ZN13QOpenGLShader17compileSourceCodeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QOpenGLShader::compileSourceFile(const QString & fileName);
  fn _ZN13QOpenGLShader17compileSourceFileERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QByteArray QOpenGLShader::sourceCode();
  fn _ZNK13QOpenGLShader10sourceCodeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLShader::~QOpenGLShader();
  fn _ZN13QOpenGLShaderD0Ev(qthis: *mut c_void);
  // proto:  bool QOpenGLShader::compileSourceCode(const QByteArray & source);
  fn _ZN13QOpenGLShader17compileSourceCodeERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  GLuint QOpenGLShader::shaderId();
  fn _ZNK13QOpenGLShader8shaderIdEv(qthis: *mut c_void);
  // proto:  bool QOpenGLShader::compileSourceCode(const char * source);
  fn _ZN13QOpenGLShader17compileSourceCodeEPKc(qthis: *mut c_void, arg0: *mut c_char) -> c_char;
}

// body block begin
// class sizeof(QOpenGLShader)=1
pub struct QOpenGLShader {
  pub qclsinst: *mut c_void,
}

  // proto:  void QOpenGLShader::QOpenGLShader(const QOpenGLShader & );
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

  // proto:  void QOpenGLShader::QOpenGLShader(const QOpenGLShader & );
impl<'a> /*trait*/ QOpenGLShader_NewQOpenGLShader for (QOpenGLShader) {
  fn NewQOpenGLShader(self) -> QOpenGLShader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QOpenGLShaderC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLShader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::isCompiled();
impl /*struct*/ QOpenGLShader {
  pub fn isCompiled<RetType, T: QOpenGLShader_isCompiled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCompiled(self);
    // return 1;
  }
}

pub trait QOpenGLShader_isCompiled<RetType> {
  fn isCompiled(self , rsthis: &mut QOpenGLShader) -> RetType;
}

  // proto:  bool QOpenGLShader::isCompiled();
impl<'a> /*trait*/ QOpenGLShader_isCompiled<i8> for () {
  fn isCompiled(self , rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10isCompiledEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader10isCompiledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLShader::metaObject();
impl /*struct*/ QOpenGLShader {
  pub fn metaObject<RetType, T: QOpenGLShader_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLShader_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QOpenGLShader) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLShader::metaObject();
impl<'a> /*trait*/ QOpenGLShader_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QOpenGLShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10metaObjectEv()};
     unsafe {_ZNK13QOpenGLShader10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QOpenGLShader::log();
impl /*struct*/ QOpenGLShader {
  pub fn log<RetType, T: QOpenGLShader_log<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.log(self);
    // return 1;
  }
}

pub trait QOpenGLShader_log<RetType> {
  fn log(self , rsthis: &mut QOpenGLShader) -> RetType;
}

  // proto:  QString QOpenGLShader::log();
impl<'a> /*trait*/ QOpenGLShader_log<QString> for () {
  fn log(self , rsthis: &mut QOpenGLShader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader3logEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader3logEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::compileSourceCode(const QString & source);
impl /*struct*/ QOpenGLShader {
  pub fn compileSourceCode<RetType, T: QOpenGLShader_compileSourceCode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.compileSourceCode(self);
    // return 1;
  }
}

pub trait QOpenGLShader_compileSourceCode<RetType> {
  fn compileSourceCode(self , rsthis: &mut QOpenGLShader) -> RetType;
}

  // proto:  bool QOpenGLShader::compileSourceCode(const QString & source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (QString) {
  fn compileSourceCode(self , rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::compileSourceFile(const QString & fileName);
impl /*struct*/ QOpenGLShader {
  pub fn compileSourceFile<RetType, T: QOpenGLShader_compileSourceFile<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.compileSourceFile(self);
    // return 1;
  }
}

pub trait QOpenGLShader_compileSourceFile<RetType> {
  fn compileSourceFile(self , rsthis: &mut QOpenGLShader) -> RetType;
}

  // proto:  bool QOpenGLShader::compileSourceFile(const QString & fileName);
impl<'a> /*trait*/ QOpenGLShader_compileSourceFile<i8> for (QString) {
  fn compileSourceFile(self , rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceFileERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QOpenGLShader::sourceCode();
impl /*struct*/ QOpenGLShader {
  pub fn sourceCode<RetType, T: QOpenGLShader_sourceCode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sourceCode(self);
    // return 1;
  }
}

pub trait QOpenGLShader_sourceCode<RetType> {
  fn sourceCode(self , rsthis: &mut QOpenGLShader) -> RetType;
}

  // proto:  QByteArray QOpenGLShader::sourceCode();
impl<'a> /*trait*/ QOpenGLShader_sourceCode<QByteArray> for () {
  fn sourceCode(self , rsthis: &mut QOpenGLShader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader10sourceCodeEv()};
    let mut ret = unsafe {_ZNK13QOpenGLShader10sourceCodeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLShader::~QOpenGLShader();
impl /*struct*/ QOpenGLShader {
  pub fn FreeQOpenGLShader<RetType, T: QOpenGLShader_FreeQOpenGLShader<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLShader(self);
    // return 1;
  }
}

pub trait QOpenGLShader_FreeQOpenGLShader<RetType> {
  fn FreeQOpenGLShader(self , rsthis: &mut QOpenGLShader) -> RetType;
}

  // proto:  void QOpenGLShader::~QOpenGLShader();
impl<'a> /*trait*/ QOpenGLShader_FreeQOpenGLShader<()> for () {
  fn FreeQOpenGLShader(self , rsthis: &mut QOpenGLShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShaderD0Ev()};
     unsafe {_ZN13QOpenGLShaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::compileSourceCode(const QByteArray & source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (QByteArray) {
  fn compileSourceCode(self , rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLShader::shaderId();
impl /*struct*/ QOpenGLShader {
  pub fn shaderId<RetType, T: QOpenGLShader_shaderId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.shaderId(self);
    // return 1;
  }
}

pub trait QOpenGLShader_shaderId<RetType> {
  fn shaderId(self , rsthis: &mut QOpenGLShader) -> RetType;
}

  // proto:  GLuint QOpenGLShader::shaderId();
impl<'a> /*trait*/ QOpenGLShader_shaderId<()> for () {
  fn shaderId(self , rsthis: &mut QOpenGLShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLShader8shaderIdEv()};
     unsafe {_ZNK13QOpenGLShader8shaderIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLShader::compileSourceCode(const char * source);
impl<'a> /*trait*/ QOpenGLShader_compileSourceCode<i8> for (&'a  String) {
  fn compileSourceCode(self , rsthis: &mut QOpenGLShader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLShader17compileSourceCodeEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN13QOpenGLShader17compileSourceCodeEPKc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

