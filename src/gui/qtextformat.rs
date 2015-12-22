// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
// use super::qtextformat::QTextLength; // 773
use super::qcolor::QColor; // 773
use super::qbrush::QBrush; // 773
// use super::qtextformat::QTextFrameFormat; // 773
use super::qpen::QPen; // 773
use super::super::core::qvariant::QVariant; // 771
// use super::qtextformat::QTextImageFormat; // 773
// use super::qtextformat::QTextTableFormat; // 773
// use super::qtextformat::QTextTableCellFormat; // 773
// use super::qtextformat::QTextListFormat; // 773
use super::qfont::QFont; // 773
use super::super::core::qstringlist::QStringList; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  qreal QTextLength::value(qreal maximumLength);
  fn _ZNK11QTextLength5valueEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QTextLength::QTextLength();
  fn _ZN11QTextLengthC1Ev(qthis: *mut c_void);
  // proto:  qreal QTextLength::rawValue();
  fn _ZNK11QTextLength8rawValueEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextImageFormat::QTextImageFormat();
  fn _ZN16QTextImageFormatC1Ev(qthis: *mut c_void);
  // proto:  bool QTextImageFormat::isValid();
  fn _ZNK16QTextImageFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  qreal QTextImageFormat::width();
  fn _ZNK16QTextImageFormat5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextImageFormat::QTextImageFormat(const QTextFormat & format);
  fn _ZN16QTextImageFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextImageFormat::setHeight(qreal height);
  fn _ZN16QTextImageFormat9setHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextImageFormat::setWidth(qreal width);
  fn _ZN16QTextImageFormat8setWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextImageFormat::setName(const QString & name);
  fn _ZN16QTextImageFormat7setNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QTextImageFormat::name();
  fn _ZNK16QTextImageFormat4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTextImageFormat::height();
  fn _ZNK16QTextImageFormat6heightEv(qthis: *mut c_void) -> c_double;
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
  // proto:  int QTextBlockFormat::indent();
  fn _ZNK16QTextBlockFormat6indentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBlockFormat::setTextIndent(qreal aindent);
  fn _ZN16QTextBlockFormat13setTextIndentEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextBlockFormat::setNonBreakableLines(bool b);
  fn _ZN16QTextBlockFormat20setNonBreakableLinesEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextBlockFormat::setIndent(int indent);
  fn _ZN16QTextBlockFormat9setIndentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  qreal QTextBlockFormat::textIndent();
  fn _ZNK16QTextBlockFormat10textIndentEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTextBlockFormat::lineHeight();
  fn _ZNK16QTextBlockFormat10lineHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextBlockFormat::QTextBlockFormat(const QTextFormat & fmt);
  fn _ZN16QTextBlockFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
  fn _ZNK16QTextBlockFormat10lineHeightEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> c_double;
  // proto:  void QTextBlockFormat::setRightMargin(qreal margin);
  fn _ZN16QTextBlockFormat14setRightMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextBlockFormat::topMargin();
  fn _ZNK16QTextBlockFormat9topMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextBlockFormat::QTextBlockFormat();
  fn _ZN16QTextBlockFormatC1Ev(qthis: *mut c_void);
  // proto:  qreal QTextBlockFormat::rightMargin();
  fn _ZNK16QTextBlockFormat11rightMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTextBlockFormat::bottomMargin();
  fn _ZNK16QTextBlockFormat12bottomMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextBlockFormat::setTopMargin(qreal margin);
  fn _ZN16QTextBlockFormat12setTopMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextBlockFormat::leftMargin();
  fn _ZNK16QTextBlockFormat10leftMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextBlockFormat::setLineHeight(qreal height, int heightType);
  fn _ZN16QTextBlockFormat13setLineHeightEdi(qthis: *mut c_void, arg0: c_double, arg1: c_int);
  // proto:  void QTextBlockFormat::setBottomMargin(qreal margin);
  fn _ZN16QTextBlockFormat15setBottomMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  int QTextBlockFormat::lineHeightType();
  fn _ZNK16QTextBlockFormat14lineHeightTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBlockFormat::setLeftMargin(qreal margin);
  fn _ZN16QTextBlockFormat13setLeftMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  bool QTextBlockFormat::isValid();
  fn _ZNK16QTextBlockFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTextBlockFormat::nonBreakableLines();
  fn _ZNK16QTextBlockFormat17nonBreakableLinesEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCharFormat::setFontLetterSpacing(qreal spacing);
  fn _ZN15QTextCharFormat20setFontLetterSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  bool QTextCharFormat::isAnchor();
  fn _ZNK15QTextCharFormat8isAnchorEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCharFormat::setFont(const QFont & font);
  fn _ZN15QTextCharFormat7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextCharFormat::fontOverline();
  fn _ZNK15QTextCharFormat12fontOverlineEv(qthis: *mut c_void) -> c_char;
  // proto:  QFont QTextCharFormat::font();
  fn _ZNK15QTextCharFormat4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextCharFormat::fontFamily();
  fn _ZNK15QTextCharFormat10fontFamilyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextCharFormat::fontStrikeOut();
  fn _ZNK15QTextCharFormat13fontStrikeOutEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCharFormat::setFontPointSize(qreal size);
  fn _ZN15QTextCharFormat16setFontPointSizeEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextCharFormat::setUnderlineColor(const QColor & color);
  fn _ZN15QTextCharFormat17setUnderlineColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextCharFormat::tableCellRowSpan();
  fn _ZNK15QTextCharFormat16tableCellRowSpanEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCharFormat::setFontUnderline(bool underline);
  fn _ZN15QTextCharFormat16setFontUnderlineEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QTextCharFormat::isValid();
  fn _ZNK15QTextCharFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTextCharFormat::fontItalic();
  fn _ZNK15QTextCharFormat10fontItalicEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCharFormat::setToolTip(const QString & tip);
  fn _ZN15QTextCharFormat10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextCharFormat::setTextOutline(const QPen & pen);
  fn _ZN15QTextCharFormat14setTextOutlineERK4QPen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
  fn _ZN15QTextCharFormat19setTableCellRowSpanEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextCharFormat::setAnchor(bool anchor);
  fn _ZN15QTextCharFormat9setAnchorEb(qthis: *mut c_void, arg0: c_char);
  // proto:  qreal QTextCharFormat::fontPointSize();
  fn _ZNK15QTextCharFormat13fontPointSizeEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextCharFormat::QTextCharFormat(const QTextFormat & fmt);
  fn _ZN15QTextCharFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextCharFormat::setFontStrikeOut(bool strikeOut);
  fn _ZN15QTextCharFormat16setFontStrikeOutEb(qthis: *mut c_void, arg0: c_char);
  // proto:  qreal QTextCharFormat::fontWordSpacing();
  fn _ZNK15QTextCharFormat15fontWordSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QTextCharFormat::toolTip();
  fn _ZNK15QTextCharFormat7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCharFormat::setAnchorNames(const QStringList & names);
  fn _ZN15QTextCharFormat14setAnchorNamesERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QTextCharFormat::anchorNames();
  fn _ZNK15QTextCharFormat11anchorNamesEv(qthis: *mut c_void);
  // proto:  void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
  fn _ZN15QTextCharFormat17setFontFixedPitchEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextCharFormat::setFontItalic(bool italic);
  fn _ZN15QTextCharFormat13setFontItalicEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextCharFormat::setFontFamily(const QString & family);
  fn _ZN15QTextCharFormat13setFontFamilyERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextCharFormat::fontFixedPitch();
  fn _ZNK15QTextCharFormat14fontFixedPitchEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCharFormat::setAnchorHref(const QString & value);
  fn _ZN15QTextCharFormat13setAnchorHrefERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextCharFormat::fontStretch();
  fn _ZNK15QTextCharFormat11fontStretchEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCharFormat::setFontKerning(bool enable);
  fn _ZN15QTextCharFormat14setFontKerningEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QTextCharFormat::tableCellColumnSpan();
  fn _ZNK15QTextCharFormat19tableCellColumnSpanEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCharFormat::QTextCharFormat();
  fn _ZN15QTextCharFormatC1Ev(qthis: *mut c_void);
  // proto:  qreal QTextCharFormat::fontLetterSpacing();
  fn _ZNK15QTextCharFormat17fontLetterSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QTextCharFormat::anchorHref();
  fn _ZNK15QTextCharFormat10anchorHrefEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextCharFormat::anchorName();
  fn _ZNK15QTextCharFormat10anchorNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCharFormat::setFontStretch(int factor);
  fn _ZN15QTextCharFormat14setFontStretchEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextCharFormat::setAnchorName(const QString & name);
  fn _ZN15QTextCharFormat13setAnchorNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextCharFormat::fontKerning();
  fn _ZNK15QTextCharFormat11fontKerningEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCharFormat::setFontWeight(int weight);
  fn _ZN15QTextCharFormat13setFontWeightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QTextCharFormat::fontUnderline();
  fn _ZNK15QTextCharFormat13fontUnderlineEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextCharFormat::setFontWordSpacing(qreal spacing);
  fn _ZN15QTextCharFormat18setFontWordSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QColor QTextCharFormat::underlineColor();
  fn _ZNK15QTextCharFormat14underlineColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextCharFormat::fontWeight();
  fn _ZNK15QTextCharFormat10fontWeightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCharFormat::setFontOverline(bool overline);
  fn _ZN15QTextCharFormat15setFontOverlineEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
  fn _ZN15QTextCharFormat22setTableCellColumnSpanEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QPen QTextCharFormat::textOutline();
  fn _ZNK15QTextCharFormat11textOutlineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextTableFormat::QTextTableFormat();
  fn _ZN16QTextTableFormatC1Ev(qthis: *mut c_void);
  // proto:  bool QTextTableFormat::isValid();
  fn _ZNK16QTextTableFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTextTableFormat::headerRowCount();
  fn _ZNK16QTextTableFormat14headerRowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextTableFormat::columns();
  fn _ZNK16QTextTableFormat7columnsEv(qthis: *mut c_void) -> c_int;
  // proto:  QVector<QTextLength> QTextTableFormat::columnWidthConstraints();
  fn _ZNK16QTextTableFormat22columnWidthConstraintsEv(qthis: *mut c_void);
  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
  fn _ZN16QTextTableFormat14setCellPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextTableFormat::cellPadding();
  fn _ZNK16QTextTableFormat11cellPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableFormat::setCellSpacing(qreal spacing);
  fn _ZN16QTextTableFormat14setCellSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextTableFormat::setColumns(int columns);
  fn _ZN16QTextTableFormat10setColumnsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextTableFormat::QTextTableFormat(const QTextFormat & fmt);
  fn _ZN16QTextTableFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextTableFormat::clearColumnWidthConstraints();
  fn _ZN16QTextTableFormat27clearColumnWidthConstraintsEv(qthis: *mut c_void);
  // proto:  void QTextTableFormat::setHeaderRowCount(int count);
  fn _ZN16QTextTableFormat17setHeaderRowCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  qreal QTextTableFormat::cellSpacing();
  fn _ZNK16QTextTableFormat11cellSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableCellFormat::QTextTableCellFormat();
  fn _ZN20QTextTableCellFormatC1Ev(qthis: *mut c_void);
  // proto:  void QTextTableCellFormat::setLeftPadding(qreal padding);
  fn _ZN20QTextTableCellFormat14setLeftPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  bool QTextTableCellFormat::isValid();
  fn _ZNK20QTextTableCellFormat7isValidEv(qthis: *mut c_void) -> c_char;
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
  fn _ZN20QTextTableCellFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextTableCellFormat::bottomPadding();
  fn _ZNK20QTextTableCellFormat13bottomPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableCellFormat::setRightPadding(qreal padding);
  fn _ZN20QTextTableCellFormat15setRightPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextTableCellFormat::setBottomPadding(qreal padding);
  fn _ZN20QTextTableCellFormat16setBottomPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  int QTextListFormat::indent();
  fn _ZNK15QTextListFormat6indentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextListFormat::QTextListFormat(const QTextFormat & fmt);
  fn _ZN15QTextListFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextListFormat::setIndent(int indent);
  fn _ZN15QTextListFormat9setIndentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString QTextListFormat::numberSuffix();
  fn _ZNK15QTextListFormat12numberSuffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextListFormat::QTextListFormat();
  fn _ZN15QTextListFormatC1Ev(qthis: *mut c_void);
  // proto:  QString QTextListFormat::numberPrefix();
  fn _ZNK15QTextListFormat12numberPrefixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextListFormat::isValid();
  fn _ZNK15QTextListFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
  fn _ZN15QTextListFormat15setNumberSuffixERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
  fn _ZN15QTextListFormat15setNumberPrefixERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextFrameFormat::isValid();
  fn _ZNK16QTextFrameFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextFrameFormat::setHeight(qreal height);
  fn _ZN16QTextFrameFormat9setHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setBorderBrush(const QBrush & brush);
  fn _ZN16QTextFrameFormat14setBorderBrushERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextFrameFormat::margin();
  fn _ZNK16QTextFrameFormat6marginEv(qthis: *mut c_void) -> c_double;
  // proto:  QBrush QTextFrameFormat::borderBrush();
  fn _ZNK16QTextFrameFormat11borderBrushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFrameFormat::setRightMargin(qreal margin);
  fn _ZN16QTextFrameFormat14setRightMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setMargin(qreal margin);
  fn _ZN16QTextFrameFormat9setMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setBorder(qreal border);
  fn _ZN16QTextFrameFormat9setBorderEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setHeight(const QTextLength & height);
  fn _ZN16QTextFrameFormat9setHeightERK11QTextLength(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextFrameFormat::setWidth(const QTextLength & length);
  fn _ZN16QTextFrameFormat8setWidthERK11QTextLength(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextFrameFormat::bottomMargin();
  fn _ZNK16QTextFrameFormat12bottomMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::setBottomMargin(qreal margin);
  fn _ZN16QTextFrameFormat15setBottomMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QTextLength QTextFrameFormat::height();
  fn _ZNK16QTextFrameFormat6heightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFrameFormat::setWidth(qreal width);
  fn _ZN16QTextFrameFormat8setWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextFrameFormat::rightMargin();
  fn _ZNK16QTextFrameFormat11rightMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::setPadding(qreal padding);
  fn _ZN16QTextFrameFormat10setPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextFrameFormat::setTopMargin(qreal margin);
  fn _ZN16QTextFrameFormat12setTopMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextFrameFormat::topMargin();
  fn _ZNK16QTextFrameFormat9topMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  QTextLength QTextFrameFormat::width();
  fn _ZNK16QTextFrameFormat5widthEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFrameFormat::QTextFrameFormat(const QTextFormat & fmt);
  fn _ZN16QTextFrameFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextFrameFormat::padding();
  fn _ZNK16QTextFrameFormat7paddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::setLeftMargin(qreal margin);
  fn _ZN16QTextFrameFormat13setLeftMarginEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextFrameFormat::border();
  fn _ZNK16QTextFrameFormat6borderEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::QTextFrameFormat();
  fn _ZN16QTextFrameFormatC1Ev(qthis: *mut c_void);
  // proto:  qreal QTextFrameFormat::leftMargin();
  fn _ZNK16QTextFrameFormat10leftMarginEv(qthis: *mut c_void) -> c_double;
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
  // proto:  qreal QTextLength::value(qreal maximumLength);
impl /*struct*/ QTextLength {
  pub fn value<RetType, T: QTextLength_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QTextLength_value<RetType> {
  fn value(self , rsthis: &mut QTextLength) -> RetType;
}

  // proto:  qreal QTextLength::value(qreal maximumLength);
impl<'a> /*trait*/ QTextLength_value<f64> for (f64) {
  fn value(self , rsthis: &mut QTextLength) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLength5valueEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK11QTextLength5valueEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextLength::QTextLength();
impl /*struct*/ QTextLength {
  pub fn NewQTextLength<T: QTextLength_NewQTextLength>(value: T) -> QTextLength {
    let rsthis = value.NewQTextLength();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLength_NewQTextLength {
  fn NewQTextLength(self) -> QTextLength;
}

  // proto:  void QTextLength::QTextLength();
impl<'a> /*trait*/ QTextLength_NewQTextLength for () {
  fn NewQTextLength(self) -> QTextLength {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLengthC1Ev()};
    unsafe {_ZN11QTextLengthC1Ev(qthis)};
    let rsthis = QTextLength{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextLength::rawValue();
impl /*struct*/ QTextLength {
  pub fn rawValue<RetType, T: QTextLength_rawValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rawValue(self);
    // return 1;
  }
}

pub trait QTextLength_rawValue<RetType> {
  fn rawValue(self , rsthis: &mut QTextLength) -> RetType;
}

  // proto:  qreal QTextLength::rawValue();
impl<'a> /*trait*/ QTextLength_rawValue<f64> for () {
  fn rawValue(self , rsthis: &mut QTextLength) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLength8rawValueEv()};
    let mut ret = unsafe {_ZNK11QTextLength8rawValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
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
    return &self.qbase;
  }
}
impl AsRef<QTextCharFormat> for QTextImageFormat {
  fn as_ref(&self) -> &QTextCharFormat {
    return &self.qbase;
  }
}
  // proto:  void QTextImageFormat::QTextImageFormat();
impl /*struct*/ QTextImageFormat {
  pub fn NewQTextImageFormat<T: QTextImageFormat_NewQTextImageFormat>(value: T) -> QTextImageFormat {
    let rsthis = value.NewQTextImageFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextImageFormat_NewQTextImageFormat {
  fn NewQTextImageFormat(self) -> QTextImageFormat;
}

  // proto:  void QTextImageFormat::QTextImageFormat();
impl<'a> /*trait*/ QTextImageFormat_NewQTextImageFormat for () {
  fn NewQTextImageFormat(self) -> QTextImageFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormatC1Ev()};
    unsafe {_ZN16QTextImageFormatC1Ev(qthis)};
    let rsthis = QTextImageFormat{/**/qbase: QTextCharFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextImageFormat::isValid();
impl /*struct*/ QTextImageFormat {
  pub fn isValid<RetType, T: QTextImageFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextImageFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  bool QTextImageFormat::isValid();
impl<'a> /*trait*/ QTextImageFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextImageFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat7isValidEv()};
    let mut ret = unsafe {_ZNK16QTextImageFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QTextImageFormat::width();
impl /*struct*/ QTextImageFormat {
  pub fn width<RetType, T: QTextImageFormat_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextImageFormat_width<RetType> {
  fn width(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  qreal QTextImageFormat::width();
impl<'a> /*trait*/ QTextImageFormat_width<f64> for () {
  fn width(self , rsthis: &mut QTextImageFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat5widthEv()};
    let mut ret = unsafe {_ZNK16QTextImageFormat5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextImageFormat::QTextImageFormat(const QTextFormat & format);
impl<'a> /*trait*/ QTextImageFormat_NewQTextImageFormat for (QTextFormat) {
  fn NewQTextImageFormat(self) -> QTextImageFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTextImageFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextImageFormat{/**/qbase: QTextCharFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setHeight(qreal height);
impl /*struct*/ QTextImageFormat {
  pub fn setHeight<RetType, T: QTextImageFormat_setHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setHeight(qreal height);
impl<'a> /*trait*/ QTextImageFormat_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: &mut QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextImageFormat9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setWidth(qreal width);
impl /*struct*/ QTextImageFormat {
  pub fn setWidth<RetType, T: QTextImageFormat_setWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setWidth(qreal width);
impl<'a> /*trait*/ QTextImageFormat_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: &mut QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextImageFormat8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextImageFormat::setName(const QString & name);
impl /*struct*/ QTextImageFormat {
  pub fn setName<RetType, T: QTextImageFormat_setName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setName(self);
    // return 1;
  }
}

pub trait QTextImageFormat_setName<RetType> {
  fn setName(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  void QTextImageFormat::setName(const QString & name);
impl<'a> /*trait*/ QTextImageFormat_setName<()> for (QString) {
  fn setName(self , rsthis: &mut QTextImageFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextImageFormat7setNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextImageFormat7setNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextImageFormat::name();
impl /*struct*/ QTextImageFormat {
  pub fn name<RetType, T: QTextImageFormat_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QTextImageFormat_name<RetType> {
  fn name(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  QString QTextImageFormat::name();
impl<'a> /*trait*/ QTextImageFormat_name<QString> for () {
  fn name(self , rsthis: &mut QTextImageFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat4nameEv()};
    let mut ret = unsafe {_ZNK16QTextImageFormat4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTextImageFormat::height();
impl /*struct*/ QTextImageFormat {
  pub fn height<RetType, T: QTextImageFormat_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QTextImageFormat_height<RetType> {
  fn height(self , rsthis: &mut QTextImageFormat) -> RetType;
}

  // proto:  qreal QTextImageFormat::height();
impl<'a> /*trait*/ QTextImageFormat_height<f64> for () {
  fn height(self , rsthis: &mut QTextImageFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextImageFormat6heightEv()};
    let mut ret = unsafe {_ZNK16QTextImageFormat6heightEv(rsthis.qclsinst)};
    return ret as f64;
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
    let mut ret1 = QTextBlockFormat::inheritFrom(ret);
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
    let mut ret1 = QString::inheritFrom(ret);
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
    let mut ret1 = QTextLength::inheritFrom(ret);
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
    let mut ret1 = QColor::inheritFrom(ret);
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
    let mut ret1 = QTextFrameFormat::inheritFrom(ret);
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
    let mut ret1 = QBrush::inheritFrom(ret);
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
    let mut ret1 = QPen::inheritFrom(ret);
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
    let mut ret1 = QVariant::inheritFrom(ret);
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
    let mut ret1 = QBrush::inheritFrom(ret);
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
    let mut ret1 = QTextImageFormat::inheritFrom(ret);
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
    let mut ret1 = QBrush::inheritFrom(ret);
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
    let mut ret1 = QTextTableFormat::inheritFrom(ret);
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
    let mut ret1 = QTextCharFormat::inheritFrom(ret);
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
    let mut ret1 = QTextTableCellFormat::inheritFrom(ret);
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
    let mut ret1 = QTextListFormat::inheritFrom(ret);
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

impl /*struct*/ QTextBlockFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextBlockFormat {
    return QTextBlockFormat{qbase: QTextFormat::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTextBlockFormat {
  type Target = QTextFormat;

  fn deref(&self) -> &QTextFormat {
    return &self.qbase;
  }
}
impl AsRef<QTextFormat> for QTextBlockFormat {
  fn as_ref(&self) -> &QTextFormat {
    return &self.qbase;
  }
}
  // proto:  int QTextBlockFormat::indent();
impl /*struct*/ QTextBlockFormat {
  pub fn indent<RetType, T: QTextBlockFormat_indent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_indent<RetType> {
  fn indent(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  int QTextBlockFormat::indent();
impl<'a> /*trait*/ QTextBlockFormat_indent<i32> for () {
  fn indent(self , rsthis: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat6indentEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat6indentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setTextIndent(qreal aindent);
impl /*struct*/ QTextBlockFormat {
  pub fn setTextIndent<RetType, T: QTextBlockFormat_setTextIndent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTextIndent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setTextIndent<RetType> {
  fn setTextIndent(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setTextIndent(qreal aindent);
impl<'a> /*trait*/ QTextBlockFormat_setTextIndent<()> for (f64) {
  fn setTextIndent(self , rsthis: &mut QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setTextIndentEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat13setTextIndentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setNonBreakableLines(bool b);
impl /*struct*/ QTextBlockFormat {
  pub fn setNonBreakableLines<RetType, T: QTextBlockFormat_setNonBreakableLines<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNonBreakableLines(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setNonBreakableLines<RetType> {
  fn setNonBreakableLines(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setNonBreakableLines(bool b);
impl<'a> /*trait*/ QTextBlockFormat_setNonBreakableLines<()> for (i8) {
  fn setNonBreakableLines(self , rsthis: &mut QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat20setNonBreakableLinesEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QTextBlockFormat20setNonBreakableLinesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setIndent(int indent);
impl /*struct*/ QTextBlockFormat {
  pub fn setIndent<RetType, T: QTextBlockFormat_setIndent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIndent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setIndent<RetType> {
  fn setIndent(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setIndent(int indent);
impl<'a> /*trait*/ QTextBlockFormat_setIndent<()> for (i32) {
  fn setIndent(self , rsthis: &mut QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat9setIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTextBlockFormat9setIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextBlockFormat::textIndent();
impl /*struct*/ QTextBlockFormat {
  pub fn textIndent<RetType, T: QTextBlockFormat_textIndent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textIndent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_textIndent<RetType> {
  fn textIndent(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  qreal QTextBlockFormat::textIndent();
impl<'a> /*trait*/ QTextBlockFormat_textIndent<f64> for () {
  fn textIndent(self , rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10textIndentEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat10textIndentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextBlockFormat::lineHeight();
impl /*struct*/ QTextBlockFormat {
  pub fn lineHeight<RetType, T: QTextBlockFormat_lineHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineHeight(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_lineHeight<RetType> {
  fn lineHeight(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  qreal QTextBlockFormat::lineHeight();
impl<'a> /*trait*/ QTextBlockFormat_lineHeight<f64> for () {
  fn lineHeight(self , rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10lineHeightEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat10lineHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::QTextBlockFormat(const QTextFormat & fmt);
impl /*struct*/ QTextBlockFormat {
  pub fn NewQTextBlockFormat<T: QTextBlockFormat_NewQTextBlockFormat>(value: T) -> QTextBlockFormat {
    let rsthis = value.NewQTextBlockFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockFormat_NewQTextBlockFormat {
  fn NewQTextBlockFormat(self) -> QTextBlockFormat;
}

  // proto:  void QTextBlockFormat::QTextBlockFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextBlockFormat_NewQTextBlockFormat for (QTextFormat) {
  fn NewQTextBlockFormat(self) -> QTextBlockFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTextBlockFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextBlockFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
impl<'a> /*trait*/ QTextBlockFormat_lineHeight<f64> for (f64, f64) {
  fn lineHeight(self , rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10lineHeightEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK16QTextBlockFormat10lineHeightEdd(rsthis.qclsinst, arg0, arg1)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setRightMargin(qreal margin);
impl /*struct*/ QTextBlockFormat {
  pub fn setRightMargin<RetType, T: QTextBlockFormat_setRightMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRightMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setRightMargin<RetType> {
  fn setRightMargin(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setRightMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setRightMargin<()> for (f64) {
  fn setRightMargin(self , rsthis: &mut QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat14setRightMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat14setRightMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextBlockFormat::topMargin();
impl /*struct*/ QTextBlockFormat {
  pub fn topMargin<RetType, T: QTextBlockFormat_topMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_topMargin<RetType> {
  fn topMargin(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  qreal QTextBlockFormat::topMargin();
impl<'a> /*trait*/ QTextBlockFormat_topMargin<f64> for () {
  fn topMargin(self , rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat9topMarginEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat9topMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::QTextBlockFormat();
impl<'a> /*trait*/ QTextBlockFormat_NewQTextBlockFormat for () {
  fn NewQTextBlockFormat(self) -> QTextBlockFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormatC1Ev()};
    unsafe {_ZN16QTextBlockFormatC1Ev(qthis)};
    let rsthis = QTextBlockFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextBlockFormat::rightMargin();
impl /*struct*/ QTextBlockFormat {
  pub fn rightMargin<RetType, T: QTextBlockFormat_rightMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rightMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_rightMargin<RetType> {
  fn rightMargin(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  qreal QTextBlockFormat::rightMargin();
impl<'a> /*trait*/ QTextBlockFormat_rightMargin<f64> for () {
  fn rightMargin(self , rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat11rightMarginEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat11rightMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextBlockFormat::bottomMargin();
impl /*struct*/ QTextBlockFormat {
  pub fn bottomMargin<RetType, T: QTextBlockFormat_bottomMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_bottomMargin<RetType> {
  fn bottomMargin(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  qreal QTextBlockFormat::bottomMargin();
impl<'a> /*trait*/ QTextBlockFormat_bottomMargin<f64> for () {
  fn bottomMargin(self , rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat12bottomMarginEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat12bottomMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setTopMargin(qreal margin);
impl /*struct*/ QTextBlockFormat {
  pub fn setTopMargin<RetType, T: QTextBlockFormat_setTopMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTopMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setTopMargin<RetType> {
  fn setTopMargin(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setTopMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setTopMargin<()> for (f64) {
  fn setTopMargin(self , rsthis: &mut QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat12setTopMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat12setTopMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextBlockFormat::leftMargin();
impl /*struct*/ QTextBlockFormat {
  pub fn leftMargin<RetType, T: QTextBlockFormat_leftMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leftMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_leftMargin<RetType> {
  fn leftMargin(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  qreal QTextBlockFormat::leftMargin();
impl<'a> /*trait*/ QTextBlockFormat_leftMargin<f64> for () {
  fn leftMargin(self , rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10leftMarginEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat10leftMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setLineHeight(qreal height, int heightType);
impl /*struct*/ QTextBlockFormat {
  pub fn setLineHeight<RetType, T: QTextBlockFormat_setLineHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLineHeight(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setLineHeight<RetType> {
  fn setLineHeight(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setLineHeight(qreal height, int heightType);
impl<'a> /*trait*/ QTextBlockFormat_setLineHeight<()> for (f64, i32) {
  fn setLineHeight(self , rsthis: &mut QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setLineHeightEdi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QTextBlockFormat13setLineHeightEdi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setBottomMargin(qreal margin);
impl /*struct*/ QTextBlockFormat {
  pub fn setBottomMargin<RetType, T: QTextBlockFormat_setBottomMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottomMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setBottomMargin<RetType> {
  fn setBottomMargin(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setBottomMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setBottomMargin<()> for (f64) {
  fn setBottomMargin(self , rsthis: &mut QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat15setBottomMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat15setBottomMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextBlockFormat::lineHeightType();
impl /*struct*/ QTextBlockFormat {
  pub fn lineHeightType<RetType, T: QTextBlockFormat_lineHeightType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineHeightType(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_lineHeightType<RetType> {
  fn lineHeightType(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  int QTextBlockFormat::lineHeightType();
impl<'a> /*trait*/ QTextBlockFormat_lineHeightType<i32> for () {
  fn lineHeightType(self , rsthis: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat14lineHeightTypeEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat14lineHeightTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBlockFormat::setLeftMargin(qreal margin);
impl /*struct*/ QTextBlockFormat {
  pub fn setLeftMargin<RetType, T: QTextBlockFormat_setLeftMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLeftMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setLeftMargin<RetType> {
  fn setLeftMargin(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  void QTextBlockFormat::setLeftMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setLeftMargin<()> for (f64) {
  fn setLeftMargin(self , rsthis: &mut QTextBlockFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setLeftMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat13setLeftMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextBlockFormat::isValid();
impl /*struct*/ QTextBlockFormat {
  pub fn isValid<RetType, T: QTextBlockFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  bool QTextBlockFormat::isValid();
impl<'a> /*trait*/ QTextBlockFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextBlockFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat7isValidEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTextBlockFormat::nonBreakableLines();
impl /*struct*/ QTextBlockFormat {
  pub fn nonBreakableLines<RetType, T: QTextBlockFormat_nonBreakableLines<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nonBreakableLines(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_nonBreakableLines<RetType> {
  fn nonBreakableLines(self , rsthis: &mut QTextBlockFormat) -> RetType;
}

  // proto:  bool QTextBlockFormat::nonBreakableLines();
impl<'a> /*trait*/ QTextBlockFormat_nonBreakableLines<i8> for () {
  fn nonBreakableLines(self , rsthis: &mut QTextBlockFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat17nonBreakableLinesEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat17nonBreakableLinesEv(rsthis.qclsinst)};
    return ret as i8;
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
    return &self.qbase;
  }
}
impl AsRef<QTextFormat> for QTextCharFormat {
  fn as_ref(&self) -> &QTextFormat {
    return &self.qbase;
  }
}
  // proto:  void QTextCharFormat::setFontLetterSpacing(qreal spacing);
impl /*struct*/ QTextCharFormat {
  pub fn setFontLetterSpacing<RetType, T: QTextCharFormat_setFontLetterSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontLetterSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontLetterSpacing<RetType> {
  fn setFontLetterSpacing(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontLetterSpacing(qreal spacing);
impl<'a> /*trait*/ QTextCharFormat_setFontLetterSpacing<()> for (f64) {
  fn setFontLetterSpacing(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat20setFontLetterSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat20setFontLetterSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::isAnchor();
impl /*struct*/ QTextCharFormat {
  pub fn isAnchor<RetType, T: QTextCharFormat_isAnchor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isAnchor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_isAnchor<RetType> {
  fn isAnchor(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::isAnchor();
impl<'a> /*trait*/ QTextCharFormat_isAnchor<i8> for () {
  fn isAnchor(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat8isAnchorEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat8isAnchorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFont(const QFont & font);
impl /*struct*/ QTextCharFormat {
  pub fn setFont<RetType, T: QTextCharFormat_setFont<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFont<RetType> {
  fn setFont(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFont(const QFont & font);
impl<'a> /*trait*/ QTextCharFormat_setFont<()> for (QFont) {
  fn setFont(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::fontOverline();
impl /*struct*/ QTextCharFormat {
  pub fn fontOverline<RetType, T: QTextCharFormat_fontOverline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontOverline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontOverline<RetType> {
  fn fontOverline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::fontOverline();
impl<'a> /*trait*/ QTextCharFormat_fontOverline<i8> for () {
  fn fontOverline(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat12fontOverlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat12fontOverlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QFont QTextCharFormat::font();
impl /*struct*/ QTextCharFormat {
  pub fn font<RetType, T: QTextCharFormat_font<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTextCharFormat_font<RetType> {
  fn font(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  QFont QTextCharFormat::font();
impl<'a> /*trait*/ QTextCharFormat_font<QFont> for () {
  fn font(self , rsthis: &mut QTextCharFormat) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat4fontEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextCharFormat::fontFamily();
impl /*struct*/ QTextCharFormat {
  pub fn fontFamily<RetType, T: QTextCharFormat_fontFamily<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontFamily(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontFamily<RetType> {
  fn fontFamily(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  QString QTextCharFormat::fontFamily();
impl<'a> /*trait*/ QTextCharFormat_fontFamily<QString> for () {
  fn fontFamily(self , rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontFamilyEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontFamilyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::fontStrikeOut();
impl /*struct*/ QTextCharFormat {
  pub fn fontStrikeOut<RetType, T: QTextCharFormat_fontStrikeOut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontStrikeOut(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontStrikeOut<RetType> {
  fn fontStrikeOut(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::fontStrikeOut();
impl<'a> /*trait*/ QTextCharFormat_fontStrikeOut<i8> for () {
  fn fontStrikeOut(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontStrikeOutEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontStrikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontPointSize(qreal size);
impl /*struct*/ QTextCharFormat {
  pub fn setFontPointSize<RetType, T: QTextCharFormat_setFontPointSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontPointSize(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontPointSize<RetType> {
  fn setFontPointSize(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontPointSize(qreal size);
impl<'a> /*trait*/ QTextCharFormat_setFontPointSize<()> for (f64) {
  fn setFontPointSize(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontPointSizeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat16setFontPointSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setUnderlineColor(const QColor & color);
impl /*struct*/ QTextCharFormat {
  pub fn setUnderlineColor<RetType, T: QTextCharFormat_setUnderlineColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUnderlineColor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setUnderlineColor<RetType> {
  fn setUnderlineColor(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setUnderlineColor(const QColor & color);
impl<'a> /*trait*/ QTextCharFormat_setUnderlineColor<()> for (QColor) {
  fn setUnderlineColor(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat17setUnderlineColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat17setUnderlineColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextCharFormat::tableCellRowSpan();
impl /*struct*/ QTextCharFormat {
  pub fn tableCellRowSpan<RetType, T: QTextCharFormat_tableCellRowSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tableCellRowSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_tableCellRowSpan<RetType> {
  fn tableCellRowSpan(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  int QTextCharFormat::tableCellRowSpan();
impl<'a> /*trait*/ QTextCharFormat_tableCellRowSpan<i32> for () {
  fn tableCellRowSpan(self , rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat16tableCellRowSpanEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat16tableCellRowSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontUnderline(bool underline);
impl /*struct*/ QTextCharFormat {
  pub fn setFontUnderline<RetType, T: QTextCharFormat_setFontUnderline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontUnderline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontUnderline<RetType> {
  fn setFontUnderline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontUnderline(bool underline);
impl<'a> /*trait*/ QTextCharFormat_setFontUnderline<()> for (i8) {
  fn setFontUnderline(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontUnderlineEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTextCharFormat16setFontUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::isValid();
impl /*struct*/ QTextCharFormat {
  pub fn isValid<RetType, T: QTextCharFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextCharFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::isValid();
impl<'a> /*trait*/ QTextCharFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat7isValidEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::fontItalic();
impl /*struct*/ QTextCharFormat {
  pub fn fontItalic<RetType, T: QTextCharFormat_fontItalic<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontItalic(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontItalic<RetType> {
  fn fontItalic(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::fontItalic();
impl<'a> /*trait*/ QTextCharFormat_fontItalic<i8> for () {
  fn fontItalic(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontItalicEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontItalicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setToolTip(const QString & tip);
impl /*struct*/ QTextCharFormat {
  pub fn setToolTip<RetType, T: QTextCharFormat_setToolTip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setToolTip<RetType> {
  fn setToolTip(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setToolTip(const QString & tip);
impl<'a> /*trait*/ QTextCharFormat_setToolTip<()> for (QString) {
  fn setToolTip(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setTextOutline(const QPen & pen);
impl /*struct*/ QTextCharFormat {
  pub fn setTextOutline<RetType, T: QTextCharFormat_setTextOutline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTextOutline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTextOutline<RetType> {
  fn setTextOutline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setTextOutline(const QPen & pen);
impl<'a> /*trait*/ QTextCharFormat_setTextOutline<()> for (QPen) {
  fn setTextOutline(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setTextOutlineERK4QPen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat14setTextOutlineERK4QPen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
impl /*struct*/ QTextCharFormat {
  pub fn setTableCellRowSpan<RetType, T: QTextCharFormat_setTableCellRowSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTableCellRowSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTableCellRowSpan<RetType> {
  fn setTableCellRowSpan(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellRowSpan<()> for (i32) {
  fn setTableCellRowSpan(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat19setTableCellRowSpanEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat19setTableCellRowSpanEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setAnchor(bool anchor);
impl /*struct*/ QTextCharFormat {
  pub fn setAnchor<RetType, T: QTextCharFormat_setAnchor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAnchor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchor<RetType> {
  fn setAnchor(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setAnchor(bool anchor);
impl<'a> /*trait*/ QTextCharFormat_setAnchor<()> for (i8) {
  fn setAnchor(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat9setAnchorEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTextCharFormat9setAnchorEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextCharFormat::fontPointSize();
impl /*struct*/ QTextCharFormat {
  pub fn fontPointSize<RetType, T: QTextCharFormat_fontPointSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontPointSize(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontPointSize<RetType> {
  fn fontPointSize(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  qreal QTextCharFormat::fontPointSize();
impl<'a> /*trait*/ QTextCharFormat_fontPointSize<f64> for () {
  fn fontPointSize(self , rsthis: &mut QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontPointSizeEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontPointSizeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::QTextCharFormat(const QTextFormat & fmt);
impl /*struct*/ QTextCharFormat {
  pub fn NewQTextCharFormat<T: QTextCharFormat_NewQTextCharFormat>(value: T) -> QTextCharFormat {
    let rsthis = value.NewQTextCharFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCharFormat_NewQTextCharFormat {
  fn NewQTextCharFormat(self) -> QTextCharFormat;
}

  // proto:  void QTextCharFormat::QTextCharFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextCharFormat_NewQTextCharFormat for (QTextFormat) {
  fn NewQTextCharFormat(self) -> QTextCharFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTextCharFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextCharFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontStrikeOut(bool strikeOut);
impl /*struct*/ QTextCharFormat {
  pub fn setFontStrikeOut<RetType, T: QTextCharFormat_setFontStrikeOut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontStrikeOut(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontStrikeOut<RetType> {
  fn setFontStrikeOut(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontStrikeOut(bool strikeOut);
impl<'a> /*trait*/ QTextCharFormat_setFontStrikeOut<()> for (i8) {
  fn setFontStrikeOut(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontStrikeOutEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTextCharFormat16setFontStrikeOutEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextCharFormat::fontWordSpacing();
impl /*struct*/ QTextCharFormat {
  pub fn fontWordSpacing<RetType, T: QTextCharFormat_fontWordSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontWordSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontWordSpacing<RetType> {
  fn fontWordSpacing(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  qreal QTextCharFormat::fontWordSpacing();
impl<'a> /*trait*/ QTextCharFormat_fontWordSpacing<f64> for () {
  fn fontWordSpacing(self , rsthis: &mut QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat15fontWordSpacingEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat15fontWordSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QString QTextCharFormat::toolTip();
impl /*struct*/ QTextCharFormat {
  pub fn toolTip<RetType, T: QTextCharFormat_toolTip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QTextCharFormat_toolTip<RetType> {
  fn toolTip(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  QString QTextCharFormat::toolTip();
impl<'a> /*trait*/ QTextCharFormat_toolTip<QString> for () {
  fn toolTip(self , rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat7toolTipEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setAnchorNames(const QStringList & names);
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorNames<RetType, T: QTextCharFormat_setAnchorNames<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAnchorNames(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorNames<RetType> {
  fn setAnchorNames(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setAnchorNames(const QStringList & names);
impl<'a> /*trait*/ QTextCharFormat_setAnchorNames<()> for (QStringList) {
  fn setAnchorNames(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setAnchorNamesERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat14setAnchorNamesERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QTextCharFormat::anchorNames();
impl /*struct*/ QTextCharFormat {
  pub fn anchorNames<RetType, T: QTextCharFormat_anchorNames<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.anchorNames(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorNames<RetType> {
  fn anchorNames(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  QStringList QTextCharFormat::anchorNames();
impl<'a> /*trait*/ QTextCharFormat_anchorNames<()> for () {
  fn anchorNames(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11anchorNamesEv()};
     unsafe {_ZNK15QTextCharFormat11anchorNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
impl /*struct*/ QTextCharFormat {
  pub fn setFontFixedPitch<RetType, T: QTextCharFormat_setFontFixedPitch<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontFixedPitch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontFixedPitch<RetType> {
  fn setFontFixedPitch(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
impl<'a> /*trait*/ QTextCharFormat_setFontFixedPitch<()> for (i8) {
  fn setFontFixedPitch(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat17setFontFixedPitchEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTextCharFormat17setFontFixedPitchEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontItalic(bool italic);
impl /*struct*/ QTextCharFormat {
  pub fn setFontItalic<RetType, T: QTextCharFormat_setFontItalic<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontItalic(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontItalic<RetType> {
  fn setFontItalic(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontItalic(bool italic);
impl<'a> /*trait*/ QTextCharFormat_setFontItalic<()> for (i8) {
  fn setFontItalic(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontItalicEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTextCharFormat13setFontItalicEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontFamily(const QString & family);
impl /*struct*/ QTextCharFormat {
  pub fn setFontFamily<RetType, T: QTextCharFormat_setFontFamily<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontFamily(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontFamily<RetType> {
  fn setFontFamily(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontFamily(const QString & family);
impl<'a> /*trait*/ QTextCharFormat_setFontFamily<()> for (QString) {
  fn setFontFamily(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setFontFamilyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::fontFixedPitch();
impl /*struct*/ QTextCharFormat {
  pub fn fontFixedPitch<RetType, T: QTextCharFormat_fontFixedPitch<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontFixedPitch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontFixedPitch<RetType> {
  fn fontFixedPitch(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::fontFixedPitch();
impl<'a> /*trait*/ QTextCharFormat_fontFixedPitch<i8> for () {
  fn fontFixedPitch(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat14fontFixedPitchEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat14fontFixedPitchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setAnchorHref(const QString & value);
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorHref<RetType, T: QTextCharFormat_setAnchorHref<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAnchorHref(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorHref<RetType> {
  fn setAnchorHref(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setAnchorHref(const QString & value);
impl<'a> /*trait*/ QTextCharFormat_setAnchorHref<()> for (QString) {
  fn setAnchorHref(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setAnchorHrefERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setAnchorHrefERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextCharFormat::fontStretch();
impl /*struct*/ QTextCharFormat {
  pub fn fontStretch<RetType, T: QTextCharFormat_fontStretch<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontStretch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontStretch<RetType> {
  fn fontStretch(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  int QTextCharFormat::fontStretch();
impl<'a> /*trait*/ QTextCharFormat_fontStretch<i32> for () {
  fn fontStretch(self , rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11fontStretchEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11fontStretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontKerning(bool enable);
impl /*struct*/ QTextCharFormat {
  pub fn setFontKerning<RetType, T: QTextCharFormat_setFontKerning<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontKerning(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontKerning<RetType> {
  fn setFontKerning(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontKerning(bool enable);
impl<'a> /*trait*/ QTextCharFormat_setFontKerning<()> for (i8) {
  fn setFontKerning(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setFontKerningEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTextCharFormat14setFontKerningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextCharFormat::tableCellColumnSpan();
impl /*struct*/ QTextCharFormat {
  pub fn tableCellColumnSpan<RetType, T: QTextCharFormat_tableCellColumnSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tableCellColumnSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_tableCellColumnSpan<RetType> {
  fn tableCellColumnSpan(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  int QTextCharFormat::tableCellColumnSpan();
impl<'a> /*trait*/ QTextCharFormat_tableCellColumnSpan<i32> for () {
  fn tableCellColumnSpan(self , rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat19tableCellColumnSpanEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat19tableCellColumnSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::QTextCharFormat();
impl<'a> /*trait*/ QTextCharFormat_NewQTextCharFormat for () {
  fn NewQTextCharFormat(self) -> QTextCharFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormatC1Ev()};
    unsafe {_ZN15QTextCharFormatC1Ev(qthis)};
    let rsthis = QTextCharFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextCharFormat::fontLetterSpacing();
impl /*struct*/ QTextCharFormat {
  pub fn fontLetterSpacing<RetType, T: QTextCharFormat_fontLetterSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontLetterSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontLetterSpacing<RetType> {
  fn fontLetterSpacing(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  qreal QTextCharFormat::fontLetterSpacing();
impl<'a> /*trait*/ QTextCharFormat_fontLetterSpacing<f64> for () {
  fn fontLetterSpacing(self , rsthis: &mut QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat17fontLetterSpacingEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat17fontLetterSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QString QTextCharFormat::anchorHref();
impl /*struct*/ QTextCharFormat {
  pub fn anchorHref<RetType, T: QTextCharFormat_anchorHref<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.anchorHref(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorHref<RetType> {
  fn anchorHref(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  QString QTextCharFormat::anchorHref();
impl<'a> /*trait*/ QTextCharFormat_anchorHref<QString> for () {
  fn anchorHref(self , rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorHrefEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10anchorHrefEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextCharFormat::anchorName();
impl /*struct*/ QTextCharFormat {
  pub fn anchorName<RetType, T: QTextCharFormat_anchorName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.anchorName(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorName<RetType> {
  fn anchorName(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  QString QTextCharFormat::anchorName();
impl<'a> /*trait*/ QTextCharFormat_anchorName<QString> for () {
  fn anchorName(self , rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorNameEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10anchorNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontStretch(int factor);
impl /*struct*/ QTextCharFormat {
  pub fn setFontStretch<RetType, T: QTextCharFormat_setFontStretch<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontStretch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontStretch<RetType> {
  fn setFontStretch(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontStretch(int factor);
impl<'a> /*trait*/ QTextCharFormat_setFontStretch<()> for (i32) {
  fn setFontStretch(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setFontStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat14setFontStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setAnchorName(const QString & name);
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorName<RetType, T: QTextCharFormat_setAnchorName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAnchorName(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorName<RetType> {
  fn setAnchorName(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setAnchorName(const QString & name);
impl<'a> /*trait*/ QTextCharFormat_setAnchorName<()> for (QString) {
  fn setAnchorName(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setAnchorNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setAnchorNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::fontKerning();
impl /*struct*/ QTextCharFormat {
  pub fn fontKerning<RetType, T: QTextCharFormat_fontKerning<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontKerning(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontKerning<RetType> {
  fn fontKerning(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::fontKerning();
impl<'a> /*trait*/ QTextCharFormat_fontKerning<i8> for () {
  fn fontKerning(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11fontKerningEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11fontKerningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontWeight(int weight);
impl /*struct*/ QTextCharFormat {
  pub fn setFontWeight<RetType, T: QTextCharFormat_setFontWeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontWeight(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontWeight<RetType> {
  fn setFontWeight(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontWeight(int weight);
impl<'a> /*trait*/ QTextCharFormat_setFontWeight<()> for (i32) {
  fn setFontWeight(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontWeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat13setFontWeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextCharFormat::fontUnderline();
impl /*struct*/ QTextCharFormat {
  pub fn fontUnderline<RetType, T: QTextCharFormat_fontUnderline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontUnderline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontUnderline<RetType> {
  fn fontUnderline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  bool QTextCharFormat::fontUnderline();
impl<'a> /*trait*/ QTextCharFormat_fontUnderline<i8> for () {
  fn fontUnderline(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontUnderlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontUnderlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontWordSpacing(qreal spacing);
impl /*struct*/ QTextCharFormat {
  pub fn setFontWordSpacing<RetType, T: QTextCharFormat_setFontWordSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontWordSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontWordSpacing<RetType> {
  fn setFontWordSpacing(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontWordSpacing(qreal spacing);
impl<'a> /*trait*/ QTextCharFormat_setFontWordSpacing<()> for (f64) {
  fn setFontWordSpacing(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat18setFontWordSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat18setFontWordSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QTextCharFormat::underlineColor();
impl /*struct*/ QTextCharFormat {
  pub fn underlineColor<RetType, T: QTextCharFormat_underlineColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.underlineColor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_underlineColor<RetType> {
  fn underlineColor(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  QColor QTextCharFormat::underlineColor();
impl<'a> /*trait*/ QTextCharFormat_underlineColor<QColor> for () {
  fn underlineColor(self , rsthis: &mut QTextCharFormat) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat14underlineColorEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat14underlineColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextCharFormat::fontWeight();
impl /*struct*/ QTextCharFormat {
  pub fn fontWeight<RetType, T: QTextCharFormat_fontWeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fontWeight(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontWeight<RetType> {
  fn fontWeight(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  int QTextCharFormat::fontWeight();
impl<'a> /*trait*/ QTextCharFormat_fontWeight<i32> for () {
  fn fontWeight(self , rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontWeightEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontWeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setFontOverline(bool overline);
impl /*struct*/ QTextCharFormat {
  pub fn setFontOverline<RetType, T: QTextCharFormat_setFontOverline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFontOverline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontOverline<RetType> {
  fn setFontOverline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setFontOverline(bool overline);
impl<'a> /*trait*/ QTextCharFormat_setFontOverline<()> for (i8) {
  fn setFontOverline(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat15setFontOverlineEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTextCharFormat15setFontOverlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
impl /*struct*/ QTextCharFormat {
  pub fn setTableCellColumnSpan<RetType, T: QTextCharFormat_setTableCellColumnSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTableCellColumnSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTableCellColumnSpan<RetType> {
  fn setTableCellColumnSpan(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellColumnSpan<()> for (i32) {
  fn setTableCellColumnSpan(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat22setTableCellColumnSpanEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat22setTableCellColumnSpanEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPen QTextCharFormat::textOutline();
impl /*struct*/ QTextCharFormat {
  pub fn textOutline<RetType, T: QTextCharFormat_textOutline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textOutline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_textOutline<RetType> {
  fn textOutline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

  // proto:  QPen QTextCharFormat::textOutline();
impl<'a> /*trait*/ QTextCharFormat_textOutline<QPen> for () {
  fn textOutline(self , rsthis: &mut QTextCharFormat) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11textOutlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11textOutlineEv(rsthis.qclsinst)};
    let mut ret1 = QPen::inheritFrom(ret);
    return ret1;
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
    return &self.qbase;
  }
}
impl AsRef<QTextFrameFormat> for QTextTableFormat {
  fn as_ref(&self) -> &QTextFrameFormat {
    return &self.qbase;
  }
}
  // proto:  void QTextTableFormat::QTextTableFormat();
impl /*struct*/ QTextTableFormat {
  pub fn NewQTextTableFormat<T: QTextTableFormat_NewQTextTableFormat>(value: T) -> QTextTableFormat {
    let rsthis = value.NewQTextTableFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableFormat_NewQTextTableFormat {
  fn NewQTextTableFormat(self) -> QTextTableFormat;
}

  // proto:  void QTextTableFormat::QTextTableFormat();
impl<'a> /*trait*/ QTextTableFormat_NewQTextTableFormat for () {
  fn NewQTextTableFormat(self) -> QTextTableFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormatC1Ev()};
    unsafe {_ZN16QTextTableFormatC1Ev(qthis)};
    let rsthis = QTextTableFormat{/**/qbase: QTextFrameFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextTableFormat::isValid();
impl /*struct*/ QTextTableFormat {
  pub fn isValid<RetType, T: QTextTableFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextTableFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  bool QTextTableFormat::isValid();
impl<'a> /*trait*/ QTextTableFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextTableFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat7isValidEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextTableFormat::headerRowCount();
impl /*struct*/ QTextTableFormat {
  pub fn headerRowCount<RetType, T: QTextTableFormat_headerRowCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.headerRowCount(self);
    // return 1;
  }
}

pub trait QTextTableFormat_headerRowCount<RetType> {
  fn headerRowCount(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  int QTextTableFormat::headerRowCount();
impl<'a> /*trait*/ QTextTableFormat_headerRowCount<i32> for () {
  fn headerRowCount(self , rsthis: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat14headerRowCountEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat14headerRowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextTableFormat::columns();
impl /*struct*/ QTextTableFormat {
  pub fn columns<RetType, T: QTextTableFormat_columns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columns(self);
    // return 1;
  }
}

pub trait QTextTableFormat_columns<RetType> {
  fn columns(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  int QTextTableFormat::columns();
impl<'a> /*trait*/ QTextTableFormat_columns<i32> for () {
  fn columns(self , rsthis: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat7columnsEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat7columnsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QVector<QTextLength> QTextTableFormat::columnWidthConstraints();
impl /*struct*/ QTextTableFormat {
  pub fn columnWidthConstraints<RetType, T: QTextTableFormat_columnWidthConstraints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnWidthConstraints(self);
    // return 1;
  }
}

pub trait QTextTableFormat_columnWidthConstraints<RetType> {
  fn columnWidthConstraints(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  QVector<QTextLength> QTextTableFormat::columnWidthConstraints();
impl<'a> /*trait*/ QTextTableFormat_columnWidthConstraints<()> for () {
  fn columnWidthConstraints(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat22columnWidthConstraintsEv()};
     unsafe {_ZNK16QTextTableFormat22columnWidthConstraintsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
impl /*struct*/ QTextTableFormat {
  pub fn setCellPadding<RetType, T: QTextTableFormat_setCellPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCellPadding(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setCellPadding<RetType> {
  fn setCellPadding(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
impl<'a> /*trait*/ QTextTableFormat_setCellPadding<()> for (f64) {
  fn setCellPadding(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat14setCellPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextTableFormat14setCellPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextTableFormat::cellPadding();
impl /*struct*/ QTextTableFormat {
  pub fn cellPadding<RetType, T: QTextTableFormat_cellPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cellPadding(self);
    // return 1;
  }
}

pub trait QTextTableFormat_cellPadding<RetType> {
  fn cellPadding(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  qreal QTextTableFormat::cellPadding();
impl<'a> /*trait*/ QTextTableFormat_cellPadding<f64> for () {
  fn cellPadding(self , rsthis: &mut QTextTableFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat11cellPaddingEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat11cellPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setCellSpacing(qreal spacing);
impl /*struct*/ QTextTableFormat {
  pub fn setCellSpacing<RetType, T: QTextTableFormat_setCellSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCellSpacing(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setCellSpacing<RetType> {
  fn setCellSpacing(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setCellSpacing(qreal spacing);
impl<'a> /*trait*/ QTextTableFormat_setCellSpacing<()> for (f64) {
  fn setCellSpacing(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat14setCellSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextTableFormat14setCellSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setColumns(int columns);
impl /*struct*/ QTextTableFormat {
  pub fn setColumns<RetType, T: QTextTableFormat_setColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColumns(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setColumns<RetType> {
  fn setColumns(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setColumns(int columns);
impl<'a> /*trait*/ QTextTableFormat_setColumns<()> for (i32) {
  fn setColumns(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat10setColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTextTableFormat10setColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::QTextTableFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextTableFormat_NewQTextTableFormat for (QTextFormat) {
  fn NewQTextTableFormat(self) -> QTextTableFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTextTableFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextTableFormat{/**/qbase: QTextFrameFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextTableFormat::clearColumnWidthConstraints();
impl /*struct*/ QTextTableFormat {
  pub fn clearColumnWidthConstraints<RetType, T: QTextTableFormat_clearColumnWidthConstraints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearColumnWidthConstraints(self);
    // return 1;
  }
}

pub trait QTextTableFormat_clearColumnWidthConstraints<RetType> {
  fn clearColumnWidthConstraints(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::clearColumnWidthConstraints();
impl<'a> /*trait*/ QTextTableFormat_clearColumnWidthConstraints<()> for () {
  fn clearColumnWidthConstraints(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat27clearColumnWidthConstraintsEv()};
     unsafe {_ZN16QTextTableFormat27clearColumnWidthConstraintsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setHeaderRowCount(int count);
impl /*struct*/ QTextTableFormat {
  pub fn setHeaderRowCount<RetType, T: QTextTableFormat_setHeaderRowCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeaderRowCount(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setHeaderRowCount<RetType> {
  fn setHeaderRowCount(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setHeaderRowCount(int count);
impl<'a> /*trait*/ QTextTableFormat_setHeaderRowCount<()> for (i32) {
  fn setHeaderRowCount(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat17setHeaderRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTextTableFormat17setHeaderRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextTableFormat::cellSpacing();
impl /*struct*/ QTextTableFormat {
  pub fn cellSpacing<RetType, T: QTextTableFormat_cellSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cellSpacing(self);
    // return 1;
  }
}

pub trait QTextTableFormat_cellSpacing<RetType> {
  fn cellSpacing(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  qreal QTextTableFormat::cellSpacing();
impl<'a> /*trait*/ QTextTableFormat_cellSpacing<f64> for () {
  fn cellSpacing(self , rsthis: &mut QTextTableFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat11cellSpacingEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat11cellSpacingEv(rsthis.qclsinst)};
    return ret as f64;
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
    return &self.qbase;
  }
}
impl AsRef<QTextCharFormat> for QTextTableCellFormat {
  fn as_ref(&self) -> &QTextCharFormat {
    return &self.qbase;
  }
}
  // proto:  void QTextTableCellFormat::QTextTableCellFormat();
impl /*struct*/ QTextTableCellFormat {
  pub fn NewQTextTableCellFormat<T: QTextTableCellFormat_NewQTextTableCellFormat>(value: T) -> QTextTableCellFormat {
    let rsthis = value.NewQTextTableCellFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCellFormat_NewQTextTableCellFormat {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat;
}

  // proto:  void QTextTableCellFormat::QTextTableCellFormat();
impl<'a> /*trait*/ QTextTableCellFormat_NewQTextTableCellFormat for () {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormatC1Ev()};
    unsafe {_ZN20QTextTableCellFormatC1Ev(qthis)};
    let rsthis = QTextTableCellFormat{/**/qbase: QTextCharFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setLeftPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setLeftPadding<RetType, T: QTextTableCellFormat_setLeftPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLeftPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setLeftPadding<RetType> {
  fn setLeftPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setLeftPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setLeftPadding<()> for (f64) {
  fn setLeftPadding(self , rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat14setLeftPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat14setLeftPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextTableCellFormat::isValid();
impl /*struct*/ QTextTableCellFormat {
  pub fn isValid<RetType, T: QTextTableCellFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  bool QTextTableCellFormat::isValid();
impl<'a> /*trait*/ QTextTableCellFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextTableCellFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat7isValidEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setTopPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setTopPadding<RetType, T: QTextTableCellFormat_setTopPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTopPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setTopPadding<RetType> {
  fn setTopPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setTopPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setTopPadding<()> for (f64) {
  fn setTopPadding(self , rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat13setTopPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat13setTopPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextTableCellFormat::leftPadding();
impl /*struct*/ QTextTableCellFormat {
  pub fn leftPadding<RetType, T: QTextTableCellFormat_leftPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leftPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_leftPadding<RetType> {
  fn leftPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  qreal QTextTableCellFormat::leftPadding();
impl<'a> /*trait*/ QTextTableCellFormat_leftPadding<f64> for () {
  fn leftPadding(self , rsthis: &mut QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat11leftPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat11leftPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setPadding<RetType, T: QTextTableCellFormat_setPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setPadding<RetType> {
  fn setPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setPadding<()> for (f64) {
  fn setPadding(self , rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat10setPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat10setPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextTableCellFormat::topPadding();
impl /*struct*/ QTextTableCellFormat {
  pub fn topPadding<RetType, T: QTextTableCellFormat_topPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_topPadding<RetType> {
  fn topPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  qreal QTextTableCellFormat::topPadding();
impl<'a> /*trait*/ QTextTableCellFormat_topPadding<f64> for () {
  fn topPadding(self , rsthis: &mut QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat10topPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat10topPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextTableCellFormat::rightPadding();
impl /*struct*/ QTextTableCellFormat {
  pub fn rightPadding<RetType, T: QTextTableCellFormat_rightPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rightPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_rightPadding<RetType> {
  fn rightPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  qreal QTextTableCellFormat::rightPadding();
impl<'a> /*trait*/ QTextTableCellFormat_rightPadding<f64> for () {
  fn rightPadding(self , rsthis: &mut QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat12rightPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat12rightPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::QTextTableCellFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextTableCellFormat_NewQTextTableCellFormat for (QTextFormat) {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QTextTableCellFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextTableCellFormat{/**/qbase: QTextCharFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextTableCellFormat::bottomPadding();
impl /*struct*/ QTextTableCellFormat {
  pub fn bottomPadding<RetType, T: QTextTableCellFormat_bottomPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_bottomPadding<RetType> {
  fn bottomPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  qreal QTextTableCellFormat::bottomPadding();
impl<'a> /*trait*/ QTextTableCellFormat_bottomPadding<f64> for () {
  fn bottomPadding(self , rsthis: &mut QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat13bottomPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat13bottomPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setRightPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setRightPadding<RetType, T: QTextTableCellFormat_setRightPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRightPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setRightPadding<RetType> {
  fn setRightPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setRightPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setRightPadding<()> for (f64) {
  fn setRightPadding(self , rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat15setRightPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat15setRightPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTableCellFormat::setBottomPadding(qreal padding);
impl /*struct*/ QTextTableCellFormat {
  pub fn setBottomPadding<RetType, T: QTextTableCellFormat_setBottomPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottomPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setBottomPadding<RetType> {
  fn setBottomPadding(self , rsthis: &mut QTextTableCellFormat) -> RetType;
}

  // proto:  void QTextTableCellFormat::setBottomPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setBottomPadding<()> for (f64) {
  fn setBottomPadding(self , rsthis: &mut QTextTableCellFormat) -> () {
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
    return &self.qbase;
  }
}
impl AsRef<QTextFormat> for QTextListFormat {
  fn as_ref(&self) -> &QTextFormat {
    return &self.qbase;
  }
}
  // proto:  int QTextListFormat::indent();
impl /*struct*/ QTextListFormat {
  pub fn indent<RetType, T: QTextListFormat_indent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indent(self);
    // return 1;
  }
}

pub trait QTextListFormat_indent<RetType> {
  fn indent(self , rsthis: &mut QTextListFormat) -> RetType;
}

  // proto:  int QTextListFormat::indent();
impl<'a> /*trait*/ QTextListFormat_indent<i32> for () {
  fn indent(self , rsthis: &mut QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat6indentEv()};
    let mut ret = unsafe {_ZNK15QTextListFormat6indentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextListFormat::QTextListFormat(const QTextFormat & fmt);
impl /*struct*/ QTextListFormat {
  pub fn NewQTextListFormat<T: QTextListFormat_NewQTextListFormat>(value: T) -> QTextListFormat {
    let rsthis = value.NewQTextListFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextListFormat_NewQTextListFormat {
  fn NewQTextListFormat(self) -> QTextListFormat;
}

  // proto:  void QTextListFormat::QTextListFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextListFormat_NewQTextListFormat for (QTextFormat) {
  fn NewQTextListFormat(self) -> QTextListFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTextListFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextListFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextListFormat::setIndent(int indent);
impl /*struct*/ QTextListFormat {
  pub fn setIndent<RetType, T: QTextListFormat_setIndent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIndent(self);
    // return 1;
  }
}

pub trait QTextListFormat_setIndent<RetType> {
  fn setIndent(self , rsthis: &mut QTextListFormat) -> RetType;
}

  // proto:  void QTextListFormat::setIndent(int indent);
impl<'a> /*trait*/ QTextListFormat_setIndent<()> for (i32) {
  fn setIndent(self , rsthis: &mut QTextListFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat9setIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextListFormat9setIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextListFormat::numberSuffix();
impl /*struct*/ QTextListFormat {
  pub fn numberSuffix<RetType, T: QTextListFormat_numberSuffix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.numberSuffix(self);
    // return 1;
  }
}

pub trait QTextListFormat_numberSuffix<RetType> {
  fn numberSuffix(self , rsthis: &mut QTextListFormat) -> RetType;
}

  // proto:  QString QTextListFormat::numberSuffix();
impl<'a> /*trait*/ QTextListFormat_numberSuffix<QString> for () {
  fn numberSuffix(self , rsthis: &mut QTextListFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat12numberSuffixEv()};
    let mut ret = unsafe {_ZNK15QTextListFormat12numberSuffixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextListFormat::QTextListFormat();
impl<'a> /*trait*/ QTextListFormat_NewQTextListFormat for () {
  fn NewQTextListFormat(self) -> QTextListFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormatC1Ev()};
    unsafe {_ZN15QTextListFormatC1Ev(qthis)};
    let rsthis = QTextListFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QTextListFormat::numberPrefix();
impl /*struct*/ QTextListFormat {
  pub fn numberPrefix<RetType, T: QTextListFormat_numberPrefix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.numberPrefix(self);
    // return 1;
  }
}

pub trait QTextListFormat_numberPrefix<RetType> {
  fn numberPrefix(self , rsthis: &mut QTextListFormat) -> RetType;
}

  // proto:  QString QTextListFormat::numberPrefix();
impl<'a> /*trait*/ QTextListFormat_numberPrefix<QString> for () {
  fn numberPrefix(self , rsthis: &mut QTextListFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat12numberPrefixEv()};
    let mut ret = unsafe {_ZNK15QTextListFormat12numberPrefixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextListFormat::isValid();
impl /*struct*/ QTextListFormat {
  pub fn isValid<RetType, T: QTextListFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextListFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextListFormat) -> RetType;
}

  // proto:  bool QTextListFormat::isValid();
impl<'a> /*trait*/ QTextListFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextListFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextListFormat7isValidEv()};
    let mut ret = unsafe {_ZNK15QTextListFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
impl /*struct*/ QTextListFormat {
  pub fn setNumberSuffix<RetType, T: QTextListFormat_setNumberSuffix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNumberSuffix(self);
    // return 1;
  }
}

pub trait QTextListFormat_setNumberSuffix<RetType> {
  fn setNumberSuffix(self , rsthis: &mut QTextListFormat) -> RetType;
}

  // proto:  void QTextListFormat::setNumberSuffix(const QString & numberSuffix);
impl<'a> /*trait*/ QTextListFormat_setNumberSuffix<()> for (QString) {
  fn setNumberSuffix(self , rsthis: &mut QTextListFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextListFormat15setNumberSuffixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextListFormat15setNumberSuffixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
impl /*struct*/ QTextListFormat {
  pub fn setNumberPrefix<RetType, T: QTextListFormat_setNumberPrefix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNumberPrefix(self);
    // return 1;
  }
}

pub trait QTextListFormat_setNumberPrefix<RetType> {
  fn setNumberPrefix(self , rsthis: &mut QTextListFormat) -> RetType;
}

  // proto:  void QTextListFormat::setNumberPrefix(const QString & numberPrefix);
impl<'a> /*trait*/ QTextListFormat_setNumberPrefix<()> for (QString) {
  fn setNumberPrefix(self , rsthis: &mut QTextListFormat) -> () {
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
    return &self.qbase;
  }
}
impl AsRef<QTextFormat> for QTextFrameFormat {
  fn as_ref(&self) -> &QTextFormat {
    return &self.qbase;
  }
}
  // proto:  bool QTextFrameFormat::isValid();
impl /*struct*/ QTextFrameFormat {
  pub fn isValid<RetType, T: QTextFrameFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  bool QTextFrameFormat::isValid();
impl<'a> /*trait*/ QTextFrameFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextFrameFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat7isValidEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setHeight(qreal height);
impl /*struct*/ QTextFrameFormat {
  pub fn setHeight<RetType, T: QTextFrameFormat_setHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setHeight(qreal height);
impl<'a> /*trait*/ QTextFrameFormat_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setBorderBrush(const QBrush & brush);
impl /*struct*/ QTextFrameFormat {
  pub fn setBorderBrush<RetType, T: QTextFrameFormat_setBorderBrush<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBorderBrush(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setBorderBrush<RetType> {
  fn setBorderBrush(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setBorderBrush(const QBrush & brush);
impl<'a> /*trait*/ QTextFrameFormat_setBorderBrush<()> for (QBrush) {
  fn setBorderBrush(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat14setBorderBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextFrameFormat14setBorderBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::margin();
impl /*struct*/ QTextFrameFormat {
  pub fn margin<RetType, T: QTextFrameFormat_margin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.margin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_margin<RetType> {
  fn margin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::margin();
impl<'a> /*trait*/ QTextFrameFormat_margin<f64> for () {
  fn margin(self , rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6marginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat6marginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QBrush QTextFrameFormat::borderBrush();
impl /*struct*/ QTextFrameFormat {
  pub fn borderBrush<RetType, T: QTextFrameFormat_borderBrush<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.borderBrush(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_borderBrush<RetType> {
  fn borderBrush(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  QBrush QTextFrameFormat::borderBrush();
impl<'a> /*trait*/ QTextFrameFormat_borderBrush<QBrush> for () {
  fn borderBrush(self , rsthis: &mut QTextFrameFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat11borderBrushEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat11borderBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setRightMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setRightMargin<RetType, T: QTextFrameFormat_setRightMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRightMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setRightMargin<RetType> {
  fn setRightMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setRightMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setRightMargin<()> for (f64) {
  fn setRightMargin(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat14setRightMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat14setRightMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setMargin<RetType, T: QTextFrameFormat_setMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setMargin<RetType> {
  fn setMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setMargin<()> for (f64) {
  fn setMargin(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setBorder(qreal border);
impl /*struct*/ QTextFrameFormat {
  pub fn setBorder<RetType, T: QTextFrameFormat_setBorder<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBorder(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setBorder<RetType> {
  fn setBorder(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setBorder(qreal border);
impl<'a> /*trait*/ QTextFrameFormat_setBorder<()> for (f64) {
  fn setBorder(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setBorderEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setBorderEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setHeight(const QTextLength & height);
impl<'a> /*trait*/ QTextFrameFormat_setHeight<()> for (QTextLength) {
  fn setHeight(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setHeightERK11QTextLength()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextFrameFormat9setHeightERK11QTextLength(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setWidth(const QTextLength & length);
impl /*struct*/ QTextFrameFormat {
  pub fn setWidth<RetType, T: QTextFrameFormat_setWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setWidth(const QTextLength & length);
impl<'a> /*trait*/ QTextFrameFormat_setWidth<()> for (QTextLength) {
  fn setWidth(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat8setWidthERK11QTextLength()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextFrameFormat8setWidthERK11QTextLength(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::bottomMargin();
impl /*struct*/ QTextFrameFormat {
  pub fn bottomMargin<RetType, T: QTextFrameFormat_bottomMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_bottomMargin<RetType> {
  fn bottomMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::bottomMargin();
impl<'a> /*trait*/ QTextFrameFormat_bottomMargin<f64> for () {
  fn bottomMargin(self , rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat12bottomMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat12bottomMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setBottomMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setBottomMargin<RetType, T: QTextFrameFormat_setBottomMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottomMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setBottomMargin<RetType> {
  fn setBottomMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setBottomMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setBottomMargin<()> for (f64) {
  fn setBottomMargin(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat15setBottomMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat15setBottomMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextLength QTextFrameFormat::height();
impl /*struct*/ QTextFrameFormat {
  pub fn height<RetType, T: QTextFrameFormat_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_height<RetType> {
  fn height(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  QTextLength QTextFrameFormat::height();
impl<'a> /*trait*/ QTextFrameFormat_height<QTextLength> for () {
  fn height(self , rsthis: &mut QTextFrameFormat) -> QTextLength {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6heightEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat6heightEv(rsthis.qclsinst)};
    let mut ret1 = QTextLength::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setWidth(qreal width);
impl<'a> /*trait*/ QTextFrameFormat_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::rightMargin();
impl /*struct*/ QTextFrameFormat {
  pub fn rightMargin<RetType, T: QTextFrameFormat_rightMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rightMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_rightMargin<RetType> {
  fn rightMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::rightMargin();
impl<'a> /*trait*/ QTextFrameFormat_rightMargin<f64> for () {
  fn rightMargin(self , rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat11rightMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat11rightMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setPadding(qreal padding);
impl /*struct*/ QTextFrameFormat {
  pub fn setPadding<RetType, T: QTextFrameFormat_setPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPadding(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setPadding<RetType> {
  fn setPadding(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setPadding(qreal padding);
impl<'a> /*trait*/ QTextFrameFormat_setPadding<()> for (f64) {
  fn setPadding(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat10setPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat10setPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setTopMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setTopMargin<RetType, T: QTextFrameFormat_setTopMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTopMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setTopMargin<RetType> {
  fn setTopMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setTopMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setTopMargin<()> for (f64) {
  fn setTopMargin(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat12setTopMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat12setTopMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::topMargin();
impl /*struct*/ QTextFrameFormat {
  pub fn topMargin<RetType, T: QTextFrameFormat_topMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_topMargin<RetType> {
  fn topMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::topMargin();
impl<'a> /*trait*/ QTextFrameFormat_topMargin<f64> for () {
  fn topMargin(self , rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat9topMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat9topMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QTextLength QTextFrameFormat::width();
impl /*struct*/ QTextFrameFormat {
  pub fn width<RetType, T: QTextFrameFormat_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_width<RetType> {
  fn width(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  QTextLength QTextFrameFormat::width();
impl<'a> /*trait*/ QTextFrameFormat_width<QTextLength> for () {
  fn width(self , rsthis: &mut QTextFrameFormat) -> QTextLength {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat5widthEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat5widthEv(rsthis.qclsinst)};
    let mut ret1 = QTextLength::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::QTextFrameFormat(const QTextFormat & fmt);
impl /*struct*/ QTextFrameFormat {
  pub fn NewQTextFrameFormat<T: QTextFrameFormat_NewQTextFrameFormat>(value: T) -> QTextFrameFormat {
    let rsthis = value.NewQTextFrameFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFrameFormat_NewQTextFrameFormat {
  fn NewQTextFrameFormat(self) -> QTextFrameFormat;
}

  // proto:  void QTextFrameFormat::QTextFrameFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextFrameFormat_NewQTextFrameFormat for (QTextFormat) {
  fn NewQTextFrameFormat(self) -> QTextFrameFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTextFrameFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextFrameFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::padding();
impl /*struct*/ QTextFrameFormat {
  pub fn padding<RetType, T: QTextFrameFormat_padding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.padding(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_padding<RetType> {
  fn padding(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::padding();
impl<'a> /*trait*/ QTextFrameFormat_padding<f64> for () {
  fn padding(self , rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat7paddingEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat7paddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::setLeftMargin(qreal margin);
impl /*struct*/ QTextFrameFormat {
  pub fn setLeftMargin<RetType, T: QTextFrameFormat_setLeftMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLeftMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setLeftMargin<RetType> {
  fn setLeftMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  void QTextFrameFormat::setLeftMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setLeftMargin<()> for (f64) {
  fn setLeftMargin(self , rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat13setLeftMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat13setLeftMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::border();
impl /*struct*/ QTextFrameFormat {
  pub fn border<RetType, T: QTextFrameFormat_border<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.border(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_border<RetType> {
  fn border(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::border();
impl<'a> /*trait*/ QTextFrameFormat_border<f64> for () {
  fn border(self , rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6borderEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat6borderEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextFrameFormat::QTextFrameFormat();
impl<'a> /*trait*/ QTextFrameFormat_NewQTextFrameFormat for () {
  fn NewQTextFrameFormat(self) -> QTextFrameFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormatC1Ev()};
    unsafe {_ZN16QTextFrameFormatC1Ev(qthis)};
    let rsthis = QTextFrameFormat{/**/qbase: QTextFormat::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextFrameFormat::leftMargin();
impl /*struct*/ QTextFrameFormat {
  pub fn leftMargin<RetType, T: QTextFrameFormat_leftMargin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leftMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_leftMargin<RetType> {
  fn leftMargin(self , rsthis: &mut QTextFrameFormat) -> RetType;
}

  // proto:  qreal QTextFrameFormat::leftMargin();
impl<'a> /*trait*/ QTextFrameFormat_leftMargin<f64> for () {
  fn leftMargin(self , rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat10leftMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat10leftMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

