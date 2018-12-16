

// mod ::gui::QTextBlock
// package qtgui
// /usr/include/qt/QtGui/qtextobject.h
// #include <qtextobject.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTextBlock)=16
pub struct QTextBlock {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextBlock_ITF interface {
//    QTextBlock_PTR() *QTextBlock
//}
//func (ptr *QTextBlock) QTextBlock_PTR() *QTextBlock { return ptr }

impl /*struct*/ QTextBlock {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextBlock {
    return QTextBlock{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextBlock {
//  type Target = QTextBlockBASE;
//
//  fn deref(&self) -> &QTextBlockBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextBlockBASE> for QTextBlock {
//  fn as_ref(& self) -> & QTextBlockBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextobject.h:206
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTextBlock()

/*

*/
// QTextBlock() ctx.fn_proto_cpp
impl /*struct*/ QTextBlock {
  pub fn QTextBlock_0<T: QTextBlock_QTextBlock_0>(value: T) -> QTextBlock {
    let rsthis = value.QTextBlock_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlock_QTextBlock_0 {
  fn QTextBlock_0(self) -> QTextBlock;
}
// QTextBlock() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextBlock_QTextBlock_0 for () {
  fn QTextBlock_0(self) -> QTextBlock {
    // unsafe{_ZN10QTextBlockC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTextBlockC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextBlock{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:208
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QTextBlock & operator=(const QTextBlock &)

/*

*/
impl /*struct*/ QTextBlock {
  pub fn operator_equal_0<RetType, T: QTextBlock_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTextBlock_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTextBlockaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:210
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn isValid_0<RetType, T: QTextBlock_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextBlock_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextBlock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:212
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QTextBlock &) const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn operator_equal_equal_0<RetType, T: QTextBlock_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTextBlock_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTextBlock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlockeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:213
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QTextBlock &) const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn operator_not_equal_0<RetType, T: QTextBlock_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTextBlock_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTextBlock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlockneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:214
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QTextBlock &) const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn operator_less_than_0<RetType, T: QTextBlock_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QTextBlock_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QTextBlock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlockltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:216
// index:0
// Public Visibility=Default Availability=Available
// [4] int position() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn position_0<RetType, T: QTextBlock_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTextBlock_position_0<RetType> {
  fn position_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_position_0<i32> for () {
  fn position_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:217
// index:0
// Public Visibility=Default Availability=Available
// [4] int length() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn length_0<RetType, T: QTextBlock_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QTextBlock_length_0<RetType> {
  fn length_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_length_0<i32> for () {
  fn length_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:218
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(int) const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn contains_0<RetType, T: QTextBlock_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QTextBlock_contains_0<RetType> {
  fn contains_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_contains_0<bool> for (i32) {
  fn contains_0(self , rsthis: & QTextBlock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock8containsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:220
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextLayout * layout() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn layout_0<RetType, T: QTextBlock_layout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layout_0(self);
    // return 1;
  }
}
pub trait QTextBlock_layout_0<RetType> {
  fn layout_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_layout_0<usize> for () {
  fn layout_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock6layoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearLayout()

/*

*/
impl /*struct*/ QTextBlock {
  pub fn clearLayout_0<RetType, T: QTextBlock_clearLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearLayout_0(self);
    // return 1;
  }
}
pub trait QTextBlock_clearLayout_0<RetType> {
  fn clearLayout_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_clearLayout_0<(/*void*/)> for () {
  fn clearLayout_0(self , rsthis: & QTextBlock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QTextBlock11clearLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:222
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlockFormat blockFormat() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn blockFormat_0<RetType, T: QTextBlock_blockFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockFormat_0(self);
    // return 1;
  }
}
pub trait QTextBlock_blockFormat_0<RetType> {
  fn blockFormat_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_blockFormat_0<usize> for () {
  fn blockFormat_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock11blockFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:223
// index:0
// Public Visibility=Default Availability=Available
// [4] int blockFormatIndex() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn blockFormatIndex_0<RetType, T: QTextBlock_blockFormatIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockFormatIndex_0(self);
    // return 1;
  }
}
pub trait QTextBlock_blockFormatIndex_0<RetType> {
  fn blockFormatIndex_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_blockFormatIndex_0<i32> for () {
  fn blockFormatIndex_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock16blockFormatIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:224
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat charFormat() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn charFormat_0<RetType, T: QTextBlock_charFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.charFormat_0(self);
    // return 1;
  }
}
pub trait QTextBlock_charFormat_0<RetType> {
  fn charFormat_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_charFormat_0<usize> for () {
  fn charFormat_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock10charFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:225
// index:0
// Public Visibility=Default Availability=Available
// [4] int charFormatIndex() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn charFormatIndex_0<RetType, T: QTextBlock_charFormatIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.charFormatIndex_0(self);
    // return 1;
  }
}
pub trait QTextBlock_charFormatIndex_0<RetType> {
  fn charFormatIndex_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_charFormatIndex_0<i32> for () {
  fn charFormatIndex_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock15charFormatIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:227
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::LayoutDirection textDirection() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn textDirection_0<RetType, T: QTextBlock_textDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textDirection_0(self);
    // return 1;
  }
}
pub trait QTextBlock_textDirection_0<RetType> {
  fn textDirection_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_textDirection_0<i32> for () {
  fn textDirection_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock13textDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:229
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn text_0<RetType, T: QTextBlock_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QTextBlock_text_0<RetType> {
  fn text_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_text_0<usize> for () {
  fn text_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:233
// index:0
// Public Visibility=Default Availability=Available
// [8] const QTextDocument * document() const

/*
Returns the document this object belongs to.

See also format().
*/
impl /*struct*/ QTextBlock {
  pub fn document_0<RetType, T: QTextBlock_document_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.document_0(self);
    // return 1;
  }
}
pub trait QTextBlock_document_0<RetType> {
  fn document_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_document_0<usize> for () {
  fn document_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock8documentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:235
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextList * textList() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn textList_0<RetType, T: QTextBlock_textList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textList_0(self);
    // return 1;
  }
}
pub trait QTextBlock_textList_0<RetType> {
  fn textList_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_textList_0<usize> for () {
  fn textList_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock8textListEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:237
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextBlockUserData * userData() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn userData_0<RetType, T: QTextBlock_userData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.userData_0(self);
    // return 1;
  }
}
pub trait QTextBlock_userData_0<RetType> {
  fn userData_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_userData_0<usize> for () {
  fn userData_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock8userDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:238
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUserData(QTextBlockUserData *)

/*

*/
impl /*struct*/ QTextBlock {
  pub fn setUserData_0<RetType, T: QTextBlock_setUserData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUserData_0(self);
    // return 1;
  }
}
pub trait QTextBlock_setUserData_0<RetType> {
  fn setUserData_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_setUserData_0<(/*void*/)> for (usize) {
  fn setUserData_0(self , rsthis: & QTextBlock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextBlock11setUserDataEP18QTextBlockUserData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:240
// index:0
// Public Visibility=Default Availability=Available
// [4] int userState() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn userState_0<RetType, T: QTextBlock_userState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.userState_0(self);
    // return 1;
  }
}
pub trait QTextBlock_userState_0<RetType> {
  fn userState_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_userState_0<i32> for () {
  fn userState_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock9userStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUserState(int)

/*

*/
impl /*struct*/ QTextBlock {
  pub fn setUserState_0<RetType, T: QTextBlock_setUserState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUserState_0(self);
    // return 1;
  }
}
pub trait QTextBlock_setUserState_0<RetType> {
  fn setUserState_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_setUserState_0<(/*void*/)> for (i32) {
  fn setUserState_0(self , rsthis: & QTextBlock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextBlock12setUserStateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:243
// index:0
// Public Visibility=Default Availability=Available
// [4] int revision() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn revision_0<RetType, T: QTextBlock_revision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.revision_0(self);
    // return 1;
  }
}
pub trait QTextBlock_revision_0<RetType> {
  fn revision_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_revision_0<i32> for () {
  fn revision_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock8revisionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:244
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRevision(int)

/*

*/
impl /*struct*/ QTextBlock {
  pub fn setRevision_0<RetType, T: QTextBlock_setRevision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRevision_0(self);
    // return 1;
  }
}
pub trait QTextBlock_setRevision_0<RetType> {
  fn setRevision_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_setRevision_0<(/*void*/)> for (i32) {
  fn setRevision_0(self , rsthis: & QTextBlock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextBlock11setRevisionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:246
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisible() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn isVisible_0<RetType, T: QTextBlock_isVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisible_0(self);
    // return 1;
  }
}
pub trait QTextBlock_isVisible_0<RetType> {
  fn isVisible_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_isVisible_0<bool> for () {
  fn isVisible_0(self , rsthis: & QTextBlock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:247
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*

*/
impl /*struct*/ QTextBlock {
  pub fn setVisible_0<RetType, T: QTextBlock_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QTextBlock_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QTextBlock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextBlock10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:249
// index:0
// Public Visibility=Default Availability=Available
// [4] int blockNumber() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn blockNumber_0<RetType, T: QTextBlock_blockNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockNumber_0(self);
    // return 1;
  }
}
pub trait QTextBlock_blockNumber_0<RetType> {
  fn blockNumber_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_blockNumber_0<i32> for () {
  fn blockNumber_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock11blockNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:250
// index:0
// Public Visibility=Default Availability=Available
// [4] int firstLineNumber() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn firstLineNumber_0<RetType, T: QTextBlock_firstLineNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstLineNumber_0(self);
    // return 1;
  }
}
pub trait QTextBlock_firstLineNumber_0<RetType> {
  fn firstLineNumber_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_firstLineNumber_0<i32> for () {
  fn firstLineNumber_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock15firstLineNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:252
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLineCount(int)

/*

*/
impl /*struct*/ QTextBlock {
  pub fn setLineCount_0<RetType, T: QTextBlock_setLineCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineCount_0(self);
    // return 1;
  }
}
pub trait QTextBlock_setLineCount_0<RetType> {
  fn setLineCount_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_setLineCount_0<(/*void*/)> for (i32) {
  fn setLineCount_0(self , rsthis: & QTextBlock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextBlock12setLineCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:253
// index:0
// Public Visibility=Default Availability=Available
// [4] int lineCount() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn lineCount_0<RetType, T: QTextBlock_lineCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineCount_0(self);
    // return 1;
  }
}
pub trait QTextBlock_lineCount_0<RetType> {
  fn lineCount_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_lineCount_0<i32> for () {
  fn lineCount_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock9lineCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:283
// index:0
// Public Visibility=Default Availability=Available
// [24] QTextBlock::iterator begin() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn begin_0<RetType, T: QTextBlock_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QTextBlock_begin_0<RetType> {
  fn begin_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:284
// index:0
// Public Visibility=Default Availability=Available
// [24] QTextBlock::iterator end() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn end_0<RetType, T: QTextBlock_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QTextBlock_end_0<RetType> {
  fn end_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_end_0<usize> for () {
  fn end_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:286
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock next() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn next_0<RetType, T: QTextBlock_next_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.next_0(self);
    // return 1;
  }
}
pub trait QTextBlock_next_0<RetType> {
  fn next_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_next_0<usize> for () {
  fn next_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock4nextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:287
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock previous() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn previous_0<RetType, T: QTextBlock_previous_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.previous_0(self);
    // return 1;
  }
}
pub trait QTextBlock_previous_0<RetType> {
  fn previous_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_previous_0<usize> for () {
  fn previous_0(self , rsthis: & QTextBlock) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock8previousEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:290
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int fragmentIndex() const

/*

*/
impl /*struct*/ QTextBlock {
  pub fn fragmentIndex_0<RetType, T: QTextBlock_fragmentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fragmentIndex_0(self);
    // return 1;
  }
}
pub trait QTextBlock_fragmentIndex_0<RetType> {
  fn fragmentIndex_0(self , rsthis: & QTextBlock) -> RetType;
}
impl<'a> /*trait*/ QTextBlock_fragmentIndex_0<i32> for () {
  fn fragmentIndex_0(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextBlock13fragmentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQTextBlock(this :*mut QTextBlock) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QTextBlockD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
