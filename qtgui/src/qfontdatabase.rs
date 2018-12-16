

// mod ::gui::QFontDatabase
// package qtgui
// /usr/include/qt/QtGui/qfontdatabase.h
// #include <qfontdatabase.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 12
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
#[derive(Default)] // class sizeof(QFontDatabase)=8
pub struct QFontDatabase {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFontDatabase_ITF interface {
//    QFontDatabase_PTR() *QFontDatabase
//}
//func (ptr *QFontDatabase) QFontDatabase_PTR() *QFontDatabase { return ptr }

impl /*struct*/ QFontDatabase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFontDatabase {
    return QFontDatabase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFontDatabase {
//  type Target = QFontDatabaseBASE;
//
//  fn deref(&self) -> &QFontDatabaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFontDatabaseBASE> for QFontDatabase {
//  fn as_ref(& self) -> & QFontDatabaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qfontdatabase.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFontDatabase()

/*
Creates a font database object.
*/
// QFontDatabase() ctx.fn_proto_cpp
impl /*struct*/ QFontDatabase {
  pub fn QFontDatabase_0<T: QFontDatabase_QFontDatabase_0>(value: T) -> QFontDatabase {
    let rsthis = value.QFontDatabase_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFontDatabase_QFontDatabase_0 {
  fn QFontDatabase_0(self) -> QFontDatabase;
}
// QFontDatabase() ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontDatabase_QFontDatabase_0 for () {
  fn QFontDatabase_0(self) -> QFontDatabase {
    // unsafe{_ZN13QFontDatabaseC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QFontDatabaseC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontDatabase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:123
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList families(QFontDatabase::WritingSystem) const

/*
Returns a sorted list of the available font families which support the writingSystem.

If a family exists in several foundries, the returned name for that font is in the form "family [foundry]". Examples: "Times [Adobe]", "Times [Cronyx]", "Palatino".

See also writingSystems().
*/
impl /*struct*/ QFontDatabase {
  pub fn families_0<RetType, T: QFontDatabase_families_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.families_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_families_0<RetType> {
  fn families_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_families_0<usize> for (i32) {
  fn families_0(self , rsthis: & QFontDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase8familiesENS_13WritingSystemE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:124
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList styles(const QString &) const

/*
Returns a list of the styles available for the font family family. Some example styles: "Light", "Light Italic", "Bold", "Oblique", "Demi". The list may be empty.

See also families().
*/
impl /*struct*/ QFontDatabase {
  pub fn styles_0<RetType, T: QFontDatabase_styles_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styles_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_styles_0<RetType> {
  fn styles_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_styles_0<usize> for (usize) {
  fn styles_0(self , rsthis: & QFontDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase6stylesERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] QString styleString(const QFont &)

/*
Returns a string that describes the style of the font. For example, "Bold Italic", "Bold", "Italic" or "Normal". An empty string may be returned.
*/
impl /*struct*/ QFontDatabase {
  pub fn styleString_0<RetType, T: QFontDatabase_styleString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleString_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_styleString_0<RetType> {
  fn styleString_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_styleString_0<usize> for (usize) {
  fn styleString_0(self , rsthis: & QFontDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase11styleStringERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:128
// index:1
// Public Visibility=Default Availability=Available
// [8] QString styleString(const QFontInfo &)

/*
Returns a string that describes the style of the font. For example, "Bold Italic", "Bold", "Italic" or "Normal". An empty string may be returned.
*/
impl /*struct*/ QFontDatabase {
  pub fn styleString_1<RetType, T: QFontDatabase_styleString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleString_1(self);
    // return 1;
  }
}
pub trait QFontDatabase_styleString_1<RetType> {
  fn styleString_1(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_styleString_1<usize> for (usize) {
  fn styleString_1(self , rsthis: & QFontDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase11styleStringERK9QFontInfo", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:130
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font(const QString &, const QString &, int) const

/*
Returns a QFont object that has family family, style style and point size pointSize. If no matching font could be created, a QFont object that uses the application's default font is returned.
*/
impl /*struct*/ QFontDatabase {
  pub fn font_0<RetType, T: QFontDatabase_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_font_0<RetType> {
  fn font_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_font_0<usize> for (usize,usize,i32) {
  fn font_0(self , rsthis: & QFontDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase4fontERK7QStringS2_i", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:132
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isBitmapScalable(const QString &, const QString &) const

/*
Returns true if the font that has family family and style style is a scalable bitmap font; otherwise returns false. Scaling a bitmap font usually produces an unattractive hardly readable result, because the pixels of the font are scaled. If you need to scale a bitmap font it is better to scale it to one of the fixed sizes returned by smoothSizes().

See also isScalable() and isSmoothlyScalable().
*/
impl /*struct*/ QFontDatabase {
  pub fn isBitmapScalable_0<RetType, T: QFontDatabase_isBitmapScalable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isBitmapScalable_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_isBitmapScalable_0<RetType> {
  fn isBitmapScalable_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_isBitmapScalable_0<bool> for (usize,usize) {
  fn isBitmapScalable_0(self , rsthis: & QFontDatabase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase16isBitmapScalableERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:133
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSmoothlyScalable(const QString &, const QString &) const

/*
Returns true if the font that has family family and style style is smoothly scalable; otherwise returns false. If this function returns true, it's safe to scale this font to any size, and the result will always look attractive.

See also isScalable() and isBitmapScalable().
*/
impl /*struct*/ QFontDatabase {
  pub fn isSmoothlyScalable_0<RetType, T: QFontDatabase_isSmoothlyScalable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSmoothlyScalable_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_isSmoothlyScalable_0<RetType> {
  fn isSmoothlyScalable_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_isSmoothlyScalable_0<bool> for (usize,usize) {
  fn isSmoothlyScalable_0(self , rsthis: & QFontDatabase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase18isSmoothlyScalableERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:134
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isScalable(const QString &, const QString &) const

/*
Returns true if the font that has family family and style style is scalable; otherwise returns false.

See also isBitmapScalable() and isSmoothlyScalable().
*/
impl /*struct*/ QFontDatabase {
  pub fn isScalable_0<RetType, T: QFontDatabase_isScalable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isScalable_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_isScalable_0<RetType> {
  fn isScalable_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_isScalable_0<bool> for (usize,usize) {
  fn isScalable_0(self , rsthis: & QFontDatabase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase10isScalableERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:135
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFixedPitch(const QString &, const QString &) const

/*
Returns true if the font that has family family and style style is fixed pitch; otherwise returns false.
*/
impl /*struct*/ QFontDatabase {
  pub fn isFixedPitch_0<RetType, T: QFontDatabase_isFixedPitch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFixedPitch_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_isFixedPitch_0<RetType> {
  fn isFixedPitch_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_isFixedPitch_0<bool> for (usize,usize) {
  fn isFixedPitch_0(self , rsthis: & QFontDatabase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase12isFixedPitchERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:137
// index:0
// Public Visibility=Default Availability=Available
// [1] bool italic(const QString &, const QString &) const

/*
Returns true if the font that has family family and style style is italic; otherwise returns false.

See also weight() and bold().
*/
impl /*struct*/ QFontDatabase {
  pub fn italic_0<RetType, T: QFontDatabase_italic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.italic_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_italic_0<RetType> {
  fn italic_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_italic_0<bool> for (usize,usize) {
  fn italic_0(self , rsthis: & QFontDatabase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase6italicERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:138
// index:0
// Public Visibility=Default Availability=Available
// [1] bool bold(const QString &, const QString &) const

/*
Returns true if the font that has family family and style style is bold; otherwise returns false.

See also italic() and weight().
*/
impl /*struct*/ QFontDatabase {
  pub fn bold_0<RetType, T: QFontDatabase_bold_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bold_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_bold_0<RetType> {
  fn bold_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_bold_0<bool> for (usize,usize) {
  fn bold_0(self , rsthis: & QFontDatabase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase4boldERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:139
// index:0
// Public Visibility=Default Availability=Available
// [4] int weight(const QString &, const QString &) const

/*
Returns the weight of the font that has family family and style style. If there is no such family and style combination, returns -1.

See also italic() and bold().
*/
impl /*struct*/ QFontDatabase {
  pub fn weight_0<RetType, T: QFontDatabase_weight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.weight_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_weight_0<RetType> {
  fn weight_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_weight_0<i32> for (usize,usize) {
  fn weight_0(self , rsthis: & QFontDatabase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase6weightERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:141
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFamily(const QString &) const

/*

*/
impl /*struct*/ QFontDatabase {
  pub fn hasFamily_0<RetType, T: QFontDatabase_hasFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFamily_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_hasFamily_0<RetType> {
  fn hasFamily_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_hasFamily_0<bool> for (usize) {
  fn hasFamily_0(self , rsthis: & QFontDatabase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase9hasFamilyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:142
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPrivateFamily(const QString &) const

/*
Returns true if and only if the family font family is private.

This happens, for instance, on macOS and iOS, where the system UI fonts are not accessible to the user. For completeness, QFontDatabase::families() returns all font families, including the private ones. You should use this function if you are developing a font selection control in order to keep private fonts hidden.

This function was introduced in  Qt 5.5.

See also families().
*/
impl /*struct*/ QFontDatabase {
  pub fn isPrivateFamily_0<RetType, T: QFontDatabase_isPrivateFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPrivateFamily_0(self);
    // return 1;
  }
}
pub trait QFontDatabase_isPrivateFamily_0<RetType> {
  fn isPrivateFamily_0(self , rsthis: & QFontDatabase) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_isPrivateFamily_0<bool> for (usize) {
  fn isPrivateFamily_0(self , rsthis: & QFontDatabase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontDatabase15isPrivateFamilyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:144
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString writingSystemName(QFontDatabase::WritingSystem)

/*
Returns the names the writingSystem (e.g. for displaying to the user in a dialog).
*/
impl /*struct*/ QFontDatabase {
  pub fn writingSystemName_0<RetType, T: QFontDatabase_writingSystemName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.writingSystemName_0();
    // return 1;
  }
}
pub trait QFontDatabase_writingSystemName_0<RetType> {
  fn writingSystemName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_writingSystemName_0<usize> for (i32) {
  fn writingSystemName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase17writingSystemNameENS_13WritingSystemE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:145
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString writingSystemSample(QFontDatabase::WritingSystem)

/*
Returns a string with sample characters from writingSystem.
*/
impl /*struct*/ QFontDatabase {
  pub fn writingSystemSample_0<RetType, T: QFontDatabase_writingSystemSample_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.writingSystemSample_0();
    // return 1;
  }
}
pub trait QFontDatabase_writingSystemSample_0<RetType> {
  fn writingSystemSample_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_writingSystemSample_0<usize> for (i32) {
  fn writingSystemSample_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase19writingSystemSampleENS_13WritingSystemE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:147
// index:0
// Public static Visibility=Default Availability=Available
// [4] int addApplicationFont(const QString &)

/*
Loads the font from the file specified by fileName and makes it available to the application. An ID is returned that can be used to remove the font again with removeApplicationFont() or to retrieve the list of family names contained in the font.

The function returns -1 if the font could not be loaded.

Currently only TrueType fonts, TrueType font collections, and OpenType fonts are supported.

Note: Adding application fonts on Unix/X11 platforms without fontconfig is currently not supported.

This function was introduced in  Qt 4.2.

See also addApplicationFontFromData(), applicationFontFamilies(), and removeApplicationFont().
*/
impl /*struct*/ QFontDatabase {
  pub fn addApplicationFont_0<RetType, T: QFontDatabase_addApplicationFont_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.addApplicationFont_0();
    // return 1;
  }
}
pub trait QFontDatabase_addApplicationFont_0<RetType> {
  fn addApplicationFont_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_addApplicationFont_0<i32> for (usize) {
  fn addApplicationFont_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase18addApplicationFontERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:148
// index:0
// Public static Visibility=Default Availability=Available
// [4] int addApplicationFontFromData(const QByteArray &)

/*
Loads the font from binary data specified by fontData and makes it available to the application. An ID is returned that can be used to remove the font again with removeApplicationFont() or to retrieve the list of family names contained in the font.

The function returns -1 if the font could not be loaded.

Currently only TrueType fonts and TrueType font collections are supported.

Note: Adding application fonts on Unix/X11 platforms without fontconfig is currently not supported.

This function was introduced in  Qt 4.2.

See also addApplicationFont(), applicationFontFamilies(), and removeApplicationFont().
*/
impl /*struct*/ QFontDatabase {
  pub fn addApplicationFontFromData_0<RetType, T: QFontDatabase_addApplicationFontFromData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.addApplicationFontFromData_0();
    // return 1;
  }
}
pub trait QFontDatabase_addApplicationFontFromData_0<RetType> {
  fn addApplicationFontFromData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_addApplicationFontFromData_0<i32> for (usize) {
  fn addApplicationFontFromData_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase26addApplicationFontFromDataERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:149
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList applicationFontFamilies(int)

/*
Returns a list of font families for the given application font identified by id.

This function was introduced in  Qt 4.2.

See also addApplicationFont() and addApplicationFontFromData().
*/
impl /*struct*/ QFontDatabase {
  pub fn applicationFontFamilies_0<RetType, T: QFontDatabase_applicationFontFamilies_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationFontFamilies_0();
    // return 1;
  }
}
pub trait QFontDatabase_applicationFontFamilies_0<RetType> {
  fn applicationFontFamilies_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_applicationFontFamilies_0<usize> for (i32) {
  fn applicationFontFamilies_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase23applicationFontFamiliesEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:150
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool removeApplicationFont(int)

/*
Removes the previously loaded application font identified by id. Returns true if unloading of the font succeeded; otherwise returns false.

This function was introduced in  Qt 4.2.

See also removeAllApplicationFonts(), addApplicationFont(), and addApplicationFontFromData().
*/
impl /*struct*/ QFontDatabase {
  pub fn removeApplicationFont_0<RetType, T: QFontDatabase_removeApplicationFont_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeApplicationFont_0();
    // return 1;
  }
}
pub trait QFontDatabase_removeApplicationFont_0<RetType> {
  fn removeApplicationFont_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_removeApplicationFont_0<bool> for (i32) {
  fn removeApplicationFont_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase21removeApplicationFontEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:151
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool removeAllApplicationFonts()

/*
Removes all application-local fonts previously added using addApplicationFont() and addApplicationFontFromData().

Returns true if unloading of the fonts succeeded; otherwise returns false.

This function was introduced in  Qt 4.2.

See also removeApplicationFont(), addApplicationFont(), and addApplicationFontFromData().
*/
impl /*struct*/ QFontDatabase {
  pub fn removeAllApplicationFonts_0<RetType, T: QFontDatabase_removeAllApplicationFonts_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeAllApplicationFonts_0();
    // return 1;
  }
}
pub trait QFontDatabase_removeAllApplicationFonts_0<RetType> {
  fn removeAllApplicationFonts_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_removeAllApplicationFonts_0<bool> for () {
  fn removeAllApplicationFonts_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase25removeAllApplicationFontsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:154
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool supportsThreadedFontRendering()

/*

*/
impl /*struct*/ QFontDatabase {
  pub fn supportsThreadedFontRendering_0<RetType, T: QFontDatabase_supportsThreadedFontRendering_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportsThreadedFontRendering_0();
    // return 1;
  }
}
pub trait QFontDatabase_supportsThreadedFontRendering_0<RetType> {
  fn supportsThreadedFontRendering_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_supportsThreadedFontRendering_0<bool> for () {
  fn supportsThreadedFontRendering_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase29supportsThreadedFontRenderingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontdatabase.h:157
// index:0
// Public static Visibility=Default Availability=Available
// [16] QFont systemFont(QFontDatabase::SystemFont)

/*
Returns the most adequate font for a given type case for proper integration with the system's look and feel.

This function was introduced in  Qt 5.2.

See also QGuiApplication::font().
*/
impl /*struct*/ QFontDatabase {
  pub fn systemFont_0<RetType, T: QFontDatabase_systemFont_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemFont_0();
    // return 1;
  }
}
pub trait QFontDatabase_systemFont_0<RetType> {
  fn systemFont_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFontDatabase_systemFont_0<usize> for (i32) {
  fn systemFont_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontDatabase10systemFontENS_10SystemFontE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQFontDatabase(this :*mut QFontDatabase) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QFontDatabaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
QFontDatabase::OtherSymbol(the same as Symbol)
QFontDatabase::Ogham? 
QFontDatabase::Runic? 
QFontDatabase::Nko?

*/
pub type QFontDatabase__WritingSystem = i32;
//  
pub const QFontDatabase__Any :QFontDatabase__WritingSystem = 0;
//  
pub const QFontDatabase__Latin :QFontDatabase__WritingSystem = 1;
//  
pub const QFontDatabase__Greek :QFontDatabase__WritingSystem = 2;
//  
pub const QFontDatabase__Cyrillic :QFontDatabase__WritingSystem = 3;
//  
pub const QFontDatabase__Armenian :QFontDatabase__WritingSystem = 4;
//  
pub const QFontDatabase__Hebrew :QFontDatabase__WritingSystem = 5;
//  
pub const QFontDatabase__Arabic :QFontDatabase__WritingSystem = 6;
//  
pub const QFontDatabase__Syriac :QFontDatabase__WritingSystem = 7;
//  
pub const QFontDatabase__Thaana :QFontDatabase__WritingSystem = 8;
//  
pub const QFontDatabase__Devanagari :QFontDatabase__WritingSystem = 9;
// 
pub const QFontDatabase__Bengali :QFontDatabase__WritingSystem = 10;
// 
pub const QFontDatabase__Gurmukhi :QFontDatabase__WritingSystem = 11;
// 
pub const QFontDatabase__Gujarati :QFontDatabase__WritingSystem = 12;
// 
pub const QFontDatabase__Oriya :QFontDatabase__WritingSystem = 13;
// 
pub const QFontDatabase__Tamil :QFontDatabase__WritingSystem = 14;
// 
pub const QFontDatabase__Telugu :QFontDatabase__WritingSystem = 15;
// 
pub const QFontDatabase__Kannada :QFontDatabase__WritingSystem = 16;
// 
pub const QFontDatabase__Malayalam :QFontDatabase__WritingSystem = 17;
// 
pub const QFontDatabase__Sinhala :QFontDatabase__WritingSystem = 18;
// 
pub const QFontDatabase__Thai :QFontDatabase__WritingSystem = 19;
// 
pub const QFontDatabase__Lao :QFontDatabase__WritingSystem = 20;
// 
pub const QFontDatabase__Tibetan :QFontDatabase__WritingSystem = 21;
// 
pub const QFontDatabase__Myanmar :QFontDatabase__WritingSystem = 22;
// 
pub const QFontDatabase__Georgian :QFontDatabase__WritingSystem = 23;
// 
pub const QFontDatabase__Khmer :QFontDatabase__WritingSystem = 24;
// 
pub const QFontDatabase__SimplifiedChinese :QFontDatabase__WritingSystem = 25;
// 
pub const QFontDatabase__TraditionalChinese :QFontDatabase__WritingSystem = 26;
// 
pub const QFontDatabase__Japanese :QFontDatabase__WritingSystem = 27;
// 
pub const QFontDatabase__Korean :QFontDatabase__WritingSystem = 28;
// 
pub const QFontDatabase__Vietnamese :QFontDatabase__WritingSystem = 29;
// 
pub const QFontDatabase__Symbol :QFontDatabase__WritingSystem = 30;
// 
pub const QFontDatabase__Other :QFontDatabase__WritingSystem = 30;
// 
pub const QFontDatabase__Ogham :QFontDatabase__WritingSystem = 31;
// 
pub const QFontDatabase__Runic :QFontDatabase__WritingSystem = 32;
// 
pub const QFontDatabase__Nko :QFontDatabase__WritingSystem = 33;
// 
pub const QFontDatabase__WritingSystemsCount :QFontDatabase__WritingSystem = 34;
pub fn QFontDatabase_WritingSystemItemName(val: i32) ->String {
  match val {
     QFontDatabase__Any => // 0
     {return String::from("Any");}
     QFontDatabase__Latin => // 1
     {return String::from("Latin");}
     QFontDatabase__Greek => // 2
     {return String::from("Greek");}
     QFontDatabase__Cyrillic => // 3
     {return String::from("Cyrillic");}
     QFontDatabase__Armenian => // 4
     {return String::from("Armenian");}
     QFontDatabase__Hebrew => // 5
     {return String::from("Hebrew");}
     QFontDatabase__Arabic => // 6
     {return String::from("Arabic");}
     QFontDatabase__Syriac => // 7
     {return String::from("Syriac");}
     QFontDatabase__Thaana => // 8
     {return String::from("Thaana");}
     QFontDatabase__Devanagari => // 9
     {return String::from("Devanagari");}
     QFontDatabase__Bengali => // 10
     {return String::from("Bengali");}
     QFontDatabase__Gurmukhi => // 11
     {return String::from("Gurmukhi");}
     QFontDatabase__Gujarati => // 12
     {return String::from("Gujarati");}
     QFontDatabase__Oriya => // 13
     {return String::from("Oriya");}
     QFontDatabase__Tamil => // 14
     {return String::from("Tamil");}
     QFontDatabase__Telugu => // 15
     {return String::from("Telugu");}
     QFontDatabase__Kannada => // 16
     {return String::from("Kannada");}
     QFontDatabase__Malayalam => // 17
     {return String::from("Malayalam");}
     QFontDatabase__Sinhala => // 18
     {return String::from("Sinhala");}
     QFontDatabase__Thai => // 19
     {return String::from("Thai");}
     QFontDatabase__Lao => // 20
     {return String::from("Lao");}
     QFontDatabase__Tibetan => // 21
     {return String::from("Tibetan");}
     QFontDatabase__Myanmar => // 22
     {return String::from("Myanmar");}
     QFontDatabase__Georgian => // 23
     {return String::from("Georgian");}
     QFontDatabase__Khmer => // 24
     {return String::from("Khmer");}
     QFontDatabase__SimplifiedChinese => // 25
     {return String::from("SimplifiedChinese");}
     QFontDatabase__TraditionalChinese => // 26
     {return String::from("TraditionalChinese");}
     QFontDatabase__Japanese => // 27
     {return String::from("Japanese");}
     QFontDatabase__Korean => // 28
     {return String::from("Korean");}
     QFontDatabase__Vietnamese => // 29
     {return String::from("Vietnamese");}
     QFontDatabase__Symbol => // 30
     {return String::from("Symbol,Other");}
    // QFontDatabase__Other => // 30
    // {return String::from("");}
     QFontDatabase__Ogham => // 31
     {return String::from("Ogham");}
     QFontDatabase__Runic => // 32
     {return String::from("Runic");}
     QFontDatabase__Nko => // 33
     {return String::from("Nko");}
     QFontDatabase__WritingSystemsCount => // 34
     {return String::from("WritingSystemsCount");}
  _ => {return format!("{}", val);}
}
}
pub fn QFontDatabase_WritingSystemItemName_s(val: i32) ->String {
  //var nilthis *QFontDatabase
  //return nilthis.WritingSystemItemName(val);
  return QFontDatabase_WritingSystemItemName(val);
}


/*


This enum was introduced or modified in  Qt 5.2.

*/
pub type QFontDatabase__SystemFont = i32;
// The default system font.
pub const QFontDatabase__GeneralFont :QFontDatabase__SystemFont = 0;
// The fixed font that the system recommends.
pub const QFontDatabase__FixedFont :QFontDatabase__SystemFont = 1;
// The system standard font for titles.
pub const QFontDatabase__TitleFont :QFontDatabase__SystemFont = 2;
// The smallest readable system font.
pub const QFontDatabase__SmallestReadableFont :QFontDatabase__SystemFont = 3;
pub fn QFontDatabase_SystemFontItemName(val: i32) ->String {
  match val {
     QFontDatabase__GeneralFont => // 0
     {return String::from("GeneralFont");}
     QFontDatabase__FixedFont => // 1
     {return String::from("FixedFont");}
     QFontDatabase__TitleFont => // 2
     {return String::from("TitleFont");}
     QFontDatabase__SmallestReadableFont => // 3
     {return String::from("SmallestReadableFont");}
  _ => {return format!("{}", val);}
}
}
pub fn QFontDatabase_SystemFontItemName_s(val: i32) ->String {
  //var nilthis *QFontDatabase
  //return nilthis.SystemFontItemName(val);
  return QFontDatabase_SystemFontItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
