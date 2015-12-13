// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbrush::QBrush;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QTextBlockFormat QTextFormat::toBlockFormat();
  fn _ZNK11QTextFormat13toBlockFormatEv() -> i32;
  // proto: QString QTextFormat::stringProperty(int propertyId);
  fn _ZNK11QTextFormat14stringPropertyEi(arg0: c_int) -> i32;
  // proto: QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
  fn _ZNK11QTextFormat20lengthVectorPropertyEi(arg0: c_int) -> i32;
  // proto: int QTextFormat::objectIndex();
  fn _ZNK11QTextFormat11objectIndexEv() -> i32;
  // proto: void QTextFormat::setObjectIndex(int object);
  fn _ZN11QTextFormat14setObjectIndexEi(arg0: c_int) -> i32;
  // proto: void QTextFormat::clearForeground();
  fn _ZN11QTextFormat15clearForegroundEv() -> i32;
  // proto: bool QTextFormat::isTableCellFormat();
  fn _ZNK11QTextFormat17isTableCellFormatEv() -> i32;
  // proto: void QTextFormat::FreeQTextFormat();
  fn _ZN11QTextFormatD0Ev() -> i32;
  // proto: bool QTextFormat::isValid();
  fn _ZNK11QTextFormat7isValidEv() -> i32;
  // proto: void QTextFormat::NewQTextFormat(const QTextFormat & rhs);
  fn _ZN11QTextFormatC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QTextLength QTextFormat::lengthProperty(int propertyId);
  fn _ZNK11QTextFormat14lengthPropertyEi(arg0: c_int) -> i32;
  // proto: void QTextFormat::merge(const QTextFormat & other);
  fn _ZN11QTextFormat5mergeERKS_(arg0: *const c_void) -> i32;
  // proto: QColor QTextFormat::colorProperty(int propertyId);
  fn _ZNK11QTextFormat13colorPropertyEi(arg0: c_int) -> i32;
  // proto: void QTextFormat::NewQTextFormat();
  fn _ZN11QTextFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QTextFormat::setForeground(const QBrush & brush);
  fn _ZN11QTextFormat13setForegroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: bool QTextFormat::boolProperty(int propertyId);
  fn _ZNK11QTextFormat12boolPropertyEi(arg0: c_int) -> i32;
  // proto: bool QTextFormat::isListFormat();
  fn _ZNK11QTextFormat12isListFormatEv() -> i32;
  // proto: void QTextFormat::NewQTextFormat(int type);
  fn _ZN11QTextFormatC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: bool QTextFormat::isImageFormat();
  fn _ZNK11QTextFormat13isImageFormatEv() -> i32;
  // proto: void QTextFormat::clearProperty(int propertyId);
  fn _ZN11QTextFormat13clearPropertyEi(arg0: c_int) -> i32;
  // proto: QTextFrameFormat QTextFormat::toFrameFormat();
  fn _ZNK11QTextFormat13toFrameFormatEv() -> i32;
  // proto: QBrush QTextFormat::brushProperty(int propertyId);
  fn _ZNK11QTextFormat13brushPropertyEi(arg0: c_int) -> i32;
  // proto: int QTextFormat::propertyCount();
  fn _ZNK11QTextFormat13propertyCountEv() -> i32;
  // proto: QPen QTextFormat::penProperty(int propertyId);
  fn _ZNK11QTextFormat11penPropertyEi(arg0: c_int) -> i32;
  // proto: QVariant QTextFormat::property(int propertyId);
  fn _ZNK11QTextFormat8propertyEi(arg0: c_int) -> i32;
  // proto: bool QTextFormat::isTableFormat();
  fn _ZNK11QTextFormat13isTableFormatEv() -> i32;
  // proto: void QTextFormat::setProperty(int propertyId, const QVariant & value);
  fn _ZN11QTextFormat11setPropertyEiRK8QVariant(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: int QTextFormat::type_();
  fn _ZNK11QTextFormat4typeEv() -> i32;
  // proto: bool QTextFormat::isCharFormat();
  fn _ZNK11QTextFormat12isCharFormatEv() -> i32;
  // proto: void QTextFormat::clearBackground();
  fn _ZN11QTextFormat15clearBackgroundEv() -> i32;
  // proto: bool QTextFormat::isBlockFormat();
  fn _ZNK11QTextFormat13isBlockFormatEv() -> i32;
  // proto: QBrush QTextFormat::background();
  fn _ZNK11QTextFormat10backgroundEv() -> i32;
  // proto: double QTextFormat::doubleProperty(int propertyId);
  fn _ZNK11QTextFormat14doublePropertyEi(arg0: c_int) -> i32;
  // proto: void QTextFormat::swap(QTextFormat & other);
  fn _ZN11QTextFormat4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QTextImageFormat QTextFormat::toImageFormat();
  fn _ZNK11QTextFormat13toImageFormatEv() -> i32;
  // proto: bool QTextFormat::hasProperty(int propertyId);
  fn _ZNK11QTextFormat11hasPropertyEi(arg0: c_int) -> i32;
  // proto: QBrush QTextFormat::foreground();
  fn _ZNK11QTextFormat10foregroundEv() -> i32;
  // proto: void QTextFormat::setObjectType(int type);
  fn _ZN11QTextFormat13setObjectTypeEi(arg0: c_int) -> i32;
  // proto: void QTextFormat::setBackground(const QBrush & brush);
  fn _ZN11QTextFormat13setBackgroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: QTextTableFormat QTextFormat::toTableFormat();
  fn _ZNK11QTextFormat13toTableFormatEv() -> i32;
  // proto: bool QTextFormat::isFrameFormat();
  fn _ZNK11QTextFormat13isFrameFormatEv() -> i32;
  // proto: int QTextFormat::intProperty(int propertyId);
  fn _ZNK11QTextFormat11intPropertyEi(arg0: c_int) -> i32;
  // proto: QTextCharFormat QTextFormat::toCharFormat();
  fn _ZNK11QTextFormat12toCharFormatEv() -> i32;
  // proto: bool QTextFormat::isEmpty();
  fn _ZNK11QTextFormat7isEmptyEv() -> i32;
  // proto: QTextTableCellFormat QTextFormat::toTableCellFormat();
  fn _ZNK11QTextFormat17toTableCellFormatEv() -> i32;
  // proto: int QTextFormat::objectType();
  fn _ZNK11QTextFormat10objectTypeEv() -> i32;
  // proto: QTextListFormat QTextFormat::toListFormat();
  fn _ZNK11QTextFormat12toListFormatEv() -> i32;
  // proto: QMap<int, QVariant> QTextFormat::properties();
  fn _ZNK11QTextFormat10propertiesEv() -> i32;
}

// body block begin
// class sizeof(QTextFormat)=1
pub struct QTextFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextFormat {
  pub fn toBlockFormat<T: QTextFormat_toBlockFormat>(&mut self, value: T) -> i32 {
    value.toBlockFormat(self);
    return 1;
  }
}

pub trait QTextFormat_toBlockFormat {
  fn toBlockFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: QTextBlockFormat QTextFormat::toBlockFormat();
impl<'a> /*trait*/ QTextFormat_toBlockFormat for () {
  fn toBlockFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toBlockFormatEv()};
    unsafe {_ZNK11QTextFormat13toBlockFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn stringProperty<T: QTextFormat_stringProperty>(&mut self, value: T) -> i32 {
    value.stringProperty(self);
    return 1;
  }
}

pub trait QTextFormat_stringProperty {
  fn stringProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: QString QTextFormat::stringProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_stringProperty for (i32) {
  fn stringProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14stringPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat14stringPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn lengthVectorProperty<T: QTextFormat_lengthVectorProperty>(&mut self, value: T) -> i32 {
    value.lengthVectorProperty(self);
    return 1;
  }
}

pub trait QTextFormat_lengthVectorProperty {
  fn lengthVectorProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_lengthVectorProperty for (i32) {
  fn lengthVectorProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat20lengthVectorPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat20lengthVectorPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn objectIndex<T: QTextFormat_objectIndex>(&mut self, value: T) -> i32 {
    value.objectIndex(self);
    return 1;
  }
}

pub trait QTextFormat_objectIndex {
  fn objectIndex(self, this: &mut QTextFormat) -> i32;
}

// proto: int QTextFormat::objectIndex();
impl<'a> /*trait*/ QTextFormat_objectIndex for () {
  fn objectIndex(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11objectIndexEv()};
    unsafe {_ZNK11QTextFormat11objectIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn setObjectIndex<T: QTextFormat_setObjectIndex>(&mut self, value: T) -> i32 {
    value.setObjectIndex(self);
    return 1;
  }
}

pub trait QTextFormat_setObjectIndex {
  fn setObjectIndex(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::setObjectIndex(int object);
impl<'a> /*trait*/ QTextFormat_setObjectIndex for (i32) {
  fn setObjectIndex(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat14setObjectIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextFormat14setObjectIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn clearForeground<T: QTextFormat_clearForeground>(&mut self, value: T) -> i32 {
    value.clearForeground(self);
    return 1;
  }
}

pub trait QTextFormat_clearForeground {
  fn clearForeground(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::clearForeground();
impl<'a> /*trait*/ QTextFormat_clearForeground for () {
  fn clearForeground(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat15clearForegroundEv()};
    unsafe {_ZN11QTextFormat15clearForegroundEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isTableCellFormat<T: QTextFormat_isTableCellFormat>(&mut self, value: T) -> i32 {
    value.isTableCellFormat(self);
    return 1;
  }
}

pub trait QTextFormat_isTableCellFormat {
  fn isTableCellFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isTableCellFormat();
impl<'a> /*trait*/ QTextFormat_isTableCellFormat for () {
  fn isTableCellFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat17isTableCellFormatEv()};
    unsafe {_ZNK11QTextFormat17isTableCellFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn FreeQTextFormat<T: QTextFormat_FreeQTextFormat>(&mut self, value: T) -> i32 {
    value.FreeQTextFormat(self);
    return 1;
  }
}

pub trait QTextFormat_FreeQTextFormat {
  fn FreeQTextFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::FreeQTextFormat();
impl<'a> /*trait*/ QTextFormat_FreeQTextFormat for () {
  fn FreeQTextFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatD0Ev()};
    unsafe {_ZN11QTextFormatD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isValid<T: QTextFormat_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextFormat_isValid {
  fn isValid(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isValid();
impl<'a> /*trait*/ QTextFormat_isValid for () {
  fn isValid(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat7isValidEv()};
    unsafe {_ZNK11QTextFormat7isValidEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextFormatC1ERKS_(qthis, arg0)};
    let rsthis = QTextFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn lengthProperty<T: QTextFormat_lengthProperty>(&mut self, value: T) -> i32 {
    value.lengthProperty(self);
    return 1;
  }
}

pub trait QTextFormat_lengthProperty {
  fn lengthProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: QTextLength QTextFormat::lengthProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_lengthProperty for (i32) {
  fn lengthProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14lengthPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat14lengthPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn merge<T: QTextFormat_merge>(&mut self, value: T) -> i32 {
    value.merge(self);
    return 1;
  }
}

pub trait QTextFormat_merge {
  fn merge(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::merge(const QTextFormat & other);
impl<'a> /*trait*/ QTextFormat_merge for (&'a  QTextFormat) {
  fn merge(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat5mergeERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextFormat5mergeERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn colorProperty<T: QTextFormat_colorProperty>(&mut self, value: T) -> i32 {
    value.colorProperty(self);
    return 1;
  }
}

pub trait QTextFormat_colorProperty {
  fn colorProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: QColor QTextFormat::colorProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_colorProperty for (i32) {
  fn colorProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13colorPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat13colorPropertyEi(arg0)};
    return 1;
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
  pub fn setForeground<T: QTextFormat_setForeground>(&mut self, value: T) -> i32 {
    value.setForeground(self);
    return 1;
  }
}

pub trait QTextFormat_setForeground {
  fn setForeground(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QTextFormat_setForeground for (&'a  QBrush) {
  fn setForeground(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextFormat13setForegroundERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn boolProperty<T: QTextFormat_boolProperty>(&mut self, value: T) -> i32 {
    value.boolProperty(self);
    return 1;
  }
}

pub trait QTextFormat_boolProperty {
  fn boolProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::boolProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_boolProperty for (i32) {
  fn boolProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12boolPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat12boolPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isListFormat<T: QTextFormat_isListFormat>(&mut self, value: T) -> i32 {
    value.isListFormat(self);
    return 1;
  }
}

pub trait QTextFormat_isListFormat {
  fn isListFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isListFormat();
impl<'a> /*trait*/ QTextFormat_isListFormat for () {
  fn isListFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12isListFormatEv()};
    unsafe {_ZNK11QTextFormat12isListFormatEv()};
    return 1;
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
  pub fn isImageFormat<T: QTextFormat_isImageFormat>(&mut self, value: T) -> i32 {
    value.isImageFormat(self);
    return 1;
  }
}

pub trait QTextFormat_isImageFormat {
  fn isImageFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isImageFormat();
impl<'a> /*trait*/ QTextFormat_isImageFormat for () {
  fn isImageFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isImageFormatEv()};
    unsafe {_ZNK11QTextFormat13isImageFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn clearProperty<T: QTextFormat_clearProperty>(&mut self, value: T) -> i32 {
    value.clearProperty(self);
    return 1;
  }
}

pub trait QTextFormat_clearProperty {
  fn clearProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::clearProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_clearProperty for (i32) {
  fn clearProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13clearPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextFormat13clearPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toFrameFormat<T: QTextFormat_toFrameFormat>(&mut self, value: T) -> i32 {
    value.toFrameFormat(self);
    return 1;
  }
}

pub trait QTextFormat_toFrameFormat {
  fn toFrameFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: QTextFrameFormat QTextFormat::toFrameFormat();
impl<'a> /*trait*/ QTextFormat_toFrameFormat for () {
  fn toFrameFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toFrameFormatEv()};
    unsafe {_ZNK11QTextFormat13toFrameFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn brushProperty<T: QTextFormat_brushProperty>(&mut self, value: T) -> i32 {
    value.brushProperty(self);
    return 1;
  }
}

pub trait QTextFormat_brushProperty {
  fn brushProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: QBrush QTextFormat::brushProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_brushProperty for (i32) {
  fn brushProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13brushPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat13brushPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn propertyCount<T: QTextFormat_propertyCount>(&mut self, value: T) -> i32 {
    value.propertyCount(self);
    return 1;
  }
}

pub trait QTextFormat_propertyCount {
  fn propertyCount(self, this: &mut QTextFormat) -> i32;
}

// proto: int QTextFormat::propertyCount();
impl<'a> /*trait*/ QTextFormat_propertyCount for () {
  fn propertyCount(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13propertyCountEv()};
    unsafe {_ZNK11QTextFormat13propertyCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn penProperty<T: QTextFormat_penProperty>(&mut self, value: T) -> i32 {
    value.penProperty(self);
    return 1;
  }
}

pub trait QTextFormat_penProperty {
  fn penProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: QPen QTextFormat::penProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_penProperty for (i32) {
  fn penProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11penPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat11penPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn property<T: QTextFormat_property>(&mut self, value: T) -> i32 {
    value.property(self);
    return 1;
  }
}

pub trait QTextFormat_property {
  fn property(self, this: &mut QTextFormat) -> i32;
}

// proto: QVariant QTextFormat::property(int propertyId);
impl<'a> /*trait*/ QTextFormat_property for (i32) {
  fn property(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat8propertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat8propertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isTableFormat<T: QTextFormat_isTableFormat>(&mut self, value: T) -> i32 {
    value.isTableFormat(self);
    return 1;
  }
}

pub trait QTextFormat_isTableFormat {
  fn isTableFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isTableFormat();
impl<'a> /*trait*/ QTextFormat_isTableFormat for () {
  fn isTableFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isTableFormatEv()};
    unsafe {_ZNK11QTextFormat13isTableFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn setProperty<T: QTextFormat_setProperty>(&mut self, value: T) -> i32 {
    value.setProperty(self);
    return 1;
  }
}

pub trait QTextFormat_setProperty {
  fn setProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::setProperty(int propertyId, const QVariant & value);
impl<'a> /*trait*/ QTextFormat_setProperty for (i32, &'a  QVariant) {
  fn setProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat11setPropertyEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QTextFormat11setPropertyEiRK8QVariant(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn type_<T: QTextFormat_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QTextFormat_type_ {
  fn type_(self, this: &mut QTextFormat) -> i32;
}

// proto: int QTextFormat::type_();
impl<'a> /*trait*/ QTextFormat_type_ for () {
  fn type_(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat4typeEv()};
    unsafe {_ZNK11QTextFormat4typeEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isCharFormat<T: QTextFormat_isCharFormat>(&mut self, value: T) -> i32 {
    value.isCharFormat(self);
    return 1;
  }
}

pub trait QTextFormat_isCharFormat {
  fn isCharFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isCharFormat();
impl<'a> /*trait*/ QTextFormat_isCharFormat for () {
  fn isCharFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12isCharFormatEv()};
    unsafe {_ZNK11QTextFormat12isCharFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn clearBackground<T: QTextFormat_clearBackground>(&mut self, value: T) -> i32 {
    value.clearBackground(self);
    return 1;
  }
}

pub trait QTextFormat_clearBackground {
  fn clearBackground(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::clearBackground();
impl<'a> /*trait*/ QTextFormat_clearBackground for () {
  fn clearBackground(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat15clearBackgroundEv()};
    unsafe {_ZN11QTextFormat15clearBackgroundEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isBlockFormat<T: QTextFormat_isBlockFormat>(&mut self, value: T) -> i32 {
    value.isBlockFormat(self);
    return 1;
  }
}

pub trait QTextFormat_isBlockFormat {
  fn isBlockFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isBlockFormat();
impl<'a> /*trait*/ QTextFormat_isBlockFormat for () {
  fn isBlockFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isBlockFormatEv()};
    unsafe {_ZNK11QTextFormat13isBlockFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn background<T: QTextFormat_background>(&mut self, value: T) -> i32 {
    value.background(self);
    return 1;
  }
}

pub trait QTextFormat_background {
  fn background(self, this: &mut QTextFormat) -> i32;
}

// proto: QBrush QTextFormat::background();
impl<'a> /*trait*/ QTextFormat_background for () {
  fn background(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10backgroundEv()};
    unsafe {_ZNK11QTextFormat10backgroundEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn doubleProperty<T: QTextFormat_doubleProperty>(&mut self, value: T) -> i32 {
    value.doubleProperty(self);
    return 1;
  }
}

pub trait QTextFormat_doubleProperty {
  fn doubleProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: double QTextFormat::doubleProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_doubleProperty for (i32) {
  fn doubleProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14doublePropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat14doublePropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn swap<T: QTextFormat_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QTextFormat_swap {
  fn swap(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::swap(QTextFormat & other);
impl<'a> /*trait*/ QTextFormat_swap for (&'a mut QTextFormat) {
  fn swap(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextFormat4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toImageFormat<T: QTextFormat_toImageFormat>(&mut self, value: T) -> i32 {
    value.toImageFormat(self);
    return 1;
  }
}

pub trait QTextFormat_toImageFormat {
  fn toImageFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: QTextImageFormat QTextFormat::toImageFormat();
impl<'a> /*trait*/ QTextFormat_toImageFormat for () {
  fn toImageFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toImageFormatEv()};
    unsafe {_ZNK11QTextFormat13toImageFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn hasProperty<T: QTextFormat_hasProperty>(&mut self, value: T) -> i32 {
    value.hasProperty(self);
    return 1;
  }
}

pub trait QTextFormat_hasProperty {
  fn hasProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::hasProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_hasProperty for (i32) {
  fn hasProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11hasPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat11hasPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn foreground<T: QTextFormat_foreground>(&mut self, value: T) -> i32 {
    value.foreground(self);
    return 1;
  }
}

pub trait QTextFormat_foreground {
  fn foreground(self, this: &mut QTextFormat) -> i32;
}

// proto: QBrush QTextFormat::foreground();
impl<'a> /*trait*/ QTextFormat_foreground for () {
  fn foreground(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10foregroundEv()};
    unsafe {_ZNK11QTextFormat10foregroundEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn setObjectType<T: QTextFormat_setObjectType>(&mut self, value: T) -> i32 {
    value.setObjectType(self);
    return 1;
  }
}

pub trait QTextFormat_setObjectType {
  fn setObjectType(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::setObjectType(int type);
impl<'a> /*trait*/ QTextFormat_setObjectType for (i32) {
  fn setObjectType(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setObjectTypeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextFormat13setObjectTypeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn setBackground<T: QTextFormat_setBackground>(&mut self, value: T) -> i32 {
    value.setBackground(self);
    return 1;
  }
}

pub trait QTextFormat_setBackground {
  fn setBackground(self, this: &mut QTextFormat) -> i32;
}

// proto: void QTextFormat::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QTextFormat_setBackground for (&'a  QBrush) {
  fn setBackground(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextFormat13setBackgroundERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toTableFormat<T: QTextFormat_toTableFormat>(&mut self, value: T) -> i32 {
    value.toTableFormat(self);
    return 1;
  }
}

pub trait QTextFormat_toTableFormat {
  fn toTableFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: QTextTableFormat QTextFormat::toTableFormat();
impl<'a> /*trait*/ QTextFormat_toTableFormat for () {
  fn toTableFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toTableFormatEv()};
    unsafe {_ZNK11QTextFormat13toTableFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isFrameFormat<T: QTextFormat_isFrameFormat>(&mut self, value: T) -> i32 {
    value.isFrameFormat(self);
    return 1;
  }
}

pub trait QTextFormat_isFrameFormat {
  fn isFrameFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isFrameFormat();
impl<'a> /*trait*/ QTextFormat_isFrameFormat for () {
  fn isFrameFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13isFrameFormatEv()};
    unsafe {_ZNK11QTextFormat13isFrameFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn intProperty<T: QTextFormat_intProperty>(&mut self, value: T) -> i32 {
    value.intProperty(self);
    return 1;
  }
}

pub trait QTextFormat_intProperty {
  fn intProperty(self, this: &mut QTextFormat) -> i32;
}

// proto: int QTextFormat::intProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_intProperty for (i32) {
  fn intProperty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11intPropertyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextFormat11intPropertyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toCharFormat<T: QTextFormat_toCharFormat>(&mut self, value: T) -> i32 {
    value.toCharFormat(self);
    return 1;
  }
}

pub trait QTextFormat_toCharFormat {
  fn toCharFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: QTextCharFormat QTextFormat::toCharFormat();
impl<'a> /*trait*/ QTextFormat_toCharFormat for () {
  fn toCharFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12toCharFormatEv()};
    unsafe {_ZNK11QTextFormat12toCharFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn isEmpty<T: QTextFormat_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QTextFormat_isEmpty {
  fn isEmpty(self, this: &mut QTextFormat) -> i32;
}

// proto: bool QTextFormat::isEmpty();
impl<'a> /*trait*/ QTextFormat_isEmpty for () {
  fn isEmpty(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat7isEmptyEv()};
    unsafe {_ZNK11QTextFormat7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toTableCellFormat<T: QTextFormat_toTableCellFormat>(&mut self, value: T) -> i32 {
    value.toTableCellFormat(self);
    return 1;
  }
}

pub trait QTextFormat_toTableCellFormat {
  fn toTableCellFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: QTextTableCellFormat QTextFormat::toTableCellFormat();
impl<'a> /*trait*/ QTextFormat_toTableCellFormat for () {
  fn toTableCellFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat17toTableCellFormatEv()};
    unsafe {_ZNK11QTextFormat17toTableCellFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn objectType<T: QTextFormat_objectType>(&mut self, value: T) -> i32 {
    value.objectType(self);
    return 1;
  }
}

pub trait QTextFormat_objectType {
  fn objectType(self, this: &mut QTextFormat) -> i32;
}

// proto: int QTextFormat::objectType();
impl<'a> /*trait*/ QTextFormat_objectType for () {
  fn objectType(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10objectTypeEv()};
    unsafe {_ZNK11QTextFormat10objectTypeEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn toListFormat<T: QTextFormat_toListFormat>(&mut self, value: T) -> i32 {
    value.toListFormat(self);
    return 1;
  }
}

pub trait QTextFormat_toListFormat {
  fn toListFormat(self, this: &mut QTextFormat) -> i32;
}

// proto: QTextListFormat QTextFormat::toListFormat();
impl<'a> /*trait*/ QTextFormat_toListFormat for () {
  fn toListFormat(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12toListFormatEv()};
    unsafe {_ZNK11QTextFormat12toListFormatEv()};
    return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn properties<T: QTextFormat_properties>(&mut self, value: T) -> i32 {
    value.properties(self);
    return 1;
  }
}

pub trait QTextFormat_properties {
  fn properties(self, this: &mut QTextFormat) -> i32;
}

// proto: QMap<int, QVariant> QTextFormat::properties();
impl<'a> /*trait*/ QTextFormat_properties for () {
  fn properties(self, this: &mut QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10propertiesEv()};
    unsafe {_ZNK11QTextFormat10propertiesEv()};
    return 1;
  }
}

