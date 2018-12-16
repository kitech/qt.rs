
package qtcore
import "unsafe"
import "github.com/kitech/qt.go/qtrt"
func init(){
  if false{_=unsafe.Pointer(uintptr(0))}
  if false{qtrt.KeepMe()}
  if false{qtrt.KeepMe()}
}
//  header block end

//  body block begin
// /usr/include/qt/QtCore/qglobal.h:360
// index:0
// Invalid Visibility=Default Availability=Available
// [8] const char * qVersion()

/*

*/
func QVersion() string {
  rv, err := qtrt.InvokeQtFunc6("qVersion", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qmargins.h:471
// index:0
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator+(const QMarginsF &)

/*

*/
func Operator+(margins QMarginsF_ITF) *QMarginsF/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMarginsF_PTR() != nil {
        convArg0 = margins.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZpsRK9QMarginsF", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:267
// index:1
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator+(const QMargins &)

/*

*/
func Operator+_1(margins QMargins_ITF) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg0 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZpsRK8QMargins", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:378
// index:2
// Invalid inline Visibility=Default Availability=Available
// [16] const QPointF operator+(const QPointF &)

/*

*/
func Operator+_2(p QPointF_ITF) *QPointF/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPointF_PTR() != nil {
        convArg0 = p.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZpsRK7QPointF", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPointF)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:193
// index:3
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator+(const QPoint &)

/*

*/
func Operator+_3(p QPoint_ITF) *QPoint/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg0 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZpsRK6QPoint", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:171
// index:4
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator+(int, const QMargins &)

/*

*/
func Operator+_4(lhs int, rhs QMargins_ITF) *QMargins/*123*/ {
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMargins_PTR() != nil {
        convArg1 = rhs.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZpliRK8QMargins", qtrt.FFI_TYPE_POINTER, lhs, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:403
// index:5
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator+(qreal, const QMarginsF &)

/*

*/
func Operator+_5(lhs float64, rhs QMarginsF_ITF) *QMarginsF/*123*/ {
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMarginsF_PTR() != nil {
        convArg1 = rhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZpldRK9QMarginsF", qtrt.FFI_TYPE_POINTER, lhs, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qstring.h:1344
// index:6
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(char, const QString &)

/*

*/
func Operator+_6(c byte, s string) string {
    var tmpArg1 = NewQString_5(s)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZplcRK7QString", qtrt.FFI_TYPE_POINTER, c, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qbytearray.h:648
// index:7
// Invalid inline Visibility=Default Availability=Available
// [8] const QByteArray operator+(char, const QByteArray &)

/*

*/
func Operator+_7(a1 byte, a2 QByteArray_ITF) *QByteArray/*123*/ {
    var convArg1 unsafe.Pointer
    if a2 != nil && a2.QByteArray_PTR() != nil {
        convArg1 = a2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplcRK10QByteArray", qtrt.FFI_TYPE_POINTER, a1, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:397
// index:8
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator+(const QMarginsF &, qreal)

/*

*/
func Operator+_8(lhs QMarginsF_ITF, rhs float64) *QMarginsF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK9QMarginsFd", qtrt.FFI_TYPE_POINTER, convArg0, rhs)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:385
// index:9
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator+(const QMarginsF &, const QMarginsF &)

/*

*/
func Operator+_9(lhs QMarginsF_ITF, rhs QMarginsF_ITF) *QMarginsF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMarginsF_PTR() != nil {
        convArg1 = rhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK9QMarginsFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qrect.h:882
// index:10
// Invalid inline Visibility=Default Availability=Available
// [32] QRectF operator+(const QMarginsF &, const QRectF &)

/*

*/
func Operator+_10(lhs QMarginsF_ITF, rhs QRectF_ITF) *QRectF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QRectF_PTR() != nil {
        convArg1 = rhs.QRectF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK9QMarginsFRK6QRectF", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQRectFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQRectF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:165
// index:11
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator+(const QMargins &, int)

/*

*/
func Operator+_11(lhs QMargins_ITF, rhs int) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMargins_PTR() != nil {
        convArg0 = lhs.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK8QMarginsi", qtrt.FFI_TYPE_POINTER, convArg0, rhs)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:153
// index:12
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator+(const QMargins &, const QMargins &)

/*

*/
func Operator+_12(m1 QMargins_ITF, m2 QMargins_ITF) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if m1 != nil && m1.QMargins_PTR() != nil {
        convArg0 = m1.QMargins_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if m2 != nil && m2.QMargins_PTR() != nil {
        convArg1 = m2.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK8QMarginsS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qrect.h:470
// index:13
// Invalid inline Visibility=Default Availability=Available
// [16] QRect operator+(const QMargins &, const QRect &)

/*

*/
func Operator+_13(margins QMargins_ITF, rectangle QRect_ITF) *QRect/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg0 = margins.QMargins_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rectangle != nil && rectangle.QRect_PTR() != nil {
        convArg1 = rectangle.QRect_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK8QMarginsRK5QRect", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQRectFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQRect)
    return rv2
}

// /usr/include/qt/QtCore/qstring.h:1346
// index:14
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(const QString &, char)

/*

*/
func Operator+_14(s string, c byte) string {
    var tmpArg0 = NewQString_5(s)
    var convArg0 = tmpArg0.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZplRK7QStringc", qtrt.FFI_TYPE_POINTER, convArg0, c)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1333
// index:15
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(const QString &, const QString &)

/*

*/
func Operator+_15(s1 string, s2 string) string {
    var tmpArg0 = NewQString_5(s1)
    var convArg0 = tmpArg0.GetCthis()
    var tmpArg1 = NewQString_5(s2)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZplRK7QStringS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1825
// index:16
// Invalid inline Visibility=Default Availability=Available
// [8] QString operator+(const QString &, const QStringRef &)

/*

*/
func Operator+_16(s1 string, s2 QStringRef_ITF) string {
    var tmpArg0 = NewQString_5(s1)
    var convArg0 = tmpArg0.GetCthis()
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QStringRef_PTR() != nil {
        convArg1 = s2.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK7QStringRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1350
// index:17
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(const QString &, const QByteArray &)

/*

*/
func Operator+_17(s string, ba QByteArray_ITF) string {
    var tmpArg0 = NewQString_5(s)
    var convArg0 = tmpArg0.GetCthis()
    var convArg1 unsafe.Pointer
    if ba != nil && ba.QByteArray_PTR() != nil {
        convArg1 = ba.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK7QStringRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1340
// index:18
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(const QString &, const char *)

/*

*/
func Operator+_18(s1 string, s2 string) string {
    var tmpArg0 = NewQString_5(s1)
    var convArg0 = tmpArg0.GetCthis()
    var convArg1 = qtrt.CString(s2)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_ZplRK7QStringPKc", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1335
// index:19
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(const QString &, QChar)

/*

*/
func Operator+_19(s1 string, s2 QChar_ITF/*123*/) string {
    var tmpArg0 = NewQString_5(s1)
    var convArg0 = tmpArg0.GetCthis()
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QChar_PTR() != nil {
        convArg1 = s2.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK7QString5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qpoint.h:358
// index:20
// Invalid inline Visibility=Default Availability=Available
// [16] const QPointF operator+(const QPointF &, const QPointF &)

/*

*/
func Operator+_20(p1 QPointF_ITF, p2 QPointF_ITF) *QPointF/*123*/ {
    var convArg0 unsafe.Pointer
    if p1 != nil && p1.QPointF_PTR() != nil {
        convArg0 = p1.QPointF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if p2 != nil && p2.QPointF_PTR() != nil {
        convArg1 = p2.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK7QPointFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPointF)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:345
// index:21
// Invalid inline Visibility=Default Availability=Available
// [16] const QSizeF operator+(const QSizeF &, const QSizeF &)

/*

*/
func Operator+_21(s1 QSizeF_ITF, s2 QSizeF_ITF) *QSizeF/*123*/ {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QSizeF_PTR() != nil {
        convArg0 = s1.QSizeF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QSizeF_PTR() != nil {
        convArg1 = s2.QSizeF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK6QSizeFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSizeF)
    return rv2
}

// /usr/include/qt/QtCore/qrect.h:876
// index:22
// Invalid inline Visibility=Default Availability=Available
// [32] QRectF operator+(const QRectF &, const QMarginsF &)

/*

*/
func Operator+_22(lhs QRectF_ITF, rhs QMarginsF_ITF) *QRectF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QRectF_PTR() != nil {
        convArg0 = lhs.QRectF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMarginsF_PTR() != nil {
        convArg1 = rhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK6QRectFRK9QMarginsF", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQRectFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQRectF)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:169
// index:23
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator+(const QPoint &, const QPoint &)

/*

*/
func Operator+_23(p1 QPoint_ITF, p2 QPoint_ITF) *QPoint/*123*/ {
    var convArg0 unsafe.Pointer
    if p1 != nil && p1.QPoint_PTR() != nil {
        convArg0 = p1.QPoint_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if p2 != nil && p2.QPoint_PTR() != nil {
        convArg1 = p2.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK6QPointS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:175
// index:24
// Invalid inline Visibility=Default Availability=Available
// [8] const QSize operator+(const QSize &, const QSize &)

/*

*/
func Operator+_24(s1 QSize_ITF, s2 QSize_ITF) *QSize/*123*/ {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QSize_PTR() != nil {
        convArg0 = s1.QSize_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QSize_PTR() != nil {
        convArg1 = s2.QSize_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK5QSizeS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSize)
    return rv2
}

// /usr/include/qt/QtCore/qrect.h:464
// index:25
// Invalid inline Visibility=Default Availability=Available
// [16] QRect operator+(const QRect &, const QMargins &)

/*

*/
func Operator+_25(rectangle QRect_ITF, margins QMargins_ITF) *QRect/*123*/ {
    var convArg0 unsafe.Pointer
    if rectangle != nil && rectangle.QRect_PTR() != nil {
        convArg0 = rectangle.QRect_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg1 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK5QRectRK8QMargins", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQRectFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQRect)
    return rv2
}

// /usr/include/qt/QtCore/qstring.h:1833
// index:26
// Invalid inline Visibility=Default Availability=Available
// [8] QString operator+(const QStringRef &, const QStringRef &)

/*

*/
func Operator+_26(s1 QStringRef_ITF, s2 QStringRef_ITF) string {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QStringRef_PTR() != nil {
        convArg0 = s1.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QStringRef_PTR() != nil {
        convArg1 = s2.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK10QStringRefS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1827
// index:27
// Invalid inline Visibility=Default Availability=Available
// [8] QString operator+(const QStringRef &, const QString &)

/*

*/
func Operator+_27(s1 QStringRef_ITF, s2 string) string {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QStringRef_PTR() != nil {
        convArg0 = s1.QStringRef_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(s2)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZplRK10QStringRefRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1835
// index:28
// Invalid inline Visibility=Default Availability=Available
// [8] QString operator+(const QStringRef &, QChar)

/*

*/
func Operator+_28(s1 QStringRef_ITF, s2 QChar_ITF/*123*/) string {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QStringRef_PTR() != nil {
        convArg0 = s1.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QChar_PTR() != nil {
        convArg1 = s2.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK10QStringRef5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1829
// index:29
// Invalid inline Visibility=Default Availability=Available
// [8] QString operator+(const QStringRef &, QLatin1String)

/*

*/
func Operator+_29(s1 QStringRef_ITF, s2 QLatin1String_ITF/*123*/) string {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QStringRef_PTR() != nil {
        convArg0 = s1.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QLatin1String_PTR() != nil {
        convArg1 = s2.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK10QStringRef13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qbytearray.h:644
// index:30
// Invalid inline Visibility=Default Availability=Available
// [8] const QByteArray operator+(const QByteArray &, char)

/*

*/
func Operator+_30(a1 QByteArray_ITF, a2 byte) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if a1 != nil && a1.QByteArray_PTR() != nil {
        convArg0 = a1.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK10QByteArrayc", qtrt.FFI_TYPE_POINTER, convArg0, a2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qbytearray.h:640
// index:31
// Invalid inline Visibility=Default Availability=Available
// [8] const QByteArray operator+(const QByteArray &, const QByteArray &)

/*

*/
func Operator+_31(a1 QByteArray_ITF, a2 QByteArray_ITF) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if a1 != nil && a1.QByteArray_PTR() != nil {
        convArg0 = a1.QByteArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if a2 != nil && a2.QByteArray_PTR() != nil {
        convArg1 = a2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplRK10QByteArrayS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qstring.h:1348
// index:32
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(const QByteArray &, const QString &)

/*

*/
func Operator+_32(ba QByteArray_ITF, s string) string {
    var convArg0 unsafe.Pointer
    if ba != nil && ba.QByteArray_PTR() != nil {
        convArg0 = ba.QByteArray_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(s)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZplRK10QByteArrayRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qbytearray.h:642
// index:33
// Invalid inline Visibility=Default Availability=Available
// [8] const QByteArray operator+(const QByteArray &, const char *)

/*

*/
func Operator+_33(a1 QByteArray_ITF, a2 string) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if a1 != nil && a1.QByteArray_PTR() != nil {
        convArg0 = a1.QByteArray_PTR().GetCthis()
    }
    var convArg1 = qtrt.CString(a2)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_ZplRK10QByteArrayPKc", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qstring.h:1342
// index:34
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(const char *, const QString &)

/*

*/
func Operator+_34(s1 string, s2 string) string {
    var convArg0 = qtrt.CString(s1)
    defer qtrt.FreeMem(convArg0)
    var tmpArg1 = NewQString_5(s2)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZplPKcRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qbytearray.h:646
// index:35
// Invalid inline Visibility=Default Availability=Available
// [8] const QByteArray operator+(const char *, const QByteArray &)

/*

*/
func Operator+_35(a1 string, a2 QByteArray_ITF) *QByteArray/*123*/ {
    var convArg0 = qtrt.CString(a1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if a2 != nil && a2.QByteArray_PTR() != nil {
        convArg1 = a2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZplPKcRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qstring.h:1337
// index:36
// Invalid inline Visibility=Default Availability=Available
// [8] const QString operator+(QChar, const QString &)

/*

*/
func Operator+_36(s1 QChar_ITF/*123*/, s2 string) string {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QChar_PTR() != nil {
        convArg0 = s1.QChar_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(s2)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Zpl5QCharRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1837
// index:37
// Invalid inline Visibility=Default Availability=Available
// [8] QString operator+(QChar, const QStringRef &)

/*

*/
func Operator+_37(s1 QChar_ITF/*123*/, s2 QStringRef_ITF) string {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QChar_PTR() != nil {
        convArg0 = s1.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QStringRef_PTR() != nil {
        convArg1 = s2.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zpl5QCharRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstring.h:1831
// index:38
// Invalid inline Visibility=Default Availability=Available
// [8] QString operator+(QLatin1String, const QStringRef &)

/*

*/
func Operator+_38(s1 QLatin1String_ITF/*123*/, s2 QStringRef_ITF) string {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QLatin1String_PTR() != nil {
        convArg0 = s1.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QStringRef_PTR() != nil {
        convArg1 = s2.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zpl13QLatin1StringRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qbitarray.h:116
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QBitArray operator|(const QBitArray &, const QBitArray &)

/*

*/
func Operator|(arg0 QBitArray_ITF, arg1 QBitArray_ITF) *QBitArray/*123*/ {
    var convArg0 unsafe.Pointer
    if arg0 != nil && arg0.QBitArray_PTR() != nil {
        convArg0 = arg0.QBitArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if arg1 != nil && arg1.QBitArray_PTR() != nil {
        convArg1 = arg1.QBitArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZorRK9QBitArrayS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQBitArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQBitArray)
    return rv2
}

// /usr/include/qt/QtCore/qmetatype.h:727
// index:1
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QMetaType::TypeFlags::enum_type, int)

/*

*/
func Operator|_1(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN9QMetaType8TypeFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qiodevice.h:183
// index:2
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QIODevice::OpenMode::enum_type, int)

/*

*/
func Operator|_2(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN9QIODevice12OpenModeFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qlibrary.h:100
// index:3
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QLibrary::LoadHints::enum_type, int)

/*

*/
func Operator|_3(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN8QLibrary8LoadHintEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qstring.h:1404
// index:4
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QString::SectionFlags::enum_type, int)

/*

*/
func Operator|_4(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN7QString11SectionFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qlocale.h:1096
// index:5
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QLocale::NumberOptions::enum_type, int)

/*

*/
func Operator|_5(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN7QLocale12NumberOptionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qurl.h:376
// index:6
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QUrl::ComponentFormattingOptions::enum_type, int)

/*

*/
func Operator|_6(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QUrl25ComponentFormattingOptionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qurl.h:394
// index:7
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::ComponentFormattingOption, QUrl::UrlFormattingOption)

/*

*/
func Operator|_7(f int, i int) int {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QUrl25ComponentFormattingOptionENS_19UrlFormattingOptionE", qtrt.FFI_TYPE_POINTER, f, i)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qurl.h:400
// index:8
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::ComponentFormattingOption, QUrl::FormattingOptions)

/*

*/
func Operator|_8(f int, i int) int {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QUrl25ComponentFormattingOptionE12QUrlTwoFlagsINS_19UrlFormattingOptionES0_E", qtrt.FFI_TYPE_POINTER, f, i)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qurl.h:384
// index:9
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QUrl::UrlFormattingOption, int)

/*

*/
func Operator|_9(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QUrl19UrlFormattingOptionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qurl.h:380
// index:10
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::UrlFormattingOption, QUrl::UrlFormattingOption)

/*

*/
func Operator|_10(f1 int, f2 int) int {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QUrl19UrlFormattingOptionES0_", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qurl.h:390
// index:11
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption)

/*

*/
func Operator|_11(i int, f int) int {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QUrl19UrlFormattingOptionENS_25ComponentFormattingOptionE", qtrt.FFI_TYPE_POINTER, i, f)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qurl.h:392
// index:12
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::UrlFormattingOption, QUrl::ComponentFormattingOptions)

/*

*/
func Operator|_12(i int, f int) int {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QUrl19UrlFormattingOptionE6QFlagsINS_25ComponentFormattingOptionEE", qtrt.FFI_TYPE_POINTER, i, f)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qurl.h:382
// index:13
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::UrlFormattingOption, QUrl::FormattingOptions)

/*

*/
func Operator|_13(f1 int, f2 int) int {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QUrl19UrlFormattingOptionE12QUrlTwoFlagsIS0_NS_25ComponentFormattingOptionEE", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qdir.h:235
// index:14
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QDir::SortFlags::enum_type, int)

/*

*/
func Operator|_14(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QDir8SortFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qdir.h:234
// index:15
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QDir::Filters::enum_type, int)

/*

*/
func Operator|_15(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN4QDir6FilterEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1783
// index:16
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::MatchFlags::enum_type, int)

/*

*/
func Operator|_16(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt9MatchFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1782
// index:17
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::ItemFlags::enum_type, int)

/*

*/
func Operator|_17(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt8ItemFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1775
// index:18
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::Edges::enum_type, int)

/*

*/
func Operator|_18(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt4EdgeEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1784
// index:19
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::TextInteractionFlags::enum_type, int)

/*

*/
func Operator|_19(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt19TextInteractionFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1776
// index:20
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::ImageConversionFlags::enum_type, int)

/*

*/
func Operator|_20(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt19ImageConversionFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1780
// index:21
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::ScreenOrientations::enum_type, int)

/*

*/
func Operator|_21(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt17ScreenOrientationEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1772
// index:22
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::KeyboardModifiers::enum_type, int)

/*

*/
func Operator|_22(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt16KeyboardModifierEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1785
// index:23
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::InputMethodQueries::enum_type, int)

/*

*/
func Operator|_23(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt16InputMethodQueryEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1787
// index:24
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::TouchPointStates::enum_type, int)

/*

*/
func Operator|_24(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt15TouchPointStateEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1786
// index:25
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::InputMethodHints::enum_type, int)

/*

*/
func Operator|_25(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt15InputMethodHintEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1788
// index:26
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::MouseEventFlags::enum_type, int)

/*

*/
func Operator|_26(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt14MouseEventFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1777
// index:27
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::DockWidgetAreas::enum_type, int)

/*

*/
func Operator|_27(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt14DockWidgetAreaEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1774
// index:28
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::Alignment::enum_type, int)

/*

*/
func Operator|_28(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt13AlignmentFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1779
// index:29
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::WindowStates::enum_type, int)

/*

*/
func Operator|_29(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt11WindowStateEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1778
// index:30
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::ToolBarAreas::enum_type, int)

/*

*/
func Operator|_30(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt11ToolBarAreaEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1771
// index:31
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::Orientations::enum_type, int)

/*

*/
func Operator|_31(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt11OrientationEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1770
// index:32
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::MouseButtons::enum_type, int)

/*

*/
func Operator|_32(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt11MouseButtonEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1790
// index:33
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::GestureFlags::enum_type, int)

/*

*/
func Operator|_33(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt11GestureFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1773
// index:34
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::WindowFlags::enum_type, int)

/*

*/
func Operator|_34(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt10WindowTypeEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qnamespace.h:1781
// index:35
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(Qt::DropActions::enum_type, int)

/*

*/
func Operator|_35(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN2Qt10DropActionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qtextboundaryfinder.h:106
// index:36
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QTextBoundaryFinder::BoundaryReasons::enum_type, int)

/*

*/
func Operator|_36(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN19QTextBoundaryFinder14BoundaryReasonEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:224
// index:37
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QItemSelectionModel::SelectionFlags::enum_type, int)

/*

*/
func Operator|_37(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN19QItemSelectionModel13SelectionFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qregularexpression.h:160
// index:38
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QRegularExpression::PatternOptions::enum_type, int)

/*

*/
func Operator|_38(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN18QRegularExpression13PatternOptionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qregularexpression.h:161
// index:39
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QRegularExpression::MatchOptions::enum_type, int)

/*

*/
func Operator|_39(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN18QRegularExpression11MatchOptionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qcommandlineoption.h:109
// index:40
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QCommandLineOption::Flags::enum_type, int)

/*

*/
func Operator|_40(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN18QCommandLineOption4FlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qstandardpaths.h:111
// index:41
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QStandardPaths::LocateOptions::enum_type, int)

/*

*/
func Operator|_41(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN14QStandardPaths12LocateOptionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qdiriterator.h:86
// index:42
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QDirIterator::IteratorFlags::enum_type, int)

/*

*/
func Operator|_42(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN12QDirIterator12IteratorFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:201
// index:43
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QTextStream::NumberFlags::enum_type, int)

/*

*/
func Operator|_43(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN11QTextStream10NumberFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qfiledevice.h:150
// index:44
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QFileDevice::Permissions::enum_type, int)

/*

*/
func Operator|_44(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN11QFileDevice10PermissionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qtextcodec.h:136
// index:45
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QTextCodec::ConversionFlags::enum_type, int)

/*

*/
func Operator|_45(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN10QTextCodec14ConversionFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qeventloop.h:85
// index:46
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QEventLoop::ProcessEventsFlags::enum_type, int)

/*

*/
func Operator|_46(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN10QEventLoop17ProcessEventsFlagEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qbytearray.h:466
// index:47
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QByteArray::Base64Options::enum_type, int)

/*

*/
func Operator|_47(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN10QByteArray12Base64OptionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qarraydata.h:126
// index:48
// Invalid inline Visibility=Default Availability=Available
// [4] QIncompatibleFlag operator|(QArrayData::AllocationOptions::enum_type, int)

/*

*/
func Operator|_48(f1 int, f2 int) *QIncompatibleFlag/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_ZorN10QArrayData16AllocationOptionEi", qtrt.FFI_TYPE_POINTER, f1, f2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQIncompatibleFlagFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQIncompatibleFlag)
    return rv2
}

// /usr/include/qt/QtCore/qurl.h:396
// index:49
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::ComponentFormattingOptions, QUrl::UrlFormattingOption)

/*

*/
func Operator|_49(f int, i int) int {
  rv, err := qtrt.InvokeQtFunc6("_Zor6QFlagsIN4QUrl25ComponentFormattingOptionEENS0_19UrlFormattingOptionE", qtrt.FFI_TYPE_POINTER, f, i)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qurl.h:402
// index:50
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::ComponentFormattingOptions, QUrl::FormattingOptions)

/*

*/
func Operator|_50(f int, i int) int {
  rv, err := qtrt.InvokeQtFunc6("_Zor6QFlagsIN4QUrl25ComponentFormattingOptionEE12QUrlTwoFlagsINS0_19UrlFormattingOptionES1_E", qtrt.FFI_TYPE_POINTER, f, i)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qurl.h:398
// index:51
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions operator|(QUrl::FormattingOptions, QUrl::ComponentFormattingOptions)

/*

*/
func Operator|_51(i int, f int) int {
  rv, err := qtrt.InvokeQtFunc6("_Zor12QUrlTwoFlagsIN4QUrl19UrlFormattingOptionENS0_25ComponentFormattingOptionEE6QFlagsIS2_E", qtrt.FFI_TYPE_POINTER, i, f)
  qtrt.ErrPrint(err, rv)
    return int(rv)
}

// /usr/include/qt/QtCore/qurl.h:388
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] QUrl::FormattingOptions & operator|=(QUrl::FormattingOptions &, QUrl::ComponentFormattingOptions)

/*

*/
func Operator|=(i int, f int) int {
  rv, err := qtrt.InvokeQtFunc6("_ZoRR12QUrlTwoFlagsIN4QUrl19UrlFormattingOptionENS0_25ComponentFormattingOptionEE6QFlagsIS2_E", qtrt.FFI_TYPE_POINTER, &i, f)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cpretval2go("int", rv).(int) // 3331
}

// /usr/include/qt/QtCore/qmargins.h:476
// index:0
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator-(const QMarginsF &)

/*
Returns a QMargins object that is formed by subtracting m2 from m1; each component is subtracted separately.

This function was introduced in  Qt 5.1.

See also QMargins::operator+=() and QMargins::operator-=().
*/
func Operator-(margins QMarginsF_ITF) *QMarginsF/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMarginsF_PTR() != nil {
        convArg0 = margins.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZngRK9QMarginsF", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:272
// index:1
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator-(const QMargins &)

/*
Returns a QMargins object that is formed by subtracting m2 from m1; each component is subtracted separately.

This function was introduced in  Qt 5.1.

See also QMargins::operator+=() and QMargins::operator-=().
*/
func Operator-_1(margins QMargins_ITF) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg0 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZngRK8QMargins", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:383
// index:2
// Invalid inline Visibility=Default Availability=Available
// [16] const QPointF operator-(const QPointF &)

/*
Returns a QPoint object that is formed by subtracting p2 from p1; each component is subtracted separately.

See also QPoint::operator-=().
*/
func Operator-_2(p QPointF_ITF) *QPointF/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPointF_PTR() != nil {
        convArg0 = p.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZngRK7QPointF", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPointF)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:196
// index:3
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator-(const QPoint &)

/*
Returns a QPoint object that is formed by subtracting p2 from p1; each component is subtracted separately.

See also QPoint::operator-=().
*/
func Operator-_3(p QPoint_ITF) *QPoint/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg0 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZngRK6QPoint", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:380
// index:0
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QMarginsF &, const QMarginsF &)

/*

*/
func Operator!=(lhs QMarginsF_ITF, rhs QMarginsF_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMarginsF_PTR() != nil {
        convArg1 = rhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK9QMarginsFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qvariant.h:572
// index:1
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QVariant &, const QVariantComparisonHelper &)

/*

*/
func Operator!=_1(v1 QVariant_ITF, v2 QVariantComparisonHelper_ITF) bool {
    var convArg0 unsafe.Pointer
    if v1 != nil && v1.QVariant_PTR() != nil {
        convArg0 = v1.QVariant_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if v2 != nil && v2.QVariantComparisonHelper_PTR() != nil {
        convArg1 = v2.QVariantComparisonHelper_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK8QVariantRK24QVariantComparisonHelper", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qmargins.h:144
// index:2
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QMargins &, const QMargins &)

/*

*/
func Operator!=_2(m1 QMargins_ITF, m2 QMargins_ITF) bool {
    var convArg0 unsafe.Pointer
    if m1 != nil && m1.QMargins_PTR() != nil {
        convArg0 = m1.QMargins_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if m2 != nil && m2.QMargins_PTR() != nil {
        convArg1 = m2.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK8QMarginsS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1615
// index:3
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QString &, const QStringRef &)

/*

*/
func Operator!=_3(lhs string, rhs QStringRef_ITF) bool {
    var tmpArg0 = NewQString_5(lhs)
    var convArg0 = tmpArg0.GetCthis()
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringRef_PTR() != nil {
        convArg1 = rhs.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK7QStringRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1187
// index:4
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QString &, QString::Null)

/*

*/
func Operator!=_4(s string, arg1 int) bool {
    var tmpArg0 = NewQString_5(s)
    var convArg0 = tmpArg0.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZneRK7QStringNS_4NullE", qtrt.FFI_TYPE_POINTER, convArg0, arg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1673
// index:5
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QString &, QChar)

/*

*/
func Operator!=_5(lhs string, rhs QChar_ITF/*123*/) bool {
    var tmpArg0 = NewQString_5(lhs)
    var convArg0 = tmpArg0.GetCthis()
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK7QString5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qpoint.h:353
// index:6
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QPointF &, const QPointF &)

/*

*/
func Operator!=_6(p1 QPointF_ITF, p2 QPointF_ITF) bool {
    var convArg0 unsafe.Pointer
    if p1 != nil && p1.QPointF_PTR() != nil {
        convArg0 = p1.QPointF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if p2 != nil && p2.QPointF_PTR() != nil {
        convArg1 = p2.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK7QPointFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qsize.h:342
// index:7
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QSizeF &, const QSizeF &)

/*

*/
func Operator!=_7(s1 QSizeF_ITF, s2 QSizeF_ITF) bool {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QSizeF_PTR() != nil {
        convArg0 = s1.QSizeF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QSizeF_PTR() != nil {
        convArg1 = s2.QSizeF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK6QSizeFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qrect.h:865
// index:8
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QRectF &, const QRectF &)

/*

*/
func Operator!=_8(r1 QRectF_ITF, r2 QRectF_ITF) bool {
    var convArg0 unsafe.Pointer
    if r1 != nil && r1.QRectF_PTR() != nil {
        convArg0 = r1.QRectF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if r2 != nil && r2.QRectF_PTR() != nil {
        convArg1 = r2.QRectF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK6QRectFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qpoint.h:166
// index:9
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QPoint &, const QPoint &)

/*

*/
func Operator!=_9(p1 QPoint_ITF, p2 QPoint_ITF) bool {
    var convArg0 unsafe.Pointer
    if p1 != nil && p1.QPoint_PTR() != nil {
        convArg0 = p1.QPoint_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if p2 != nil && p2.QPoint_PTR() != nil {
        convArg1 = p2.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK6QPointS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qsize.h:172
// index:10
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QSize &, const QSize &)

/*

*/
func Operator!=_10(s1 QSize_ITF, s2 QSize_ITF) bool {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QSize_PTR() != nil {
        convArg0 = s1.QSize_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QSize_PTR() != nil {
        convArg1 = s2.QSize_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK5QSizeS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qrect.h:459
// index:11
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QRect &, const QRect &)

/*

*/
func Operator!=_11(r1 QRect_ITF, r2 QRect_ITF) bool {
    var convArg0 unsafe.Pointer
    if r1 != nil && r1.QRect_PTR() != nil {
        convArg0 = r1.QRect_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if r2 != nil && r2.QRect_PTR() != nil {
        convArg1 = r2.QRect_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK5QRectS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qversionnumber.h:317
// index:12
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QVersionNumber &, const QVersionNumber &)

/*

*/
func Operator!=_12(lhs QVersionNumber_ITF, rhs QVersionNumber_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QVersionNumber_PTR() != nil {
        convArg0 = lhs.QVersionNumber_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QVersionNumber_PTR() != nil {
        convArg1 = rhs.QVersionNumber_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK14QVersionNumberS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstorageinfo.h:110
// index:13
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QStorageInfo &, const QStorageInfo &)

/*

*/
func Operator!=_13(first QStorageInfo_ITF, second QStorageInfo_ITF) bool {
    var convArg0 unsafe.Pointer
    if first != nil && first.QStorageInfo_PTR() != nil {
        convArg0 = first.QStorageInfo_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if second != nil && second.QStorageInfo_PTR() != nil {
        convArg1 = second.QStorageInfo_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK12QStorageInfoS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qmetaobject.h:203
// index:14
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QMetaMethod &, const QMetaMethod &)

/*

*/
func Operator!=_14(m1 QMetaMethod_ITF, m2 QMetaMethod_ITF) bool {
    var convArg0 unsafe.Pointer
    if m1 != nil && m1.QMetaMethod_PTR() != nil {
        convArg0 = m1.QMetaMethod_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if m2 != nil && m2.QMetaMethod_PTR() != nil {
        convArg1 = m2.QMetaMethod_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK11QMetaMethodS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1603
// index:15
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QStringRef &, const QStringRef &)

/*

*/
func Operator!=_15(s1 QStringRef_ITF, s2 QStringRef_ITF) bool {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QStringRef_PTR() != nil {
        convArg0 = s1.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QStringRef_PTR() != nil {
        convArg1 = s2.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK10QStringRefS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1622
// index:16
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QStringRef &, const QString &)

/*

*/
func Operator!=_16(lhs QStringRef_ITF, rhs string) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringRef_PTR() != nil {
        convArg0 = lhs.QStringRef_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(rhs)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZneRK10QStringRefRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1758
// index:17
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QStringRef &, const QByteArray &)

/*

*/
func Operator!=_17(lhs QStringRef_ITF, rhs QByteArray_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringRef_PTR() != nil {
        convArg0 = lhs.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QByteArray_PTR() != nil {
        convArg1 = rhs.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK10QStringRefRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1692
// index:18
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QStringRef &, QChar)

/*

*/
func Operator!=_18(lhs QStringRef_ITF, rhs QChar_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringRef_PTR() != nil {
        convArg0 = lhs.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK10QStringRef5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1654
// index:19
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QStringRef &, QLatin1String)

/*

*/
func Operator!=_19(lhs QStringRef_ITF, rhs QLatin1String_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringRef_PTR() != nil {
        convArg0 = lhs.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QLatin1String_PTR() != nil {
        convArg1 = rhs.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK10QStringRef13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qbytearray.h:609
// index:20
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QByteArray &, const QByteArray &)

/*

*/
func Operator!=_20(a1 QByteArray_ITF, a2 QByteArray_ITF) bool {
    var convArg0 unsafe.Pointer
    if a1 != nil && a1.QByteArray_PTR() != nil {
        convArg0 = a1.QByteArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if a2 != nil && a2.QByteArray_PTR() != nil {
        convArg1 = a2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK10QByteArrayS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1765
// index:21
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QByteArray &, const QStringRef &)

/*

*/
func Operator!=_21(lhs QByteArray_ITF, rhs QStringRef_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QByteArray_PTR() != nil {
        convArg0 = lhs.QByteArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringRef_PTR() != nil {
        convArg1 = rhs.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneRK10QByteArrayRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qbytearray.h:611
// index:22
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const QByteArray &, const char *)

/*

*/
func Operator!=_22(a1 QByteArray_ITF, a2 string) bool {
    var convArg0 unsafe.Pointer
    if a1 != nil && a1.QByteArray_PTR() != nil {
        convArg0 = a1.QByteArray_PTR().GetCthis()
    }
    var convArg1 = qtrt.CString(a2)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_ZneRK10QByteArrayPKc", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1236
// index:23
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const char *, const QString &)

/*

*/
func Operator!=_23(s1 string, s2 string) bool {
    var convArg0 = qtrt.CString(s1)
    defer qtrt.FreeMem(convArg0)
    var tmpArg1 = NewQString_5(s2)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZnePKcRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1787
// index:24
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const char *, const QStringRef &)

/*

*/
func Operator!=_24(s1 string, s2 QStringRef_ITF) bool {
    var convArg0 = qtrt.CString(s1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QStringRef_PTR() != nil {
        convArg1 = s2.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZnePKcRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qbytearray.h:613
// index:25
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const char *, const QByteArray &)

/*

*/
func Operator!=_25(a1 string, a2 QByteArray_ITF) bool {
    var convArg0 = qtrt.CString(a1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if a2 != nil && a2.QByteArray_PTR() != nil {
        convArg1 = a2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZnePKcRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1249
// index:26
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(const char *, QLatin1String)

/*

*/
func Operator!=_26(s1 string, s2 QLatin1String_ITF/*123*/) bool {
    var convArg0 = qtrt.CString(s1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QLatin1String_PTR() != nil {
        convArg1 = s2.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZnePKc13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1183
// index:27
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QString::Null, QString::Null)

/*

*/
func Operator!=_27(arg0 int, arg1 int) bool {
  rv, err := qtrt.InvokeQtFunc6("_ZneN7QString4NullES0_", qtrt.FFI_TYPE_POINTER, arg0, arg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1185
// index:28
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QString::Null, const QString &)

/*

*/
func Operator!=_28(arg0 int, s string) bool {
    var tmpArg1 = NewQString_5(s)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZneN7QString4NullERKS_", qtrt.FFI_TYPE_POINTER, arg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qchar.h:599
// index:29
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(std::nullptr_t, QChar)

/*

*/
func Operator!=_29(arg0 int, rhs QChar_ITF/*123*/) bool {
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZneDn5QChar", qtrt.FFI_TYPE_POINTER, arg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qchar.h:583
// index:30
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QChar, QChar)

/*

*/
func Operator!=_30(c1 QChar_ITF/*123*/, c2 QChar_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if c1 != nil && c1.QChar_PTR() != nil {
        convArg0 = c1.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if c2 != nil && c2.QChar_PTR() != nil {
        convArg1 = c2.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne5QCharS_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1668
// index:31
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QChar, const QString &)

/*

*/
func Operator!=_31(lhs QChar_ITF/*123*/, rhs string) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(rhs)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Zne5QCharRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1687
// index:32
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QChar, const QStringRef &)

/*

*/
func Operator!=_32(lhs QChar_ITF/*123*/, rhs QStringRef_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringRef_PTR() != nil {
        convArg1 = rhs.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne5QCharRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qchar.h:594
// index:33
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QChar, std::nullptr_t)

/*

*/
func Operator!=_33(lhs QChar_ITF/*123*/, arg1 int) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne5QCharDn", qtrt.FFI_TYPE_POINTER, convArg0, arg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1706
// index:34
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QChar, QLatin1String)

/*

*/
func Operator!=_34(lhs QChar_ITF/*123*/, rhs QLatin1String_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QLatin1String_PTR() != nil {
        convArg1 = rhs.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne5QChar13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1734
// index:35
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QChar, QStringView)

/*

*/
func Operator!=_35(lhs QChar_ITF/*123*/, rhs QStringView_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringView_PTR() != nil {
        convArg1 = rhs.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne5QChar11QStringView", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1192
// index:36
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QLatin1String, QLatin1String)

/*

*/
func Operator!=_36(s1 QLatin1String_ITF/*123*/, s2 QLatin1String_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QLatin1String_PTR() != nil {
        convArg0 = s1.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QLatin1String_PTR() != nil {
        convArg1 = s2.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne13QLatin1StringS_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1647
// index:37
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QLatin1String, const QStringRef &)

/*

*/
func Operator!=_37(lhs QLatin1String_ITF/*123*/, rhs QStringRef_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QLatin1String_PTR() != nil {
        convArg0 = lhs.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringRef_PTR() != nil {
        convArg1 = rhs.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne13QLatin1StringRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1711
// index:38
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QLatin1String, QChar)

/*

*/
func Operator!=_38(lhs QLatin1String_ITF/*123*/, rhs QChar_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QLatin1String_PTR() != nil {
        convArg0 = lhs.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne13QLatin1String5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1749
// index:39
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QLatin1String, QStringView)

/*

*/
func Operator!=_39(lhs QLatin1String_ITF/*123*/, rhs QStringView_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QLatin1String_PTR() != nil {
        convArg0 = lhs.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringView_PTR() != nil {
        convArg1 = rhs.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne13QLatin1String11QStringView", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1719
// index:40
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QStringView, QStringView)

/*

*/
func Operator!=_40(lhs QStringView_ITF/*123*/, rhs QStringView_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringView_PTR() != nil {
        convArg0 = lhs.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringView_PTR() != nil {
        convArg1 = rhs.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne11QStringViewS_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1727
// index:41
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QStringView, QChar)

/*

*/
func Operator!=_41(lhs QStringView_ITF/*123*/, rhs QChar_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringView_PTR() != nil {
        convArg0 = lhs.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne11QStringView5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1742
// index:42
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator!=(QStringView, QLatin1String)

/*

*/
func Operator!=_42(lhs QStringView_ITF/*123*/, rhs QLatin1String_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringView_PTR() != nil {
        convArg0 = lhs.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QLatin1String_PTR() != nil {
        convArg1 = rhs.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zne11QStringView13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qmargins.h:189
// index:0
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator*(int, const QMargins &)

/*

*/
func Operator*(factor int, margins QMargins_ITF) *QMargins/*123*/ {
    var convArg1 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg1 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmliRK8QMargins", qtrt.FFI_TYPE_POINTER, factor, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:190
// index:1
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator*(int, const QPoint &)

/*

*/
func Operator*_1(factor int, p QPoint_ITF) *QPoint/*123*/ {
    var convArg1 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg1 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmliRK6QPoint", qtrt.FFI_TYPE_POINTER, factor, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:184
// index:2
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator*(float, const QPoint &)

/*

*/
func Operator*_2(factor float32, p QPoint_ITF) *QPoint/*123*/ {
    var convArg1 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg1 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlfRK6QPoint", qtrt.FFI_TYPE_POINTER, factor, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:421
// index:3
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator*(qreal, const QMarginsF &)

/*

*/
func Operator*_3(lhs float64, rhs QMarginsF_ITF) *QMarginsF/*123*/ {
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMarginsF_PTR() != nil {
        convArg1 = rhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmldRK9QMarginsF", qtrt.FFI_TYPE_POINTER, lhs, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:201
// index:4
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator*(qreal, const QMargins &)

/*

*/
func Operator*_4(factor float64, margins QMargins_ITF) *QMargins/*123*/ {
    var convArg1 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg1 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmldRK8QMargins", qtrt.FFI_TYPE_POINTER, factor, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:373
// index:5
// Invalid inline Visibility=Default Availability=Available
// [16] const QPointF operator*(qreal, const QPointF &)

/*

*/
func Operator*_5(c float64, p QPointF_ITF) *QPointF/*123*/ {
    var convArg1 unsafe.Pointer
    if p != nil && p.QPointF_PTR() != nil {
        convArg1 = p.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmldRK7QPointF", qtrt.FFI_TYPE_POINTER, c, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPointF)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:354
// index:6
// Invalid inline Visibility=Default Availability=Available
// [16] const QSizeF operator*(qreal, const QSizeF &)

/*

*/
func Operator*_6(c float64, s QSizeF_ITF) *QSizeF/*123*/ {
    var convArg1 unsafe.Pointer
    if s != nil && s.QSizeF_PTR() != nil {
        convArg1 = s.QSizeF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmldRK6QSizeF", qtrt.FFI_TYPE_POINTER, c, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSizeF)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:187
// index:7
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator*(double, const QPoint &)

/*

*/
func Operator*_7(factor float64, p QPoint_ITF) *QPoint/*123*/ {
    var convArg1 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg1 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmldRK6QPoint", qtrt.FFI_TYPE_POINTER, factor, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:184
// index:8
// Invalid inline Visibility=Default Availability=Available
// [8] const QSize operator*(qreal, const QSize &)

/*

*/
func Operator*_8(c float64, s QSize_ITF) *QSize/*123*/ {
    var convArg1 unsafe.Pointer
    if s != nil && s.QSize_PTR() != nil {
        convArg1 = s.QSize_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmldRK5QSize", qtrt.FFI_TYPE_POINTER, c, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSize)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:415
// index:9
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator*(const QMarginsF &, qreal)

/*

*/
func Operator*_9(lhs QMarginsF_ITF, rhs float64) *QMarginsF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK9QMarginsFd", qtrt.FFI_TYPE_POINTER, convArg0, rhs)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:183
// index:10
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator*(const QMargins &, int)

/*

*/
func Operator*_10(margins QMargins_ITF, factor int) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg0 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK8QMarginsi", qtrt.FFI_TYPE_POINTER, convArg0, factor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:195
// index:11
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator*(const QMargins &, qreal)

/*

*/
func Operator*_11(margins QMargins_ITF, factor float64) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg0 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK8QMarginsd", qtrt.FFI_TYPE_POINTER, convArg0, factor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:368
// index:12
// Invalid inline Visibility=Default Availability=Available
// [16] const QPointF operator*(const QPointF &, qreal)

/*

*/
func Operator*_12(p QPointF_ITF, c float64) *QPointF/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPointF_PTR() != nil {
        convArg0 = p.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK7QPointFd", qtrt.FFI_TYPE_POINTER, convArg0, c)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPointF)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:351
// index:13
// Invalid inline Visibility=Default Availability=Available
// [16] const QSizeF operator*(const QSizeF &, qreal)

/*

*/
func Operator*_13(s QSizeF_ITF, c float64) *QSizeF/*123*/ {
    var convArg0 unsafe.Pointer
    if s != nil && s.QSizeF_PTR() != nil {
        convArg0 = s.QSizeF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK6QSizeFd", qtrt.FFI_TYPE_POINTER, convArg0, c)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSizeF)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:181
// index:14
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator*(const QPoint &, int)

/*

*/
func Operator*_14(p QPoint_ITF, factor int) *QPoint/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg0 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK6QPointi", qtrt.FFI_TYPE_POINTER, convArg0, factor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:175
// index:15
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator*(const QPoint &, float)

/*

*/
func Operator*_15(p QPoint_ITF, factor float32) *QPoint/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg0 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK6QPointf", qtrt.FFI_TYPE_POINTER, convArg0, factor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:178
// index:16
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator*(const QPoint &, double)

/*

*/
func Operator*_16(p QPoint_ITF, factor float64) *QPoint/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg0 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK6QPointd", qtrt.FFI_TYPE_POINTER, convArg0, factor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:181
// index:17
// Invalid inline Visibility=Default Availability=Available
// [8] const QSize operator*(const QSize &, qreal)

/*

*/
func Operator*_17(s QSize_ITF, c float64) *QSize/*123*/ {
    var convArg0 unsafe.Pointer
    if s != nil && s.QSize_PTR() != nil {
        convArg0 = s.QSize_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmlRK5QSized", qtrt.FFI_TYPE_POINTER, convArg0, c)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSize)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:409
// index:4
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator-(const QMarginsF &, qreal)

/*
Returns a QMargins object that is formed by subtracting m2 from m1; each component is subtracted separately.

This function was introduced in  Qt 5.1.

See also QMargins::operator+=() and QMargins::operator-=().
*/
func Operator-_4(lhs QMarginsF_ITF, rhs float64) *QMarginsF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK9QMarginsFd", qtrt.FFI_TYPE_POINTER, convArg0, rhs)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:391
// index:5
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator-(const QMarginsF &, const QMarginsF &)

/*
Returns a QMargins object that is formed by subtracting m2 from m1; each component is subtracted separately.

This function was introduced in  Qt 5.1.

See also QMargins::operator+=() and QMargins::operator-=().
*/
func Operator-_5(lhs QMarginsF_ITF, rhs QMarginsF_ITF) *QMarginsF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMarginsF_PTR() != nil {
        convArg1 = rhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK9QMarginsFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:177
// index:6
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator-(const QMargins &, int)

/*
Returns a QMargins object that is formed by subtracting m2 from m1; each component is subtracted separately.

This function was introduced in  Qt 5.1.

See also QMargins::operator+=() and QMargins::operator-=().
*/
func Operator-_6(lhs QMargins_ITF, rhs int) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMargins_PTR() != nil {
        convArg0 = lhs.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK8QMarginsi", qtrt.FFI_TYPE_POINTER, convArg0, rhs)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:159
// index:7
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator-(const QMargins &, const QMargins &)

/*
Returns a QMargins object that is formed by subtracting m2 from m1; each component is subtracted separately.

This function was introduced in  Qt 5.1.

See also QMargins::operator+=() and QMargins::operator-=().
*/
func Operator-_7(m1 QMargins_ITF, m2 QMargins_ITF) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if m1 != nil && m1.QMargins_PTR() != nil {
        convArg0 = m1.QMargins_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if m2 != nil && m2.QMargins_PTR() != nil {
        convArg1 = m2.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK8QMarginsS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:363
// index:8
// Invalid inline Visibility=Default Availability=Available
// [16] const QPointF operator-(const QPointF &, const QPointF &)

/*
Returns a QPoint object that is formed by subtracting p2 from p1; each component is subtracted separately.

See also QPoint::operator-=().
*/
func Operator-_8(p1 QPointF_ITF, p2 QPointF_ITF) *QPointF/*123*/ {
    var convArg0 unsafe.Pointer
    if p1 != nil && p1.QPointF_PTR() != nil {
        convArg0 = p1.QPointF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if p2 != nil && p2.QPointF_PTR() != nil {
        convArg1 = p2.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK7QPointFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPointF)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:348
// index:9
// Invalid inline Visibility=Default Availability=Available
// [16] const QSizeF operator-(const QSizeF &, const QSizeF &)

/*
Returns s2 subtracted from s1; each component is subtracted separately.
*/
func Operator-_9(s1 QSizeF_ITF, s2 QSizeF_ITF) *QSizeF/*123*/ {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QSizeF_PTR() != nil {
        convArg0 = s1.QSizeF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QSizeF_PTR() != nil {
        convArg1 = s2.QSizeF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK6QSizeFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSizeF)
    return rv2
}

// /usr/include/qt/QtCore/qrect.h:888
// index:10
// Invalid inline Visibility=Default Availability=Available
// [32] QRectF operator-(const QRectF &, const QMarginsF &)

/*
Returns the lhs rectangle shrunken by the rhs margins.

This function was introduced in  Qt 5.3.
*/
func Operator-_10(lhs QRectF_ITF, rhs QMarginsF_ITF) *QRectF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QRectF_PTR() != nil {
        convArg0 = lhs.QRectF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMarginsF_PTR() != nil {
        convArg1 = rhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK6QRectFRK9QMarginsF", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQRectFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQRectF)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:172
// index:11
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator-(const QPoint &, const QPoint &)

/*
Returns a QPoint object that is formed by subtracting p2 from p1; each component is subtracted separately.

See also QPoint::operator-=().
*/
func Operator-_11(p1 QPoint_ITF, p2 QPoint_ITF) *QPoint/*123*/ {
    var convArg0 unsafe.Pointer
    if p1 != nil && p1.QPoint_PTR() != nil {
        convArg0 = p1.QPoint_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if p2 != nil && p2.QPoint_PTR() != nil {
        convArg1 = p2.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK6QPointS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:178
// index:12
// Invalid inline Visibility=Default Availability=Available
// [8] const QSize operator-(const QSize &, const QSize &)

/*
Returns s2 subtracted from s1; each component is subtracted separately.
*/
func Operator-_12(s1 QSize_ITF, s2 QSize_ITF) *QSize/*123*/ {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QSize_PTR() != nil {
        convArg0 = s1.QSize_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QSize_PTR() != nil {
        convArg1 = s2.QSize_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK5QSizeS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSize)
    return rv2
}

// /usr/include/qt/QtCore/qrect.h:476
// index:13
// Invalid inline Visibility=Default Availability=Available
// [16] QRect operator-(const QRect &, const QMargins &)

/*
Returns the lhs rectangle shrunken by the rhs margins.

This function was introduced in  Qt 5.3.
*/
func Operator-_13(lhs QRect_ITF, rhs QMargins_ITF) *QRect/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QRect_PTR() != nil {
        convArg0 = lhs.QRect_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMargins_PTR() != nil {
        convArg1 = rhs.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZmiRK5QRectRK8QMargins", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQRectFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQRect)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:372
// index:0
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QMarginsF &, const QMarginsF &)

/*

*/
func Operator==(lhs QMarginsF_ITF, rhs QMarginsF_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QMarginsF_PTR() != nil {
        convArg1 = rhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK9QMarginsFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qvariant.h:567
// index:1
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QVariant &, const QVariantComparisonHelper &)

/*

*/
func Operator==_1(v1 QVariant_ITF, v2 QVariantComparisonHelper_ITF) bool {
    var convArg0 unsafe.Pointer
    if v1 != nil && v1.QVariant_PTR() != nil {
        convArg0 = v1.QVariant_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if v2 != nil && v2.QVariantComparisonHelper_PTR() != nil {
        convArg1 = v2.QVariantComparisonHelper_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK8QVariantRK24QVariantComparisonHelper", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qmargins.h:135
// index:2
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QMargins &, const QMargins &)

/*

*/
func Operator==_2(m1 QMargins_ITF, m2 QMargins_ITF) bool {
    var convArg0 unsafe.Pointer
    if m1 != nil && m1.QMargins_PTR() != nil {
        convArg0 = m1.QMargins_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if m2 != nil && m2.QMargins_PTR() != nil {
        convArg1 = m2.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK8QMarginsS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1614
// index:3
// Invalid Visibility=Default Availability=Available
// [1] bool operator==(const QString &, const QStringRef &)

/*

*/
func Operator==_3(lhs string, rhs QStringRef_ITF) bool {
    var tmpArg0 = NewQString_5(lhs)
    var convArg0 = tmpArg0.GetCthis()
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringRef_PTR() != nil {
        convArg1 = rhs.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK7QStringRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1182
// index:4
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QString &, QString::Null)

/*

*/
func Operator==_4(s string, arg1 int) bool {
    var tmpArg0 = NewQString_5(s)
    var convArg0 = tmpArg0.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK7QStringNS_4NullE", qtrt.FFI_TYPE_POINTER, convArg0, arg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1672
// index:5
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QString &, QChar)

/*

*/
func Operator==_5(lhs string, rhs QChar_ITF/*123*/) bool {
    var tmpArg0 = NewQString_5(lhs)
    var convArg0 = tmpArg0.GetCthis()
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK7QString5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qpoint.h:348
// index:6
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QPointF &, const QPointF &)

/*

*/
func Operator==_6(p1 QPointF_ITF, p2 QPointF_ITF) bool {
    var convArg0 unsafe.Pointer
    if p1 != nil && p1.QPointF_PTR() != nil {
        convArg0 = p1.QPointF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if p2 != nil && p2.QPointF_PTR() != nil {
        convArg1 = p2.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK7QPointFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qsize.h:339
// index:7
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QSizeF &, const QSizeF &)

/*

*/
func Operator==_7(s1 QSizeF_ITF, s2 QSizeF_ITF) bool {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QSizeF_PTR() != nil {
        convArg0 = s1.QSizeF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QSizeF_PTR() != nil {
        convArg1 = s2.QSizeF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK6QSizeFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qrect.h:859
// index:8
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QRectF &, const QRectF &)

/*

*/
func Operator==_8(r1 QRectF_ITF, r2 QRectF_ITF) bool {
    var convArg0 unsafe.Pointer
    if r1 != nil && r1.QRectF_PTR() != nil {
        convArg0 = r1.QRectF_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if r2 != nil && r2.QRectF_PTR() != nil {
        convArg1 = r2.QRectF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK6QRectFS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qpoint.h:163
// index:9
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QPoint &, const QPoint &)

/*

*/
func Operator==_9(p1 QPoint_ITF, p2 QPoint_ITF) bool {
    var convArg0 unsafe.Pointer
    if p1 != nil && p1.QPoint_PTR() != nil {
        convArg0 = p1.QPoint_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if p2 != nil && p2.QPoint_PTR() != nil {
        convArg1 = p2.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK6QPointS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qsize.h:169
// index:10
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QSize &, const QSize &)

/*

*/
func Operator==_10(s1 QSize_ITF, s2 QSize_ITF) bool {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QSize_PTR() != nil {
        convArg0 = s1.QSize_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QSize_PTR() != nil {
        convArg1 = s2.QSize_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK5QSizeS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qrect.h:454
// index:11
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QRect &, const QRect &)

/*

*/
func Operator==_11(r1 QRect_ITF, r2 QRect_ITF) bool {
    var convArg0 unsafe.Pointer
    if r1 != nil && r1.QRect_PTR() != nil {
        convArg0 = r1.QRect_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if r2 != nil && r2.QRect_PTR() != nil {
        convArg1 = r2.QRect_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK5QRectS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qhash.h:141
// index:12
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QHashDummyValue &, const QHashDummyValue &)

/*

*/
func Operator==_12(arg0 QHashDummyValue_ITF, arg1 QHashDummyValue_ITF) bool {
    var convArg0 unsafe.Pointer
    if arg0 != nil && arg0.QHashDummyValue_PTR() != nil {
        convArg0 = arg0.QHashDummyValue_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if arg1 != nil && arg1.QHashDummyValue_PTR() != nil {
        convArg1 = arg1.QHashDummyValue_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK15QHashDummyValueS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qversionnumber.h:314
// index:13
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QVersionNumber &, const QVersionNumber &)

/*

*/
func Operator==_13(lhs QVersionNumber_ITF, rhs QVersionNumber_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QVersionNumber_PTR() != nil {
        convArg0 = lhs.QVersionNumber_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QVersionNumber_PTR() != nil {
        convArg1 = rhs.QVersionNumber_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK14QVersionNumberS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstorageinfo.h:103
// index:14
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QStorageInfo &, const QStorageInfo &)

/*

*/
func Operator==_14(first QStorageInfo_ITF, second QStorageInfo_ITF) bool {
    var convArg0 unsafe.Pointer
    if first != nil && first.QStorageInfo_PTR() != nil {
        convArg0 = first.QStorageInfo_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if second != nil && second.QStorageInfo_PTR() != nil {
        convArg1 = second.QStorageInfo_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK12QStorageInfoS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qmetaobject.h:201
// index:15
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QMetaMethod &, const QMetaMethod &)

/*

*/
func Operator==_15(m1 QMetaMethod_ITF, m2 QMetaMethod_ITF) bool {
    var convArg0 unsafe.Pointer
    if m1 != nil && m1.QMetaMethod_PTR() != nil {
        convArg0 = m1.QMetaMethod_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if m2 != nil && m2.QMetaMethod_PTR() != nil {
        convArg1 = m2.QMetaMethod_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK11QMetaMethodS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1602
// index:16
// Invalid Visibility=Default Availability=Available
// [1] bool operator==(const QStringRef &, const QStringRef &)

/*

*/
func Operator==_16(s1 QStringRef_ITF, s2 QStringRef_ITF) bool {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QStringRef_PTR() != nil {
        convArg0 = s1.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QStringRef_PTR() != nil {
        convArg1 = s2.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK10QStringRefS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1621
// index:17
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QStringRef &, const QString &)

/*

*/
func Operator==_17(lhs QStringRef_ITF, rhs string) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringRef_PTR() != nil {
        convArg0 = lhs.QStringRef_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(rhs)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK10QStringRefRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1757
// index:18
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QStringRef &, const QByteArray &)

/*

*/
func Operator==_18(lhs QStringRef_ITF, rhs QByteArray_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringRef_PTR() != nil {
        convArg0 = lhs.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QByteArray_PTR() != nil {
        convArg1 = rhs.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK10QStringRefRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1691
// index:19
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QStringRef &, QChar)

/*

*/
func Operator==_19(lhs QStringRef_ITF, rhs QChar_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringRef_PTR() != nil {
        convArg0 = lhs.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK10QStringRef5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1653
// index:20
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QStringRef &, QLatin1String)

/*

*/
func Operator==_20(lhs QStringRef_ITF, rhs QLatin1String_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringRef_PTR() != nil {
        convArg0 = lhs.QStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QLatin1String_PTR() != nil {
        convArg1 = rhs.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK10QStringRef13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qbytearray.h:603
// index:21
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QByteArray &, const QByteArray &)

/*

*/
func Operator==_21(a1 QByteArray_ITF, a2 QByteArray_ITF) bool {
    var convArg0 unsafe.Pointer
    if a1 != nil && a1.QByteArray_PTR() != nil {
        convArg0 = a1.QByteArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if a2 != nil && a2.QByteArray_PTR() != nil {
        convArg1 = a2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK10QByteArrayS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1764
// index:22
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QByteArray &, const QStringRef &)

/*

*/
func Operator==_22(lhs QByteArray_ITF, rhs QStringRef_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QByteArray_PTR() != nil {
        convArg0 = lhs.QByteArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringRef_PTR() != nil {
        convArg1 = rhs.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK10QByteArrayRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qbytearray.h:605
// index:23
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const QByteArray &, const char *)

/*

*/
func Operator==_23(a1 QByteArray_ITF, a2 string) bool {
    var convArg0 unsafe.Pointer
    if a1 != nil && a1.QByteArray_PTR() != nil {
        convArg0 = a1.QByteArray_PTR().GetCthis()
    }
    var convArg1 = qtrt.CString(a2)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_ZeqRK10QByteArrayPKc", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1234
// index:24
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const char *, const QString &)

/*

*/
func Operator==_24(s1 string, s2 string) bool {
    var convArg0 = qtrt.CString(s1)
    defer qtrt.FreeMem(convArg0)
    var tmpArg1 = NewQString_5(s2)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZeqPKcRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1785
// index:25
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const char *, const QStringRef &)

/*

*/
func Operator==_25(s1 string, s2 QStringRef_ITF) bool {
    var convArg0 = qtrt.CString(s1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QStringRef_PTR() != nil {
        convArg1 = s2.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqPKcRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qbytearray.h:607
// index:26
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const char *, const QByteArray &)

/*

*/
func Operator==_26(a1 string, a2 QByteArray_ITF) bool {
    var convArg0 = qtrt.CString(a1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if a2 != nil && a2.QByteArray_PTR() != nil {
        convArg1 = a2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqPKcRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1247
// index:27
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(const char *, QLatin1String)

/*

*/
func Operator==_27(s1 string, s2 QLatin1String_ITF/*123*/) bool {
    var convArg0 = qtrt.CString(s1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QLatin1String_PTR() != nil {
        convArg1 = s2.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqPKc13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1178
// index:28
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QString::Null, QString::Null)

/*

*/
func Operator==_28(arg0 int, arg1 int) bool {
  rv, err := qtrt.InvokeQtFunc6("_ZeqN7QString4NullES0_", qtrt.FFI_TYPE_POINTER, arg0, arg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1180
// index:29
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QString::Null, const QString &)

/*

*/
func Operator==_29(arg0 int, s string) bool {
    var tmpArg1 = NewQString_5(s)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZeqN7QString4NullERKS_", qtrt.FFI_TYPE_POINTER, arg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qchar.h:591
// index:30
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(std::nullptr_t, QChar)

/*

*/
func Operator==_30(arg0 int, rhs QChar_ITF/*123*/) bool {
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeqDn5QChar", qtrt.FFI_TYPE_POINTER, arg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qchar.h:580
// index:31
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QChar, QChar)

/*

*/
func Operator==_31(c1 QChar_ITF/*123*/, c2 QChar_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if c1 != nil && c1.QChar_PTR() != nil {
        convArg0 = c1.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if c2 != nil && c2.QChar_PTR() != nil {
        convArg1 = c2.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq5QCharS_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1661
// index:32
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QChar, const QString &)

/*

*/
func Operator==_32(lhs QChar_ITF/*123*/, rhs string) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(rhs)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Zeq5QCharRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1680
// index:33
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QChar, const QStringRef &)

/*

*/
func Operator==_33(lhs QChar_ITF/*123*/, rhs QStringRef_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringRef_PTR() != nil {
        convArg1 = rhs.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq5QCharRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qchar.h:589
// index:34
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QChar, std::nullptr_t)

/*

*/
func Operator==_34(lhs QChar_ITF/*123*/, arg1 int) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq5QCharDn", qtrt.FFI_TYPE_POINTER, convArg0, arg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1699
// index:35
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QChar, QLatin1String)

/*

*/
func Operator==_35(lhs QChar_ITF/*123*/, rhs QLatin1String_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QLatin1String_PTR() != nil {
        convArg1 = rhs.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq5QChar13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1733
// index:36
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QChar, QStringView)

/*

*/
func Operator==_36(lhs QChar_ITF/*123*/, rhs QStringView_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QChar_PTR() != nil {
        convArg0 = lhs.QChar_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringView_PTR() != nil {
        convArg1 = rhs.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq5QChar11QStringView", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1190
// index:37
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QLatin1String, QLatin1String)

/*

*/
func Operator==_37(s1 QLatin1String_ITF/*123*/, s2 QLatin1String_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if s1 != nil && s1.QLatin1String_PTR() != nil {
        convArg0 = s1.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if s2 != nil && s2.QLatin1String_PTR() != nil {
        convArg1 = s2.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq13QLatin1StringS_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1646
// index:38
// Invalid Visibility=Default Availability=Available
// [1] bool operator==(QLatin1String, const QStringRef &)

/*

*/
func Operator==_38(lhs QLatin1String_ITF/*123*/, rhs QStringRef_ITF) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QLatin1String_PTR() != nil {
        convArg0 = lhs.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringRef_PTR() != nil {
        convArg1 = rhs.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq13QLatin1StringRK10QStringRef", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1710
// index:39
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QLatin1String, QChar)

/*

*/
func Operator==_39(lhs QLatin1String_ITF/*123*/, rhs QChar_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QLatin1String_PTR() != nil {
        convArg0 = lhs.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq13QLatin1String5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1748
// index:40
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QLatin1String, QStringView)

/*

*/
func Operator==_40(lhs QLatin1String_ITF/*123*/, rhs QStringView_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QLatin1String_PTR() != nil {
        convArg0 = lhs.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringView_PTR() != nil {
        convArg1 = rhs.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq13QLatin1String11QStringView", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1718
// index:41
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QStringView, QStringView)

/*

*/
func Operator==_41(lhs QStringView_ITF/*123*/, rhs QStringView_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringView_PTR() != nil {
        convArg0 = lhs.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringView_PTR() != nil {
        convArg1 = rhs.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq11QStringViewS_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1726
// index:42
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QStringView, QChar)

/*

*/
func Operator==_42(lhs QStringView_ITF/*123*/, rhs QChar_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringView_PTR() != nil {
        convArg0 = lhs.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QChar_PTR() != nil {
        convArg1 = rhs.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq11QStringView5QChar", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstring.h:1741
// index:43
// Invalid inline Visibility=Default Availability=Available
// [1] bool operator==(QStringView, QLatin1String)

/*

*/
func Operator==_43(lhs QStringView_ITF/*123*/, rhs QLatin1String_ITF/*123*/) bool {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringView_PTR() != nil {
        convArg0 = lhs.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QLatin1String_PTR() != nil {
        convArg1 = rhs.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Zeq11QStringView13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qbitarray.h:117
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QBitArray operator^(const QBitArray &, const QBitArray &)

/*

*/
func Operator^(arg0 QBitArray_ITF, arg1 QBitArray_ITF) *QBitArray/*123*/ {
    var convArg0 unsafe.Pointer
    if arg0 != nil && arg0.QBitArray_PTR() != nil {
        convArg0 = arg0.QBitArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if arg1 != nil && arg1.QBitArray_PTR() != nil {
        convArg1 = arg1.QBitArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZeoRK9QBitArrayS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQBitArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQBitArray)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:427
// index:0
// Invalid inline Visibility=Default Availability=Available
// [32] QMarginsF operator/(const QMarginsF &, qreal)

/*

*/
func Operator/(lhs QMarginsF_ITF, divisor float64) *QMarginsF/*123*/ {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QMarginsF_PTR() != nil {
        convArg0 = lhs.QMarginsF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZdvRK9QMarginsFd", qtrt.FFI_TYPE_POINTER, convArg0, divisor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMarginsF)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:207
// index:1
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator/(const QMargins &, int)

/*

*/
func Operator/_1(margins QMargins_ITF, divisor int) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg0 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZdvRK8QMarginsi", qtrt.FFI_TYPE_POINTER, convArg0, divisor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qmargins.h:213
// index:2
// Invalid inline Visibility=Default Availability=Available
// [16] QMargins operator/(const QMargins &, qreal)

/*

*/
func Operator/_2(margins QMargins_ITF, divisor float64) *QMargins/*123*/ {
    var convArg0 unsafe.Pointer
    if margins != nil && margins.QMargins_PTR() != nil {
        convArg0 = margins.QMargins_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZdvRK8QMarginsd", qtrt.FFI_TYPE_POINTER, convArg0, divisor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQMarginsFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQMargins)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:395
// index:3
// Invalid inline Visibility=Default Availability=Available
// [16] const QPointF operator/(const QPointF &, qreal)

/*

*/
func Operator/_3(p QPointF_ITF, divisor float64) *QPointF/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPointF_PTR() != nil {
        convArg0 = p.QPointF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZdvRK7QPointFd", qtrt.FFI_TYPE_POINTER, convArg0, divisor)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPointF)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:364
// index:4
// Invalid inline Visibility=Default Availability=Available
// [16] const QSizeF operator/(const QSizeF &, qreal)

/*

*/
func Operator/_4(s QSizeF_ITF, c float64) *QSizeF/*123*/ {
    var convArg0 unsafe.Pointer
    if s != nil && s.QSizeF_PTR() != nil {
        convArg0 = s.QSizeF_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZdvRK6QSizeFd", qtrt.FFI_TYPE_POINTER, convArg0, c)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSizeF)
    return rv2
}

// /usr/include/qt/QtCore/qpoint.h:206
// index:5
// Invalid inline Visibility=Default Availability=Available
// [8] const QPoint operator/(const QPoint &, qreal)

/*

*/
func Operator/_5(p QPoint_ITF, c float64) *QPoint/*123*/ {
    var convArg0 unsafe.Pointer
    if p != nil && p.QPoint_PTR() != nil {
        convArg0 = p.QPoint_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZdvRK6QPointd", qtrt.FFI_TYPE_POINTER, convArg0, c)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQPointFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQPoint)
    return rv2
}

// /usr/include/qt/QtCore/qsize.h:194
// index:6
// Invalid inline Visibility=Default Availability=Available
// [8] const QSize operator/(const QSize &, qreal)

/*

*/
func Operator/_6(s QSize_ITF, c float64) *QSize/*123*/ {
    var convArg0 unsafe.Pointer
    if s != nil && s.QSize_PTR() != nil {
        convArg0 = s.QSize_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZdvRK5QSized", qtrt.FFI_TYPE_POINTER, convArg0, c)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQSizeFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQSize)
    return rv2
}

// /usr/include/qt/QtCore/qbitarray.h:115
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QBitArray operator&(const QBitArray &, const QBitArray &)

/*

*/
func Operator&(arg0 QBitArray_ITF, arg1 QBitArray_ITF) *QBitArray/*123*/ {
    var convArg0 unsafe.Pointer
    if arg0 != nil && arg0.QBitArray_PTR() != nil {
        convArg0 = arg0.QBitArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if arg1 != nil && arg1.QBitArray_PTR() != nil {
        convArg1 = arg1.QBitArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZanRK9QBitArrayS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQBitArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQBitArray)
    return rv2
}

// /usr/include/qt/QtCore/qstringalgorithms.h:74
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool endsWith(QLatin1String, QLatin1String, Qt::CaseSensitivity)

/*

*/
func EndsWith(haystack QLatin1String_ITF/*123*/, needle QLatin1String_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if haystack != nil && haystack.QLatin1String_PTR() != nil {
        convArg0 = haystack.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if needle != nil && needle.QLatin1String_PTR() != nil {
        convArg1 = needle.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate8endsWithE13QLatin1StringS0_N2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringalgorithms.h:73
// index:1
// Invalid Visibility=Default Availability=Available
// [1] bool endsWith(QLatin1String, QStringView, Qt::CaseSensitivity)

/*

*/
func EndsWith_1(haystack QLatin1String_ITF/*123*/, needle QStringView_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if haystack != nil && haystack.QLatin1String_PTR() != nil {
        convArg0 = haystack.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if needle != nil && needle.QStringView_PTR() != nil {
        convArg1 = needle.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate8endsWithE13QLatin1String11QStringViewN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringalgorithms.h:71
// index:2
// Invalid Visibility=Default Availability=Available
// [1] bool endsWith(QStringView, QStringView, Qt::CaseSensitivity)

/*

*/
func EndsWith_2(haystack QStringView_ITF/*123*/, needle QStringView_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if haystack != nil && haystack.QStringView_PTR() != nil {
        convArg0 = haystack.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if needle != nil && needle.QStringView_PTR() != nil {
        convArg1 = needle.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate8endsWithE11QStringViewS0_N2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringalgorithms.h:72
// index:3
// Invalid Visibility=Default Availability=Available
// [1] bool endsWith(QStringView, QLatin1String, Qt::CaseSensitivity)

/*

*/
func EndsWith_3(haystack QStringView_ITF/*123*/, needle QLatin1String_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if haystack != nil && haystack.QStringView_PTR() != nil {
        convArg0 = haystack.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if needle != nil && needle.QLatin1String_PTR() != nil {
        convArg1 = needle.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate8endsWithE11QStringView13QLatin1StringN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringalgorithms.h:77
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QLatin1String trimmed(QLatin1String)

/*

*/
func Trimmed(s QLatin1String_ITF/*123*/) *QLatin1String/*123*/ {
    var convArg0 unsafe.Pointer
    if s != nil && s.QLatin1String_PTR() != nil {
        convArg0 = s.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate7trimmedE13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQLatin1StringFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQLatin1String)
    return rv2
}

// /usr/include/qt/QtCore/qstringalgorithms.h:76
// index:1
// Invalid Visibility=Default Availability=Available
// [16] QStringView trimmed(QStringView)

/*

*/
func Trimmed_1(s QStringView_ITF/*123*/) *QStringView/*123*/ {
    var convArg0 unsafe.Pointer
    if s != nil && s.QStringView_PTR() != nil {
        convArg0 = s.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate7trimmedE11QStringView", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringViewFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQStringView)
    return rv2
}

// /usr/include/qt/QtCore/qstringlist.h:170
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void QStringList_replaceInStrings(QStringList *, const QString &, const QString &, Qt::CaseSensitivity)

/*

*/
func QStringList_replaceInStrings(that QStringList_ITF/*777 QStringList **/, before string, after string, cs int)  {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(before)
    var convArg1 = tmpArg1.GetCthis()
    var tmpArg2 = NewQString_5(after)
    var convArg2 = tmpArg2.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate28QStringList_replaceInStringsEP11QStringListRK7QStringS4_N2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, convArg2, cs)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qstringlist.h:174
// index:1
// Invalid Visibility=Default Availability=Available
// [-2] void QStringList_replaceInStrings(QStringList *, const QRegExp &, const QString &)

/*

*/
func QStringList_replaceInStrings_1(that QStringList_ITF/*777 QStringList **/, rx QRegExp_ITF, after string)  {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rx != nil && rx.QRegExp_PTR() != nil {
        convArg1 = rx.QRegExp_PTR().GetCthis()
    }
    var tmpArg2 = NewQString_5(after)
    var convArg2 = tmpArg2.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate28QStringList_replaceInStringsEP11QStringListRK7QRegExpRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, convArg2)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qstringlist.h:184
// index:2
// Invalid Visibility=Default Availability=Available
// [-2] void QStringList_replaceInStrings(QStringList *, const QRegularExpression &, const QString &)

/*

*/
func QStringList_replaceInStrings_2(that QStringList_ITF/*777 QStringList **/, rx QRegularExpression_ITF, after string)  {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rx != nil && rx.QRegularExpression_PTR() != nil {
        convArg1 = rx.QRegularExpression_PTR().GetCthis()
    }
    var tmpArg2 = NewQString_5(after)
    var convArg2 = tmpArg2.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate28QStringList_replaceInStringsEP11QStringListRK18QRegularExpressionRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, convArg2)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qstringlist.h:162
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int QStringList_removeDuplicates(QStringList *)

/*

*/
func QStringList_removeDuplicates(that QStringList_ITF/*777 QStringList **/) int {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate28QStringList_removeDuplicatesEP11QStringList", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qstringlist.h:177
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int QStringList_lastIndexOf(const QStringList *, const QRegExp &, int)

/*

*/
func QStringList_lastIndexOf(that QStringList_ITF/*777 const QStringList **/, rx QRegExp_ITF, from int) int {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rx != nil && rx.QRegExp_PTR() != nil {
        convArg1 = rx.QRegExp_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate23QStringList_lastIndexOfEPK11QStringListRK7QRegExpi", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, from)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qstringlist.h:187
// index:1
// Invalid Visibility=Default Availability=Available
// [4] int QStringList_lastIndexOf(const QStringList *, const QRegularExpression &, int)

/*

*/
func QStringList_lastIndexOf_1(that QStringList_ITF/*777 const QStringList **/, re QRegularExpression_ITF, from int) int {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if re != nil && re.QRegularExpression_PTR() != nil {
        convArg1 = re.QRegularExpression_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate23QStringList_lastIndexOfEPK11QStringListRK18QRegularExpressioni", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, from)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qstringlist.h:179
// index:2
// Invalid Visibility=Default Availability=Available
// [4] int QStringList_lastIndexOf(const QStringList *, QRegExp &, int)

/*

*/
func QStringList_lastIndexOf_2(that QStringList_ITF/*777 const QStringList **/, rx QRegExp_ITF, from int) int {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rx != nil && rx.QRegExp_PTR() != nil {
        convArg1 = rx.QRegExp_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate23QStringList_lastIndexOfEPK11QStringListR7QRegExpi", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, from)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qobject.h:623
// index:0
// Invalid inline Visibility=Default Availability=Available
// [16] QObject & deref_for_methodcall(QObject &)

/*

*/
func Deref_for_methodcall(o QObject_ITF) *QObject {
    var convArg0 unsafe.Pointer
    if o != nil && o.QObject_PTR() != nil {
        convArg0 = o.QObject_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate20deref_for_methodcallER7QObject", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQObject)
    return rv2
}

// /usr/include/qt/QtCore/qobject.h:624
// index:1
// Invalid inline Visibility=Default Availability=Available
// [16] QObject & deref_for_methodcall(QObject *)

/*

*/
func Deref_for_methodcall_1(o QObject_ITF/*777 QObject **/) *QObject {
    var convArg0 unsafe.Pointer
    if o != nil && o.QObject_PTR() != nil {
        convArg0 = o.QObject_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate20deref_for_methodcallEP7QObject", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQObject)
    return rv2
}

// /usr/include/qt/QtCore/qstringlist.h:168
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool QStringList_contains(const QStringList *, const QString &, Qt::CaseSensitivity)

/*

*/
func QStringList_contains(that QStringList_ITF/*777 const QStringList **/, str string, cs int) bool {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(str)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate20QStringList_containsEPK11QStringListRK7QStringN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringlist.h:169
// index:1
// Invalid Visibility=Default Availability=Available
// [1] bool QStringList_contains(const QStringList *, QLatin1String, Qt::CaseSensitivity)

/*

*/
func QStringList_contains_1(that QStringList_ITF/*777 const QStringList **/, str QLatin1String_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if str != nil && str.QLatin1String_PTR() != nil {
        convArg1 = str.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate20QStringList_containsEPK11QStringList13QLatin1StringN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringlist.h:176
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int QStringList_indexOf(const QStringList *, const QRegExp &, int)

/*

*/
func QStringList_indexOf(that QStringList_ITF/*777 const QStringList **/, rx QRegExp_ITF, from int) int {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rx != nil && rx.QRegExp_PTR() != nil {
        convArg1 = rx.QRegExp_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate19QStringList_indexOfEPK11QStringListRK7QRegExpi", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, from)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qstringlist.h:186
// index:1
// Invalid Visibility=Default Availability=Available
// [4] int QStringList_indexOf(const QStringList *, const QRegularExpression &, int)

/*

*/
func QStringList_indexOf_1(that QStringList_ITF/*777 const QStringList **/, re QRegularExpression_ITF, from int) int {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if re != nil && re.QRegularExpression_PTR() != nil {
        convArg1 = re.QRegularExpression_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate19QStringList_indexOfEPK11QStringListRK18QRegularExpressioni", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, from)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qstringlist.h:178
// index:2
// Invalid Visibility=Default Availability=Available
// [4] int QStringList_indexOf(const QStringList *, QRegExp &, int)

/*

*/
func QStringList_indexOf_2(that QStringList_ITF/*777 const QStringList **/, rx QRegExp_ITF, from int) int {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rx != nil && rx.QRegExp_PTR() != nil {
        convArg1 = rx.QRegExp_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate19QStringList_indexOfEPK11QStringListR7QRegExpi", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, from)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qbytearraylist.h:57
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QByteArray QByteArrayList_join(const QByteArrayList *, const char *, int)

/*

*/
func QByteArrayList_join(that QByteArrayList_ITF/*777 const QByteArrayList **/, separator string, separatorLength int) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if that != nil && that.QByteArrayList_PTR() != nil {
        convArg0 = that.QByteArrayList_PTR().GetCthis()
    }
    var convArg1 = qtrt.CString(separator)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate19QByteArrayList_joinEPK5QListI10QByteArrayEPKci", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, separatorLength)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qstringalgorithms.h:81
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QByteArray convertToLocal8Bit(QStringView)

/*

*/
func ConvertToLocal8Bit(str QStringView_ITF/*123*/) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if str != nil && str.QStringView_PTR() != nil {
        convArg0 = str.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate18convertToLocal8BitE11QStringView", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qstringlist.h:165
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QStringList QStringList_filter(const QStringList *, const QString &, Qt::CaseSensitivity)

/*

*/
func QStringList_filter(that QStringList_ITF/*777 const QStringList **/, str string, cs int) *QStringList/*123*/ {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(str)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate18QStringList_filterEPK11QStringListRK7QStringN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringListFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQStringList)
    return rv2
}

// /usr/include/qt/QtCore/qstringlist.h:175
// index:1
// Invalid Visibility=Default Availability=Available
// [8] QStringList QStringList_filter(const QStringList *, const QRegExp &)

/*

*/
func QStringList_filter_1(that QStringList_ITF/*777 const QStringList **/, re QRegExp_ITF) *QStringList/*123*/ {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if re != nil && re.QRegExp_PTR() != nil {
        convArg1 = re.QRegExp_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate18QStringList_filterEPK11QStringListRK7QRegExp", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringListFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQStringList)
    return rv2
}

// /usr/include/qt/QtCore/qstringlist.h:185
// index:2
// Invalid Visibility=Default Availability=Available
// [8] QStringList QStringList_filter(const QStringList *, const QRegularExpression &)

/*

*/
func QStringList_filter_2(that QStringList_ITF/*777 const QStringList **/, re QRegularExpression_ITF) *QStringList/*123*/ {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if re != nil && re.QRegularExpression_PTR() != nil {
        convArg1 = re.QRegularExpression_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate18QStringList_filterEPK11QStringListRK18QRegularExpression", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringListFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQStringList)
    return rv2
}

// /usr/include/qt/QtCore/qstringlist.h:161
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void QStringList_sort(QStringList *, Qt::CaseSensitivity)

/*

*/
func QStringList_sort(that QStringList_ITF/*777 QStringList **/, cs int)  {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate16QStringList_sortEP11QStringListN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, cs)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qstringlist.h:164
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QString QStringList_join(const QStringList &, QLatin1String)

/*

*/
func QStringList_join(list QStringList_ITF, sep QLatin1String_ITF/*123*/) string {
    var convArg0 unsafe.Pointer
    if list != nil && list.QStringList_PTR() != nil {
        convArg0 = list.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if sep != nil && sep.QLatin1String_PTR() != nil {
        convArg1 = sep.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate16QStringList_joinERK11QStringList13QLatin1String", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstringlist.h:163
// index:1
// Invalid Visibility=Default Availability=Available
// [8] QString QStringList_join(const QStringList *, const QChar *, int)

/*

*/
func QStringList_join_1(that QStringList_ITF/*777 const QStringList **/, sep QChar_ITF/*777 const QChar **/, seplen int) string {
    var convArg0 unsafe.Pointer
    if that != nil && that.QStringList_PTR() != nil {
        convArg0 = that.QStringList_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if sep != nil && sep.QChar_PTR() != nil {
        convArg1 = sep.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate16QStringList_joinEPK11QStringListPK5QChari", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, seplen)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qstringalgorithms.h:79
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QByteArray convertToLatin1(QStringView)

/*

*/
func ConvertToLatin1(str QStringView_ITF/*123*/) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if str != nil && str.QStringView_PTR() != nil {
        convArg0 = str.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate15convertToLatin1E11QStringView", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qstringalgorithms.h:63
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int compareStrings(QLatin1String, QLatin1String, Qt::CaseSensitivity)

/*

*/
func CompareStrings(lhs QLatin1String_ITF/*123*/, rhs QLatin1String_ITF/*123*/, cs int) int {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QLatin1String_PTR() != nil {
        convArg0 = lhs.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QLatin1String_PTR() != nil {
        convArg1 = rhs.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate14compareStringsE13QLatin1StringS0_N2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qstringalgorithms.h:62
// index:1
// Invalid Visibility=Default Availability=Available
// [4] int compareStrings(QLatin1String, QStringView, Qt::CaseSensitivity)

/*

*/
func CompareStrings_1(lhs QLatin1String_ITF/*123*/, rhs QStringView_ITF/*123*/, cs int) int {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QLatin1String_PTR() != nil {
        convArg0 = lhs.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringView_PTR() != nil {
        convArg1 = rhs.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate14compareStringsE13QLatin1String11QStringViewN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qstringalgorithms.h:60
// index:2
// Invalid Visibility=Default Availability=Available
// [4] int compareStrings(QStringView, QStringView, Qt::CaseSensitivity)

/*

*/
func CompareStrings_2(lhs QStringView_ITF/*123*/, rhs QStringView_ITF/*123*/, cs int) int {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringView_PTR() != nil {
        convArg0 = lhs.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QStringView_PTR() != nil {
        convArg1 = rhs.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate14compareStringsE11QStringViewS0_N2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qstringalgorithms.h:61
// index:3
// Invalid Visibility=Default Availability=Available
// [4] int compareStrings(QStringView, QLatin1String, Qt::CaseSensitivity)

/*

*/
func CompareStrings_3(lhs QStringView_ITF/*123*/, rhs QLatin1String_ITF/*123*/, cs int) int {
    var convArg0 unsafe.Pointer
    if lhs != nil && lhs.QStringView_PTR() != nil {
        convArg0 = lhs.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if rhs != nil && rhs.QLatin1String_PTR() != nil {
        convArg1 = rhs.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate14compareStringsE11QStringView13QLatin1StringN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qmetatype.h:1594
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool isBuiltinType(const QByteArray &)

/*

*/
func IsBuiltinType(type_ QByteArray_ITF) bool {
    var convArg0 unsafe.Pointer
    if type_ != nil && type_.QByteArray_PTR() != nil {
        convArg0 = type_.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate13isBuiltinTypeERK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringalgorithms.h:80
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QByteArray convertToUtf8(QStringView)

/*

*/
func ConvertToUtf8(str QStringView_ITF/*123*/) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if str != nil && str.QStringView_PTR() != nil {
        convArg0 = str.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate13convertToUtf8E11QStringView", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qstringalgorithms.h:69
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool startsWith(QLatin1String, QLatin1String, Qt::CaseSensitivity)

/*

*/
func StartsWith(haystack QLatin1String_ITF/*123*/, needle QLatin1String_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if haystack != nil && haystack.QLatin1String_PTR() != nil {
        convArg0 = haystack.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if needle != nil && needle.QLatin1String_PTR() != nil {
        convArg1 = needle.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate10startsWithE13QLatin1StringS0_N2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringalgorithms.h:68
// index:1
// Invalid Visibility=Default Availability=Available
// [1] bool startsWith(QLatin1String, QStringView, Qt::CaseSensitivity)

/*

*/
func StartsWith_1(haystack QLatin1String_ITF/*123*/, needle QStringView_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if haystack != nil && haystack.QLatin1String_PTR() != nil {
        convArg0 = haystack.QLatin1String_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if needle != nil && needle.QStringView_PTR() != nil {
        convArg1 = needle.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate10startsWithE13QLatin1String11QStringViewN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringalgorithms.h:66
// index:2
// Invalid Visibility=Default Availability=Available
// [1] bool startsWith(QStringView, QStringView, Qt::CaseSensitivity)

/*

*/
func StartsWith_2(haystack QStringView_ITF/*123*/, needle QStringView_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if haystack != nil && haystack.QStringView_PTR() != nil {
        convArg0 = haystack.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if needle != nil && needle.QStringView_PTR() != nil {
        convArg1 = needle.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate10startsWithE11QStringViewS0_N2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qstringalgorithms.h:67
// index:3
// Invalid Visibility=Default Availability=Available
// [1] bool startsWith(QStringView, QLatin1String, Qt::CaseSensitivity)

/*

*/
func StartsWith_3(haystack QStringView_ITF/*123*/, needle QLatin1String_ITF/*123*/, cs int) bool {
    var convArg0 unsafe.Pointer
    if haystack != nil && haystack.QStringView_PTR() != nil {
        convArg0 = haystack.QStringView_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if needle != nil && needle.QLatin1String_PTR() != nil {
        convArg1 = needle.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZN9QtPrivate10startsWithE11QStringView13QLatin1StringN2Qt15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, cs)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qnamespace.h:1754
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TimerType)

/*

*/
func Qt_getEnumMetaObject(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_9TimerTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1725
// index:1
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::SortOrder)

/*

*/
func Qt_getEnumMetaObject_1(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_9SortOrderE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1710
// index:2
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::DayOfWeek)

/*

*/
func Qt_getEnumMetaObject_2(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_9DayOfWeekE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1679
// index:3
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ArrowType)

/*

*/
func Qt_getEnumMetaObject_3(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_9ArrowTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1709
// index:4
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TimeSpec)

/*

*/
func Qt_getEnumMetaObject_4(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_8TimeSpecE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1698
// index:5
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TextFlag)

/*

*/
func Qt_getEnumMetaObject_5(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_8TextFlagE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1689
// index:6
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::SizeMode)

/*

*/
func Qt_getEnumMetaObject_6(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_8SizeModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1694
// index:7
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::SizeHint)

/*

*/
func Qt_getEnumMetaObject_7(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_8SizeHintE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1681
// index:8
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::PenStyle)

/*

*/
func Qt_getEnumMetaObject_8(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_8PenStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1686
// index:9
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::MaskMode)

/*

*/
func Qt_getEnumMetaObject_9(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_8MaskModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1685
// index:10
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::FillRule)

/*

*/
func Qt_getEnumMetaObject_10(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_8FillRuleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1691
// index:11
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::Corner)

/*

*/
func Qt_getEnumMetaObject_11(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_6CornerE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1687
// index:12
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::BGMode)

/*

*/
func Qt_getEnumMetaObject_12(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_6BGModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1692
// index:13
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::Edge)

/*

*/
func Qt_getEnumMetaObject_13(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_4EdgeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1690
// index:14
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::Axis)

/*

*/
func Qt_getEnumMetaObject_14(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_4AxisE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1716
// index:15
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::Key)

/*

*/
func Qt_getEnumMetaObject_15(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_3KeyE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1721
// index:16
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ItemSelectionOperation)

/*

*/
func Qt_getEnumMetaObject_16(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_22ItemSelectionOperationE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1734
// index:17
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ApplicationAttribute)

/*

*/
func Qt_getEnumMetaObject_17(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_20ApplicationAttributeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1718
// index:18
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TextInteractionFlag)

/*

*/
func Qt_getEnumMetaObject_18(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_19TextInteractionFlagE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1714
// index:19
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TransformationMode)

/*

*/
func Qt_getEnumMetaObject_19(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_18TransformationModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1744
// index:20
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ScreenOrientation)

/*

*/
func Qt_getEnumMetaObject_20(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_17ScreenOrientationE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1751
// index:21
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::NativeGestureType)

/*

*/
func Qt_getEnumMetaObject_21(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_17NativeGestureTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1720
// index:22
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ItemSelectionMode)

/*

*/
func Qt_getEnumMetaObject_22(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_17ItemSelectionModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1678
// index:23
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ContextMenuPolicy)

/*

*/
func Qt_getEnumMetaObject_23(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_17ContextMenuPolicyE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1759
// index:24
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TabFocusBehavior)

/*

*/
func Qt_getEnumMetaObject_24(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_16TabFocusBehaviorE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1756
// index:25
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::MouseEventSource)

/*

*/
func Qt_getEnumMetaObject_25(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_16MouseEventSourceE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1739
// index:26
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::InputMethodQuery)

/*

*/
func Qt_getEnumMetaObject_26(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_16InputMethodQueryE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1747
// index:27
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ApplicationState)

/*

*/
func Qt_getEnumMetaObject_27(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_16ApplicationStateE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1733
// index:28
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::WidgetAttribute)

/*

*/
func Qt_getEnumMetaObject_28(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15WidgetAttributeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1680
// index:29
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ToolButtonStyle)

/*

*/
func Qt_getEnumMetaObject_29(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15ToolButtonStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1717
// index:30
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ShortcutContext)

/*

*/
func Qt_getEnumMetaObject_30(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15ShortcutContextE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1676
// index:31
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ScrollBarPolicy)

/*

*/
func Qt_getEnumMetaObject_31(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15ScrollBarPolicyE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1693
// index:32
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::LayoutDirection)

/*

*/
func Qt_getEnumMetaObject_32(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15LayoutDirectionE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1738
// index:33
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::InputMethodHint)

/*

*/
func Qt_getEnumMetaObject_33(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15InputMethodHintE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1753
// index:34
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::CursorMoveStyle)

/*

*/
func Qt_getEnumMetaObject_34(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15CursorMoveStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1726
// index:35
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::CaseSensitivity)

/*

*/
func Qt_getEnumMetaObject_35(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1713
// index:36
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::AspectRatioMode)

/*

*/
func Qt_getEnumMetaObject_36(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_15AspectRatioModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1732
// index:37
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::WindowModality)

/*

*/
func Qt_getEnumMetaObject_37(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_14WindowModalityE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1757
// index:38
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::MouseEventFlag)

/*

*/
func Qt_getEnumMetaObject_38(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_14MouseEventFlagE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1704
// index:39
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::DockWidgetArea)

/*

*/
func Qt_getEnumMetaObject_39(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_14DockWidgetAreaE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1746
// index:40
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ConnectionType)

/*

*/
func Qt_getEnumMetaObject_40(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_14ConnectionTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1707
// index:41
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TextElideMode)

/*

*/
func Qt_getEnumMetaObject_41(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_13TextElideModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1688
// index:42
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ClipOperation)

/*

*/
func Qt_getEnumMetaObject_42(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_13ClipOperationE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1683
// index:43
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::PenJoinStyle)

/*

*/
func Qt_getEnumMetaObject_43(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_12PenJoinStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1724
// index:44
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ItemDataRole)

/*

*/
func Qt_getEnumMetaObject_44(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_12ItemDataRoleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1749
// index:45
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::GestureState)

/*

*/
func Qt_getEnumMetaObject_45(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_12GestureStateE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1741
// index:46
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::EnterKeyType)

/*

*/
func Qt_getEnumMetaObject_46(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_12EnterKeyTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1758
// index:47
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ChecksumType)

/*

*/
func Qt_getEnumMetaObject_47(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_12ChecksumTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1731
// index:48
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::WindowState)

/*

*/
func Qt_getEnumMetaObject_48(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11WindowStateE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1705
// index:49
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ToolBarArea)

/*

*/
func Qt_getEnumMetaObject_49(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11ToolBarAreaE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1755
// index:50
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ScrollPhase)

/*

*/
func Qt_getEnumMetaObject_50(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11ScrollPhaseE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1682
// index:51
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::PenCapStyle)

/*

*/
func Qt_getEnumMetaObject_51(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11PenCapStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1695
// index:52
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::Orientation)

/*

*/
func Qt_getEnumMetaObject_52(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11OrientationE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1712
// index:53
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::GlobalColor)

/*

*/
func Qt_getEnumMetaObject_53(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11GlobalColorE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1750
// index:54
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::GestureType)

/*

*/
func Qt_getEnumMetaObject_54(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11GestureTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1737
// index:55
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::FocusReason)

/*

*/
func Qt_getEnumMetaObject_55(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11FocusReasonE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1677
// index:56
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::FocusPolicy)

/*

*/
func Qt_getEnumMetaObject_56(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11FocusPolicyE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1711
// index:57
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::CursorShape)

/*

*/
func Qt_getEnumMetaObject_57(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_11CursorShapeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1730
// index:58
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::WindowType)

/*

*/
func Qt_getEnumMetaObject_58(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_10WindowTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1706
// index:59
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TextFormat)

/*

*/
func Qt_getEnumMetaObject_59(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_10TextFormatE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1696
// index:60
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::DropAction)

/*

*/
func Qt_getEnumMetaObject_60(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_10DropActionE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1708
// index:61
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::DateFormat)

/*

*/
func Qt_getEnumMetaObject_61(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_10DateFormatE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1723
// index:62
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::CheckState)

/*

*/
func Qt_getEnumMetaObject_62(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_10CheckStateE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1684
// index:63
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::BrushStyle)

/*

*/
func Qt_getEnumMetaObject_63(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectENS_10BrushStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1727
// index:64
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::MatchFlags)

/*

*/
func Qt_getEnumMetaObject_64(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_9MatchFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1722
// index:65
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ItemFlags)

/*

*/
func Qt_getEnumMetaObject_65(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_8ItemFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1701
// index:66
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::Edges)

/*

*/
func Qt_getEnumMetaObject_66(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_4EdgeEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1719
// index:67
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TextInteractionFlags)

/*

*/
func Qt_getEnumMetaObject_67(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_19TextInteractionFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1715
// index:68
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ImageConversionFlags)

/*

*/
func Qt_getEnumMetaObject_68(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_19ImageConversionFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1745
// index:69
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ScreenOrientations)

/*

*/
func Qt_getEnumMetaObject_69(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_17ScreenOrientationEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1728
// index:70
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::KeyboardModifiers)

/*

*/
func Qt_getEnumMetaObject_70(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_16KeyboardModifierEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1742
// index:71
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::InputMethodQueries)

/*

*/
func Qt_getEnumMetaObject_71(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_16InputMethodQueryEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1743
// index:72
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::TouchPointStates)

/*

*/
func Qt_getEnumMetaObject_72(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_15TouchPointStateEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1740
// index:73
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::InputMethodHints)

/*

*/
func Qt_getEnumMetaObject_73(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_15InputMethodHintEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1702
// index:74
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::DockWidgetAreas)

/*

*/
func Qt_getEnumMetaObject_74(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_14DockWidgetAreaEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1697
// index:75
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::Alignment)

/*

*/
func Qt_getEnumMetaObject_75(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_13AlignmentFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1736
// index:76
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::WindowStates)

/*

*/
func Qt_getEnumMetaObject_76(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_11WindowStateEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1703
// index:77
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::ToolBarAreas)

/*

*/
func Qt_getEnumMetaObject_77(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_11ToolBarAreaEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1699
// index:78
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::Orientations)

/*

*/
func Qt_getEnumMetaObject_78(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_11OrientationEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1729
// index:79
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::MouseButtons)

/*

*/
func Qt_getEnumMetaObject_79(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_11MouseButtonEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1735
// index:80
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::WindowFlags)

/*

*/
func Qt_getEnumMetaObject_80(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_10WindowTypeEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1700
// index:81
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getEnumMetaObject(Qt::DropActions)

/*

*/
func Qt_getEnumMetaObject_81(arg0 int) *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt20qt_getEnumMetaObjectE6QFlagsINS_10DropActionEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:1754
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TimerType)

/*

*/
func Qt_getEnumName(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_9TimerTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1725
// index:1
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::SortOrder)

/*

*/
func Qt_getEnumName_1(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_9SortOrderE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1710
// index:2
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::DayOfWeek)

/*

*/
func Qt_getEnumName_2(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_9DayOfWeekE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1679
// index:3
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ArrowType)

/*

*/
func Qt_getEnumName_3(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_9ArrowTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1709
// index:4
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TimeSpec)

/*

*/
func Qt_getEnumName_4(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_8TimeSpecE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1698
// index:5
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TextFlag)

/*

*/
func Qt_getEnumName_5(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_8TextFlagE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1689
// index:6
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::SizeMode)

/*

*/
func Qt_getEnumName_6(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_8SizeModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1694
// index:7
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::SizeHint)

/*

*/
func Qt_getEnumName_7(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_8SizeHintE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1681
// index:8
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::PenStyle)

/*

*/
func Qt_getEnumName_8(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_8PenStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1686
// index:9
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::MaskMode)

/*

*/
func Qt_getEnumName_9(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_8MaskModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1685
// index:10
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::FillRule)

/*

*/
func Qt_getEnumName_10(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_8FillRuleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1691
// index:11
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::Corner)

/*

*/
func Qt_getEnumName_11(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_6CornerE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1687
// index:12
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::BGMode)

/*

*/
func Qt_getEnumName_12(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_6BGModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1692
// index:13
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::Edge)

/*

*/
func Qt_getEnumName_13(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_4EdgeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1690
// index:14
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::Axis)

/*

*/
func Qt_getEnumName_14(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_4AxisE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1716
// index:15
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::Key)

/*

*/
func Qt_getEnumName_15(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_3KeyE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1721
// index:16
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ItemSelectionOperation)

/*

*/
func Qt_getEnumName_16(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_22ItemSelectionOperationE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1734
// index:17
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ApplicationAttribute)

/*

*/
func Qt_getEnumName_17(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_20ApplicationAttributeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1718
// index:18
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TextInteractionFlag)

/*

*/
func Qt_getEnumName_18(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_19TextInteractionFlagE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1714
// index:19
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TransformationMode)

/*

*/
func Qt_getEnumName_19(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_18TransformationModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1744
// index:20
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ScreenOrientation)

/*

*/
func Qt_getEnumName_20(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_17ScreenOrientationE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1751
// index:21
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::NativeGestureType)

/*

*/
func Qt_getEnumName_21(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_17NativeGestureTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1720
// index:22
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ItemSelectionMode)

/*

*/
func Qt_getEnumName_22(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_17ItemSelectionModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1678
// index:23
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ContextMenuPolicy)

/*

*/
func Qt_getEnumName_23(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_17ContextMenuPolicyE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1759
// index:24
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TabFocusBehavior)

/*

*/
func Qt_getEnumName_24(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_16TabFocusBehaviorE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1756
// index:25
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::MouseEventSource)

/*

*/
func Qt_getEnumName_25(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_16MouseEventSourceE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1739
// index:26
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::InputMethodQuery)

/*

*/
func Qt_getEnumName_26(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_16InputMethodQueryE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1747
// index:27
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ApplicationState)

/*

*/
func Qt_getEnumName_27(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_16ApplicationStateE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1733
// index:28
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::WidgetAttribute)

/*

*/
func Qt_getEnumName_28(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15WidgetAttributeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1680
// index:29
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ToolButtonStyle)

/*

*/
func Qt_getEnumName_29(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15ToolButtonStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1717
// index:30
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ShortcutContext)

/*

*/
func Qt_getEnumName_30(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15ShortcutContextE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1676
// index:31
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ScrollBarPolicy)

/*

*/
func Qt_getEnumName_31(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15ScrollBarPolicyE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1693
// index:32
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::LayoutDirection)

/*

*/
func Qt_getEnumName_32(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15LayoutDirectionE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1738
// index:33
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::InputMethodHint)

/*

*/
func Qt_getEnumName_33(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15InputMethodHintE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1753
// index:34
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::CursorMoveStyle)

/*

*/
func Qt_getEnumName_34(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15CursorMoveStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1726
// index:35
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::CaseSensitivity)

/*

*/
func Qt_getEnumName_35(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15CaseSensitivityE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1713
// index:36
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::AspectRatioMode)

/*

*/
func Qt_getEnumName_36(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_15AspectRatioModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1732
// index:37
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::WindowModality)

/*

*/
func Qt_getEnumName_37(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_14WindowModalityE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1757
// index:38
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::MouseEventFlag)

/*

*/
func Qt_getEnumName_38(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_14MouseEventFlagE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1704
// index:39
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::DockWidgetArea)

/*

*/
func Qt_getEnumName_39(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_14DockWidgetAreaE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1746
// index:40
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ConnectionType)

/*

*/
func Qt_getEnumName_40(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_14ConnectionTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1707
// index:41
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TextElideMode)

/*

*/
func Qt_getEnumName_41(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_13TextElideModeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1688
// index:42
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ClipOperation)

/*

*/
func Qt_getEnumName_42(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_13ClipOperationE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1683
// index:43
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::PenJoinStyle)

/*

*/
func Qt_getEnumName_43(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_12PenJoinStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1724
// index:44
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ItemDataRole)

/*

*/
func Qt_getEnumName_44(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_12ItemDataRoleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1749
// index:45
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::GestureState)

/*

*/
func Qt_getEnumName_45(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_12GestureStateE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1741
// index:46
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::EnterKeyType)

/*

*/
func Qt_getEnumName_46(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_12EnterKeyTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1758
// index:47
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ChecksumType)

/*

*/
func Qt_getEnumName_47(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_12ChecksumTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1731
// index:48
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::WindowState)

/*

*/
func Qt_getEnumName_48(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11WindowStateE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1705
// index:49
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ToolBarArea)

/*

*/
func Qt_getEnumName_49(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11ToolBarAreaE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1755
// index:50
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ScrollPhase)

/*

*/
func Qt_getEnumName_50(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11ScrollPhaseE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1682
// index:51
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::PenCapStyle)

/*

*/
func Qt_getEnumName_51(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11PenCapStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1695
// index:52
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::Orientation)

/*

*/
func Qt_getEnumName_52(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11OrientationE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1712
// index:53
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::GlobalColor)

/*

*/
func Qt_getEnumName_53(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11GlobalColorE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1750
// index:54
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::GestureType)

/*

*/
func Qt_getEnumName_54(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11GestureTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1737
// index:55
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::FocusReason)

/*

*/
func Qt_getEnumName_55(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11FocusReasonE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1677
// index:56
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::FocusPolicy)

/*

*/
func Qt_getEnumName_56(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11FocusPolicyE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1711
// index:57
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::CursorShape)

/*

*/
func Qt_getEnumName_57(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_11CursorShapeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1730
// index:58
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::WindowType)

/*

*/
func Qt_getEnumName_58(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_10WindowTypeE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1706
// index:59
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TextFormat)

/*

*/
func Qt_getEnumName_59(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_10TextFormatE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1696
// index:60
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::DropAction)

/*

*/
func Qt_getEnumName_60(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_10DropActionE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1708
// index:61
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::DateFormat)

/*

*/
func Qt_getEnumName_61(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_10DateFormatE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1723
// index:62
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::CheckState)

/*

*/
func Qt_getEnumName_62(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_10CheckStateE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1684
// index:63
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::BrushStyle)

/*

*/
func Qt_getEnumName_63(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameENS_10BrushStyleE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1727
// index:64
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::MatchFlags)

/*

*/
func Qt_getEnumName_64(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_9MatchFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1722
// index:65
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ItemFlags)

/*

*/
func Qt_getEnumName_65(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_8ItemFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1701
// index:66
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::Edges)

/*

*/
func Qt_getEnumName_66(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_4EdgeEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1719
// index:67
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TextInteractionFlags)

/*

*/
func Qt_getEnumName_67(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_19TextInteractionFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1715
// index:68
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ImageConversionFlags)

/*

*/
func Qt_getEnumName_68(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_19ImageConversionFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1745
// index:69
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ScreenOrientations)

/*

*/
func Qt_getEnumName_69(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_17ScreenOrientationEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1728
// index:70
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::KeyboardModifiers)

/*

*/
func Qt_getEnumName_70(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_16KeyboardModifierEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1742
// index:71
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::InputMethodQueries)

/*

*/
func Qt_getEnumName_71(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_16InputMethodQueryEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1743
// index:72
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::TouchPointStates)

/*

*/
func Qt_getEnumName_72(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_15TouchPointStateEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1740
// index:73
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::InputMethodHints)

/*

*/
func Qt_getEnumName_73(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_15InputMethodHintEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1702
// index:74
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::DockWidgetAreas)

/*

*/
func Qt_getEnumName_74(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_14DockWidgetAreaEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1697
// index:75
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::Alignment)

/*

*/
func Qt_getEnumName_75(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_13AlignmentFlagEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1736
// index:76
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::WindowStates)

/*

*/
func Qt_getEnumName_76(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_11WindowStateEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1703
// index:77
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::ToolBarAreas)

/*

*/
func Qt_getEnumName_77(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_11ToolBarAreaEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1699
// index:78
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::Orientations)

/*

*/
func Qt_getEnumName_78(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_11OrientationEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1729
// index:79
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::MouseButtons)

/*

*/
func Qt_getEnumName_79(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_11MouseButtonEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1735
// index:80
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::WindowFlags)

/*

*/
func Qt_getEnumName_80(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_10WindowTypeEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qnamespace.h:1700
// index:81
// Invalid inline Visibility=Default Availability=Available
// [8] const char * qt_getEnumName(Qt::DropActions)

/*

*/
func Qt_getEnumName_81(arg0 int) string {
  rv, err := qtrt.InvokeQtFunc6("_ZN2Qt14qt_getEnumNameE6QFlagsINS_10DropActionEE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qsharedpointer_impl.h:115
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void internalSafetyCheckRemove(const void *)

/*

*/
func InternalSafetyCheckRemove(arg0 unsafe.Pointer /*666*/)  {
  rv, err := qtrt.InvokeQtFunc6("_ZN15QtSharedPointer25internalSafetyCheckRemoveEPKv", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qsharedpointer_impl.h:114
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void internalSafetyCheckAdd(const void *, const volatile void *)

/*

*/
func InternalSafetyCheckAdd(arg0 unsafe.Pointer /*666*/, arg1 unsafe.Pointer /*666*/)  {
  rv, err := qtrt.InvokeQtFunc6("_ZN15QtSharedPointer22internalSafetyCheckAddEPKvPVKv", qtrt.FFI_TYPE_POINTER, arg0, arg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qbytearray.h:92
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] int qstrcmp(const char *, const QByteArray &)

/*
A safe strcmp() function.

Compares str1 and str2. Returns a negative value if str1 is less than str2, 0 if str1 is equal to str2 or a positive value if str1 is greater than str2.

Special case 1: Returns 0 if str1 and str2 are both 0.

Special case 2: Returns an arbitrary non-zero value if str1 is 0 or str2 is 0 (but not both).

See also qstrncmp(), qstricmp(), qstrnicmp(), and 8-bit Character Comparisons.
*/
func Qstrcmp(str1 string, str2 QByteArray_ITF) int {
    var convArg0 = qtrt.CString(str1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if str2 != nil && str2.QByteArray_PTR() != nil {
        convArg1 = str2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_ZL7qstrcmpPKcRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qglobal.h:843
// index:0
// Invalid inline Visibility=Default Availability=Available
// [1] bool qIsNull(float)

/*

*/
func QIsNull(f float32) bool {
  rv, err := qtrt.InvokeQtFunc6("_ZL7qIsNullf", qtrt.FFI_TYPE_POINTER, f)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:827
// index:1
// Invalid inline Visibility=Default Availability=Available
// [1] bool qIsNull(double)

/*

*/
func QIsNull_1(d float64) bool {
  rv, err := qtrt.InvokeQtFunc6("_ZL7qIsNulld", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:807
// index:0
// Invalid inline Visibility=Default Availability=Available
// [1] bool qFuzzyCompare(float, float)

/*

*/
func QFuzzyCompare(p1 float32, p2 float32) bool {
  rv, err := qtrt.InvokeQtFunc6("_ZL13qFuzzyCompareff", qtrt.FFI_TYPE_POINTER, p1, p2)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:802
// index:1
// Invalid inline Visibility=Default Availability=Available
// [1] bool qFuzzyCompare(double, double)

/*

*/
func QFuzzyCompare_1(p1 float64, p2 float64) bool {
  rv, err := qtrt.InvokeQtFunc6("_ZL13qFuzzyComparedd", qtrt.FFI_TYPE_POINTER, p1, p2)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:817
// index:0
// Invalid inline Visibility=Default Availability=Available
// [1] bool qFuzzyIsNull(float)

/*

*/
func QFuzzyIsNull(f float32) bool {
  rv, err := qtrt.InvokeQtFunc6("_ZL12qFuzzyIsNullf", qtrt.FFI_TYPE_POINTER, f)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:812
// index:1
// Invalid inline Visibility=Default Availability=Available
// [1] bool qFuzzyIsNull(double)

/*

*/
func QFuzzyIsNull_1(d float64) bool {
  rv, err := qtrt.InvokeQtFunc6("_ZL12qFuzzyIsNulld", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:1143
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qunsetenv(const char *)

/*

*/
func Qunsetenv(varName string) bool {
    var convArg0 = qtrt.CString(varName)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z9qunsetenvPKc", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:732
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qt_assert(const char *, const char *, int)

/*

*/
func Qt_assert(assertion string, file string, line int)  {
    var convArg0 = qtrt.CString(assertion)
    defer qtrt.FreeMem(convArg0)
    var convArg1 = qtrt.CString(file)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_Z9qt_assertPKcS0_i", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, line)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qbytearray.h:101
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int qstrnicmp(const char *, const char *, uint)

/*
A safe strnicmp() function.

Compares at most len bytes of str1 and str2 ignoring the case of the characters. The encoding of the strings is assumed to be Latin-1.

Returns a negative value if str1 is less than str2, 0 if str1 is equal to str2 or a positive value if str1 is greater than str2.

Special case 1: Returns 0 if str1 and str2 are both 0.

Special case 2: Returns a random non-zero value if str1 is 0 or str2 is 0 (but not both).

See also qstrcmp(), qstrncmp(), qstricmp(), and 8-bit Character Comparisons.
*/
func Qstrnicmp(arg0 string, arg1 string, len_ uint) int {
    var convArg0 = qtrt.CString(arg0)
    defer qtrt.FreeMem(convArg0)
    var convArg1 = qtrt.CString(arg1)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_Z9qstrnicmpPKcS0_j", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, len_)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qnumeric.h:53
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qIsFinite(float)

/*

*/
func QIsFinite(f float32) bool {
  rv, err := qtrt.InvokeQtFunc6("_Z9qIsFinitef", qtrt.FFI_TYPE_POINTER, f)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qnumeric.h:50
// index:1
// Invalid Visibility=Default Availability=Available
// [1] bool qIsFinite(double)

/*

*/
func QIsFinite_1(d float64) bool {
  rv, err := qtrt.InvokeQtFunc6("_Z9qIsFinited", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qhashfunctions.h:70
// index:0
// Invalid Visibility=Default Availability=Available
// [4] uint qHashBits(const void *, size_t, uint)

/*

*/
func QHashBits(p unsafe.Pointer /*666*/, size uint, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z9qHashBitsPKvmj", qtrt.FFI_TYPE_POINTER, p, size, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qbytearray.h:685
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] QByteArray qCompress(const QByteArray &, int)

/*
This is an overloaded function.

Compresses the first nbytes of data at compression level compressionLevel and returns the compressed data in a new byte array.
*/
func QCompress(data QByteArray_ITF, compressionLevel int) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if data != nil && data.QByteArray_PTR() != nil {
        convArg0 = data.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z9qCompressRK10QByteArrayi", qtrt.FFI_TYPE_POINTER, convArg0, compressionLevel)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qbytearray.h:683
// index:1
// Invalid Visibility=Default Availability=Available
// [8] QByteArray qCompress(const uchar *, int, int)

/*
This is an overloaded function.

Compresses the first nbytes of data at compression level compressionLevel and returns the compressed data in a new byte array.
*/
func QCompress_1(data unsafe.Pointer /*666*/, nbytes int, compressionLevel int) *QByteArray/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_Z9qCompressPKhii", qtrt.FFI_TYPE_POINTER, data, nbytes, compressionLevel)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qbytearray.h:109
// index:0
// Invalid Visibility=Default Availability=Available
// [2] quint16 qChecksum(const char *, uint, Qt::ChecksumType)

/*
Returns the CRC-16 checksum of the first len bytes of data.

The checksum is independent of the byte order (endianness) and will be calculated accorded to the algorithm published in ISO 3309 (Qt::ChecksumIso3309).

Note: This function is a 16-bit cache conserving (16 entry table) implementation of the CRC-16-CCITT algorithm.
*/
func QChecksum(s string, len_ uint, standard int) uint16 {
    var convArg0 = qtrt.CString(s)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z9qChecksumPKcjN2Qt12ChecksumTypeE", qtrt.FFI_TYPE_POINTER, convArg0, len_, standard)
  qtrt.ErrPrint(err, rv)
    return uint16(rv) // 222
}

// /usr/include/qt/QtCore/qbytearray.h:108
// index:1
// Invalid Visibility=Default Availability=Available
// [2] quint16 qChecksum(const char *, uint)

/*
Returns the CRC-16 checksum of the first len bytes of data.

The checksum is independent of the byte order (endianness) and will be calculated accorded to the algorithm published in ISO 3309 (Qt::ChecksumIso3309).

Note: This function is a 16-bit cache conserving (16 entry table) implementation of the CRC-16-CCITT algorithm.
*/
func QChecksum_1(s string, len_ uint) uint16 {
    var convArg0 = qtrt.CString(s)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z9qChecksumPKcj", qtrt.FFI_TYPE_POINTER, convArg0, len_)
  qtrt.ErrPrint(err, rv)
    return uint16(rv) // 222
}

// /usr/include/qt/QtCore/qglobal.h:781
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qBadAlloc()

/*

*/
func QBadAlloc()  {
  rv, err := qtrt.InvokeQtFunc6("_Z9qBadAllocv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qtextstream.h:241
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & forcesign(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() | QTextStream::ForceSign) on stream and returns stream.

See also noforcesign(), forcepoint(), showbase(), and QTextStream manipulators.
*/
func Forcesign(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z9forcesignR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:240
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & showbase(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() | QTextStream::ShowBase) on stream and returns stream.

See also noshowbase(), forcesign(), forcepoint(), and QTextStream manipulators.
*/
func Showbase(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z8showbaseR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qbytearray.h:76
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] uint qstrnlen(const char *, uint)

/*
A safe strnlen() function.

Returns the number of characters that precede the terminating '\0', but at most maxlen. If str is 0, returns 0.

This function was introduced in  Qt 4.2.

See also qstrlen().
*/
func Qstrnlen(str string, maxlen uint) uint {
    var convArg0 = qtrt.CString(str)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z8qstrnlenPKcj", qtrt.FFI_TYPE_POINTER, convArg0, maxlen)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qbytearray.h:87
// index:0
// Invalid Visibility=Default Availability=Available
// [8] char * qstrncpy(char *, const char *, uint)

/*
A safe strncpy() function.

Copies at most len bytes from src (stopping at len or the terminating '\0' whichever comes first) into dst and returns a pointer to dst. Guarantees that dst is '\0'-terminated. If src or dst is 0, returns 0 immediately.

This function assumes that dst is at least len characters long.

Note: When compiling with Visual C++ compiler version 14.00 (Visual C++ 2005) or later, internally the function strncpy_s will be used.

See also qstrcpy().
*/
func Qstrncpy(dst string, src string, len_ uint) string {
    var convArg0 = qtrt.CString(dst)
    defer qtrt.FreeMem(convArg0)
    var convArg1 = qtrt.CString(src)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_Z8qstrncpyPcPKcj", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, len_)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qbytearray.h:95
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] int qstrncmp(const char *, const char *, uint)

/*
A safe strncmp() function.

Compares at most len bytes of str1 and str2.

Returns a negative value if str1 is less than str2, 0 if str1 is equal to str2 or a positive value if str1 is greater than str2.

Special case 1: Returns 0 if str1 and str2 are both 0.

Special case 2: Returns a random non-zero value if str1 is 0 or str2 is 0 (but not both).

See also qstrcmp(), qstricmp(), qstrnicmp(), and 8-bit Character Comparisons.
*/
func Qstrncmp(str1 string, str2 string, len_ uint) int {
    var convArg0 = qtrt.CString(str1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 = qtrt.CString(str2)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_Z8qstrncmpPKcS0_j", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, len_)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qbytearray.h:100
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int qstricmp(const char *, const char *)

/*
A safe stricmp() function.

Compares str1 and str2 ignoring the case of the characters. The encoding of the strings is assumed to be Latin-1.

Returns a negative value if str1 is less than str2, 0 if str1 is equal to str2 or a positive value if str1 is greater than str2.

Special case 1: Returns 0 if str1 and str2 are both 0.

Special case 2: Returns a random non-zero value if str1 is 0 or str2 is 0 (but not both).

See also qstrcmp(), qstrncmp(), qstrnicmp(), and 8-bit Character Comparisons.
*/
func Qstricmp(arg0 string, arg1 string) int {
    var convArg0 = qtrt.CString(arg0)
    defer qtrt.FreeMem(convArg0)
    var convArg1 = qtrt.CString(arg1)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_Z8qstricmpPKcS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qglobal.h:544
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qint64 qRound64(float)

/*

*/
func QRound64(d float32) int64 {
  rv, err := qtrt.InvokeQtFunc6("_Z8qRound64f", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return int64(rv) // 222
}

// /usr/include/qt/QtCore/qglobal.h:542
// index:1
// Invalid inline Visibility=Default Availability=Available
// [8] qint64 qRound64(double)

/*

*/
func QRound64_1(d float64) int64 {
  rv, err := qtrt.InvokeQtFunc6("_Z8qRound64d", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return int64(rv) // 222
}

// /usr/include/qt/QtCore/qglobal.h:1150
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] int qIntCast(float)

/*

*/
func QIntCast(f float32) int {
  rv, err := qtrt.InvokeQtFunc6("_Z8qIntCastf", qtrt.FFI_TYPE_POINTER, f)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qglobal.h:1149
// index:1
// Invalid inline Visibility=Default Availability=Available
// [4] int qIntCast(double)

/*

*/
func QIntCast_1(f float64) int {
  rv, err := qtrt.InvokeQtFunc6("_Z8qIntCastd", qtrt.FFI_TYPE_POINTER, f)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qmath.h:206
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qFastSin(qreal)

/*

*/
func QFastSin(x float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z8qFastSind", qtrt.FFI_TYPE_POINTER, x)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qmath.h:216
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qFastCos(qreal)

/*

*/
func QFastCos(x float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z8qFastCosd", qtrt.FFI_TYPE_POINTER, x)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qcoreapplication.h:261
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QString qAppName()

/*

*/
func QAppName() string {
  rv, err := qtrt.InvokeQtFunc6("_Z8qAppNamev", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qglobal.h:647
// index:0
// Invalid inline Visibility=Default Availability=Available
// [-2] void qt_noop()

/*

*/
func Qt_noop()  {
  rv, err := qtrt.InvokeQtFunc6("_Z7qt_noopv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qhashfunctions.h:105
// index:0
// Invalid Visibility=Default Availability=Available
// [4] uint qt_hash(QStringView, uint)

/*

*/
func Qt_hash(key QStringView_ITF/*123*/, chained uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QStringView_PTR() != nil {
        convArg0 = key.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z7qt_hash11QStringViewj", qtrt.FFI_TYPE_POINTER, convArg0, chained)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qbytearray.h:73
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] uint qstrlen(const char *)

/*
A safe strlen() function.

Returns the number of characters that precede the terminating '\0', or 0 if str is 0.

See also qstrnlen().
*/
func Qstrlen(str string) uint {
    var convArg0 = qtrt.CString(str)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z7qstrlenPKc", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qbytearray.h:71
// index:0
// Invalid Visibility=Default Availability=Available
// [8] char * qstrdup(const char *)

/*
Returns a duplicate string.

Allocates space for a copy of src, copies it, and returns a pointer to the copy. If src is 0, it immediately returns 0.

Ownership is passed to the caller, so the returned string must be deleted using delete[].
*/
func Qstrdup(arg0 string) string {
    var convArg0 = qtrt.CString(arg0)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z7qstrdupPKc", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qbytearray.h:86
// index:0
// Invalid Visibility=Default Availability=Available
// [8] char * qstrcpy(char *, const char *)

/*
Copies all the characters up to and including the '\0' from src into dst and returns a pointer to dst. If src is 0, it immediately returns 0.

This function assumes that dst is large enough to hold the contents of src.

See also qstrncpy().
*/
func Qstrcpy(dst string, src string) string {
    var convArg0 = qtrt.CString(dst)
    defer qtrt.FreeMem(convArg0)
    var convArg1 = qtrt.CString(src)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_Z7qstrcpyPcPKc", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qbytearray.h:90
// index:1
// Invalid Visibility=Default Availability=Available
// [4] int qstrcmp(const QByteArray &, const QByteArray &)

/*
A safe strcmp() function.

Compares str1 and str2. Returns a negative value if str1 is less than str2, 0 if str1 is equal to str2 or a positive value if str1 is greater than str2.

Special case 1: Returns 0 if str1 and str2 are both 0.

Special case 2: Returns an arbitrary non-zero value if str1 is 0 or str2 is 0 (but not both).

See also qstrncmp(), qstricmp(), qstrnicmp(), and 8-bit Character Comparisons.
*/
func Qstrcmp_1(str1 QByteArray_ITF, str2 QByteArray_ITF) int {
    var convArg0 unsafe.Pointer
    if str1 != nil && str1.QByteArray_PTR() != nil {
        convArg0 = str1.QByteArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if str2 != nil && str2.QByteArray_PTR() != nil {
        convArg1 = str2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z7qstrcmpRK10QByteArrayS1_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qbytearray.h:91
// index:2
// Invalid Visibility=Default Availability=Available
// [4] int qstrcmp(const QByteArray &, const char *)

/*
A safe strcmp() function.

Compares str1 and str2. Returns a negative value if str1 is less than str2, 0 if str1 is equal to str2 or a positive value if str1 is greater than str2.

Special case 1: Returns 0 if str1 and str2 are both 0.

Special case 2: Returns an arbitrary non-zero value if str1 is 0 or str2 is 0 (but not both).

See also qstrncmp(), qstricmp(), qstrnicmp(), and 8-bit Character Comparisons.
*/
func Qstrcmp_2(str1 QByteArray_ITF, str2 string) int {
    var convArg0 unsafe.Pointer
    if str1 != nil && str1.QByteArray_PTR() != nil {
        convArg0 = str1.QByteArray_PTR().GetCthis()
    }
    var convArg1 = qtrt.CString(str2)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_Z7qstrcmpRK10QByteArrayPKc", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qbytearray.h:89
// index:3
// Invalid Visibility=Default Availability=Available
// [4] int qstrcmp(const char *, const char *)

/*
A safe strcmp() function.

Compares str1 and str2. Returns a negative value if str1 is less than str2, 0 if str1 is equal to str2 or a positive value if str1 is greater than str2.

Special case 1: Returns 0 if str1 and str2 are both 0.

Special case 2: Returns an arbitrary non-zero value if str1 is 0 or str2 is 0 (but not both).

See also qstrncmp(), qstricmp(), qstrnicmp(), and 8-bit Character Comparisons.
*/
func Qstrcmp_3(str1 string, str2 string) int {
    var convArg0 = qtrt.CString(str1)
    defer qtrt.FreeMem(convArg0)
    var convArg1 = qtrt.CString(str2)
    defer qtrt.FreeMem(convArg1)
  rv, err := qtrt.InvokeQtFunc6("_Z7qstrcmpPKcS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qglobal.h:1142
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qputenv(const char *, const QByteArray &)

/*

*/
func Qputenv(varName string, value QByteArray_ITF) bool {
    var convArg0 = qtrt.CString(varName)
    defer qtrt.FreeMem(convArg0)
    var convArg1 unsafe.Pointer
    if value != nil && value.QByteArray_PTR() != nil {
        convArg1 = value.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z7qputenvPKcRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:1134
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QByteArray qgetenv(const char *)

/*

*/
func Qgetenv(varName string) *QByteArray/*123*/ {
    var convArg0 = qtrt.CString(varName)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z7qgetenvPKc", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qglobal.h:1051
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QString qtTrId(const char *, int)

/*

*/
func QtTrId(id string, n int) string {
    var convArg0 = qtrt.CString(id)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z6qtTrIdPKci", qtrt.FFI_TYPE_POINTER, convArg0, n)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qglobal.h:1155
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qsrand(uint)

/*

*/
func Qsrand(seed uint)  {
  rv, err := qtrt.InvokeQtFunc6("_Z6qsrandj", qtrt.FFI_TYPE_POINTER, seed)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qglobal.h:539
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] int qRound(float)

/*

*/
func QRound(d float32) int {
  rv, err := qtrt.InvokeQtFunc6("_Z6qRoundf", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qglobal.h:537
// index:1
// Invalid inline Visibility=Default Availability=Available
// [4] int qRound(double)

/*

*/
func QRound_1(d float64) int {
  rv, err := qtrt.InvokeQtFunc6("_Z6qRoundd", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qnumeric.h:52
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qIsNaN(float)

/*

*/
func QIsNaN(f float32) bool {
  rv, err := qtrt.InvokeQtFunc6("_Z6qIsNaNf", qtrt.FFI_TYPE_POINTER, f)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qnumeric.h:49
// index:1
// Invalid Visibility=Default Availability=Available
// [1] bool qIsNaN(double)

/*

*/
func QIsNaN_1(d float64) bool {
  rv, err := qtrt.InvokeQtFunc6("_Z6qIsNaNd", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qnumeric.h:51
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qIsInf(float)

/*

*/
func QIsInf(f float32) bool {
  rv, err := qtrt.InvokeQtFunc6("_Z6qIsInff", qtrt.FFI_TYPE_POINTER, f)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qnumeric.h:48
// index:1
// Invalid Visibility=Default Availability=Available
// [1] bool qIsInf(double)

/*

*/
func QIsInf_1(d float64) bool {
  rv, err := qtrt.InvokeQtFunc6("_Z6qIsInfd", qtrt.FFI_TYPE_POINTER, d)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qmath.h:74
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] int qFloor(qreal)

/*

*/
func QFloor(v float64) int {
  rv, err := qtrt.InvokeQtFunc6("_Z6qFloord", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qmath.h:122
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qAtan2(qreal, qreal)

/*

*/
func QAtan2(y float64, x float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z6qAtan2dd", qtrt.FFI_TYPE_POINTER, y, x)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qtextstream.h:257
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & center(QTextStream &)

/*
Calls QTextStream::setFieldAlignment(QTextStream::AlignCenter) on stream and returns stream.

See also left(), right(), and QTextStream manipulators.
*/
func Center(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z6centerR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:256
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & right(QTextStream &)

/*
Calls QTextStream::setFieldAlignment(QTextStream::AlignRight) on stream and returns stream.

See also left(), center(), and QTextStream manipulators.
*/
func Right(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5rightR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:261
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & reset(QTextStream &)

/*
Resets QTextStream's formatting options, bringing it back to its original constructed state. The device, string and any buffered data is left untouched.
*/
func Reset(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5resetR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qglobal.h:1156
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int qrand()

/*

*/
func Qrand() int {
  rv, err := qtrt.InvokeQtFunc6("_Z5qrandv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qmath.h:128
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qSqrt(qreal)

/*

*/
func QSqrt(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z5qSqrtd", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qnumeric.h:54
// index:0
// Invalid Visibility=Default Availability=Available
// [8] double qSNaN()

/*

*/
func QSNaN() float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z5qSNaNv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qnumeric.h:55
// index:0
// Invalid Visibility=Default Availability=Available
// [8] double qQNaN()

/*

*/
func QQNaN() float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z5qQNaNv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qhashfunctions.h:86
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(quint64, uint)

/*

*/
func QHash(key uint64, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashyj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:90
// index:1
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(qint64, uint)

/*

*/
func QHash_1(key int64, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashxj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:75
// index:2
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(ushort, uint)

/*

*/
func QHash_2(key uint16, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashtj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:76
// index:3
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(short, uint)

/*

*/
func QHash_3(key int16, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashsj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:79
// index:4
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(ulong, uint)

/*

*/
func QHash_4(key uint, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashmj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:85
// index:5
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(long, uint)

/*

*/
func QHash_5(key int, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashlj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:77
// index:6
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(uint, uint)

/*

*/
func QHash_6(key uint, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashjj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:78
// index:7
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(int, uint)

/*

*/
func QHash_7(key int, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashij", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:73
// index:8
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(uchar, uint)

/*

*/
func QHash_8(key byte, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashhj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:91
// index:9
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(float, uint)

/*

*/
func QHash_9(key float32, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashfj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:94
// index:10
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(long double, uint)

/*

*/
func QHash_10(key float64, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashej", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:92
// index:11
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(double, uint)

/*

*/
func QHash_11(key float64, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashdj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:72
// index:12
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(char, uint)

/*

*/
func QHash_12(key byte, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashcj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:74
// index:13
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(signed char, uint)

/*

*/
func QHash_13(key byte, seed uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashaj", qtrt.FFI_TYPE_POINTER, key, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qurlquery.h:53
// index:14
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QUrlQuery &, uint)

/*
Returns the hash value for key, using seed to seed the calculation.

This function was introduced in  Qt 5.6.
*/
func QHash_14(key QUrlQuery_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QUrlQuery_PTR() != nil {
        convArg0 = key.QUrlQuery_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK9QUrlQueryj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qmimetype.h:58
// index:15
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QMimeType &, uint)

/*
Returns the hash value for key, using seed to seed the calculation.

This function was introduced in  Qt 5.6.
*/
func QHash_15(key QMimeType_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QMimeType_PTR() != nil {
        convArg0 = key.QMimeType_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK9QMimeTypej", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qdatetime.h:412
// index:16
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QDateTime &, uint)

/*

*/
func QHash_16(key QDateTime_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QDateTime_PTR() != nil {
        convArg0 = key.QDateTime_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK9QDateTimej", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:103
// index:17
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QBitArray &, uint)

/*

*/
func QHash_17(key QBitArray_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QBitArray_PTR() != nil {
        convArg0 = key.QBitArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK9QBitArrayj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:99
// index:18
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QString &, uint)

/*

*/
func QHash_18(key string, seed uint) uint {
    var tmpArg0 = NewQString_5(key)
    var convArg0 = tmpArg0.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK7QStringj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qregexp.h:56
// index:19
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QRegExp &, uint)

/*
Returns the hash value for key, using seed to seed the calculation.

This function was introduced in  Qt 5.6.
*/
func QHash_19(key QRegExp_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QRegExp_PTR() != nil {
        convArg0 = key.QRegExp_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK7QRegExpj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qlocale.h:62
// index:20
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QLocale &, uint)

/*
Returns the hash value for key, using seed to seed the calculation.

This function was introduced in  Qt 5.6.
*/
func QHash_20(key QLocale_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QLocale_PTR() != nil {
        convArg0 = key.QLocale_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK7QLocalej", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/quuid.h:235
// index:21
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QUuid &, uint)

/*
Returns a hash of the UUID uuid, using seed to seed the calculation.

This function was introduced in  Qt 5.0.
*/
func QHash_21(uuid QUuid_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if uuid != nil && uuid.QUuid_PTR() != nil {
        convArg0 = uuid.QUuid_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK5QUuidj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qdatetime.h:414
// index:22
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QTime &, uint)

/*

*/
func QHash_22(key QTime_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QTime_PTR() != nil {
        convArg0 = key.QTime_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK5QTimej", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qdatetime.h:413
// index:23
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QDate &, uint)

/*

*/
func QHash_23(key QDate_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QDate_PTR() != nil {
        convArg0 = key.QDate_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK5QDatej", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qurl.h:122
// index:24
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QUrl &, uint)

/*

*/
func QHash_24(url QUrl_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if url != nil && url.QUrl_PTR() != nil {
        convArg0 = url.QUrl_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK4QUrlj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:102
// index:25
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QPersistentModelIndex &, uint)

/*

*/
func QHash_25(index QPersistentModelIndex_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if index != nil && index.QPersistentModelIndex_PTR() != nil {
        convArg0 = index.QPersistentModelIndex_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK21QPersistentModelIndexj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:149
// index:26
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(const QPersistentModelIndex &, uint)

/*

*/
func QHash_26(index QPersistentModelIndex_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if index != nil && index.QPersistentModelIndex_PTR() != nil {
        convArg0 = index.QPersistentModelIndex_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK21QPersistentModelIndexj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:227
// index:27
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(const QItemSelectionRange &)

/*

*/
func QHash_27(arg0 QItemSelectionRange_ITF) uint {
    var convArg0 unsafe.Pointer
    if arg0 != nil && arg0.QItemSelectionRange_PTR() != nil {
        convArg0 = arg0.QItemSelectionRange_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK19QItemSelectionRange", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qregularexpression.h:62
// index:28
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QRegularExpression &, uint)

/*
Returns the hash value for key, using seed to seed the calculation.

This function was introduced in  Qt 5.6.
*/
func QHash_28(key QRegularExpression_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QRegularExpression_PTR() != nil {
        convArg0 = key.QRegularExpression_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK18QRegularExpressionj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qversionnumber.h:54
// index:29
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QVersionNumber &, uint)

/*

*/
func QHash_29(key QVersionNumber_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QVersionNumber_PTR() != nil {
        convArg0 = key.QVersionNumber_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK14QVersionNumberj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:437
// index:30
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(const QModelIndex &)

/*

*/
func QHash_30(index QModelIndex_ITF) uint {
    var convArg0 unsafe.Pointer
    if index != nil && index.QModelIndex_PTR() != nil {
        convArg0 = index.QModelIndex_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK11QModelIndex", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:100
// index:31
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QStringRef &, uint)

/*

*/
func QHash_31(key QStringRef_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QStringRef_PTR() != nil {
        convArg0 = key.QStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK10QStringRefj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:97
// index:32
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(const QByteArray &, uint)

/*

*/
func QHash_32(key QByteArray_ITF, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QByteArray_PTR() != nil {
        convArg0 = key.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHashRK10QByteArrayj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:96
// index:33
// Invalid inline Visibility=Default Availability=Available
// [4] uint qHash(const QChar, uint)

/*

*/
func QHash_33(key QChar_ITF/*123*/, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QChar_PTR() != nil {
        convArg0 = key.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHash5QCharj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:104
// index:34
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(QLatin1String, uint)

/*

*/
func QHash_34(key QLatin1String_ITF/*123*/, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QLatin1String_PTR() != nil {
        convArg0 = key.QLatin1String_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHash13QLatin1Stringj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:102
// index:35
// Invalid Visibility=Default Availability=Available
// [4] uint qHash(QStringView, uint)

/*

*/
func QHash_35(key QStringView_ITF/*123*/, seed uint) uint {
    var convArg0 unsafe.Pointer
    if key != nil && key.QStringView_PTR() != nil {
        convArg0 = key.QStringView_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5qHash11QStringViewj", qtrt.FFI_TYPE_POINTER, convArg0, seed)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qmath.h:80
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qFabs(qreal)

/*

*/
func QFabs(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z5qFabsd", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qmath.h:68
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] int qCeil(qreal)

/*

*/
func QCeil(v float64) int {
  rv, err := qtrt.InvokeQtFunc6("_Z5qCeild", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qmath.h:116
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qAtan(qreal)

/*

*/
func QAtan(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z5qAtand", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qmath.h:110
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qAsin(qreal)

/*

*/
func QAsin(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z5qAsind", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qmath.h:104
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qAcos(qreal)

/*

*/
func QAcos(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z5qAcosd", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qtextstream.h:260
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & flush(QTextStream &)

/*
Flushes any buffered data waiting to be written to the device.

If QTextStream operates on a string, this function does nothing.
*/
func Flush(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5flushR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:252
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & fixed(QTextStream &)

/*
Calls QTextStream::setRealNumberNotation(QTextStream::FixedNotation) on stream and returns stream.

See also scientific() and QTextStream manipulators.
*/
func Fixed(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z5fixedR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qurlquery.h:112
// index:0
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QUrlQuery &, QUrlQuery &)

/*
Swaps this URL query instance with other. This function is very fast and never fails.
*/
func Swap(value1 QUrlQuery_ITF, value2 QUrlQuery_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QUrlQuery_PTR() != nil {
        convArg0 = value1.QUrlQuery_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QUrlQuery_PTR() != nil {
        convArg1 = value2.QUrlQuery_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR9QUrlQueryS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qtimezone.h:177
// index:1
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QTimeZone &, QTimeZone &)

/*
Swaps this time zone instance with other. This function is very fast and never fails.
*/
func Swap_1(value1 QTimeZone_ITF, value2 QTimeZone_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QTimeZone_PTR() != nil {
        convArg0 = value1.QTimeZone_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QTimeZone_PTR() != nil {
        convArg1 = value2.QTimeZone_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR9QTimeZoneS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qmimetype.h:129
// index:2
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QMimeType &, QMimeType &)

/*
Swaps QMimeType other with this QMimeType object.

This operation is very fast and never fails.

The swap() method helps with the implementation of assignment operators in an exception-safe way. For more information consult More C++ Idioms - Copy-and-swap.
*/
func Swap_2(value1 QMimeType_ITF, value2 QMimeType_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QMimeType_PTR() != nil {
        convArg0 = value1.QMimeType_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QMimeType_PTR() != nil {
        convArg1 = value2.QMimeType_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR9QMimeTypeS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qfileinfo.h:157
// index:3
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QFileInfo &, QFileInfo &)

/*
Swaps this file info with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
func Swap_3(value1 QFileInfo_ITF, value2 QFileInfo_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QFileInfo_PTR() != nil {
        convArg0 = value1.QFileInfo_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QFileInfo_PTR() != nil {
        convArg1 = value2.QFileInfo_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR9QFileInfoS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qdatetime.h:393
// index:4
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QDateTime &, QDateTime &)

/*
Swaps this datetime with other. This operation is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
func Swap_4(value1 QDateTime_ITF, value2 QDateTime_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QDateTime_PTR() != nil {
        convArg0 = value1.QDateTime_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QDateTime_PTR() != nil {
        convArg1 = value2.QDateTime_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR9QDateTimeS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qcollator.h:128
// index:5
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QCollator &, QCollator &)

/*
Swaps this collator with other. This function is very fast and never fails.
*/
func Swap_5(value1 QCollator_ITF, value2 QCollator_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QCollator_PTR() != nil {
        convArg0 = value1.QCollator_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QCollator_PTR() != nil {
        convArg1 = value2.QCollator_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR9QCollatorS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qbitarray.h:172
// index:6
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QBitArray &, QBitArray &)

/*
Swaps bit array other with this bit array. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
func Swap_6(value1 QBitArray_ITF, value2 QBitArray_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QBitArray_PTR() != nil {
        convArg0 = value1.QBitArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QBitArray_PTR() != nil {
        convArg1 = value2.QBitArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR9QBitArrayS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qvariant.h:577
// index:7
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QVariant &, QVariant &)

/*
Swaps variant other with this variant. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
func Swap_7(value1 QVariant_ITF, value2 QVariant_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QVariant_PTR() != nil {
        convArg0 = value1.QVariant_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QVariant_PTR() != nil {
        convArg1 = value2.QVariant_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR8QVariantS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qstring.h:1403
// index:8
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QString &, QString &)

/*
Swaps string other with this string. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
func Swap_8(value1 string, value2 string)  {
    var tmpArg0 = NewQString_5(value1)
    var convArg0 = tmpArg0.GetCthis()
    var tmpArg1 = NewQString_5(value2)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR7QStringS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qlocale.h:1095
// index:9
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QLocale &, QLocale &)

/*
Swaps locale other with this locale. This operation is very fast and never fails.

This function was introduced in  Qt 5.6.
*/
func Swap_9(value1 QLocale_ITF, value2 QLocale_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QLocale_PTR() != nil {
        convArg0 = value1.QLocale_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QLocale_PTR() != nil {
        convArg1 = value2.QLocale_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR7QLocaleS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qurl.h:375
// index:10
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QUrl &, QUrl &)

/*
Swaps URL other with this URL. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
func Swap_10(value1 QUrl_ITF, value2 QUrl_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QUrl_PTR() != nil {
        convArg0 = value1.QUrl_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QUrl_PTR() != nil {
        convArg1 = value2.QUrl_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR4QUrlS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qdir.h:233
// index:11
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QDir &, QDir &)

/*
Swaps this QDir instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
func Swap_11(value1 QDir_ITF, value2 QDir_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QDir_PTR() != nil {
        convArg0 = value1.QDir_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QDir_PTR() != nil {
        convArg1 = value2.QDir_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR4QDirS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qregularexpression.h:276
// index:12
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QRegularExpressionMatchIterator &, QRegularExpressionMatchIterator &)

/*
Swaps the regular expression other with this regular expression. This operation is very fast and never fails.
*/
func Swap_12(value1 QRegularExpressionMatchIterator_ITF, value2 QRegularExpressionMatchIterator_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QRegularExpressionMatchIterator_PTR() != nil {
        convArg0 = value1.QRegularExpressionMatchIterator_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QRegularExpressionMatchIterator_PTR() != nil {
        convArg1 = value2.QRegularExpressionMatchIterator_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR31QRegularExpressionMatchIteratorS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qregularexpression.h:238
// index:13
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QRegularExpressionMatch &, QRegularExpressionMatch &)

/*
Swaps the regular expression other with this regular expression. This operation is very fast and never fails.
*/
func Swap_13(value1 QRegularExpressionMatch_ITF, value2 QRegularExpressionMatch_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QRegularExpressionMatch_PTR() != nil {
        convArg0 = value1.QRegularExpressionMatch_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QRegularExpressionMatch_PTR() != nil {
        convArg1 = value2.QRegularExpressionMatch_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR23QRegularExpressionMatchS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qabstractitemmodel.h:147
// index:14
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QPersistentModelIndex &, QPersistentModelIndex &)

/*

*/
func Swap_14(value1 QPersistentModelIndex_ITF, value2 QPersistentModelIndex_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QPersistentModelIndex_PTR() != nil {
        convArg0 = value1.QPersistentModelIndex_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QPersistentModelIndex_PTR() != nil {
        convArg1 = value2.QPersistentModelIndex_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR21QPersistentModelIndexS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qxmlstream.h:95
// index:15
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QXmlStreamStringRef &, QXmlStreamStringRef &)

/*

*/
func Swap_15(value1 QXmlStreamStringRef_ITF, value2 QXmlStreamStringRef_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QXmlStreamStringRef_PTR() != nil {
        convArg0 = value1.QXmlStreamStringRef_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QXmlStreamStringRef_PTR() != nil {
        convArg1 = value2.QXmlStreamStringRef_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR19QXmlStreamStringRefS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qprocess.h:106
// index:16
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QProcessEnvironment &, QProcessEnvironment &)

/*

*/
func Swap_16(value1 QProcessEnvironment_ITF, value2 QProcessEnvironment_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QProcessEnvironment_PTR() != nil {
        convArg0 = value1.QProcessEnvironment_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QProcessEnvironment_PTR() != nil {
        convArg1 = value2.QProcessEnvironment_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR19QProcessEnvironmentS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qregularexpression.h:159
// index:17
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QRegularExpression &, QRegularExpression &)

/*
Swaps the regular expression other with this regular expression. This operation is very fast and never fails.
*/
func Swap_17(value1 QRegularExpression_ITF, value2 QRegularExpression_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QRegularExpression_PTR() != nil {
        convArg0 = value1.QRegularExpression_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QRegularExpression_PTR() != nil {
        convArg1 = value2.QRegularExpression_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR18QRegularExpressionS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qcommandlineoption.h:108
// index:18
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QCommandLineOption &, QCommandLineOption &)

/*
Swaps option other with this option. This operation is very fast and never fails.
*/
func Swap_18(value1 QCommandLineOption_ITF, value2 QCommandLineOption_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QCommandLineOption_PTR() != nil {
        convArg0 = value1.QCommandLineOption_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QCommandLineOption_PTR() != nil {
        convArg1 = value2.QCommandLineOption_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR18QCommandLineOptionS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qcollator.h:127
// index:19
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QCollatorSortKey &, QCollatorSortKey &)

/*
Swaps this collator with other. This function is very fast and never fails.
*/
func Swap_19(value1 QCollatorSortKey_ITF, value2 QCollatorSortKey_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QCollatorSortKey_PTR() != nil {
        convArg0 = value1.QCollatorSortKey_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QCollatorSortKey_PTR() != nil {
        convArg1 = value2.QCollatorSortKey_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR16QCollatorSortKeyS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qitemselectionmodel.h:263
// index:20
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QItemSelection &, QItemSelection &)

/*

*/
func Swap_20(value1 QItemSelection_ITF, value2 QItemSelection_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QItemSelection_PTR() != nil {
        convArg0 = value1.QItemSelection_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QItemSelection_PTR() != nil {
        convArg1 = value2.QItemSelection_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR14QItemSelectionS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:191
// index:21
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QDeadlineTimer &, QDeadlineTimer &)

/*
Swaps this deadline timer with the other deadline timer.
*/
func Swap_21(value1 QDeadlineTimer_ITF, value2 QDeadlineTimer_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QDeadlineTimer_PTR() != nil {
        convArg0 = value1.QDeadlineTimer_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QDeadlineTimer_PTR() != nil {
        convArg1 = value2.QDeadlineTimer_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR14QDeadlineTimerS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qjsondocument.h:171
// index:22
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QJsonDocument &, QJsonDocument &)

/*
Swaps the document other with this. This operation is very fast and never fails.

This function was introduced in  Qt 5.10.
*/
func Swap_22(value1 QJsonDocument_ITF, value2 QJsonDocument_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QJsonDocument_PTR() != nil {
        convArg0 = value1.QJsonDocument_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QJsonDocument_PTR() != nil {
        convArg1 = value2.QJsonDocument_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR13QJsonDocumentS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qstorageinfo.h:118
// index:23
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QStorageInfo &, QStorageInfo &)

/*
Swaps this volume info with other. This function is very fast and never fails.
*/
func Swap_23(value1 QStorageInfo_ITF, value2 QStorageInfo_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QStorageInfo_PTR() != nil {
        convArg0 = value1.QStorageInfo_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QStorageInfo_PTR() != nil {
        convArg1 = value2.QStorageInfo_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR12QStorageInfoS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qeasingcurve.h:128
// index:24
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QEasingCurve &, QEasingCurve &)

/*
Swaps curve other with this curve. This operation is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
func Swap_24(value1 QEasingCurve_ITF, value2 QEasingCurve_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QEasingCurve_PTR() != nil {
        convArg0 = value1.QEasingCurve_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QEasingCurve_PTR() != nil {
        convArg1 = value2.QEasingCurve_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR12QEasingCurveS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qjsonobject.h:263
// index:25
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QJsonObject &, QJsonObject &)

/*
Swaps the object other with this. This operation is very fast and never fails.

This function was introduced in  Qt 5.10.
*/
func Swap_25(value1 QJsonObject_ITF, value2 QJsonObject_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QJsonObject_PTR() != nil {
        convArg0 = value1.QJsonObject_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QJsonObject_PTR() != nil {
        convArg1 = value2.QJsonObject_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR11QJsonObjectS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qjsonvalue.h:247
// index:26
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QJsonValue &, QJsonValue &)

/*
Swaps the value other with this. This operation is very fast and never fails.

This function was introduced in  Qt 5.10.
*/
func Swap_26(value1 QJsonValue_ITF, value2 QJsonValue_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QJsonValue_PTR() != nil {
        convArg0 = value1.QJsonValue_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QJsonValue_PTR() != nil {
        convArg1 = value2.QJsonValue_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR10QJsonValueS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qjsonarray.h:266
// index:27
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QJsonArray &, QJsonArray &)

/*
Swaps the array other with this. This operation is very fast and never fails.

This function was introduced in  Qt 5.10.
*/
func Swap_27(value1 QJsonArray_ITF, value2 QJsonArray_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QJsonArray_PTR() != nil {
        convArg0 = value1.QJsonArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QJsonArray_PTR() != nil {
        convArg1 = value2.QJsonArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR10QJsonArrayS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qbytearray.h:691
// index:28
// Invalid inline Visibility=Default Availability=Available
// [-2] void swap(QByteArray &, QByteArray &)

/*
Swaps byte array other with this byte array. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
func Swap_28(value1 QByteArray_ITF, value2 QByteArray_ITF)  {
    var convArg0 unsafe.Pointer
    if value1 != nil && value1.QByteArray_PTR() != nil {
        convArg0 = value1.QByteArray_PTR().GetCthis()
    }
    var convArg1 unsafe.Pointer
    if value2 != nil && value2.QByteArray_PTR() != nil {
        convArg1 = value2.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4swapR10QByteArrayS0_", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qmath.h:98
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qTan(qreal)

/*

*/
func QTan(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z4qTand", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qmath.h:86
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qSin(qreal)

/*

*/
func QSin(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z4qSind", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qmath.h:146
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qPow(qreal, qreal)

/*

*/
func QPow(x float64, y float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z4qPowdd", qtrt.FFI_TYPE_POINTER, x, y)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qnumeric.h:56
// index:0
// Invalid Visibility=Default Availability=Available
// [8] double qInf()

/*

*/
func QInf() float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z4qInfv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qmath.h:140
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qExp(qreal)

/*

*/
func QExp(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z4qExpd", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qmath.h:92
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qCos(qreal)

/*

*/
func QCos(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z4qCosd", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qtextstream.h:255
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & left(QTextStream &)

/*
Calls QTextStream::setFieldAlignment(QTextStream::AlignLeft) on stream and returns stream.

See also right(), center(), and QTextStream manipulators.
*/
func Left(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4leftR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:259
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & endl(QTextStream &)

/*
Writes '\n' to the stream and flushes the stream.

Equivalent to


  stream << '\n' << flush;



Note: On Windows, all '\n' characters are written as '\r\n' if QTextStream's device or string is opened using the QIODevice::Text flag.

See also flush(), reset(), and QTextStream manipulators.
*/
func Endl(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z4endlR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qmath.h:134
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] qreal qLn(qreal)

/*

*/
func QLn(v float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z3qLnd", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qtextstream.h:236
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & oct(QTextStream &)

/*
Calls QTextStream::setIntegerBase(8) on stream and returns stream.

See also bin(), dec(), hex(), and QTextStream manipulators.
*/
func Oct(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z3octR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:238
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & hex(QTextStream &)

/*
Calls QTextStream::setIntegerBase(16) on stream and returns stream.

Note: The hex modifier can only be used for writing to streams.

See also bin(), oct(), dec(), and QTextStream manipulators.
*/
func Hex(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z3hexR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:237
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & dec(QTextStream &)

/*
Calls QTextStream::setIntegerBase(10) on stream and returns stream.

See also bin(), oct(), hex(), and QTextStream manipulators.
*/
func Dec(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z3decR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:263
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & bom(QTextStream &)

/*
Toggles insertion of the Byte Order Mark on stream when QTextStream is used with a UTF codec.

See also QTextStream::setGenerateByteOrderMark() and QTextStream manipulators.
*/
func Bom(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z3bomR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:235
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & bin(QTextStream &)

/*
Calls QTextStream::setIntegerBase(2) on stream and returns stream.

See also oct(), dec(), hex(), and QTextStream manipulators.
*/
func Bin(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z3binR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:265
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & ws(QTextStream &)

/*
Calls skipWhiteSpace() on stream and returns stream.

See also QTextStream manipulators.
*/
func Ws(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z2wsR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qplugin.h:78
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qRegisterStaticPluginFunction(QStaticPlugin)

/*

*/
func QRegisterStaticPluginFunction(staticPlugin QStaticPlugin_ITF/*123*/)  {
    var convArg0 unsafe.Pointer
    if staticPlugin != nil && staticPlugin.QStaticPlugin_PTR() != nil {
        convArg0 = staticPlugin.QStaticPlugin_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z29qRegisterStaticPluginFunction13QStaticPlugin", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qglobal.h:1147
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int qEnvironmentVariableIntValue(const char *, bool *)

/*

*/
func QEnvironmentVariableIntValue(varName string, ok *bool) int {
    var convArg0 = qtrt.CString(varName)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z28qEnvironmentVariableIntValuePKcPb", qtrt.FFI_TYPE_POINTER, convArg0, ok)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qglobal.h:1145
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qEnvironmentVariableIsEmpty(const char *)

/*

*/
func QEnvironmentVariableIsEmpty(varName string) bool {
    var convArg0 = qtrt.CString(varName)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z27qEnvironmentVariableIsEmptyPKc", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:1146
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qEnvironmentVariableIsSet(const char *)

/*

*/
func QEnvironmentVariableIsSet(varName string) bool {
    var convArg0 = qtrt.CString(varName)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z25qEnvironmentVariableIsSetPKc", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// ./bsheaders/QtCore/extra_export.h:12
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qUnregisterResourceData(int, const unsigned char *, const unsigned char *, const unsigned char *)

/*

*/
func QUnregisterResourceData(arg0 int, arg1 unsafe.Pointer /*666*/, arg2 unsafe.Pointer /*666*/, arg3 unsafe.Pointer /*666*/) bool {
  rv, err := qtrt.InvokeQtFunc6("_Z23qUnregisterResourceDataiPKhS0_S0_", qtrt.FFI_TYPE_POINTER, arg0, arg1, arg2, arg3)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qtextstream.h:279
// index:0
// Invalid inline Visibility=Default Availability=Available
// [40] QTextStreamManipulator qSetRealNumberPrecision(int)

/*
Equivalent to QTextStream::setRealNumberPrecision(precision).
*/
func QSetRealNumberPrecision(precision int) *QTextStreamManipulator/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_Z23qSetRealNumberPrecisioni", qtrt.FFI_TYPE_POINTER, precision)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamManipulatorFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStreamManipulator)
    return rv2
}

// /usr/include/qt/QtCore/qlogging.h:191
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QtMessageHandler qInstallMessageHandler(QtMessageHandler)

/*

*/
func QInstallMessageHandler(arg0 unsafe.Pointer /*666*/) unsafe.Pointer /*666*/ {
  rv, err := qtrt.InvokeQtFunc6("_Z22qInstallMessageHandlerPFv9QtMsgTypeRK18QMessageLogContextRK7QStringE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
    return unsafe.Pointer(uintptr(rv))
}

// /usr/include/qt/QtCore/qalgorithms.h:791
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountTrailingZeroBits(quint64)

/*

*/
func QCountTrailingZeroBits(v uint64) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z22qCountTrailingZeroBitsy", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:775
// index:1
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountTrailingZeroBits(quint16)

/*

*/
func QCountTrailingZeroBits_1(v uint16) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z22qCountTrailingZeroBitst", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:802
// index:2
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountTrailingZeroBits(unsigned long)

/*

*/
func QCountTrailingZeroBits_2(v uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z22qCountTrailingZeroBitsm", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:742
// index:3
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountTrailingZeroBits(quint32)

/*

*/
func QCountTrailingZeroBits_3(v uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z22qCountTrailingZeroBitsj", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:760
// index:4
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountTrailingZeroBits(quint8)

/*

*/
func QCountTrailingZeroBits_4(v byte) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z22qCountTrailingZeroBitsh", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// ./bsheaders/QtCore/extra_export.h:11
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qRegisterResourceData(int, const unsigned char *, const unsigned char *, const unsigned char *)

/*

*/
func QRegisterResourceData(arg0 int, arg1 unsafe.Pointer /*666*/, arg2 unsafe.Pointer /*666*/, arg3 unsafe.Pointer /*666*/) bool {
  rv, err := qtrt.InvokeQtFunc6("_Z21qRegisterResourceDataiPKhS0_S0_", qtrt.FFI_TYPE_POINTER, arg0, arg1, arg2, arg3)
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qalgorithms.h:847
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountLeadingZeroBits(quint64)

/*

*/
func QCountLeadingZeroBits(v uint64) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z21qCountLeadingZeroBitsy", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:834
// index:1
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountLeadingZeroBits(quint16)

/*

*/
func QCountLeadingZeroBits_1(v uint16) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z21qCountLeadingZeroBitst", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:862
// index:2
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountLeadingZeroBits(unsigned long)

/*

*/
func QCountLeadingZeroBits_2(v uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z21qCountLeadingZeroBitsm", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:807
// index:3
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountLeadingZeroBits(quint32)

/*

*/
func QCountLeadingZeroBits_3(v uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z21qCountLeadingZeroBitsj", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:822
// index:4
// Invalid inline Visibility=Default Availability=Available
// [4] uint qCountLeadingZeroBits(quint8)

/*

*/
func QCountLeadingZeroBits_4(v byte) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z21qCountLeadingZeroBitsh", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qobject.h:93
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QObject * qt_qFindChild_helper(const QObject *, const QString &, const QMetaObject &, Qt::FindChildOptions)

/*

*/
func Qt_qFindChild_helper(parent QObject_ITF/*777 const QObject **/, name string, mo QMetaObject_ITF, options int) *QObject/*777 QObject **/ {
    var convArg0 unsafe.Pointer
    if parent != nil && parent.QObject_PTR() != nil {
        convArg0 = parent.QObject_PTR().GetCthis()
    }
    var tmpArg1 = NewQString_5(name)
    var convArg1 = tmpArg1.GetCthis()
    var convArg2 unsafe.Pointer
    if mo != nil && mo.QMetaObject_PTR() != nil {
        convArg2 = mo.QMetaObject_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z20qt_qFindChild_helperPK7QObjectRK7QStringRK11QMetaObject6QFlagsIN2Qt15FindChildOptionEE", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, convArg2, options)
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qglobal.h:1140
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QString qEnvironmentVariable(const char *, const QString &)

/*

*/
func QEnvironmentVariable(varName string, defaultValue string) string {
    var convArg0 = qtrt.CString(varName)
    defer qtrt.FreeMem(convArg0)
    var tmpArg1 = NewQString_5(defaultValue)
    var convArg1 = tmpArg1.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Z20qEnvironmentVariablePKcRK7QString", qtrt.FFI_TYPE_POINTER, convArg0, convArg1)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qglobal.h:1139
// index:1
// Invalid Visibility=Default Availability=Available
// [8] QString qEnvironmentVariable(const char *)

/*

*/
func QEnvironmentVariable_1(varName string) string {
    var convArg0 = qtrt.CString(varName)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z20qEnvironmentVariablePKc", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qhashfunctions.h:68
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qSetGlobalQHashSeed(int)

/*

*/
func QSetGlobalQHashSeed(newSeed int)  {
  rv, err := qtrt.InvokeQtFunc6("_Z19qSetGlobalQHashSeedi", qtrt.FFI_TYPE_POINTER, newSeed)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qobject.h:473
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getQtMetaObject()

/*

*/
func Qt_getQtMetaObject() *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_Z18qt_getQtMetaObjectv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qnamespace.h:53
// index:1
// Invalid Visibility=Default Availability=Available
// [8] const QMetaObject * qt_getQtMetaObject()

/*

*/
func Qt_getQtMetaObject_1() *QMetaObject/*777 const QMetaObject **/ {
  rv, err := qtrt.InvokeQtFunc6("_Z18qt_getQtMetaObjectv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return /*==*/NewQMetaObjectFromPointer(unsafe.Pointer(uintptr(rv))) // 444
}

// /usr/include/qt/QtCore/qlogging.h:193
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qSetMessagePattern(const QString &)

/*

*/
func QSetMessagePattern(messagePattern string)  {
    var tmpArg0 = NewQString_5(messagePattern)
    var convArg0 = tmpArg0.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Z18qSetMessagePatternRK7QString", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qcoreapplication.h:260
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qRemovePostRoutine(QtCleanUpFunction)

/*
Removes the cleanup routine specified by ptr from the list of routines called by the QCoreApplication destructor. The routine must have been previously added to the list by a call to qAddPostRoutine(), otherwise this function has no effect.

Note: This function has been thread-safe since Qt 5.10.

Note: This function is thread-safe.

This function was introduced in  Qt 5.3.

See also qAddPostRoutine().
*/
func QRemovePostRoutine(arg0 unsafe.Pointer /*666*/)  {
  rv, err := qtrt.InvokeQtFunc6("_Z18qRemovePostRoutinePFvvE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qlogging.h:179
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qt_message_output(QtMsgType, const QMessageLogContext &, const QString &)

/*

*/
func Qt_message_output(arg0 int, context QMessageLogContext_ITF, message string)  {
    var convArg1 unsafe.Pointer
    if context != nil && context.QMessageLogContext_PTR() != nil {
        convArg1 = context.QMessageLogContext_PTR().GetCthis()
    }
    var tmpArg2 = NewQString_5(message)
    var convArg2 = tmpArg2.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Z17qt_message_output9QtMsgTypeRK18QMessageLogContextRK7QString", qtrt.FFI_TYPE_POINTER, arg0, convArg1, convArg2)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qmath.h:236
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] float qRadiansToDegrees(float)

/*

*/
func QRadiansToDegrees(radians float32) float32 {
  rv, err := qtrt.InvokeQtFunc6("_Z17qRadiansToDegreesf", qtrt.FFI_TYPE_POINTER, radians)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float32", rv).(float32) // 1111
}

// /usr/include/qt/QtCore/qmath.h:241
// index:1
// Invalid inline Visibility=Default Availability=Available
// [8] double qRadiansToDegrees(double)

/*

*/
func QRadiansToDegrees_1(radians float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z17qRadiansToDegreesd", qtrt.FFI_TYPE_POINTER, radians)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qlogging.h:194
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QString qFormatLogMessage(QtMsgType, const QMessageLogContext &, const QString &)

/*

*/
func QFormatLogMessage(type_ int, context QMessageLogContext_ITF, buf string) string {
    var convArg1 unsafe.Pointer
    if context != nil && context.QMessageLogContext_PTR() != nil {
        convArg1 = context.QMessageLogContext_PTR().GetCthis()
    }
    var tmpArg2 = NewQString_5(buf)
    var convArg2 = tmpArg2.GetCthis()
  rv, err := qtrt.InvokeQtFunc6("_Z17qFormatLogMessage9QtMsgTypeRK18QMessageLogContextRK7QString", qtrt.FFI_TYPE_POINTER, type_, convArg1, convArg2)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qmath.h:226
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] float qDegreesToRadians(float)

/*

*/
func QDegreesToRadians(degrees float32) float32 {
  rv, err := qtrt.InvokeQtFunc6("_Z17qDegreesToRadiansf", qtrt.FFI_TYPE_POINTER, degrees)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float32", rv).(float32) // 1111
}

// /usr/include/qt/QtCore/qmath.h:231
// index:1
// Invalid inline Visibility=Default Availability=Available
// [8] double qDegreesToRadians(double)

/*

*/
func QDegreesToRadians_1(degrees float64) float64 {
  rv, err := qtrt.InvokeQtFunc6("_Z17qDegreesToRadiansd", qtrt.FFI_TYPE_POINTER, degrees)
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("float64", rv).(float64) // 1111
}

// /usr/include/qt/QtCore/qglobal.h:780
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qt_check_pointer(const char *, int)

/*

*/
func Qt_check_pointer(arg0 string, arg1 int)  {
    var convArg0 = qtrt.CString(arg0)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z16qt_check_pointerPKci", qtrt.FFI_TYPE_POINTER, convArg0, arg1)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qalgorithms.h:717
// index:0
// Invalid inline Visibility=Default Availability=Available
// [4] uint qPopulationCount(quint64)

/*

*/
func QPopulationCount(v uint64) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z16qPopulationCounty", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:706
// index:1
// Invalid inline Visibility=Default Availability=Available
// [4] uint qPopulationCount(quint16)

/*

*/
func QPopulationCount_1(v uint16) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z16qPopulationCountt", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:732
// index:2
// Invalid inline Visibility=Default Availability=Available
// [4] uint qPopulationCount(unsigned long)

/*

*/
func QPopulationCount_2(v uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z16qPopulationCountm", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:683
// index:3
// Invalid inline Visibility=Default Availability=Available
// [4] uint qPopulationCount(quint32)

/*

*/
func QPopulationCount_3(v uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z16qPopulationCountj", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qalgorithms.h:696
// index:4
// Invalid inline Visibility=Default Availability=Available
// [4] uint qPopulationCount(quint8)

/*

*/
func QPopulationCount_4(v byte) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z16qPopulationCounth", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qhashfunctions.h:67
// index:0
// Invalid Visibility=Default Availability=Available
// [4] int qGlobalQHashSeed()

/*

*/
func QGlobalQHashSeed() int {
  rv, err := qtrt.InvokeQtFunc6("_Z16qGlobalQHashSeedv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return qtrt.Cretval2go("int", rv).(int) // 1111
}

// /usr/include/qt/QtCore/qtextstream.h:248
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & uppercasedigits(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() | QTextStream::UppercaseDigits) on stream and returns stream.

See also lowercasedigits(), uppercasebase(), and QTextStream manipulators.
*/
func Uppercasedigits(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z15uppercasedigitsR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qglobal.h:727
// index:0
// Invalid Visibility=Default Availability=Available
// [8] QString qt_error_string(int)

/*

*/
func Qt_error_string(errorCode int) string {
  rv, err := qtrt.InvokeQtFunc6("_Z15qt_error_stringi", qtrt.FFI_TYPE_POINTER, errorCode)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQStringFromPointer(unsafe.Pointer(uintptr(rv)))
    rv3 := rv2.ToLocal8Bit().Data()
    /*==*/DeleteQString(rv2)
    return rv3
}

// /usr/include/qt/QtCore/qglobal.h:901
// index:0
// Invalid Visibility=Default Availability=Available
// [8] void * qReallocAligned(void *, size_t, size_t, size_t)

/*

*/
func QReallocAligned(ptr unsafe.Pointer /*666*/, size uint, oldsize uint, alignment uint) unsafe.Pointer /*666*/ {
  rv, err := qtrt.InvokeQtFunc6("_Z15qReallocAlignedPvmmm", qtrt.FFI_TYPE_POINTER, ptr, size, oldsize, alignment)
  qtrt.ErrPrint(err, rv)
    return unsafe.Pointer(uintptr(rv))
}

// /usr/include/qt/QtCore/qmath.h:264
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] quint64 qNextPowerOfTwo(quint64)

/*

*/
func QNextPowerOfTwo(v uint64) uint64 {
  rv, err := qtrt.InvokeQtFunc6("_Z15qNextPowerOfTwoy", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint64(rv) // 222
}

// /usr/include/qt/QtCore/qmath.h:287
// index:1
// Invalid inline Visibility=Default Availability=Available
// [8] quint64 qNextPowerOfTwo(qint64)

/*

*/
func QNextPowerOfTwo_1(v int64) uint64 {
  rv, err := qtrt.InvokeQtFunc6("_Z15qNextPowerOfTwox", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint64(rv) // 222
}

// /usr/include/qt/QtCore/qmath.h:247
// index:2
// Invalid inline Visibility=Default Availability=Available
// [4] quint32 qNextPowerOfTwo(quint32)

/*

*/
func QNextPowerOfTwo_2(v uint) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z15qNextPowerOfTwoj", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qmath.h:282
// index:3
// Invalid inline Visibility=Default Availability=Available
// [4] quint32 qNextPowerOfTwo(qint32)

/*

*/
func QNextPowerOfTwo_3(v int) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z15qNextPowerOfTwoi", qtrt.FFI_TYPE_POINTER, v)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qcoreapplication.h:259
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qAddPostRoutine(QtCleanUpFunction)

/*
Adds a global routine that will be called from the QCoreApplication destructor. This function is normally used to add cleanup routines for program-wide functionality.

The cleanup routines are called in the reverse order of their addition.

The function specified by ptr should take no arguments and should return nothing. For example:


  static int *global_ptr = 0;

  static void cleanup_ptr()
  {
      delete [] global_ptr;
      global_ptr = 0;
  }

  void init_ptr()
  {
      global_ptr = new int[100];      // allocate data
      qAddPostRoutine(cleanup_ptr);   // delete later
  }



Note that for an application- or module-wide cleanup, qAddPostRoutine() is often not suitable. For example, if the program is split into dynamically loaded modules, the relevant module may be unloaded long before the QCoreApplication destructor is called. In such cases, if using qAddPostRoutine() is still desirable, qRemovePostRoutine() can be used to prevent a routine from being called by the QCoreApplication destructor. For example, if that routine was called before the module was unloaded.

For modules and libraries, using a reference-counted initialization manager or Qt's parent-child deletion mechanism may be better. Here is an example of a private class that uses the parent-child mechanism to call a cleanup function at the right time:


  class MyPrivateInitStuff : public QObject
  {
  public:
      static MyPrivateInitStuff *initStuff(QObject *parent)
      {
          if (!p)
              p = new MyPrivateInitStuff(parent);
          return p;
      }

      ~MyPrivateInitStuff()
      {
          // cleanup goes here
      }

  private:
      MyPrivateInitStuff(QObject *parent)
          : QObject(parent)
      {
          // initialization goes here
      }

      MyPrivateInitStuff *p;
  };



By selecting the right parent object, this can often be made to clean up the module's data at the right moment.

Note: This function has been thread-safe since Qt 5.10.

Note: This function is thread-safe.

See also qRemovePostRoutine().
*/
func QAddPostRoutine(arg0 unsafe.Pointer /*666*/)  {
  rv, err := qtrt.InvokeQtFunc6("_Z15qAddPostRoutinePFvvE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qtextstream.h:250
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & lowercasedigits(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() & ~QTextStream::UppercaseDigits) on stream and returns stream.

See also uppercasedigits(), lowercasebase(), and QTextStream manipulators.
*/
func Lowercasedigits(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z15lowercasedigitsR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:267
// index:0
// Invalid inline Visibility=Default Availability=Available
// [40] QTextStreamManipulator qSetFieldWidth(int)

/*
Equivalent to QTextStream::setFieldWidth(width).
*/
func QSetFieldWidth(width int) *QTextStreamManipulator/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_Z14qSetFieldWidthi", qtrt.FFI_TYPE_POINTER, width)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamManipulatorFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStreamManipulator)
    return rv2
}

// /usr/include/qt/QtCore/qglobal.h:900
// index:0
// Invalid Visibility=Default Availability=Available
// [8] void * qMallocAligned(size_t, size_t)

/*

*/
func QMallocAligned(size uint, alignment uint) unsafe.Pointer /*666*/ {
  rv, err := qtrt.InvokeQtFunc6("_Z14qMallocAlignedmm", qtrt.FFI_TYPE_POINTER, size, alignment)
  qtrt.ErrPrint(err, rv)
    return unsafe.Pointer(uintptr(rv))
}

// /usr/include/qt/QtCore/qnumeric.h:58
// index:0
// Invalid Visibility=Default Availability=Available
// [4] quint32 qFloatDistance(float, float)

/*

*/
func QFloatDistance(a float32, b float32) uint {
  rv, err := qtrt.InvokeQtFunc6("_Z14qFloatDistanceff", qtrt.FFI_TYPE_POINTER, a, b)
  qtrt.ErrPrint(err, rv)
    return uint(rv) // 222
}

// /usr/include/qt/QtCore/qnumeric.h:59
// index:1
// Invalid Visibility=Default Availability=Available
// [8] quint64 qFloatDistance(double, double)

/*

*/
func QFloatDistance_1(a float64, b float64) uint64 {
  rv, err := qtrt.InvokeQtFunc6("_Z14qFloatDistancedd", qtrt.FFI_TYPE_POINTER, a, b)
  qtrt.ErrPrint(err, rv)
    return uint64(rv) // 222
}

// /usr/include/qt/QtCore/qcoreapplication.h:258
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qAddPreRoutine(QtStartUpFunction)

/*

*/
func QAddPreRoutine(arg0 unsafe.Pointer /*666*/)  {
  rv, err := qtrt.InvokeQtFunc6("_Z14qAddPreRoutinePFvvE", qtrt.FFI_TYPE_POINTER, arg0)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qtextstream.h:247
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & uppercasebase(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() | QTextStream::UppercaseBase) on stream and returns stream.

See also lowercasebase(), uppercasedigits(), and QTextStream manipulators.
*/
func Uppercasebase(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z13uppercasebaseR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qobjectdefs.h:261
// index:0
// Invalid Visibility=Default Availability=Available
// [8] const char * qFlagLocation(const char *)

/*

*/
func QFlagLocation(method string) string {
    var convArg0 = qtrt.CString(method)
    defer qtrt.FreeMem(convArg0)
  rv, err := qtrt.InvokeQtFunc6("_Z13qFlagLocationPKc", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    return qtrt.GoStringI(rv)
}

// /usr/include/qt/QtCore/qtextstream.h:249
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & lowercasebase(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() & ~QTextStream::UppercaseBase) on stream and returns stream.

See also uppercasebase(), lowercasedigits(), and QTextStream manipulators.
*/
func Lowercasebase(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z13lowercasebaseR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qglobal.h:687
// index:0
// Invalid Visibility=Default Availability=Available
// [1] bool qSharedBuild()

/*

*/
func QSharedBuild() bool {
  rv, err := qtrt.InvokeQtFunc6("_Z12qSharedBuildv", qtrt.FFI_TYPE_POINTER, )
  qtrt.ErrPrint(err, rv)
    return rv!=0
}

// /usr/include/qt/QtCore/qglobal.h:902
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qFreeAligned(void *)

/*

*/
func QFreeAligned(ptr unsafe.Pointer /*666*/)  {
  rv, err := qtrt.InvokeQtFunc6("_Z12qFreeAlignedPv", qtrt.FFI_TYPE_POINTER, ptr)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qtextstream.h:245
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & noforcepoint(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() & ~QTextStream::ForcePoint) on stream and returns stream.

See also forcepoint(), noforcesign(), noshowbase(), and QTextStream manipulators.
*/
func Noforcepoint(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z12noforcepointR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qglobal.h:749
// index:0
// Invalid Visibility=Default Availability=Available
// [-2] void qt_assert_x(const char *, const char *, const char *, int)

/*

*/
func Qt_assert_x(where string, what string, file string, line int)  {
    var convArg0 = qtrt.CString(where)
    defer qtrt.FreeMem(convArg0)
    var convArg1 = qtrt.CString(what)
    defer qtrt.FreeMem(convArg1)
    var convArg2 = qtrt.CString(file)
    defer qtrt.FreeMem(convArg2)
  rv, err := qtrt.InvokeQtFunc6("_Z11qt_assert_xPKcS0_S0_i", qtrt.FFI_TYPE_POINTER, convArg0, convArg1, convArg2, line)
  qtrt.ErrPrint(err, rv)
}

// /usr/include/qt/QtCore/qbytearray.h:687
// index:0
// Invalid inline Visibility=Default Availability=Available
// [8] QByteArray qUncompress(const QByteArray &)

/*
This is an overloaded function.

Uncompresses the first nbytes of data and returns a new byte array with the uncompressed data.
*/
func QUncompress(data QByteArray_ITF) *QByteArray/*123*/ {
    var convArg0 unsafe.Pointer
    if data != nil && data.QByteArray_PTR() != nil {
        convArg0 = data.QByteArray_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z11qUncompressRK10QByteArray", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qbytearray.h:684
// index:1
// Invalid Visibility=Default Availability=Available
// [8] QByteArray qUncompress(const uchar *, int)

/*
This is an overloaded function.

Uncompresses the first nbytes of data and returns a new byte array with the uncompressed data.
*/
func QUncompress_1(data unsafe.Pointer /*666*/, nbytes int) *QByteArray/*123*/ {
  rv, err := qtrt.InvokeQtFunc6("_Z11qUncompressPKhi", qtrt.FFI_TYPE_POINTER, data, nbytes)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQByteArrayFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQByteArray)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:273
// index:0
// Invalid inline Visibility=Default Availability=Available
// [40] QTextStreamManipulator qSetPadChar(QChar)

/*
Equivalent to QTextStream::setPadChar(ch).
*/
func QSetPadChar(ch QChar_ITF/*123*/) *QTextStreamManipulator/*123*/ {
    var convArg0 unsafe.Pointer
    if ch != nil && ch.QChar_PTR() != nil {
        convArg0 = ch.QChar_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z11qSetPadChar5QChar", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamManipulatorFromPointer(unsafe.Pointer(uintptr(rv))) // 333
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStreamManipulator)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:244
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & noforcesign(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() & ~QTextStream::ForceSign) on stream and returns stream.

See also forcesign(), noforcepoint(), noshowbase(), and QTextStream manipulators.
*/
func Noforcesign(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z11noforcesignR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:253
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & scientific(QTextStream &)

/*
Calls QTextStream::setRealNumberNotation(QTextStream::ScientificNotation) on stream and returns stream.

See also fixed() and QTextStream manipulators.
*/
func Scientific(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z10scientificR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:243
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & noshowbase(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() & ~QTextStream::ShowBase) on stream and returns stream.

See also showbase(), noforcesign(), noforcepoint(), and QTextStream manipulators.
*/
func Noshowbase(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z10noshowbaseR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

// /usr/include/qt/QtCore/qtextstream.h:242
// index:0
// Invalid Visibility=Default Availability=Available
// [16] QTextStream & forcepoint(QTextStream &)

/*
Calls QTextStream::setNumberFlags(QTextStream::numberFlags() | QTextStream::ForcePoint) on stream and returns stream.

See also noforcepoint(), forcesign(), showbase(), and QTextStream manipulators.
*/
func Forcepoint(s QTextStream_ITF) *QTextStream {
    var convArg0 unsafe.Pointer
    if s != nil && s.QTextStream_PTR() != nil {
        convArg0 = s.QTextStream_PTR().GetCthis()
    }
  rv, err := qtrt.InvokeQtFunc6("_Z10forcepointR11QTextStream", qtrt.FFI_TYPE_POINTER, convArg0)
  qtrt.ErrPrint(err, rv)
    rv2 := /*==*/NewQTextStreamFromPointer(unsafe.Pointer(uintptr(rv))) // 4441
    qtrt.SetFinalizer(rv2, /*==*/DeleteQTextStream)
    return rv2
}

//  body block end
