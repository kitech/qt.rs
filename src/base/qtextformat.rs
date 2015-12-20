// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextblockformat::QTextBlockFormat;
use super::qstring::QString;
use super::qtextlength::QTextLength;
use super::qcolor::QColor;
use super::qbrush::QBrush;
use super::qtextframeformat::QTextFrameFormat;
use super::qpen::QPen;
use super::qvariant::QVariant;
use super::qtextimageformat::QTextImageFormat;
use super::qtexttableformat::QTextTableFormat;
use super::qtextcharformat::QTextCharFormat;
use super::qtexttablecellformat::QTextTableCellFormat;
use super::qtextlistformat::QTextListFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QTextBlockFormat QTextFormat::toBlockFormat();
  fn _ZNK11QTextFormat13toBlockFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextFormat::stringProperty(int propertyId);
  fn _ZNK11QTextFormat14stringPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
  fn _ZNK11QTextFormat20lengthVectorPropertyEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QTextFormat::objectIndex();
  fn _ZNK11QTextFormat11objectIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextFormat::setObjectIndex(int object);
  fn _ZN11QTextFormat14setObjectIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextFormat::clearForeground();
  fn _ZN11QTextFormat15clearForegroundEv(qthis: *mut c_void);
  // proto:  bool QTextFormat::isTableCellFormat();
  fn _ZNK11QTextFormat17isTableCellFormatEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextFormat::~QTextFormat();
  fn _ZN11QTextFormatD0Ev(qthis: *mut c_void);
  // proto:  bool QTextFormat::isValid();
  fn _ZNK11QTextFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextFormat::QTextFormat(const QTextFormat & rhs);
  fn _ZN11QTextFormatC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextLength QTextFormat::lengthProperty(int propertyId);
  fn _ZNK11QTextFormat14lengthPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextFormat::merge(const QTextFormat & other);
  fn _ZN11QTextFormat5mergeERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QColor QTextFormat::colorProperty(int propertyId);
  fn _ZNK11QTextFormat13colorPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextFormat::QTextFormat();
  fn _ZN11QTextFormatC1Ev(qthis: *mut c_void);
  // proto:  void QTextFormat::setForeground(const QBrush & brush);
  fn _ZN11QTextFormat13setForegroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextFormat::boolProperty(int propertyId);
  fn _ZNK11QTextFormat12boolPropertyEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  bool QTextFormat::isListFormat();
  fn _ZNK11QTextFormat12isListFormatEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextFormat::QTextFormat(int type);
  fn _ZN11QTextFormatC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QTextFormat::isImageFormat();
  fn _ZNK11QTextFormat13isImageFormatEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextFormat::clearProperty(int propertyId);
  fn _ZN11QTextFormat13clearPropertyEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QTextFrameFormat QTextFormat::toFrameFormat();
  fn _ZNK11QTextFormat13toFrameFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBrush QTextFormat::brushProperty(int propertyId);
  fn _ZNK11QTextFormat13brushPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTextFormat::propertyCount();
  fn _ZNK11QTextFormat13propertyCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QPen QTextFormat::penProperty(int propertyId);
  fn _ZNK11QTextFormat11penPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QVariant QTextFormat::property(int propertyId);
  fn _ZNK11QTextFormat8propertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTextFormat::isTableFormat();
  fn _ZNK11QTextFormat13isTableFormatEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextFormat::setProperty(int propertyId, const QVariant & value);
  fn _ZN11QTextFormat11setPropertyEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  int QTextFormat::type();
  fn _ZNK11QTextFormat4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextFormat::isCharFormat();
  fn _ZNK11QTextFormat12isCharFormatEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextFormat::clearBackground();
  fn _ZN11QTextFormat15clearBackgroundEv(qthis: *mut c_void);
  // proto:  bool QTextFormat::isBlockFormat();
  fn _ZNK11QTextFormat13isBlockFormatEv(qthis: *mut c_void) -> c_char;
  // proto:  QBrush QTextFormat::background();
  fn _ZNK11QTextFormat10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTextFormat::doubleProperty(int propertyId);
  fn _ZNK11QTextFormat14doublePropertyEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QTextFormat::swap(QTextFormat & other);
  fn _ZN11QTextFormat4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextImageFormat QTextFormat::toImageFormat();
  fn _ZNK11QTextFormat13toImageFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextFormat::hasProperty(int propertyId);
  fn _ZNK11QTextFormat11hasPropertyEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  QBrush QTextFormat::foreground();
  fn _ZNK11QTextFormat10foregroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFormat::setObjectType(int type);
  fn _ZN11QTextFormat13setObjectTypeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextFormat::setBackground(const QBrush & brush);
  fn _ZN11QTextFormat13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextTableFormat QTextFormat::toTableFormat();
  fn _ZNK11QTextFormat13toTableFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextFormat::isFrameFormat();
  fn _ZNK11QTextFormat13isFrameFormatEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTextFormat::intProperty(int propertyId);
  fn _ZNK11QTextFormat11intPropertyEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QTextCharFormat QTextFormat::toCharFormat();
  fn _ZNK11QTextFormat12toCharFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextFormat::isEmpty();
  fn _ZNK11QTextFormat7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QTextTableCellFormat QTextFormat::toTableCellFormat();
  fn _ZNK11QTextFormat17toTableCellFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextFormat::objectType();
  fn _ZNK11QTextFormat10objectTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextListFormat QTextFormat::toListFormat();
  fn _ZNK11QTextFormat12toListFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMap<int, QVariant> QTextFormat::properties();
  fn _ZNK11QTextFormat10propertiesEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QTextFormat)=1
pub struct QTextFormat {
  pub qclsinst: *mut c_void,
}

  // proto:  QTextBlockFormat QTextFormat::toBlockFormat();
impl /*struct*/ QTextFormat {
  pub fn toBlockFormat<RetType, T: QTextFormat_toBlockFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toBlockFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toBlockFormat<RetType> {
  fn toBlockFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QTextBlockFormat QTextFormat::toBlockFormat();
impl<'a> /*trait*/ QTextFormat_toBlockFormat<QTextBlockFormat> for () {
  fn toBlockFormat(self , rsthis: &mut QTextFormat) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toBlockFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toBlockFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextFormat::stringProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn stringProperty<RetType, T: QTextFormat_stringProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stringProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_stringProperty<RetType> {
  fn stringProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QString QTextFormat::stringProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_stringProperty<QString> for (i32) {
  fn stringProperty(self , rsthis: &mut QTextFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14stringPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14stringPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn lengthVectorProperty<RetType, T: QTextFormat_lengthVectorProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lengthVectorProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_lengthVectorProperty<RetType> {
  fn lengthVectorProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_lengthVectorProperty<()> for (i32) {
  fn lengthVectorProperty(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat20lengthVectorPropertyEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK11QTextFormat20lengthVectorPropertyEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextFormat::objectIndex();
impl /*struct*/ QTextFormat {
  pub fn objectIndex<RetType, T: QTextFormat_objectIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.objectIndex(self);
    // return 1;
  }
}

pub trait QTextFormat_objectIndex<RetType> {
  fn objectIndex(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::objectIndex();
impl<'a> /*trait*/ QTextFormat_objectIndex<i32> for () {
  fn objectIndex(self , rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11objectIndexEv()};
    let mut ret = unsafe {_ZNK11QTextFormat11objectIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextFormat::setObjectIndex(int object);
impl /*struct*/ QTextFormat {
  pub fn setObjectIndex<RetType, T: QTextFormat_setObjectIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setObjectIndex(self);
    // return 1;
  }
}

pub trait QTextFormat_setObjectIndex<RetType> {
  fn setObjectIndex(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::setObjectIndex(int object);
impl<'a> /*trait*/ QTextFormat_setObjectIndex<()> for (i32) {
  fn setObjectIndex(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat14setObjectIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat14setObjectIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFormat::clearForeground();
impl /*struct*/ QTextFormat {
  pub fn clearForeground<RetType, T: QTextFormat_clearForeground<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearForeground(self);
    // return 1;
  }
}

pub trait QTextFormat_clearForeground<RetType> {
  fn clearForeground(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::clearForeground();
impl<'a> /*trait*/ QTextFormat_clearForeground<()> for () {
  fn clearForeground(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat15clearForegroundEv()};
     unsafe {_ZN11QTextFormat15clearForegroundEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextFormat::isTableCellFormat();
impl /*struct*/ QTextFormat {
  pub fn isTableCellFormat<RetType, T: QTextFormat_isTableCellFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isTableCellFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isTableCellFormat<RetType> {
  fn isTableCellFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isTableCellFormat();
impl<'a> /*trait*/ QTextFormat_isTableCellFormat<i8> for () {
  fn isTableCellFormat(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat17isTableCellFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat17isTableCellFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFormat::~QTextFormat();
impl /*struct*/ QTextFormat {
  pub fn FreeQTextFormat<RetType, T: QTextFormat_FreeQTextFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_FreeQTextFormat<RetType> {
  fn FreeQTextFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::~QTextFormat();
impl<'a> /*trait*/ QTextFormat_FreeQTextFormat<()> for () {
  fn FreeQTextFormat(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatD0Ev()};
     unsafe {_ZN11QTextFormatD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextFormat::isValid();
impl /*struct*/ QTextFormat {
  pub fn isValid<RetType, T: QTextFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isValid();
impl<'a> /*trait*/ QTextFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat7isValidEv()};
    let mut ret = unsafe {_ZNK11QTextFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFormat::QTextFormat(const QTextFormat & rhs);
impl /*struct*/ QTextFormat {
  pub fn NewQTextFormat<T: QTextFormat_NewQTextFormat>(value: T) -> QTextFormat {
    let rsthis = value.NewQTextFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFormat_NewQTextFormat {
  fn NewQTextFormat(self) -> QTextFormat;
}

  // proto:  void QTextFormat::QTextFormat(const QTextFormat & rhs);
impl<'a> /*trait*/ QTextFormat_NewQTextFormat for (QTextFormat) {
  fn NewQTextFormat(self) -> QTextFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextFormatC1ERKS_(qthis, arg0)};
    let rsthis = QTextFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextLength QTextFormat::lengthProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn lengthProperty<RetType, T: QTextFormat_lengthProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lengthProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_lengthProperty<RetType> {
  fn lengthProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QTextLength QTextFormat::lengthProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_lengthProperty<QTextLength> for (i32) {
  fn lengthProperty(self , rsthis: &mut QTextFormat) -> QTextLength {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14lengthPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14lengthPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLength{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFormat::merge(const QTextFormat & other);
impl /*struct*/ QTextFormat {
  pub fn merge<RetType, T: QTextFormat_merge<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.merge(self);
    // return 1;
  }
}

pub trait QTextFormat_merge<RetType> {
  fn merge(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::merge(const QTextFormat & other);
impl<'a> /*trait*/ QTextFormat_merge<()> for (QTextFormat) {
  fn merge(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat5mergeERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat5mergeERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QTextFormat::colorProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn colorProperty<RetType, T: QTextFormat_colorProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.colorProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_colorProperty<RetType> {
  fn colorProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QColor QTextFormat::colorProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_colorProperty<QColor> for (i32) {
  fn colorProperty(self , rsthis: &mut QTextFormat) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13colorPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat13colorPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFormat::QTextFormat();
impl<'a> /*trait*/ QTextFormat_NewQTextFormat for () {
  fn NewQTextFormat(self) -> QTextFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatC1Ev()};
    unsafe {_ZN11QTextFormatC1Ev(qthis)};
    let rsthis = QTextFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextFormat::setForeground(const QBrush & brush);
impl /*struct*/ QTextFormat {
  pub fn setForeground<RetType, T: QTextFormat_setForeground<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setForeground(self);
    // return 1;
  }
}

pub trait QTextFormat_setForeground<RetType> {
  fn setForeground(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QTextFormat_setForeground<()> for (QBrush) {
  fn setForeground(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextFormat::boolProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn boolProperty<RetType, T: QTextFormat_boolProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boolProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_boolProperty<RetType> {
  fn boolProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::boolProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_boolProperty<i8> for (i32) {
  fn boolProperty(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12boolPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat12boolPropertyEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTextFormat::isListFormat();
impl /*struct*/ QTextFormat {
  pub fn isListFormat<RetType, T: QTextFormat_isListFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isListFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isListFormat<RetType> {
  fn isListFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isListFormat();
impl<'a> /*trait*/ QTextFormat_isListFormat<i8> for () {
  fn isListFormat(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12isListFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12isListFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFormat::QTextFormat(int type);
impl<'a> /*trait*/ QTextFormat_NewQTextFormat for (i32) {
  fn NewQTextFormat(self) -> QTextFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextFormatC1Ei(qthis, arg0)};
    let rsthis = QTextFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextFormat::isImageFormat();
impl /*struct*/ QTextFormat {
  pub fn isImageFormat<RetType, T: QTextFormat_isImageFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isImageFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isImageFormat<RetType> {
  fn isImageFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isImageFormat();
impl<'a> /*trait*/ QTextFormat_isImageFormat<i8> for () {
  fn isImageFormat(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isImageFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13isImageFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFormat::clearProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn clearProperty<RetType, T: QTextFormat_clearProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_clearProperty<RetType> {
  fn clearProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::clearProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_clearProperty<()> for (i32) {
  fn clearProperty(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13clearPropertyEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat13clearPropertyEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextFrameFormat QTextFormat::toFrameFormat();
impl /*struct*/ QTextFormat {
  pub fn toFrameFormat<RetType, T: QTextFormat_toFrameFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toFrameFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toFrameFormat<RetType> {
  fn toFrameFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QTextFrameFormat QTextFormat::toFrameFormat();
impl<'a> /*trait*/ QTextFormat_toFrameFormat<QTextFrameFormat> for () {
  fn toFrameFormat(self , rsthis: &mut QTextFormat) -> QTextFrameFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toFrameFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toFrameFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrameFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QBrush QTextFormat::brushProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn brushProperty<RetType, T: QTextFormat_brushProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.brushProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_brushProperty<RetType> {
  fn brushProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QBrush QTextFormat::brushProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_brushProperty<QBrush> for (i32) {
  fn brushProperty(self , rsthis: &mut QTextFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13brushPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat13brushPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextFormat::propertyCount();
impl /*struct*/ QTextFormat {
  pub fn propertyCount<RetType, T: QTextFormat_propertyCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.propertyCount(self);
    // return 1;
  }
}

pub trait QTextFormat_propertyCount<RetType> {
  fn propertyCount(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::propertyCount();
impl<'a> /*trait*/ QTextFormat_propertyCount<i32> for () {
  fn propertyCount(self , rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13propertyCountEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13propertyCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPen QTextFormat::penProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn penProperty<RetType, T: QTextFormat_penProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.penProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_penProperty<RetType> {
  fn penProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QPen QTextFormat::penProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_penProperty<QPen> for (i32) {
  fn penProperty(self , rsthis: &mut QTextFormat) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11penPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat11penPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QPen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QTextFormat::property(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn property<RetType, T: QTextFormat_property<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.property(self);
    // return 1;
  }
}

pub trait QTextFormat_property<RetType> {
  fn property(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QVariant QTextFormat::property(int propertyId);
impl<'a> /*trait*/ QTextFormat_property<QVariant> for (i32) {
  fn property(self , rsthis: &mut QTextFormat) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat8propertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat8propertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextFormat::isTableFormat();
impl /*struct*/ QTextFormat {
  pub fn isTableFormat<RetType, T: QTextFormat_isTableFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isTableFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isTableFormat<RetType> {
  fn isTableFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isTableFormat();
impl<'a> /*trait*/ QTextFormat_isTableFormat<i8> for () {
  fn isTableFormat(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isTableFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13isTableFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFormat::setProperty(int propertyId, const QVariant & value);
impl /*struct*/ QTextFormat {
  pub fn setProperty<RetType, T: QTextFormat_setProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_setProperty<RetType> {
  fn setProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::setProperty(int propertyId, const QVariant & value);
impl<'a> /*trait*/ QTextFormat_setProperty<()> for (i32, QVariant) {
  fn setProperty(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat11setPropertyEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat11setPropertyEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTextFormat::type();
impl /*struct*/ QTextFormat {
  pub fn type_<RetType, T: QTextFormat_type_<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QTextFormat_type_<RetType> {
  fn type_(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::type();
impl<'a> /*trait*/ QTextFormat_type_<i32> for () {
  fn type_(self , rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat4typeEv()};
    let mut ret = unsafe {_ZNK11QTextFormat4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextFormat::isCharFormat();
impl /*struct*/ QTextFormat {
  pub fn isCharFormat<RetType, T: QTextFormat_isCharFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCharFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isCharFormat<RetType> {
  fn isCharFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isCharFormat();
impl<'a> /*trait*/ QTextFormat_isCharFormat<i8> for () {
  fn isCharFormat(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12isCharFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12isCharFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFormat::clearBackground();
impl /*struct*/ QTextFormat {
  pub fn clearBackground<RetType, T: QTextFormat_clearBackground<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearBackground(self);
    // return 1;
  }
}

pub trait QTextFormat_clearBackground<RetType> {
  fn clearBackground(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::clearBackground();
impl<'a> /*trait*/ QTextFormat_clearBackground<()> for () {
  fn clearBackground(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat15clearBackgroundEv()};
     unsafe {_ZN11QTextFormat15clearBackgroundEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextFormat::isBlockFormat();
impl /*struct*/ QTextFormat {
  pub fn isBlockFormat<RetType, T: QTextFormat_isBlockFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isBlockFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isBlockFormat<RetType> {
  fn isBlockFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isBlockFormat();
impl<'a> /*trait*/ QTextFormat_isBlockFormat<i8> for () {
  fn isBlockFormat(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isBlockFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13isBlockFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QBrush QTextFormat::background();
impl /*struct*/ QTextFormat {
  pub fn background<RetType, T: QTextFormat_background<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QTextFormat_background<RetType> {
  fn background(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QBrush QTextFormat::background();
impl<'a> /*trait*/ QTextFormat_background<QBrush> for () {
  fn background(self , rsthis: &mut QTextFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10backgroundEv()};
    let mut ret = unsafe {_ZNK11QTextFormat10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTextFormat::doubleProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn doubleProperty<RetType, T: QTextFormat_doubleProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doubleProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_doubleProperty<RetType> {
  fn doubleProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  qreal QTextFormat::doubleProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_doubleProperty<f64> for (i32) {
  fn doubleProperty(self , rsthis: &mut QTextFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14doublePropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14doublePropertyEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextFormat::swap(QTextFormat & other);
impl /*struct*/ QTextFormat {
  pub fn swap<RetType, T: QTextFormat_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QTextFormat_swap<RetType> {
  fn swap(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::swap(QTextFormat & other);
impl<'a> /*trait*/ QTextFormat_swap<()> for (QTextFormat) {
  fn swap(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextImageFormat QTextFormat::toImageFormat();
impl /*struct*/ QTextFormat {
  pub fn toImageFormat<RetType, T: QTextFormat_toImageFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toImageFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toImageFormat<RetType> {
  fn toImageFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QTextImageFormat QTextFormat::toImageFormat();
impl<'a> /*trait*/ QTextFormat_toImageFormat<QTextImageFormat> for () {
  fn toImageFormat(self , rsthis: &mut QTextFormat) -> QTextImageFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toImageFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toImageFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextImageFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextFormat::hasProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn hasProperty<RetType, T: QTextFormat_hasProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_hasProperty<RetType> {
  fn hasProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::hasProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_hasProperty<i8> for (i32) {
  fn hasProperty(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11hasPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat11hasPropertyEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QBrush QTextFormat::foreground();
impl /*struct*/ QTextFormat {
  pub fn foreground<RetType, T: QTextFormat_foreground<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.foreground(self);
    // return 1;
  }
}

pub trait QTextFormat_foreground<RetType> {
  fn foreground(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QBrush QTextFormat::foreground();
impl<'a> /*trait*/ QTextFormat_foreground<QBrush> for () {
  fn foreground(self , rsthis: &mut QTextFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10foregroundEv()};
    let mut ret = unsafe {_ZNK11QTextFormat10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFormat::setObjectType(int type);
impl /*struct*/ QTextFormat {
  pub fn setObjectType<RetType, T: QTextFormat_setObjectType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setObjectType(self);
    // return 1;
  }
}

pub trait QTextFormat_setObjectType<RetType> {
  fn setObjectType(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::setObjectType(int type);
impl<'a> /*trait*/ QTextFormat_setObjectType<()> for (i32) {
  fn setObjectType(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setObjectTypeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat13setObjectTypeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFormat::setBackground(const QBrush & brush);
impl /*struct*/ QTextFormat {
  pub fn setBackground<RetType, T: QTextFormat_setBackground<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBackground(self);
    // return 1;
  }
}

pub trait QTextFormat_setBackground<RetType> {
  fn setBackground(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QTextFormat_setBackground<()> for (QBrush) {
  fn setBackground(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextTableFormat QTextFormat::toTableFormat();
impl /*struct*/ QTextFormat {
  pub fn toTableFormat<RetType, T: QTextFormat_toTableFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toTableFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toTableFormat<RetType> {
  fn toTableFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QTextTableFormat QTextFormat::toTableFormat();
impl<'a> /*trait*/ QTextFormat_toTableFormat<QTextTableFormat> for () {
  fn toTableFormat(self , rsthis: &mut QTextFormat) -> QTextTableFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toTableFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toTableFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextTableFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextFormat::isFrameFormat();
impl /*struct*/ QTextFormat {
  pub fn isFrameFormat<RetType, T: QTextFormat_isFrameFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isFrameFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isFrameFormat<RetType> {
  fn isFrameFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isFrameFormat();
impl<'a> /*trait*/ QTextFormat_isFrameFormat<i8> for () {
  fn isFrameFormat(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isFrameFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13isFrameFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextFormat::intProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn intProperty<RetType, T: QTextFormat_intProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_intProperty<RetType> {
  fn intProperty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::intProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_intProperty<i32> for (i32) {
  fn intProperty(self , rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11intPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat11intPropertyEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextFormat::toCharFormat();
impl /*struct*/ QTextFormat {
  pub fn toCharFormat<RetType, T: QTextFormat_toCharFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toCharFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toCharFormat<RetType> {
  fn toCharFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QTextCharFormat QTextFormat::toCharFormat();
impl<'a> /*trait*/ QTextFormat_toCharFormat<QTextCharFormat> for () {
  fn toCharFormat(self , rsthis: &mut QTextFormat) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12toCharFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12toCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextFormat::isEmpty();
impl /*struct*/ QTextFormat {
  pub fn isEmpty<RetType, T: QTextFormat_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QTextFormat_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::isEmpty();
impl<'a> /*trait*/ QTextFormat_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QTextFormat7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextTableCellFormat QTextFormat::toTableCellFormat();
impl /*struct*/ QTextFormat {
  pub fn toTableCellFormat<RetType, T: QTextFormat_toTableCellFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toTableCellFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toTableCellFormat<RetType> {
  fn toTableCellFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QTextTableCellFormat QTextFormat::toTableCellFormat();
impl<'a> /*trait*/ QTextFormat_toTableCellFormat<QTextTableCellFormat> for () {
  fn toTableCellFormat(self , rsthis: &mut QTextFormat) -> QTextTableCellFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat17toTableCellFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat17toTableCellFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextTableCellFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextFormat::objectType();
impl /*struct*/ QTextFormat {
  pub fn objectType<RetType, T: QTextFormat_objectType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.objectType(self);
    // return 1;
  }
}

pub trait QTextFormat_objectType<RetType> {
  fn objectType(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::objectType();
impl<'a> /*trait*/ QTextFormat_objectType<i32> for () {
  fn objectType(self , rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10objectTypeEv()};
    let mut ret = unsafe {_ZNK11QTextFormat10objectTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextListFormat QTextFormat::toListFormat();
impl /*struct*/ QTextFormat {
  pub fn toListFormat<RetType, T: QTextFormat_toListFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toListFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toListFormat<RetType> {
  fn toListFormat(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QTextListFormat QTextFormat::toListFormat();
impl<'a> /*trait*/ QTextFormat_toListFormat<QTextListFormat> for () {
  fn toListFormat(self , rsthis: &mut QTextFormat) -> QTextListFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12toListFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12toListFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextListFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QMap<int, QVariant> QTextFormat::properties();
impl /*struct*/ QTextFormat {
  pub fn properties<RetType, T: QTextFormat_properties<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.properties(self);
    // return 1;
  }
}

pub trait QTextFormat_properties<RetType> {
  fn properties(self , rsthis: &mut QTextFormat) -> RetType;
}

  // proto:  QMap<int, QVariant> QTextFormat::properties();
impl<'a> /*trait*/ QTextFormat_properties<()> for () {
  fn properties(self , rsthis: &mut QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10propertiesEv()};
     unsafe {_ZNK11QTextFormat10propertiesEv(rsthis.qclsinst)};
    // return 1;
  }
}

