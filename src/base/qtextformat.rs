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
  fn _ZNK11QTextFormat20lengthVectorPropertyEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTextFormat::objectIndex();
  fn _ZNK11QTextFormat11objectIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextFormat::setObjectIndex(int object);
  fn _ZN11QTextFormat14setObjectIndexEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextFormat::clearForeground();
  fn _ZN11QTextFormat15clearForegroundEv(qthis: *mut c_void) ;
  // proto:  bool QTextFormat::isTableCellFormat();
  fn _ZNK11QTextFormat17isTableCellFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextFormat::FreeQTextFormat();
  fn _ZN11QTextFormatD0Ev(qthis: *mut c_void) ;
  // proto:  bool QTextFormat::isValid();
  fn _ZNK11QTextFormat7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextFormat::NewQTextFormat(const QTextFormat & rhs);
  fn _ZN11QTextFormatC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextLength QTextFormat::lengthProperty(int propertyId);
  fn _ZNK11QTextFormat14lengthPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextFormat::merge(const QTextFormat & other);
  fn _ZN11QTextFormat5mergeERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QTextFormat::colorProperty(int propertyId);
  fn _ZNK11QTextFormat13colorPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextFormat::NewQTextFormat();
  fn _ZN11QTextFormatC1Ev(qthis: *mut c_void) ;
  // proto:  void QTextFormat::setForeground(const QBrush & brush);
  fn _ZN11QTextFormat13setForegroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextFormat::boolProperty(int propertyId);
  fn _ZNK11QTextFormat12boolPropertyEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  bool QTextFormat::isListFormat();
  fn _ZNK11QTextFormat12isListFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextFormat::NewQTextFormat(int type);
  fn _ZN11QTextFormatC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QTextFormat::isImageFormat();
  fn _ZNK11QTextFormat13isImageFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextFormat::clearProperty(int propertyId);
  fn _ZN11QTextFormat13clearPropertyEi(qthis: *mut c_void, arg0: c_int) ;
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
  fn _ZNK11QTextFormat13isTableFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextFormat::setProperty(int propertyId, const QVariant & value);
  fn _ZN11QTextFormat11setPropertyEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  int QTextFormat::type_();
  fn _ZNK11QTextFormat4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextFormat::isCharFormat();
  fn _ZNK11QTextFormat12isCharFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextFormat::clearBackground();
  fn _ZN11QTextFormat15clearBackgroundEv(qthis: *mut c_void) ;
  // proto:  bool QTextFormat::isBlockFormat();
  fn _ZNK11QTextFormat13isBlockFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  QBrush QTextFormat::background();
  fn _ZNK11QTextFormat10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QTextFormat::doubleProperty(int propertyId);
  fn _ZNK11QTextFormat14doublePropertyEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QTextFormat::swap(QTextFormat & other);
  fn _ZN11QTextFormat4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextImageFormat QTextFormat::toImageFormat();
  fn _ZNK11QTextFormat13toImageFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextFormat::hasProperty(int propertyId);
  fn _ZNK11QTextFormat11hasPropertyEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  QBrush QTextFormat::foreground();
  fn _ZNK11QTextFormat10foregroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFormat::setObjectType(int type);
  fn _ZN11QTextFormat13setObjectTypeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextFormat::setBackground(const QBrush & brush);
  fn _ZN11QTextFormat13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextTableFormat QTextFormat::toTableFormat();
  fn _ZNK11QTextFormat13toTableFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextFormat::isFrameFormat();
  fn _ZNK11QTextFormat13isFrameFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTextFormat::intProperty(int propertyId);
  fn _ZNK11QTextFormat11intPropertyEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QTextCharFormat QTextFormat::toCharFormat();
  fn _ZNK11QTextFormat12toCharFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextFormat::isEmpty();
  fn _ZNK11QTextFormat7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  QTextTableCellFormat QTextFormat::toTableCellFormat();
  fn _ZNK11QTextFormat17toTableCellFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextFormat::objectType();
  fn _ZNK11QTextFormat10objectTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextListFormat QTextFormat::toListFormat();
  fn _ZNK11QTextFormat12toListFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMap<int, QVariant> QTextFormat::properties();
  fn _ZNK11QTextFormat10propertiesEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTextFormat)=1
pub struct QTextFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextFormat {
  pub fn toBlockFormat<T: QTextFormat_toBlockFormat>(&mut self, value: T) -> QTextBlockFormat {
    return value.toBlockFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toBlockFormat {
  fn toBlockFormat(self, rsthis: &mut QTextFormat) -> QTextBlockFormat;
}

// proto:  QTextBlockFormat QTextFormat::toBlockFormat();
impl<'a> /*trait*/ QTextFormat_toBlockFormat for () {
  fn toBlockFormat(self, rsthis: &mut QTextFormat) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toBlockFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toBlockFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn stringProperty<T: QTextFormat_stringProperty>(&mut self, value: T) -> QString {
    return value.stringProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_stringProperty {
  fn stringProperty(self, rsthis: &mut QTextFormat) -> QString;
}

// proto:  QString QTextFormat::stringProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_stringProperty for (i32) {
  fn stringProperty(self, rsthis: &mut QTextFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14stringPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14stringPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn lengthVectorProperty<T: QTextFormat_lengthVectorProperty>(&mut self, value: T)  {
     value.lengthVectorProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_lengthVectorProperty {
  fn lengthVectorProperty(self, rsthis: &mut QTextFormat) ;
}

// proto:  QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_lengthVectorProperty for (i32) {
  fn lengthVectorProperty(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat20lengthVectorPropertyEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK11QTextFormat20lengthVectorPropertyEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn objectIndex<T: QTextFormat_objectIndex>(&mut self, value: T) -> i32 {
    return value.objectIndex(self);
    // return 1;
  }
}

pub trait QTextFormat_objectIndex {
  fn objectIndex(self, rsthis: &mut QTextFormat) -> i32;
}

// proto:  int QTextFormat::objectIndex();
impl<'a> /*trait*/ QTextFormat_objectIndex for () {
  fn objectIndex(self, rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11objectIndexEv()};
    let mut ret = unsafe {_ZNK11QTextFormat11objectIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn setObjectIndex<T: QTextFormat_setObjectIndex>(&mut self, value: T)  {
     value.setObjectIndex(self);
    // return 1;
  }
}

pub trait QTextFormat_setObjectIndex {
  fn setObjectIndex(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::setObjectIndex(int object);
impl<'a> /*trait*/ QTextFormat_setObjectIndex for (i32) {
  fn setObjectIndex(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat14setObjectIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat14setObjectIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn clearForeground<T: QTextFormat_clearForeground>(&mut self, value: T)  {
     value.clearForeground(self);
    // return 1;
  }
}

pub trait QTextFormat_clearForeground {
  fn clearForeground(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::clearForeground();
impl<'a> /*trait*/ QTextFormat_clearForeground for () {
  fn clearForeground(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat15clearForegroundEv()};
     unsafe {_ZN11QTextFormat15clearForegroundEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isTableCellFormat<T: QTextFormat_isTableCellFormat>(&mut self, value: T) -> i8 {
    return value.isTableCellFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isTableCellFormat {
  fn isTableCellFormat(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isTableCellFormat();
impl<'a> /*trait*/ QTextFormat_isTableCellFormat for () {
  fn isTableCellFormat(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat17isTableCellFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat17isTableCellFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn FreeQTextFormat<T: QTextFormat_FreeQTextFormat>(&mut self, value: T)  {
     value.FreeQTextFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_FreeQTextFormat {
  fn FreeQTextFormat(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::FreeQTextFormat();
impl<'a> /*trait*/ QTextFormat_FreeQTextFormat for () {
  fn FreeQTextFormat(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatD0Ev()};
     unsafe {_ZN11QTextFormatD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isValid<T: QTextFormat_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextFormat_isValid {
  fn isValid(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isValid();
impl<'a> /*trait*/ QTextFormat_isValid for () {
  fn isValid(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat7isValidEv()};
    let mut ret = unsafe {_ZNK11QTextFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

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

// proto: void QTextFormat::NewQTextFormat(const QTextFormat & rhs);
impl<'a> /*trait*/ QTextFormat_NewQTextFormat for (&'a  QTextFormat) {
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

impl /*struct*/ QTextFormat {
  pub fn lengthProperty<T: QTextFormat_lengthProperty>(&mut self, value: T) -> QTextLength {
    return value.lengthProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_lengthProperty {
  fn lengthProperty(self, rsthis: &mut QTextFormat) -> QTextLength;
}

// proto:  QTextLength QTextFormat::lengthProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_lengthProperty for (i32) {
  fn lengthProperty(self, rsthis: &mut QTextFormat) -> QTextLength {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14lengthPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14lengthPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLength{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn merge<T: QTextFormat_merge>(&mut self, value: T)  {
     value.merge(self);
    // return 1;
  }
}

pub trait QTextFormat_merge {
  fn merge(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::merge(const QTextFormat & other);
impl<'a> /*trait*/ QTextFormat_merge for (&'a  QTextFormat) {
  fn merge(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat5mergeERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat5mergeERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn colorProperty<T: QTextFormat_colorProperty>(&mut self, value: T) -> QColor {
    return value.colorProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_colorProperty {
  fn colorProperty(self, rsthis: &mut QTextFormat) -> QColor;
}

// proto:  QColor QTextFormat::colorProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_colorProperty for (i32) {
  fn colorProperty(self, rsthis: &mut QTextFormat) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13colorPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat13colorPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextFormat::NewQTextFormat();
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

impl /*struct*/ QTextFormat {
  pub fn setForeground<T: QTextFormat_setForeground>(&mut self, value: T)  {
     value.setForeground(self);
    // return 1;
  }
}

pub trait QTextFormat_setForeground {
  fn setForeground(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QTextFormat_setForeground for (&'a  QBrush) {
  fn setForeground(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn boolProperty<T: QTextFormat_boolProperty>(&mut self, value: T) -> i8 {
    return value.boolProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_boolProperty {
  fn boolProperty(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::boolProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_boolProperty for (i32) {
  fn boolProperty(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12boolPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat12boolPropertyEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isListFormat<T: QTextFormat_isListFormat>(&mut self, value: T) -> i8 {
    return value.isListFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isListFormat {
  fn isListFormat(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isListFormat();
impl<'a> /*trait*/ QTextFormat_isListFormat for () {
  fn isListFormat(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12isListFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12isListFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QTextFormat::NewQTextFormat(int type);
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

impl /*struct*/ QTextFormat {
  pub fn isImageFormat<T: QTextFormat_isImageFormat>(&mut self, value: T) -> i8 {
    return value.isImageFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isImageFormat {
  fn isImageFormat(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isImageFormat();
impl<'a> /*trait*/ QTextFormat_isImageFormat for () {
  fn isImageFormat(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isImageFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13isImageFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn clearProperty<T: QTextFormat_clearProperty>(&mut self, value: T)  {
     value.clearProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_clearProperty {
  fn clearProperty(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::clearProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_clearProperty for (i32) {
  fn clearProperty(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13clearPropertyEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat13clearPropertyEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toFrameFormat<T: QTextFormat_toFrameFormat>(&mut self, value: T) -> QTextFrameFormat {
    return value.toFrameFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toFrameFormat {
  fn toFrameFormat(self, rsthis: &mut QTextFormat) -> QTextFrameFormat;
}

// proto:  QTextFrameFormat QTextFormat::toFrameFormat();
impl<'a> /*trait*/ QTextFormat_toFrameFormat for () {
  fn toFrameFormat(self, rsthis: &mut QTextFormat) -> QTextFrameFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toFrameFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toFrameFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrameFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn brushProperty<T: QTextFormat_brushProperty>(&mut self, value: T) -> QBrush {
    return value.brushProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_brushProperty {
  fn brushProperty(self, rsthis: &mut QTextFormat) -> QBrush;
}

// proto:  QBrush QTextFormat::brushProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_brushProperty for (i32) {
  fn brushProperty(self, rsthis: &mut QTextFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13brushPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat13brushPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn propertyCount<T: QTextFormat_propertyCount>(&mut self, value: T) -> i32 {
    return value.propertyCount(self);
    // return 1;
  }
}

pub trait QTextFormat_propertyCount {
  fn propertyCount(self, rsthis: &mut QTextFormat) -> i32;
}

// proto:  int QTextFormat::propertyCount();
impl<'a> /*trait*/ QTextFormat_propertyCount for () {
  fn propertyCount(self, rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13propertyCountEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13propertyCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn penProperty<T: QTextFormat_penProperty>(&mut self, value: T) -> QPen {
    return value.penProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_penProperty {
  fn penProperty(self, rsthis: &mut QTextFormat) -> QPen;
}

// proto:  QPen QTextFormat::penProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_penProperty for (i32) {
  fn penProperty(self, rsthis: &mut QTextFormat) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11penPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat11penPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QPen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn property<T: QTextFormat_property>(&mut self, value: T) -> QVariant {
    return value.property(self);
    // return 1;
  }
}

pub trait QTextFormat_property {
  fn property(self, rsthis: &mut QTextFormat) -> QVariant;
}

// proto:  QVariant QTextFormat::property(int propertyId);
impl<'a> /*trait*/ QTextFormat_property for (i32) {
  fn property(self, rsthis: &mut QTextFormat) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat8propertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat8propertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isTableFormat<T: QTextFormat_isTableFormat>(&mut self, value: T) -> i8 {
    return value.isTableFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isTableFormat {
  fn isTableFormat(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isTableFormat();
impl<'a> /*trait*/ QTextFormat_isTableFormat for () {
  fn isTableFormat(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isTableFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13isTableFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn setProperty<T: QTextFormat_setProperty>(&mut self, value: T)  {
     value.setProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_setProperty {
  fn setProperty(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::setProperty(int propertyId, const QVariant & value);
impl<'a> /*trait*/ QTextFormat_setProperty for (i32, &'a  QVariant) {
  fn setProperty(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat11setPropertyEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat11setPropertyEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn type_<T: QTextFormat_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QTextFormat_type_ {
  fn type_(self, rsthis: &mut QTextFormat) -> i32;
}

// proto:  int QTextFormat::type_();
impl<'a> /*trait*/ QTextFormat_type_ for () {
  fn type_(self, rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat4typeEv()};
    let mut ret = unsafe {_ZNK11QTextFormat4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isCharFormat<T: QTextFormat_isCharFormat>(&mut self, value: T) -> i8 {
    return value.isCharFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isCharFormat {
  fn isCharFormat(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isCharFormat();
impl<'a> /*trait*/ QTextFormat_isCharFormat for () {
  fn isCharFormat(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12isCharFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12isCharFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn clearBackground<T: QTextFormat_clearBackground>(&mut self, value: T)  {
     value.clearBackground(self);
    // return 1;
  }
}

pub trait QTextFormat_clearBackground {
  fn clearBackground(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::clearBackground();
impl<'a> /*trait*/ QTextFormat_clearBackground for () {
  fn clearBackground(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat15clearBackgroundEv()};
     unsafe {_ZN11QTextFormat15clearBackgroundEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isBlockFormat<T: QTextFormat_isBlockFormat>(&mut self, value: T) -> i8 {
    return value.isBlockFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isBlockFormat {
  fn isBlockFormat(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isBlockFormat();
impl<'a> /*trait*/ QTextFormat_isBlockFormat for () {
  fn isBlockFormat(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isBlockFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13isBlockFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn background<T: QTextFormat_background>(&mut self, value: T) -> QBrush {
    return value.background(self);
    // return 1;
  }
}

pub trait QTextFormat_background {
  fn background(self, rsthis: &mut QTextFormat) -> QBrush;
}

// proto:  QBrush QTextFormat::background();
impl<'a> /*trait*/ QTextFormat_background for () {
  fn background(self, rsthis: &mut QTextFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10backgroundEv()};
    let mut ret = unsafe {_ZNK11QTextFormat10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn doubleProperty<T: QTextFormat_doubleProperty>(&mut self, value: T) -> f64 {
    return value.doubleProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_doubleProperty {
  fn doubleProperty(self, rsthis: &mut QTextFormat) -> f64;
}

// proto:  double QTextFormat::doubleProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_doubleProperty for (i32) {
  fn doubleProperty(self, rsthis: &mut QTextFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14doublePropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14doublePropertyEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn swap<T: QTextFormat_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QTextFormat_swap {
  fn swap(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::swap(QTextFormat & other);
impl<'a> /*trait*/ QTextFormat_swap for (&'a mut QTextFormat) {
  fn swap(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toImageFormat<T: QTextFormat_toImageFormat>(&mut self, value: T) -> QTextImageFormat {
    return value.toImageFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toImageFormat {
  fn toImageFormat(self, rsthis: &mut QTextFormat) -> QTextImageFormat;
}

// proto:  QTextImageFormat QTextFormat::toImageFormat();
impl<'a> /*trait*/ QTextFormat_toImageFormat for () {
  fn toImageFormat(self, rsthis: &mut QTextFormat) -> QTextImageFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toImageFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toImageFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextImageFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn hasProperty<T: QTextFormat_hasProperty>(&mut self, value: T) -> i8 {
    return value.hasProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_hasProperty {
  fn hasProperty(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::hasProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_hasProperty for (i32) {
  fn hasProperty(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11hasPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat11hasPropertyEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn foreground<T: QTextFormat_foreground>(&mut self, value: T) -> QBrush {
    return value.foreground(self);
    // return 1;
  }
}

pub trait QTextFormat_foreground {
  fn foreground(self, rsthis: &mut QTextFormat) -> QBrush;
}

// proto:  QBrush QTextFormat::foreground();
impl<'a> /*trait*/ QTextFormat_foreground for () {
  fn foreground(self, rsthis: &mut QTextFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10foregroundEv()};
    let mut ret = unsafe {_ZNK11QTextFormat10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn setObjectType<T: QTextFormat_setObjectType>(&mut self, value: T)  {
     value.setObjectType(self);
    // return 1;
  }
}

pub trait QTextFormat_setObjectType {
  fn setObjectType(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::setObjectType(int type);
impl<'a> /*trait*/ QTextFormat_setObjectType for (i32) {
  fn setObjectType(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setObjectTypeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat13setObjectTypeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn setBackground<T: QTextFormat_setBackground>(&mut self, value: T)  {
     value.setBackground(self);
    // return 1;
  }
}

pub trait QTextFormat_setBackground {
  fn setBackground(self, rsthis: &mut QTextFormat) ;
}

// proto:  void QTextFormat::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QTextFormat_setBackground for (&'a  QBrush) {
  fn setBackground(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toTableFormat<T: QTextFormat_toTableFormat>(&mut self, value: T) -> QTextTableFormat {
    return value.toTableFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toTableFormat {
  fn toTableFormat(self, rsthis: &mut QTextFormat) -> QTextTableFormat;
}

// proto:  QTextTableFormat QTextFormat::toTableFormat();
impl<'a> /*trait*/ QTextFormat_toTableFormat for () {
  fn toTableFormat(self, rsthis: &mut QTextFormat) -> QTextTableFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toTableFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toTableFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextTableFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isFrameFormat<T: QTextFormat_isFrameFormat>(&mut self, value: T) -> i8 {
    return value.isFrameFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_isFrameFormat {
  fn isFrameFormat(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isFrameFormat();
impl<'a> /*trait*/ QTextFormat_isFrameFormat for () {
  fn isFrameFormat(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isFrameFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13isFrameFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn intProperty<T: QTextFormat_intProperty>(&mut self, value: T) -> i32 {
    return value.intProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_intProperty {
  fn intProperty(self, rsthis: &mut QTextFormat) -> i32;
}

// proto:  int QTextFormat::intProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_intProperty for (i32) {
  fn intProperty(self, rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11intPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat11intPropertyEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toCharFormat<T: QTextFormat_toCharFormat>(&mut self, value: T) -> QTextCharFormat {
    return value.toCharFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toCharFormat {
  fn toCharFormat(self, rsthis: &mut QTextFormat) -> QTextCharFormat;
}

// proto:  QTextCharFormat QTextFormat::toCharFormat();
impl<'a> /*trait*/ QTextFormat_toCharFormat for () {
  fn toCharFormat(self, rsthis: &mut QTextFormat) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12toCharFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12toCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isEmpty<T: QTextFormat_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QTextFormat_isEmpty {
  fn isEmpty(self, rsthis: &mut QTextFormat) -> i8;
}

// proto:  bool QTextFormat::isEmpty();
impl<'a> /*trait*/ QTextFormat_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QTextFormat7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toTableCellFormat<T: QTextFormat_toTableCellFormat>(&mut self, value: T) -> QTextTableCellFormat {
    return value.toTableCellFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toTableCellFormat {
  fn toTableCellFormat(self, rsthis: &mut QTextFormat) -> QTextTableCellFormat;
}

// proto:  QTextTableCellFormat QTextFormat::toTableCellFormat();
impl<'a> /*trait*/ QTextFormat_toTableCellFormat for () {
  fn toTableCellFormat(self, rsthis: &mut QTextFormat) -> QTextTableCellFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat17toTableCellFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat17toTableCellFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextTableCellFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn objectType<T: QTextFormat_objectType>(&mut self, value: T) -> i32 {
    return value.objectType(self);
    // return 1;
  }
}

pub trait QTextFormat_objectType {
  fn objectType(self, rsthis: &mut QTextFormat) -> i32;
}

// proto:  int QTextFormat::objectType();
impl<'a> /*trait*/ QTextFormat_objectType for () {
  fn objectType(self, rsthis: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10objectTypeEv()};
    let mut ret = unsafe {_ZNK11QTextFormat10objectTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toListFormat<T: QTextFormat_toListFormat>(&mut self, value: T) -> QTextListFormat {
    return value.toListFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toListFormat {
  fn toListFormat(self, rsthis: &mut QTextFormat) -> QTextListFormat;
}

// proto:  QTextListFormat QTextFormat::toListFormat();
impl<'a> /*trait*/ QTextFormat_toListFormat for () {
  fn toListFormat(self, rsthis: &mut QTextFormat) -> QTextListFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12toListFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12toListFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextListFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn properties<T: QTextFormat_properties>(&mut self, value: T)  {
     value.properties(self);
    // return 1;
  }
}

pub trait QTextFormat_properties {
  fn properties(self, rsthis: &mut QTextFormat) ;
}

// proto:  QMap<int, QVariant> QTextFormat::properties();
impl<'a> /*trait*/ QTextFormat_properties for () {
  fn properties(self, rsthis: &mut QTextFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10propertiesEv()};
     unsafe {_ZNK11QTextFormat10propertiesEv(rsthis.qclsinst)};
    // return 1;
  }
}

