// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtGui/qtextformat.h
// dst-file: /src/gui/qtextformat.rs
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
use std::ops::Deref;
// use super::qtextformat::QTextCharFormat; // 773
// use super::qtextformat::QTextFormat; // 773
use super::super::core::qstring::QString; // 771
// use super::qtextformat::QTextBlockFormat; // 773
use super::qbrush::QBrush; // 773
// use super::qtextformat::QTextLength; // 773
use super::qcolor::QColor; // 773
// use super::qtextformat::QTextFrameFormat; // 773
use super::qpen::QPen; // 773
use super::super::core::qvariant::QVariant; // 771
// use super::qtextformat::QTextImageFormat; // 773
// use super::qtextformat::QTextTableFormat; // 773
// use super::qtextformat::QTextTableCellFormat; // 773
// use super::qtextformat::QTextListFormat; // 773
use super::qfont::QFont; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextLength_Class_Size() -> c_int;
  fn QTextImageFormat_Class_Size() -> c_int;
  // proto:  void QTextImageFormat::QTextImageFormat();
  fn dector_ZN16QTextImageFormatC1Ev() -> *mut c_void;
  fn _ZN16QTextImageFormatC1Ev(qthis: *mut c_void);
  // proto:  void QTextImageFormat::QTextImageFormat(const QTextFormat & format);
  fn dector_ZN16QTextImageFormatC1ERK11QTextFormat(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QTextImageFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextImageFormat::setName(const QString & name);
  fn _ZN16QTextImageFormat7setNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextImageFormat::setHeight(qreal height);
  fn _ZN16QTextImageFormat9setHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextImageFormat::setWidth(qreal width);
  fn _ZN16QTextImageFormat8setWidthEd(qthis: *mut c_void, arg0: c_double);
  fn QTextFormat_Class_Size() -> c_int;
  // proto:  QTextBlockFormat QTextFormat::toBlockFormat();
  fn _ZNK11QTextFormat13toBlockFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextFormat::stringProperty(int propertyId);
  fn _ZNK11QTextFormat14stringPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTextFormat::objectIndex();
  fn _ZNK11QTextFormat11objectIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextFormat::setObjectIndex(int object);
  fn _ZN11QTextFormat14setObjectIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextFormat::~QTextFormat();
  fn _ZN11QTextFormatD0Ev(qthis: *mut c_void);
  // proto:  QBrush QTextFormat::brushProperty(int propertyId);
  fn _ZNK11QTextFormat13brushPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextFormat::QTextFormat(const QTextFormat & rhs);
  fn dector_ZN11QTextFormatC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextFormatC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextLength QTextFormat::lengthProperty(int propertyId);
  fn _ZNK11QTextFormat14lengthPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextFormat::merge(const QTextFormat & other);
  fn _ZN11QTextFormat5mergeERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QColor QTextFormat::colorProperty(int propertyId);
  fn _ZNK11QTextFormat13colorPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextFormat::QTextFormat();
  fn dector_ZN11QTextFormatC1Ev() -> *mut c_void;
  fn _ZN11QTextFormatC1Ev(qthis: *mut c_void);
  // proto:  bool QTextFormat::boolProperty(int propertyId);
  fn _ZNK11QTextFormat12boolPropertyEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QTextFormat::QTextFormat(int type);
  fn dector_ZN11QTextFormatC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN11QTextFormatC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextFormat::clearProperty(int propertyId);
  fn _ZN11QTextFormat13clearPropertyEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QTextFrameFormat QTextFormat::toFrameFormat();
  fn _ZNK11QTextFormat13toFrameFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
  fn _ZNK11QTextFormat20lengthVectorPropertyEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QTextFormat::propertyCount();
  fn _ZNK11QTextFormat13propertyCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QPen QTextFormat::penProperty(int propertyId);
  fn _ZNK11QTextFormat11penPropertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QVariant QTextFormat::property(int propertyId);
  fn _ZNK11QTextFormat8propertyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextFormat::setProperty(int propertyId, const QVariant & value);
  fn _ZN11QTextFormat11setPropertyEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  int QTextFormat::type();
  fn _ZNK11QTextFormat4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  qreal QTextFormat::doubleProperty(int propertyId);
  fn _ZNK11QTextFormat14doublePropertyEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  QTextImageFormat QTextFormat::toImageFormat();
  fn _ZNK11QTextFormat13toImageFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextFormat::hasProperty(int propertyId);
  fn _ZNK11QTextFormat11hasPropertyEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QTextFormat::setObjectType(int type);
  fn _ZN11QTextFormat13setObjectTypeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QTextTableFormat QTextFormat::toTableFormat();
  fn _ZNK11QTextFormat13toTableFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextFormat::intProperty(int propertyId);
  fn _ZNK11QTextFormat11intPropertyEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QTextCharFormat QTextFormat::toCharFormat();
  fn _ZNK11QTextFormat12toCharFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextTableCellFormat QTextFormat::toTableCellFormat();
  fn _ZNK11QTextFormat17toTableCellFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextListFormat QTextFormat::toListFormat();
  fn _ZNK11QTextFormat12toListFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMap<int, QVariant> QTextFormat::properties();
  fn _ZNK11QTextFormat10propertiesEv(qthis: *mut c_void);
  fn QTextBlockFormat_Class_Size() -> c_int;
  // proto:  void QTextBlockFormat::QTextBlockFormat();
  fn dector_ZN16QTextBlockFormatC1Ev() -> *mut c_void;
  fn _ZN16QTextBlockFormatC1Ev(qthis: *mut c_void);
  // proto:  void QTextBlockFormat::setIndent(int indent);
  fn _ZN16QTextBlockFormat9setIndentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextBlockFormat::QTextBlockFormat(const QTextFormat & fmt);
  fn dector_ZN16QTextBlockFormatC1ERK11QTextFormat(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QTextBlockFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
  fn _ZNK16QTextBlockFormat10lineHeightEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> c_double;
  fn QTextCharFormat_Class_Size() -> c_int;
  // proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
  fn _ZN15QTextCharFormat22setTableCellColumnSpanEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QTextCharFormat::fontUnderline();
  fn _ZNK15QTextCharFormat13fontUnderlineEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCharFormat::setFont(const QFont & font);
  fn _ZN15QTextCharFormat7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QFont QTextCharFormat::font();
  fn _ZNK15QTextCharFormat4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCharFormat::QTextCharFormat();
  fn dector_ZN15QTextCharFormatC1Ev() -> *mut c_void;
  fn _ZN15QTextCharFormatC1Ev(qthis: *mut c_void);
  // proto:  void QTextCharFormat::QTextCharFormat(const QTextFormat & fmt);
  fn dector_ZN15QTextCharFormatC1ERK11QTextFormat(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QTextCharFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QTextCharFormat::anchorNames();
  fn _ZNK15QTextCharFormat11anchorNamesEv(qthis: *mut c_void);
  // proto:  QString QTextCharFormat::anchorName();
  fn _ZNK15QTextCharFormat10anchorNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
  fn _ZN15QTextCharFormat19setTableCellRowSpanEi(qthis: *mut c_void, arg0: c_int);
  fn QTextTableFormat_Class_Size() -> c_int;
  // proto:  void QTextTableFormat::QTextTableFormat();
  fn dector_ZN16QTextTableFormatC1Ev() -> *mut c_void;
  fn _ZN16QTextTableFormatC1Ev(qthis: *mut c_void);
  // proto:  void QTextTableFormat::setColumns(int columns);
  fn _ZN16QTextTableFormat10setColumnsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextTableFormat::QTextTableFormat(const QTextFormat & fmt);
  fn dector_ZN16QTextTableFormatC1ERK11QTextFormat(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QTextTableFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
  fn _ZN16QTextTableFormat14setCellPaddingEd(qthis: *mut c_void, arg0: c_double);
  fn QTextTableCellFormat_Class_Size() -> c_int;
  // proto:  void QTextTableCellFormat::QTextTableCellFormat();
  fn dector_ZN20QTextTableCellFormatC1Ev() -> *mut c_void;
  fn _ZN20QTextTableCellFormatC1Ev(qthis: *mut c_void);
  // proto:  void QTextTableCellFormat::setLeftPadding(qreal padding);
  fn _ZN20QTextTableCellFormat14setLeftPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextTableCellFormat::setTopPadding(qreal padding);
  fn _ZN20QTextTableCellFormat13setTopPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextTableCellFormat::leftPadding();
  fn _ZNK20QTextTableCellFormat11leftPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableCellFormat::setPadding(qreal padding);
  fn _ZN20QTextTableCellFormat10setPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextTableCellFormat::topPadding();
  fn _ZNK20QTextTableCellFormat10topPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTextTableCellFormat::rightPadding();
  fn _ZNK20QTextTableCellFormat12rightPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableCellFormat::QTextTableCellFormat(const QTextFormat & fmt);
  fn dector_ZN20QTextTableCellFormatC1ERK11QTextFormat(arg0: *mut c_void) -> *mut c_void;
  fn _ZN20QTextTableCellFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextTableCellFormat::bottomPadding();
  fn _ZNK20QTextTableCellFormat13bottomPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableCellFormat::setRightPadding(qreal padding);
  fn _ZN20QTextTableCellFormat15setRightPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextTableCellFormat::setBottomPadding(qreal padding);
  fn _ZN20QTextTableCellFormat16setBottomPaddingEd(qthis: *mut c_void, arg0: c_double);
  fn QTextListFormat_Class_Size() -> c_int;
  // proto:  void QTextListFormat::QTextListFormat(const QTextFormat & fmt);
  fn dector_ZN15QTextListFormatC1ERK11QTextFormat(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QTextListFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextListFormat::setIndent(int indent);
  fn _ZN15QTextListFormat9setIndentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
  fn _ZN15QTextListFormat15setNumberSuffixERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextListFormat::QTextListFormat();
  fn dector_ZN15QTextListFormatC1Ev() -> *mut c_void;
  fn _ZN15QTextListFormatC1Ev(qthis: *mut c_void);
  // proto:  void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
  fn _ZN15QTextListFormat15setNumberPrefixERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  fn QTextFrameFormat_Class_Size() -> c_int;
  // proto:  void QTextFrameFormat::setRightMargin(qreal margin);
  fn _ZN16QTextFrameFormat14setRightMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setMargin(qreal margin);
  fn _ZN16QTextFrameFormat9setMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextFrameFormat::bottomMargin();
  fn _ZNK16QTextFrameFormat12bottomMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::QTextFrameFormat(const QTextFormat & fmt);
  fn dector_ZN16QTextFrameFormatC1ERK11QTextFormat(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QTextFrameFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextFrameFormat::setBorder(qreal border);
  fn _ZN16QTextFrameFormat9setBorderEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setBottomMargin(qreal margin);
  fn _ZN16QTextFrameFormat15setBottomMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setHeight(const QTextLength & height);
  fn _ZN16QTextFrameFormat9setHeightERK11QTextLength(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextFrameFormat::setLeftMargin(qreal margin);
  fn _ZN16QTextFrameFormat13setLeftMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setHeight(qreal height);
  fn _ZN16QTextFrameFormat9setHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setWidth(qreal width);
  fn _ZN16QTextFrameFormat8setWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::QTextFrameFormat();
  fn dector_ZN16QTextFrameFormatC1Ev() -> *mut c_void;
  fn _ZN16QTextFrameFormatC1Ev(qthis: *mut c_void);
  // proto:  qreal QTextFrameFormat::rightMargin();
  fn _ZNK16QTextFrameFormat11rightMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::setPadding(qreal padding);
  fn _ZN16QTextFrameFormat10setPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextFrameFormat::leftMargin();
  fn _ZNK16QTextFrameFormat10leftMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::setTopMargin(qreal margin);
  fn _ZN16QTextFrameFormat12setTopMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextFrameFormat::topMargin();
  fn _ZNK16QTextFrameFormat9topMarginEv(qthis: *mut c_void) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QTextLength)=16
pub struct QTextLength {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextImageFormat)=1
pub struct QTextImageFormat {
  qbase: QTextCharFormat,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextFormat)=1
pub struct QTextFormat {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextBlockFormat)=1
pub struct QTextBlockFormat {
  qbase: QTextFormat,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextCharFormat)=1
pub struct QTextCharFormat {
  qbase: QTextFormat,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextTableFormat)=1
pub struct QTextTableFormat {
  qbase: QTextFrameFormat,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextTableCellFormat)=1
pub struct QTextTableCellFormat {
  qbase: QTextCharFormat,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextListFormat)=1
pub struct QTextListFormat {
  qbase: QTextFormat,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextFrameFormat)=1
pub struct QTextFrameFormat {
  qbase: QTextFormat,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextLength {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextLength {
    return QTextLength{qclsinst: qthis};
  }
}
impl /*struct*/ QTextImageFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextImageFormat {
    return QTextImageFormat{qbase: QTextCharFormat::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTextImageFormat {
  type Target = QTextCharFormat;

  fn deref(&self) -> &QTextCharFormat {
    return & self.qbase;
  }
}
impl AsRef<QTextCharFormat> for QTextImageFormat {
  fn as_ref(& self) -> & QTextCharFormat {
    return & self.qbase;
  }
}
  // proto:  void QTextImageFormat::QTextImageFormat();
impl /*struct*/ QTextImageFormat {
  pub fn New<T: QTextImageFormat_New>(value: T) -> QTextImageFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextImageFormat_New {
  fn New(self) -> QTextImageFormat;
}

  // proto:  void QTextImageFormat::QTextImageFormat();
impl<'a> /*trait*/ QTextImageFormat_New for () {
  fn New(self) -> QTextImageFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormatC1Ev()};
    let ctysz: c_int = unsafe{QTextImageFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN16QTextImageFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN16QTextImageFormatC1Ev()};
    let rsthis = QTextImageFormat{/**/qbase: QTextCharFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextImageFormat::QTextImageFormat(const QTextFormat & format);
impl<'a> /*trait*/ QTextImageFormat_New for (&'a QTextFormat) {
  fn New(self) -> QTextImageFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormatC1ERK11QTextFormat()};
    let ctysz: c_int = unsafe{QTextImageFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QTextImageFormatC1ERK11QTextFormat(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QTextImageFormatC1ERK11QTextFormat(arg0)};
    let rsthis = QTextImageFormat{/**/qbase: QTextCharFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setName(const QString & name);
impl /*struct*/ QTextImageFormat {
  pub fn setName<RetType, T: QTextImageFormat_setName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setName(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setName<RetType> {
  fn setName(self , rsthis: & QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setName(const QString & name);
impl<'a> /*trait*/ QTextImageFormat_setName<()> for (&'a QString) {
  fn setName(self , rsthis: & QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat7setNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextImageFormat7setNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setHeight(qreal height);
impl /*struct*/ QTextImageFormat {
  pub fn setHeight<RetType, T: QTextImageFormat_setHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setHeight<RetType> {
  fn setHeight(self , rsthis: & QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setHeight(qreal height);
impl<'a> /*trait*/ QTextImageFormat_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: & QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextImageFormat9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setWidth(qreal width);
impl /*struct*/ QTextImageFormat {
  pub fn setWidth<RetType, T: QTextImageFormat_setWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setWidth<RetType> {
  fn setWidth(self , rsthis: & QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setWidth(qreal width);
impl<'a> /*trait*/ QTextImageFormat_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: & QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextImageFormat8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextFormat {
    return QTextFormat{qclsinst: qthis};
  }
}
  // proto:  QTextBlockFormat QTextFormat::toBlockFormat();
impl /*struct*/ QTextFormat {
  pub fn toBlockFormat<RetType, T: QTextFormat_toBlockFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toBlockFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toBlockFormat<RetType> {
  fn toBlockFormat(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QTextBlockFormat QTextFormat::toBlockFormat();
impl<'a> /*trait*/ QTextFormat_toBlockFormat<QTextBlockFormat> for () {
  fn toBlockFormat(self , rsthis: & QTextFormat) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toBlockFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toBlockFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextFormat::stringProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn stringProperty<RetType, T: QTextFormat_stringProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stringProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_stringProperty<RetType> {
  fn stringProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QString QTextFormat::stringProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_stringProperty<QString> for (i32) {
  fn stringProperty(self , rsthis: & QTextFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14stringPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14stringPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextFormat::objectIndex();
impl /*struct*/ QTextFormat {
  pub fn objectIndex<RetType, T: QTextFormat_objectIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.objectIndex(self);
    // return 1;
  }
}

pub trait QTextFormat_objectIndex<RetType> {
  fn objectIndex(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::objectIndex();
impl<'a> /*trait*/ QTextFormat_objectIndex<i32> for () {
  fn objectIndex(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11objectIndexEv()};
    let mut ret = unsafe {_ZNK11QTextFormat11objectIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextFormat::setObjectIndex(int object);
impl /*struct*/ QTextFormat {
  pub fn setObjectIndex<RetType, T: QTextFormat_setObjectIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setObjectIndex(self);
    // return 1;
  }
}

pub trait QTextFormat_setObjectIndex<RetType> {
  fn setObjectIndex(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::setObjectIndex(int object);
impl<'a> /*trait*/ QTextFormat_setObjectIndex<()> for (i32) {
  fn setObjectIndex(self , rsthis: & QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat14setObjectIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat14setObjectIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFormat::~QTextFormat();
impl /*struct*/ QTextFormat {
  pub fn Free<RetType, T: QTextFormat_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextFormat_Free<RetType> {
  fn Free(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::~QTextFormat();
impl<'a> /*trait*/ QTextFormat_Free<()> for () {
  fn Free(self , rsthis: & QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatD0Ev()};
     unsafe {_ZN11QTextFormatD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QBrush QTextFormat::brushProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn brushProperty<RetType, T: QTextFormat_brushProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brushProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_brushProperty<RetType> {
  fn brushProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QBrush QTextFormat::brushProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_brushProperty<QBrush> for (i32) {
  fn brushProperty(self , rsthis: & QTextFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13brushPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat13brushPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QBrush::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFormat::QTextFormat(const QTextFormat & rhs);
impl /*struct*/ QTextFormat {
  pub fn New<T: QTextFormat_New>(value: T) -> QTextFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFormat_New {
  fn New(self) -> QTextFormat;
}

  // proto:  void QTextFormat::QTextFormat(const QTextFormat & rhs);
impl<'a> /*trait*/ QTextFormat_New for (&'a QTextFormat) {
  fn New(self) -> QTextFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatC1ERKS_()};
    let ctysz: c_int = unsafe{QTextFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextFormatC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextFormatC1ERKS_(arg0)};
    let rsthis = QTextFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextLength QTextFormat::lengthProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn lengthProperty<RetType, T: QTextFormat_lengthProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lengthProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_lengthProperty<RetType> {
  fn lengthProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QTextLength QTextFormat::lengthProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_lengthProperty<QTextLength> for (i32) {
  fn lengthProperty(self , rsthis: & QTextFormat) -> QTextLength {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14lengthPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14lengthPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLength::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFormat::merge(const QTextFormat & other);
impl /*struct*/ QTextFormat {
  pub fn merge<RetType, T: QTextFormat_merge<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.merge(self);
    // return 1;
  }
}

pub trait QTextFormat_merge<RetType> {
  fn merge(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::merge(const QTextFormat & other);
impl<'a> /*trait*/ QTextFormat_merge<()> for (&'a QTextFormat) {
  fn merge(self , rsthis: & QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat5mergeERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextFormat5mergeERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QTextFormat::colorProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn colorProperty<RetType, T: QTextFormat_colorProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.colorProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_colorProperty<RetType> {
  fn colorProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QColor QTextFormat::colorProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_colorProperty<QColor> for (i32) {
  fn colorProperty(self , rsthis: & QTextFormat) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13colorPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat13colorPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFormat::QTextFormat();
impl<'a> /*trait*/ QTextFormat_New for () {
  fn New(self) -> QTextFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatC1Ev()};
    let ctysz: c_int = unsafe{QTextFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN11QTextFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextFormatC1Ev()};
    let rsthis = QTextFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextFormat::boolProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn boolProperty<RetType, T: QTextFormat_boolProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boolProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_boolProperty<RetType> {
  fn boolProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::boolProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_boolProperty<i8> for (i32) {
  fn boolProperty(self , rsthis: & QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12boolPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat12boolPropertyEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFormat::QTextFormat(int type);
impl<'a> /*trait*/ QTextFormat_New for (i32) {
  fn New(self) -> QTextFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormatC1Ei()};
    let ctysz: c_int = unsafe{QTextFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN11QTextFormatC1Ei(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextFormatC1Ei(arg0)};
    let rsthis = QTextFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextFormat::clearProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn clearProperty<RetType, T: QTextFormat_clearProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_clearProperty<RetType> {
  fn clearProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::clearProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_clearProperty<()> for (i32) {
  fn clearProperty(self , rsthis: & QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13clearPropertyEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat13clearPropertyEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextFrameFormat QTextFormat::toFrameFormat();
impl /*struct*/ QTextFormat {
  pub fn toFrameFormat<RetType, T: QTextFormat_toFrameFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toFrameFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toFrameFormat<RetType> {
  fn toFrameFormat(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QTextFrameFormat QTextFormat::toFrameFormat();
impl<'a> /*trait*/ QTextFormat_toFrameFormat<QTextFrameFormat> for () {
  fn toFrameFormat(self , rsthis: & QTextFormat) -> QTextFrameFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toFrameFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toFrameFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrameFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn lengthVectorProperty<RetType, T: QTextFormat_lengthVectorProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lengthVectorProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_lengthVectorProperty<RetType> {
  fn lengthVectorProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_lengthVectorProperty<()> for (i32) {
  fn lengthVectorProperty(self , rsthis: & QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat20lengthVectorPropertyEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK11QTextFormat20lengthVectorPropertyEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextFormat::propertyCount();
impl /*struct*/ QTextFormat {
  pub fn propertyCount<RetType, T: QTextFormat_propertyCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.propertyCount(self);
    // return 1;
  }
}

pub trait QTextFormat_propertyCount<RetType> {
  fn propertyCount(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::propertyCount();
impl<'a> /*trait*/ QTextFormat_propertyCount<i32> for () {
  fn propertyCount(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13propertyCountEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13propertyCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPen QTextFormat::penProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn penProperty<RetType, T: QTextFormat_penProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.penProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_penProperty<RetType> {
  fn penProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QPen QTextFormat::penProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_penProperty<QPen> for (i32) {
  fn penProperty(self , rsthis: & QTextFormat) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11penPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat11penPropertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QPen::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QTextFormat::property(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn property<RetType, T: QTextFormat_property<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.property(self);
    // return 1;
  }
}

pub trait QTextFormat_property<RetType> {
  fn property(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QVariant QTextFormat::property(int propertyId);
impl<'a> /*trait*/ QTextFormat_property<QVariant> for (i32) {
  fn property(self , rsthis: & QTextFormat) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat8propertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat8propertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFormat::setProperty(int propertyId, const QVariant & value);
impl /*struct*/ QTextFormat {
  pub fn setProperty<RetType, T: QTextFormat_setProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_setProperty<RetType> {
  fn setProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::setProperty(int propertyId, const QVariant & value);
impl<'a> /*trait*/ QTextFormat_setProperty<()> for (i32, &'a QVariant) {
  fn setProperty(self , rsthis: & QTextFormat) -> () {
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
  pub fn type_<RetType, T: QTextFormat_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QTextFormat_type_<RetType> {
  fn type_(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::type();
impl<'a> /*trait*/ QTextFormat_type_<i32> for () {
  fn type_(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat4typeEv()};
    let mut ret = unsafe {_ZNK11QTextFormat4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qreal QTextFormat::doubleProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn doubleProperty<RetType, T: QTextFormat_doubleProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doubleProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_doubleProperty<RetType> {
  fn doubleProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  qreal QTextFormat::doubleProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_doubleProperty<f64> for (i32) {
  fn doubleProperty(self , rsthis: & QTextFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat14doublePropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat14doublePropertyEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QTextImageFormat QTextFormat::toImageFormat();
impl /*struct*/ QTextFormat {
  pub fn toImageFormat<RetType, T: QTextFormat_toImageFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toImageFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toImageFormat<RetType> {
  fn toImageFormat(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QTextImageFormat QTextFormat::toImageFormat();
impl<'a> /*trait*/ QTextFormat_toImageFormat<QTextImageFormat> for () {
  fn toImageFormat(self , rsthis: & QTextFormat) -> QTextImageFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toImageFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toImageFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextImageFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextFormat::hasProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn hasProperty<RetType, T: QTextFormat_hasProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_hasProperty<RetType> {
  fn hasProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  bool QTextFormat::hasProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_hasProperty<i8> for (i32) {
  fn hasProperty(self , rsthis: & QTextFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat11hasPropertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextFormat11hasPropertyEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFormat::setObjectType(int type);
impl /*struct*/ QTextFormat {
  pub fn setObjectType<RetType, T: QTextFormat_setObjectType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setObjectType(self);
    // return 1;
  }
}

pub trait QTextFormat_setObjectType<RetType> {
  fn setObjectType(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  void QTextFormat::setObjectType(int type);
impl<'a> /*trait*/ QTextFormat_setObjectType<()> for (i32) {
  fn setObjectType(self , rsthis: & QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextFormat13setObjectTypeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextFormat13setObjectTypeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextTableFormat QTextFormat::toTableFormat();
impl /*struct*/ QTextFormat {
  pub fn toTableFormat<RetType, T: QTextFormat_toTableFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toTableFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toTableFormat<RetType> {
  fn toTableFormat(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QTextTableFormat QTextFormat::toTableFormat();
impl<'a> /*trait*/ QTextFormat_toTableFormat<QTextTableFormat> for () {
  fn toTableFormat(self , rsthis: & QTextFormat) -> QTextTableFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat13toTableFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat13toTableFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextTableFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextFormat::intProperty(int propertyId);
impl /*struct*/ QTextFormat {
  pub fn intProperty<RetType, T: QTextFormat_intProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intProperty(self);
    // return 1;
  }
}

pub trait QTextFormat_intProperty<RetType> {
  fn intProperty(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  int QTextFormat::intProperty(int propertyId);
impl<'a> /*trait*/ QTextFormat_intProperty<i32> for (i32) {
  fn intProperty(self , rsthis: & QTextFormat) -> i32 {
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
  pub fn toCharFormat<RetType, T: QTextFormat_toCharFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toCharFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toCharFormat<RetType> {
  fn toCharFormat(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QTextCharFormat QTextFormat::toCharFormat();
impl<'a> /*trait*/ QTextFormat_toCharFormat<QTextCharFormat> for () {
  fn toCharFormat(self , rsthis: & QTextFormat) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12toCharFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12toCharFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextTableCellFormat QTextFormat::toTableCellFormat();
impl /*struct*/ QTextFormat {
  pub fn toTableCellFormat<RetType, T: QTextFormat_toTableCellFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toTableCellFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toTableCellFormat<RetType> {
  fn toTableCellFormat(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QTextTableCellFormat QTextFormat::toTableCellFormat();
impl<'a> /*trait*/ QTextFormat_toTableCellFormat<QTextTableCellFormat> for () {
  fn toTableCellFormat(self , rsthis: & QTextFormat) -> QTextTableCellFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat17toTableCellFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat17toTableCellFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextTableCellFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextListFormat QTextFormat::toListFormat();
impl /*struct*/ QTextFormat {
  pub fn toListFormat<RetType, T: QTextFormat_toListFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toListFormat(self);
    // return 1;
  }
}

pub trait QTextFormat_toListFormat<RetType> {
  fn toListFormat(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QTextListFormat QTextFormat::toListFormat();
impl<'a> /*trait*/ QTextFormat_toListFormat<QTextListFormat> for () {
  fn toListFormat(self , rsthis: & QTextFormat) -> QTextListFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat12toListFormatEv()};
    let mut ret = unsafe {_ZNK11QTextFormat12toListFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextListFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QMap<int, QVariant> QTextFormat::properties();
impl /*struct*/ QTextFormat {
  pub fn properties<RetType, T: QTextFormat_properties<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.properties(self);
    // return 1;
  }
}

pub trait QTextFormat_properties<RetType> {
  fn properties(self , rsthis: & QTextFormat) -> RetType;
}

  // proto:  QMap<int, QVariant> QTextFormat::properties();
impl<'a> /*trait*/ QTextFormat_properties<()> for () {
  fn properties(self , rsthis: & QTextFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextFormat10propertiesEv()};
     unsafe {_ZNK11QTextFormat10propertiesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextBlockFormat {
    return QTextBlockFormat{qbase: QTextFormat::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTextBlockFormat {
  type Target = QTextFormat;

  fn deref(&self) -> &QTextFormat {
    return & self.qbase;
  }
}
impl AsRef<QTextFormat> for QTextBlockFormat {
  fn as_ref(& self) -> & QTextFormat {
    return & self.qbase;
  }
}
  // proto:  void QTextBlockFormat::QTextBlockFormat();
impl /*struct*/ QTextBlockFormat {
  pub fn New<T: QTextBlockFormat_New>(value: T) -> QTextBlockFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockFormat_New {
  fn New(self) -> QTextBlockFormat;
}

  // proto:  void QTextBlockFormat::QTextBlockFormat();
impl<'a> /*trait*/ QTextBlockFormat_New for () {
  fn New(self) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormatC1Ev()};
    let ctysz: c_int = unsafe{QTextBlockFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN16QTextBlockFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN16QTextBlockFormatC1Ev()};
    let rsthis = QTextBlockFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setIndent(int indent);
impl /*struct*/ QTextBlockFormat {
  pub fn setIndent<RetType, T: QTextBlockFormat_setIndent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIndent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setIndent<RetType> {
  fn setIndent(self , rsthis: & QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setIndent(int indent);
impl<'a> /*trait*/ QTextBlockFormat_setIndent<()> for (i32) {
  fn setIndent(self , rsthis: & QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat9setIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTextBlockFormat9setIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::QTextBlockFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextBlockFormat_New for (&'a QTextFormat) {
  fn New(self) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormatC1ERK11QTextFormat()};
    let ctysz: c_int = unsafe{QTextBlockFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QTextBlockFormatC1ERK11QTextFormat(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QTextBlockFormatC1ERK11QTextFormat(arg0)};
    let rsthis = QTextBlockFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
impl /*struct*/ QTextBlockFormat {
  pub fn lineHeight<RetType, T: QTextBlockFormat_lineHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineHeight(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_lineHeight<RetType> {
  fn lineHeight(self , rsthis: & QTextBlockFormat) -> RetType;
}

  // proto:  qreal QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
impl<'a> /*trait*/ QTextBlockFormat_lineHeight<f64> for (f64, f64) {
  fn lineHeight(self , rsthis: & QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10lineHeightEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK16QTextBlockFormat10lineHeightEdd(rsthis.qclsinst, arg0, arg1)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextCharFormat {
    return QTextCharFormat{qbase: QTextFormat::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTextCharFormat {
  type Target = QTextFormat;

  fn deref(&self) -> &QTextFormat {
    return & self.qbase;
  }
}
impl AsRef<QTextFormat> for QTextCharFormat {
  fn as_ref(& self) -> & QTextFormat {
    return & self.qbase;
  }
}
  // proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
impl /*struct*/ QTextCharFormat {
  pub fn setTableCellColumnSpan<RetType, T: QTextCharFormat_setTableCellColumnSpan<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTableCellColumnSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTableCellColumnSpan<RetType> {
  fn setTableCellColumnSpan(self , rsthis: & QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellColumnSpan<()> for (i32) {
  fn setTableCellColumnSpan(self , rsthis: & QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat22setTableCellColumnSpanEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat22setTableCellColumnSpanEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::fontUnderline();
impl /*struct*/ QTextCharFormat {
  pub fn fontUnderline<RetType, T: QTextCharFormat_fontUnderline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontUnderline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontUnderline<RetType> {
  fn fontUnderline(self , rsthis: & QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::fontUnderline();
impl<'a> /*trait*/ QTextCharFormat_fontUnderline<i8> for () {
  fn fontUnderline(self , rsthis: & QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontUnderlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontUnderlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFont(const QFont & font);
impl /*struct*/ QTextCharFormat {
  pub fn setFont<RetType, T: QTextCharFormat_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFont<RetType> {
  fn setFont(self , rsthis: & QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFont(const QFont & font);
impl<'a> /*trait*/ QTextCharFormat_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFont QTextCharFormat::font();
impl /*struct*/ QTextCharFormat {
  pub fn font<RetType, T: QTextCharFormat_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTextCharFormat_font<RetType> {
  fn font(self , rsthis: & QTextCharFormat) -> RetType;
}

  // proto:  QFont QTextCharFormat::font();
impl<'a> /*trait*/ QTextCharFormat_font<QFont> for () {
  fn font(self , rsthis: & QTextCharFormat) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat4fontEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::QTextCharFormat();
impl /*struct*/ QTextCharFormat {
  pub fn New<T: QTextCharFormat_New>(value: T) -> QTextCharFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCharFormat_New {
  fn New(self) -> QTextCharFormat;
}

  // proto:  void QTextCharFormat::QTextCharFormat();
impl<'a> /*trait*/ QTextCharFormat_New for () {
  fn New(self) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormatC1Ev()};
    let ctysz: c_int = unsafe{QTextCharFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN15QTextCharFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTextCharFormatC1Ev()};
    let rsthis = QTextCharFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::QTextCharFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextCharFormat_New for (&'a QTextFormat) {
  fn New(self) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormatC1ERK11QTextFormat()};
    let ctysz: c_int = unsafe{QTextCharFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QTextCharFormatC1ERK11QTextFormat(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTextCharFormatC1ERK11QTextFormat(arg0)};
    let rsthis = QTextCharFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringList QTextCharFormat::anchorNames();
impl /*struct*/ QTextCharFormat {
  pub fn anchorNames<RetType, T: QTextCharFormat_anchorNames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.anchorNames(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorNames<RetType> {
  fn anchorNames(self , rsthis: & QTextCharFormat) -> RetType;
}

  // proto:  QStringList QTextCharFormat::anchorNames();
impl<'a> /*trait*/ QTextCharFormat_anchorNames<()> for () {
  fn anchorNames(self , rsthis: & QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11anchorNamesEv()};
     unsafe {_ZNK15QTextCharFormat11anchorNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTextCharFormat::anchorName();
impl /*struct*/ QTextCharFormat {
  pub fn anchorName<RetType, T: QTextCharFormat_anchorName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.anchorName(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorName<RetType> {
  fn anchorName(self , rsthis: & QTextCharFormat) -> RetType;
}

  // proto:  QString QTextCharFormat::anchorName();
impl<'a> /*trait*/ QTextCharFormat_anchorName<QString> for () {
  fn anchorName(self , rsthis: & QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorNameEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10anchorNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
impl /*struct*/ QTextCharFormat {
  pub fn setTableCellRowSpan<RetType, T: QTextCharFormat_setTableCellRowSpan<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTableCellRowSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTableCellRowSpan<RetType> {
  fn setTableCellRowSpan(self , rsthis: & QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellRowSpan<()> for (i32) {
  fn setTableCellRowSpan(self , rsthis: & QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat19setTableCellRowSpanEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat19setTableCellRowSpanEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextTableFormat {
    return QTextTableFormat{qbase: QTextFrameFormat::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTextTableFormat {
  type Target = QTextFrameFormat;

  fn deref(&self) -> &QTextFrameFormat {
    return & self.qbase;
  }
}
impl AsRef<QTextFrameFormat> for QTextTableFormat {
  fn as_ref(& self) -> & QTextFrameFormat {
    return & self.qbase;
  }
}
  // proto:  void QTextTableFormat::QTextTableFormat();
impl /*struct*/ QTextTableFormat {
  pub fn New<T: QTextTableFormat_New>(value: T) -> QTextTableFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableFormat_New {
  fn New(self) -> QTextTableFormat;
}

  // proto:  void QTextTableFormat::QTextTableFormat();
impl<'a> /*trait*/ QTextTableFormat_New for () {
  fn New(self) -> QTextTableFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormatC1Ev()};
    let ctysz: c_int = unsafe{QTextTableFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN16QTextTableFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN16QTextTableFormatC1Ev()};
    let rsthis = QTextTableFormat{/**/qbase: QTextFrameFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setColumns(int columns);
impl /*struct*/ QTextTableFormat {
  pub fn setColumns<RetType, T: QTextTableFormat_setColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumns(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setColumns<RetType> {
  fn setColumns(self , rsthis: & QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setColumns(int columns);
impl<'a> /*trait*/ QTextTableFormat_setColumns<()> for (i32) {
  fn setColumns(self , rsthis: & QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat10setColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTextTableFormat10setColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::QTextTableFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextTableFormat_New for (&'a QTextFormat) {
  fn New(self) -> QTextTableFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormatC1ERK11QTextFormat()};
    let ctysz: c_int = unsafe{QTextTableFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QTextTableFormatC1ERK11QTextFormat(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QTextTableFormatC1ERK11QTextFormat(arg0)};
    let rsthis = QTextTableFormat{/**/qbase: QTextFrameFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
impl /*struct*/ QTextTableFormat {
  pub fn setCellPadding<RetType, T: QTextTableFormat_setCellPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCellPadding(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setCellPadding<RetType> {
  fn setCellPadding(self , rsthis: & QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
impl<'a> /*trait*/ QTextTableFormat_setCellPadding<()> for (f64) {
  fn setCellPadding(self , rsthis: & QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat14setCellPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextTableFormat14setCellPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextTableCellFormat {
    return QTextTableCellFormat{qbase: QTextCharFormat::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTextTableCellFormat {
  type Target = QTextCharFormat;

  fn deref(&self) -> &QTextCharFormat {
    return & self.qbase;
  }
}
impl AsRef<QTextCharFormat> for QTextTableCellFormat {
  fn as_ref(& self) -> & QTextCharFormat {
    return & self.qbase;
  }
}
  // proto:  void QTextTableCellFormat::QTextTableCellFormat();
impl /*struct*/ QTextTableCellFormat {
  pub fn New<T: QTextTableCellFormat_New>(value: T) -> QTextTableCellFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCellFormat_New {
  fn New(self) -> QTextTableCellFormat;
}

  // proto:  void QTextTableCellFormat::QTextTableCellFormat();
impl<'a> /*trait*/ QTextTableCellFormat_New for () {
  fn New(self) -> QTextTableCellFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormatC1Ev()};
    let ctysz: c_int = unsafe{QTextTableCellFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN20QTextTableCellFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN20QTextTableCellFormatC1Ev()};
    let rsthis = QTextTableCellFormat{/**/qbase: QTextCharFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setLeftPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setLeftPadding<RetType, T: QTextTableCellFormat_setLeftPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLeftPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setLeftPadding<RetType> {
  fn setLeftPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setLeftPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setLeftPadding<()> for (f64) {
  fn setLeftPadding(self , rsthis: & QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat14setLeftPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat14setLeftPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setTopPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setTopPadding<RetType, T: QTextTableCellFormat_setTopPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTopPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setTopPadding<RetType> {
  fn setTopPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setTopPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setTopPadding<()> for (f64) {
  fn setTopPadding(self , rsthis: & QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat13setTopPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat13setTopPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextTableCellFormat::leftPadding();
impl /*struct*/ QTextTableCellFormat {
  pub fn leftPadding<RetType, T: QTextTableCellFormat_leftPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leftPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_leftPadding<RetType> {
  fn leftPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  qreal QTextTableCellFormat::leftPadding();
impl<'a> /*trait*/ QTextTableCellFormat_leftPadding<f64> for () {
  fn leftPadding(self , rsthis: & QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat11leftPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat11leftPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setPadding<RetType, T: QTextTableCellFormat_setPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setPadding<RetType> {
  fn setPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setPadding<()> for (f64) {
  fn setPadding(self , rsthis: & QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat10setPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat10setPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextTableCellFormat::topPadding();
impl /*struct*/ QTextTableCellFormat {
  pub fn topPadding<RetType, T: QTextTableCellFormat_topPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_topPadding<RetType> {
  fn topPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  qreal QTextTableCellFormat::topPadding();
impl<'a> /*trait*/ QTextTableCellFormat_topPadding<f64> for () {
  fn topPadding(self , rsthis: & QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat10topPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat10topPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextTableCellFormat::rightPadding();
impl /*struct*/ QTextTableCellFormat {
  pub fn rightPadding<RetType, T: QTextTableCellFormat_rightPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rightPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_rightPadding<RetType> {
  fn rightPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  qreal QTextTableCellFormat::rightPadding();
impl<'a> /*trait*/ QTextTableCellFormat_rightPadding<f64> for () {
  fn rightPadding(self , rsthis: & QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat12rightPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat12rightPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::QTextTableCellFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextTableCellFormat_New for (&'a QTextFormat) {
  fn New(self) -> QTextTableCellFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormatC1ERK11QTextFormat()};
    let ctysz: c_int = unsafe{QTextTableCellFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN20QTextTableCellFormatC1ERK11QTextFormat(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN20QTextTableCellFormatC1ERK11QTextFormat(arg0)};
    let rsthis = QTextTableCellFormat{/**/qbase: QTextCharFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextTableCellFormat::bottomPadding();
impl /*struct*/ QTextTableCellFormat {
  pub fn bottomPadding<RetType, T: QTextTableCellFormat_bottomPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottomPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_bottomPadding<RetType> {
  fn bottomPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  qreal QTextTableCellFormat::bottomPadding();
impl<'a> /*trait*/ QTextTableCellFormat_bottomPadding<f64> for () {
  fn bottomPadding(self , rsthis: & QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat13bottomPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat13bottomPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setRightPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setRightPadding<RetType, T: QTextTableCellFormat_setRightPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRightPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setRightPadding<RetType> {
  fn setRightPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setRightPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setRightPadding<()> for (f64) {
  fn setRightPadding(self , rsthis: & QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat15setRightPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat15setRightPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setBottomPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setBottomPadding<RetType, T: QTextTableCellFormat_setBottomPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBottomPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setBottomPadding<RetType> {
  fn setBottomPadding(self , rsthis: & QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setBottomPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setBottomPadding<()> for (f64) {
  fn setBottomPadding(self , rsthis: & QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat16setBottomPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat16setBottomPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextListFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextListFormat {
    return QTextListFormat{qbase: QTextFormat::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTextListFormat {
  type Target = QTextFormat;

  fn deref(&self) -> &QTextFormat {
    return & self.qbase;
  }
}
impl AsRef<QTextFormat> for QTextListFormat {
  fn as_ref(& self) -> & QTextFormat {
    return & self.qbase;
  }
}
  // proto:  void QTextListFormat::QTextListFormat(const QTextFormat & fmt);
impl /*struct*/ QTextListFormat {
  pub fn New<T: QTextListFormat_New>(value: T) -> QTextListFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextListFormat_New {
  fn New(self) -> QTextListFormat;
}

  // proto:  void QTextListFormat::QTextListFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextListFormat_New for (&'a QTextFormat) {
  fn New(self) -> QTextListFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormatC1ERK11QTextFormat()};
    let ctysz: c_int = unsafe{QTextListFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QTextListFormatC1ERK11QTextFormat(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTextListFormatC1ERK11QTextFormat(arg0)};
    let rsthis = QTextListFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextListFormat::setIndent(int indent);
impl /*struct*/ QTextListFormat {
  pub fn setIndent<RetType, T: QTextListFormat_setIndent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIndent(self);
    // return 1;
  }
}

pub trait QTextListFormat_setIndent<RetType> {
  fn setIndent(self , rsthis: & QTextListFormat) -> RetType;
}

  // proto:  void QTextListFormat::setIndent(int indent);
impl<'a> /*trait*/ QTextListFormat_setIndent<()> for (i32) {
  fn setIndent(self , rsthis: & QTextListFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat9setIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextListFormat9setIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
impl /*struct*/ QTextListFormat {
  pub fn setNumberSuffix<RetType, T: QTextListFormat_setNumberSuffix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNumberSuffix(self);
    // return 1;
  }
}

pub trait QTextListFormat_setNumberSuffix<RetType> {
  fn setNumberSuffix(self , rsthis: & QTextListFormat) -> RetType;
}

  // proto:  void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
impl<'a> /*trait*/ QTextListFormat_setNumberSuffix<()> for (&'a QString) {
  fn setNumberSuffix(self , rsthis: & QTextListFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat15setNumberSuffixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextListFormat15setNumberSuffixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextListFormat::QTextListFormat();
impl<'a> /*trait*/ QTextListFormat_New for () {
  fn New(self) -> QTextListFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormatC1Ev()};
    let ctysz: c_int = unsafe{QTextListFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN15QTextListFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTextListFormatC1Ev()};
    let rsthis = QTextListFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
impl /*struct*/ QTextListFormat {
  pub fn setNumberPrefix<RetType, T: QTextListFormat_setNumberPrefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNumberPrefix(self);
    // return 1;
  }
}

pub trait QTextListFormat_setNumberPrefix<RetType> {
  fn setNumberPrefix(self , rsthis: & QTextListFormat) -> RetType;
}

  // proto:  void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
impl<'a> /*trait*/ QTextListFormat_setNumberPrefix<()> for (&'a QString) {
  fn setNumberPrefix(self , rsthis: & QTextListFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat15setNumberPrefixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextListFormat15setNumberPrefixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextFrameFormat {
    return QTextFrameFormat{qbase: QTextFormat::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTextFrameFormat {
  type Target = QTextFormat;

  fn deref(&self) -> &QTextFormat {
    return & self.qbase;
  }
}
impl AsRef<QTextFormat> for QTextFrameFormat {
  fn as_ref(& self) -> & QTextFormat {
    return & self.qbase;
  }
}
  // proto:  void QTextFrameFormat::setRightMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setRightMargin<RetType, T: QTextFrameFormat_setRightMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRightMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setRightMargin<RetType> {
  fn setRightMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setRightMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setRightMargin<()> for (f64) {
  fn setRightMargin(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat14setRightMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat14setRightMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setMargin<RetType, T: QTextFrameFormat_setMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setMargin<RetType> {
  fn setMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setMargin<()> for (f64) {
  fn setMargin(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::bottomMargin();
impl /*struct*/ QTextFrameFormat {
  pub fn bottomMargin<RetType, T: QTextFrameFormat_bottomMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottomMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_bottomMargin<RetType> {
  fn bottomMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::bottomMargin();
impl<'a> /*trait*/ QTextFrameFormat_bottomMargin<f64> for () {
  fn bottomMargin(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat12bottomMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat12bottomMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::QTextFrameFormat(const QTextFormat & fmt);
impl /*struct*/ QTextFrameFormat {
  pub fn New<T: QTextFrameFormat_New>(value: T) -> QTextFrameFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFrameFormat_New {
  fn New(self) -> QTextFrameFormat;
}

  // proto:  void QTextFrameFormat::QTextFrameFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextFrameFormat_New for (&'a QTextFormat) {
  fn New(self) -> QTextFrameFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormatC1ERK11QTextFormat()};
    let ctysz: c_int = unsafe{QTextFrameFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QTextFrameFormatC1ERK11QTextFormat(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN16QTextFrameFormatC1ERK11QTextFormat(arg0)};
    let rsthis = QTextFrameFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setBorder(qreal border);
impl /*struct*/ QTextFrameFormat {
  pub fn setBorder<RetType, T: QTextFrameFormat_setBorder<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBorder(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setBorder<RetType> {
  fn setBorder(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setBorder(qreal border);
impl<'a> /*trait*/ QTextFrameFormat_setBorder<()> for (f64) {
  fn setBorder(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setBorderEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setBorderEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setBottomMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setBottomMargin<RetType, T: QTextFrameFormat_setBottomMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBottomMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setBottomMargin<RetType> {
  fn setBottomMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setBottomMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setBottomMargin<()> for (f64) {
  fn setBottomMargin(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat15setBottomMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat15setBottomMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setHeight(const QTextLength & height);
impl /*struct*/ QTextFrameFormat {
  pub fn setHeight<RetType, T: QTextFrameFormat_setHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setHeight<RetType> {
  fn setHeight(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setHeight(const QTextLength & height);
impl<'a> /*trait*/ QTextFrameFormat_setHeight<()> for (&'a QTextLength) {
  fn setHeight(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setHeightERK11QTextLength()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextFrameFormat9setHeightERK11QTextLength(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setLeftMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setLeftMargin<RetType, T: QTextFrameFormat_setLeftMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLeftMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setLeftMargin<RetType> {
  fn setLeftMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setLeftMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setLeftMargin<()> for (f64) {
  fn setLeftMargin(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat13setLeftMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat13setLeftMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setHeight(qreal height);
impl<'a> /*trait*/ QTextFrameFormat_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setWidth(qreal width);
impl /*struct*/ QTextFrameFormat {
  pub fn setWidth<RetType, T: QTextFrameFormat_setWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setWidth<RetType> {
  fn setWidth(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setWidth(qreal width);
impl<'a> /*trait*/ QTextFrameFormat_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::QTextFrameFormat();
impl<'a> /*trait*/ QTextFrameFormat_New for () {
  fn New(self) -> QTextFrameFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormatC1Ev()};
    let ctysz: c_int = unsafe{QTextFrameFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN16QTextFrameFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN16QTextFrameFormatC1Ev()};
    let rsthis = QTextFrameFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::rightMargin();
impl /*struct*/ QTextFrameFormat {
  pub fn rightMargin<RetType, T: QTextFrameFormat_rightMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rightMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_rightMargin<RetType> {
  fn rightMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::rightMargin();
impl<'a> /*trait*/ QTextFrameFormat_rightMargin<f64> for () {
  fn rightMargin(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat11rightMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat11rightMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setPadding(qreal padding);
impl /*struct*/ QTextFrameFormat {
  pub fn setPadding<RetType, T: QTextFrameFormat_setPadding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPadding(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setPadding<RetType> {
  fn setPadding(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setPadding(qreal padding);
impl<'a> /*trait*/ QTextFrameFormat_setPadding<()> for (f64) {
  fn setPadding(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat10setPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat10setPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::leftMargin();
impl /*struct*/ QTextFrameFormat {
  pub fn leftMargin<RetType, T: QTextFrameFormat_leftMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leftMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_leftMargin<RetType> {
  fn leftMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::leftMargin();
impl<'a> /*trait*/ QTextFrameFormat_leftMargin<f64> for () {
  fn leftMargin(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat10leftMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat10leftMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setTopMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setTopMargin<RetType, T: QTextFrameFormat_setTopMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTopMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setTopMargin<RetType> {
  fn setTopMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setTopMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setTopMargin<()> for (f64) {
  fn setTopMargin(self , rsthis: & QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat12setTopMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat12setTopMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::topMargin();
impl /*struct*/ QTextFrameFormat {
  pub fn topMargin<RetType, T: QTextFrameFormat_topMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_topMargin<RetType> {
  fn topMargin(self , rsthis: & QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::topMargin();
impl<'a> /*trait*/ QTextFrameFormat_topMargin<f64> for () {
  fn topMargin(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat9topMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat9topMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

